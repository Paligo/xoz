use ahash::{HashMap, HashMapExt};
use std::{collections::hash_map::Entry, io};

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Writer,
};

use crate::{document::Document, tag::TagType, TagName, TagState};

pub(crate) fn serialize_document(doc: &Document, write: &mut impl io::Write) -> io::Result<()> {
    let mut writer = Writer::new(write);
    let mut ns = NamespaceTracker::new();
    let mut full_name_scratch_buf = Vec::with_capacity(64);
    for (tag_type, tag_state, node) in doc.traverse(doc.root()) {
        match tag_type {
            TagType::Document => {
                // TODO serialize declaration if needed on opening
            }
            TagType::Element(name) => {
                if tag_state == TagState::Open {
                    // put namespace prefixes on the tracker
                    // todo!();
                }
                let qname = ns.qname(name, &mut full_name_scratch_buf);
                match tag_state {
                    TagState::Open => {
                        let elem: BytesStart = qname.into();
                        // for attribute in doc.attribute_entries(node) {
                        //     elem.push_attribute()
                        // }
                        writer.write_event(Event::Start(elem))?;
                    }
                    TagState::Close => {
                        let elem: BytesEnd = qname.into();
                        writer.write_event(Event::End(elem))?;
                    }
                    TagState::Empty => {
                        let elem: BytesStart = qname.into();
                        writer.write_event(Event::Empty(elem))?;
                    }
                }
            }
            TagType::Comment => {}
            TagType::ProcessingInstruction => {}
            TagType::Text => {}
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
    use crate::builder::parse_document;

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

    // #[test]
    // fn test_attribute() {
    //     let doc = parse_document(r#"<doc a="1"/>"#).unwrap();
    //     assert_eq!(serialize_document_to_string(&doc), r#"<doc a="1"/>"#);
    // }
}
