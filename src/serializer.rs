use ahash::{HashMap, HashMapExt};
use std::{collections::hash_map::Entry, io};

use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesPI, BytesStart, BytesText, Event},
    name::QName,
    Writer,
};

use crate::{document::Document, tag::NodeType, NodeName, TagState};

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
        let mut xmlns_scratch_buf = Vec::with_capacity(64);
        let mut attribute_name_scratch_buf = Vec::with_capacity(64);

        for (tag_type, tag_state, node) in self.doc.traverse(self.doc.root()) {
            match tag_type {
                NodeType::Document => {
                    // TODO serialize declaration if needed on opening
                }
                NodeType::Element(name) => {
                    if matches!(tag_state, TagState::Open | TagState::Empty) {
                        self.ns.push_scope();
                        for (prefix, uri) in self.doc.namespace_entries(node) {
                            self.ns.add_namespace(prefix, uri);
                        }
                    }

                    let qname = self.ns.qname(name, &mut element_name_scratch_buf);
                    match tag_state {
                        TagState::Open => {
                            let elem = self.create_elem(
                                qname,
                                node,
                                &mut xmlns_scratch_buf,
                                &mut attribute_name_scratch_buf,
                            );
                            self.writer.write_event(Event::Start(elem))?;
                        }
                        TagState::Close => {
                            let elem: BytesEnd = qname.into();
                            self.writer.write_event(Event::End(elem))?;
                            self.ns.pop_scope();
                        }
                        TagState::Empty => {
                            let elem = self.create_elem(
                                qname,
                                node,
                                &mut xmlns_scratch_buf,
                                &mut attribute_name_scratch_buf,
                            );
                            self.writer.write_event(Event::Empty(elem))?;
                            self.ns.pop_scope();
                        }
                    }
                }
                NodeType::Comment => {
                    let text = self.doc.comment_str(node).expect("Must be comment node");
                    self.writer
                        .write_event(Event::Comment(BytesText::new(text)))?;
                }
                NodeType::ProcessingInstruction => {
                    let text = self
                        .doc
                        .processing_instruction_str(node)
                        .expect("Must be PI node");
                    self.writer.write_event(Event::PI(BytesPI::new(text)))?;
                }
                NodeType::Text => {
                    let text = self.doc.text_str(node).expect("Must be text node");
                    self.writer.write_event(Event::Text(BytesText::new(text)))?;
                }
                NodeType::Attributes
                | NodeType::Namespaces
                | NodeType::Attribute(_)
                | NodeType::Namespace(_) => {
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
        xmlns_scratch_buf: &mut Vec<u8>,
        attribute_name_scratch_buf: &mut Vec<u8>,
    ) -> BytesStart<'a> {
        let mut elem: BytesStart = qname.into();

        for (prefix, uri) in self.doc.namespace_entries(node) {
            let key = if prefix.is_empty() {
                QName(b"xmlns")
            } else {
                xmlns_scratch_buf.clear();
                xmlns_scratch_buf.extend(b"xmlns:");
                xmlns_scratch_buf.extend(prefix);
                QName(xmlns_scratch_buf)
            };
            elem.push_attribute(Attribute {
                key,
                value: uri.into(),
            });
        }

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
    // TODO: could this be faster with a plain vec instead of a hashmap?
    // but perhaps that requires interning the strings

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

    fn qname(&self, name: &'a NodeName<'a>, scratch_buf: &'a mut Vec<u8>) -> QName<'a> {
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

    #[test]
    fn test_explicit_prefix() {
        let doc = parse_document(r#"<doc xmlns:ns="http://example.com"/>"#).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns:ns="http://example.com"/>"#
        );
    }

    #[test]
    fn test_default_ns() {
        let doc = parse_document(r#"<doc xmlns="http://example.com"/>"#).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns="http://example.com"/>"#
        );
    }

    #[test]
    fn test_prefixed_el_empty() {
        let doc = parse_document(r#"<prefix:doc xmlns:prefix="http://example.com"/>"#).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<prefix:doc xmlns:prefix="http://example.com"/>"#
        );
    }

    #[test]
    fn test_prefixed_el_open_close() {
        let doc =
            parse_document(r#"<prefix:doc xmlns:prefix="http://example.com">text</prefix:doc>"#)
                .unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<prefix:doc xmlns:prefix="http://example.com">text</prefix:doc>"#
        );
    }

    #[test]
    fn test_prefix_override() {
        let doc = parse_document(
            r#"<doc xmlns:p="http://example.com"><a><p:b xmlns:p="http://example.com/2" /></a></doc>"#,
        ).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns:p="http://example.com"><a><p:b xmlns:p="http://example.com/2"/></a></doc>"#
        );
    }

    #[test]
    fn test_prefer_default() {
        let doc = parse_document(
            r#"<doc xmlns="http://example.com" xmlns:prefix="http://example.com"/>"#,
        )
        .unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns="http://example.com" xmlns:prefix="http://example.com"/>"#
        );
    }

    #[test]
    fn test_prefer_default2() {
        let doc = parse_document(
            r#"<doc xmlns:prefix="http://example.com" xmlns="http://example.com"/>"#,
        )
        .unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns:prefix="http://example.com" xmlns="http://example.com"/>"#
        );
    }

    #[test]
    fn test_prefer_default3() {
        let doc = parse_document(
            r#"<prefix:doc xmlns="http://example.com" xmlns:prefix="http://example.com"/>"#,
        )
        .unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc xmlns="http://example.com" xmlns:prefix="http://example.com"/>"#
        );
    }

    #[test]
    fn test_comment() {
        let doc = parse_document(r#"<doc><!-- comment --></doc>"#).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc><!-- comment --></doc>"#
        );
    }

    #[test]
    fn test_pi() {
        let doc = parse_document(r#"<doc><?pi data?></doc>"#).unwrap();
        assert_eq!(
            serialize_document_to_string(&doc),
            r#"<doc><?pi data?></doc>"#
        );
    }
}
