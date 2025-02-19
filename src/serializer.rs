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

// The NamespaceTracker allows one to push a collection of prefix, namespace
// combinations onto it, and pop that collection as a whole too.
// It's possible to look up the namespace for a given prefix quickly.
// Please make it work AI!
struct NamespaceTracker {}
