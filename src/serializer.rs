use ahash::{HashMap, HashMapExt};
use std::{collections::hash_map::Entry, io};

use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Writer,
};

use crate::{document::Document, tag::TagType, TagName, TagState};

struct Serializer<'a, W: io::Write> {
    doc: &'a Document,
    writer: Writer<W>,
    ns: NamespaceTracker<'a>,
}

impl<'a, W: io::Write> Serializer<'a, W> {
    fn new(doc: &'a Document, write: W) -> Self {
        Self {
            doc,
            writer: Writer::new(write),
            ns: NamespaceTracker::new(),
        }
    }

    fn serialize_document(&mut self) -> io::Result<()> {
        let mut element_name_scratch_buf = Vec::with_capacity(64);
        let mut attribute_name_scratch_buf = Vec::with_capacity(64);

        for (tag_type, tag_state, node) in self.doc.traverse(self.doc.root()) {
            match tag_type {
                TagType::Document => {
                    // TODO serialize declaration if needed on opening
                }
                TagType::Element(name) => {
                    if matches!(tag_state, TagState::Open | TagState::Empty) {
                        self.ns.push_scope();
                        // for (prefix, uri) in self.doc.namespace_entries(node) {
                        //     self.ns.add_namespace(prefix, uri);
                        // }
                    }

                    let qname = self.ns.qname(name, &mut element_name_scratch_buf);
                    match tag_state {
                        TagState::Open => {
                            let elem =
                                self.create_elem(qname, node, &mut attribute_name_scratch_buf);
                            self.writer.write_event(Event::Start(elem))?;
                        }
                        TagState::Close => {
                            let elem: BytesEnd = qname.into();
                            self.writer.write_event(Event::End(elem))?;
                            self.ns.pop_scope();
                        }
                        TagState::Empty => {
                            let elem =
                                self.create_elem(qname, node, &mut attribute_name_scratch_buf);
                            self.writer.write_event(Event::Empty(elem))?;
                            self.ns.pop_scope();
                        }
                    }
                }
                TagType::Comment => {}
                TagType::ProcessingInstruction => {}
                TagType::Text => {
                    let text = self.doc.text_str(node).expect("Must be text node");
                    self.writer.write_event(Event::Text(BytesText::new(text)))?;
                }
                TagType::Attributes
                | TagType::Namespaces
                | TagType::Attribute(_)
                | TagType::Namespace(_) => {
                    unreachable!("We cannot reach these tag types during traverse");
                }
            }
        }

        Ok(())
    }

    fn create_elem(
        &self,
        qname: QName<'a>,
        node: crate::document::Node,
        attribute_name_scratch_buf: &mut Vec<u8>,
    ) -> BytesStart<'a> {
        let mut elem: BytesStart = qname.into();
        // TODO: duplicate code
        for (name, value) in self.doc.attribute_entries(node) {
            elem.push_attribute(Attribute {
                key: self.ns.qname(name, attribute_name_scratch_buf),
                value: value.as_bytes().into(),
            })
        }
        elem
    }
}

pub(crate) fn serialize_document(doc: &Document, write: &mut impl io::Write) -> io::Result<()> {
    let mut serializer = Serializer::new(doc, write);
    serializer.serialize_document()
}

pub(crate) fn serialize_document_to_string(doc: &Document) -> String {
    let mut w = Vec::new();
    serialize_document(doc, &mut w).unwrap();
    String::from_utf8(w).unwrap()
}

#[derive(Default)]
struct NamespaceTracker<'a> {
    // Stack of namespace mappings, each level contains prefix->namespace mappings
    stack: Vec<HashMap<&'a [u8], &'a [u8]>>,
}

impl<'a> NamespaceTracker<'a> {
    fn new() -> Self {
        Self {
            stack: vec![HashMap::new()], // Start with empty root scope
        }
    }

    // Push a new namespace scope
    fn push_scope(&mut self) {
        self.stack.push(HashMap::new());
    }

    // Pop the current namespace scope
    fn pop_scope(&mut self) {
        self.stack.pop();
    }

    // Add a prefix->namespace mapping to current scope
    fn add_namespace(&mut self, prefix: &'a [u8], namespace: &'a [u8]) {
        if let Some(current) = self.stack.last_mut() {
            let entry = current.entry(namespace);
            match entry {
                Entry::Occupied(mut entry) => {
                    // if there is already an empty prefix for this namespace,
                    // don't override
                    if entry.get().is_empty() {
                        return;
                    }
                    entry.insert(prefix);
                }
                Entry::Vacant(entry) => {
                    entry.insert(prefix);
                }
            }
        }
    }

    // Look up prefix for uri, checking all scopes from current to root
    fn get_prefix(&self, namespace: &[u8]) -> &[u8] {
        for scope in self.stack.iter().rev() {
            if let Some(ns) = scope.get(namespace) {
                return ns;
            }
        }
        unreachable!()
    }

    fn qname(&self, name: &'a TagName<'a>, scratch_buf: &'a mut Vec<u8>) -> QName<'a> {
        if name.namespace().is_empty() {
            QName(name.local_name())
        } else {
            let prefix = self.get_prefix(name.namespace());
            if prefix.is_empty() {
                QName(name.local_name())
            } else {
                scratch_buf.clear();
                scratch_buf.extend(prefix);
                scratch_buf.push(b':');
                scratch_buf.extend(name.local_name());
                QName(scratch_buf)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_document;

    use super::*;

    #[test]
    fn test_one_element() {
        let doc = parse_document("<doc/>").unwrap();
        assert_eq!(serialize_document_to_string(&doc), "<doc/>");
    }

    #[test]
    fn test_nested_elements() {
        let doc = parse_document("<doc><a/><b/></doc>").unwrap();
        assert_eq!(serialize_document_to_string(&doc), "<doc><a/><b/></doc>");
    }

    #[test]
    fn test_attribute() {
        let doc = parse_document(r#"<doc a="1"/>"#).unwrap();
        assert_eq!(serialize_document_to_string(&doc), r#"<doc a="1"/>"#);
    }

    #[test]
    fn test_attributes() {
        let doc = parse_document(r#"<doc a="1" b="2"/>"#).unwrap();
        assert_eq!(serialize_document_to_string(&doc), r#"<doc a="1" b="2"/>"#);
    }

    #[test]
    fn test_text() {
        let doc = parse_document("<doc>text</doc>").unwrap();
        assert_eq!(serialize_document_to_string(&doc), "<doc>text</doc>");
    }

    // #[test]
    // fn test_explicit_prefix() {
    //     let doc = parse_document(r#"<doc xmlns:ns="http://example.com"/>"#).unwrap();
    //     assert_eq!(
    //         serialize_document_to_string(&doc),
    //         r#"<doc xmlns:ns="http://example.com"/>"#
    //     );
    // }
}
