use std::io::Write;

use quick_xml::{
    events::{BytesEnd, BytesStart, BytesText, Event},
    Writer,
};

use crate::{document::Document, tag::TagType};

#[derive(Debug)]
pub enum Error {
    Xml(quick_xml::Error),
}

impl From<quick_xml::Error> for Error {
    fn from(error: quick_xml::Error) -> Self {
        Error::Xml(error)
    }
}

pub(crate) fn serialize_document(doc: &Document, write: &mut impl Write) -> Result<(), Error> {
    let mut writer = Writer::new(write);
    let mut stack = Vec::new();
    let root = doc.root();
    stack.push(root);

    while let Some(node) = stack.pop() {
        match doc.value(node) {
            TagType::Document => {
                // Push children in reverse order so they get processed in correct order
                for child in doc.children(node).collect::<Vec<_>>().into_iter().rev() {
                    stack.push(child);
                }
            }
            TagType::Element(tag_name) => {
                // Write opening tag with any attributes
                let mut elem = BytesStart::new(String::from_utf8_lossy(tag_name.local_name()).into_owned());
                
                // Add attributes if any
                for attr in doc.attributes(node) {
                    if let Some(name) = doc.node_name(attr) {
                        if let Some(value) = doc.text_str(attr) {
                            elem.push_attribute((
                                String::from_utf8_lossy(name.local_name()).into_owned(),
                                value,
                            ));
                        }
                    }
                }
                
                writer.write_event(Event::Start(elem))?;

                // Push end tag marker (None) followed by children in reverse order
                stack.push(node); // Mark for end tag
                for child in doc.children(node).collect::<Vec<_>>().into_iter().rev() {
                    stack.push(child);
                }
            }
            TagType::Text => {
                if let Some(text) = doc.text_str(node) {
                    writer.write_event(Event::Text(BytesText::new(text)))?;
                }
            }
            TagType::Comment => {
                if let Some(text) = doc.text_str(node) {
                    writer.write_event(Event::Comment(BytesText::new(text)))?;
                }
            }
            TagType::ProcessingInstruction => {
                if let Some(text) = doc.text_str(node) {
                    writer.write_event(Event::PI(BytesText::new(text)))?;
                }
            }
            TagType::Element(tag_name) if stack.last() == Some(&node) => {
                // Write end tag
                writer.write_event(Event::End(BytesEnd::new(
                    String::from_utf8_lossy(tag_name.local_name()).into_owned(),
                )))?;
            }
            _ => {} // Skip other node types
        }
    }
    Ok(())
}
