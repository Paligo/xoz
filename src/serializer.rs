use std::io::Write;

use quick_xml::{events::Event, Writer};

use crate::{document::Document, TagType};

enum Error {}

pub(crate) fn serialize_document(doc: &Document, write: &mut impl Write) -> Result<(), Error> {
    let mut writer = Writer::new(write);
    // Please write a stack-based function that uses the quick_xml writer
    // events to serialize an XML document frrom the given Document, AI!
    todo!();
}
