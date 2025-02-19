use std::io;

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{document::Document, tag::TagType};

pub(crate) fn serialize_document(doc: &Document, write: &mut impl io::Write) -> io::Result<()> {
    let mut writer = Writer::new(write);
    let mut stack = vec![doc.root()];

    while let Some(node) = stack.pop() {
        match doc.value(node) {
            TagType::Document => {
                // Push children in reverse order so they get processed in correct order
                for child in doc.children(node).rev() {
                    stack.push(child);
                }
            }
            TagType::Element(tag_name) => {
                // // Write opening tag
                // let mut elem =
                //     BytesStart::new(String::from_utf8_lossy(tag_name.local_name()).into_owned());

                // writer.write_event(Event::Start(elem))?;

                // // Push end tag marker (None) followed by children in reverse order
                // stack.push(node); // Mark for end tag
                for child in doc.children(node).rev() {
                    stack.push(child);
                }
            }
            _ => {} // Skip other node types
        }
    }
    Ok(())
}
