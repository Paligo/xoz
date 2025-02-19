use std::io;

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{document::Document, tag::TagType, TagState};

pub(crate) fn serialize_document(doc: &Document, write: &mut impl io::Write) -> io::Result<()> {
    let mut writer = Writer::new(write);
    for (tag_type, tag_state, node) in doc.traverse(doc.root()) {
        match tag_type {
            TagType::Document => {
                // TODO serialize declaration if needed on opening
            }
            TagType::Element(name) => match tag_state {
                TagState::Open => {
                    let elem = BytesStart::new(name.full_name());
                    writer.write_event(Event::Start(elem));
                }
                TagState::Close => {
                    let elem = BytesEnd::new(name.full_name());
                    writer.write_event(Event::End(elem));
                }
                TagState::Empty => {
                    let elem = BytesStart::new(name.full_name());
                    writer.write_event(Event::Empty(elem));
                }
            },
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

use std::collections::HashMap;

#[derive(Default)]
struct NamespaceTracker {
    // Stack of namespace mappings, each level contains prefix->namespace mappings
    stack: Vec<HashMap<Vec<u8>, Vec<u8>>>,
}

impl NamespaceTracker {
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
        if self.stack.len() > 1 {
            self.stack.pop();
        }
    }

    // Add a prefix->namespace mapping to current scope
    fn add_namespace(&mut self, prefix: &[u8], namespace: &[u8]) {
        if let Some(current) = self.stack.last_mut() {
            current.insert(prefix.to_vec(), namespace.to_vec());
        }
    }

    // Look up namespace for prefix, checking all scopes from current to root
    fn get_namespace(&self, prefix: &[u8]) -> Option<&[u8]> {
        for scope in self.stack.iter().rev() {
            if let Some(ns) = scope.get(prefix) {
                return Some(ns);
            }
        }
        None
    }
}
