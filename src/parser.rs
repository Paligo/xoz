use quick_xml::encoding::EncodingError as QuickXMLEncodingError;
use quick_xml::errors::Error as QuickXMLError;
use quick_xml::events::attributes::{AttrError as QuickXMLAttrError, Attributes};
use quick_xml::events::Event;
use quick_xml::name::{LocalName, PrefixDeclaration, ResolveResult};
use quick_xml::reader::NsReader;

use crate::document::Document;
use crate::structure::Structure;
use crate::tag::TagName;
use crate::tags_builder::TagsBuilder;
use crate::tagvec::SArrayMatrix;
use crate::text::TextBuilder;
use crate::TagType;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("prefix not found during parse: {0}")]
    UnknownPrefix(String),
    #[error("quick-xml attribute error: {0}")]
    QuickXMLAttr(#[from] QuickXMLAttrError),
    #[error("quick-xml encoding error: {0}")]
    QuickXMLEncoding(#[from] QuickXMLEncodingError),
    #[error("quick-xml error: {0}")]
    QuickXML(#[from] QuickXMLError),
}

pub fn parse_document(xml: &str) -> Result<Document, ParseError> {
    let mut reader = NsReader::from_str(xml);
    reader.config_mut().enable_all_checks(true);
    let mut tags_builder = TagsBuilder::new();
    let mut text_builder = TextBuilder::new();
    tags_builder.open(TagType::Document);
    loop {
        match reader.read_event() {
            Err(e) => return Err(ParseError::QuickXML(e)),
            Ok(event) => match event {
                Event::Start(start) => {
                    let qname = start.name();
                    let name = tag_name(reader.resolve_element(qname))?;
                    let tag_type = TagType::Element(name);
                    tags_builder.open(tag_type);
                    build_element_attributes(
                        &reader,
                        &mut tags_builder,
                        &mut text_builder,
                        start.attributes(),
                    )?;
                }
                Event::End(end) => {
                    let qname = end.name();
                    let name = tag_name(reader.resolve_element(qname))?;
                    let tag_type = TagType::Element(name);
                    tags_builder.close(tag_type);
                }
                Event::Empty(empty) => {
                    let qname = empty.name();
                    let name = tag_name(reader.resolve_element(qname))?;
                    let tag_type = TagType::Element(name);
                    tags_builder.open(tag_type.clone());
                    build_element_attributes(
                        &reader,
                        &mut tags_builder,
                        &mut text_builder,
                        empty.attributes(),
                    )?;
                    tags_builder.close(tag_type);
                }
                Event::Text(text) => {
                    tags_builder.open(TagType::Text);
                    text_builder.text_node(&text.unescape()?);
                    tags_builder.close(TagType::Text);
                }
                Event::CData(text) => {
                    tags_builder.open(TagType::Text);
                    text_builder.text_node(&text.minimal_escape()?.unescape()?);
                    tags_builder.close(TagType::Text);
                }
                Event::Comment(comment) => {}
                Event::PI(pi) => {}
                Event::Decl(_decl) => {}
                Event::DocType(doctype) => {
                    todo!()
                }
                Event::Eof => {
                    // quick-xml seems to check unmatched stuff
                    break;
                }
            },
        }
    }
    tags_builder.close(TagType::Document);
    // TODO: an unwrap here is not great
    let structure = Structure::new(tags_builder, |tags_builder| {
        SArrayMatrix::new(tags_builder.usage(), tags_builder.tags_amount())
    })
    .unwrap();
    let text_usage = text_builder.build();
    Ok(Document {
        structure,
        text_usage,
    })
}

fn build_element_attributes(
    reader: &NsReader<&[u8]>,
    tags_builder: &mut TagsBuilder,
    text_builder: &mut TextBuilder,
    attributes_iter: Attributes<'_>,
) -> Result<(), ParseError> {
    let mut namespaces = Vec::new();
    let mut attributes = Vec::new();
    for attribute in attributes_iter {
        let attribute = attribute?;
        let qname = attribute.key;
        let value = attribute.decode_and_unescape_value(reader.decoder())?;
        if let Some(prefix_declaration) = qname.as_namespace_binding() {
            let tag_type = match prefix_declaration {
                PrefixDeclaration::Default => TagType::Namespace {
                    prefix: "".to_string(),
                    uri: value.to_string(),
                },
                PrefixDeclaration::Named(prefix) => TagType::Namespace {
                    prefix: to_string(prefix),
                    uri: value.to_string(),
                },
            };
            namespaces.push(tag_type);
        } else {
            let name = tag_name(reader.resolve_attribute(qname))?;
            let tag_type = TagType::Attribute(name);
            attributes.push((tag_type, value));
        }
    }
    if !namespaces.is_empty() {
        tags_builder.open(TagType::Namespaces);
        for namespace in namespaces {
            tags_builder.open(namespace.clone());
            tags_builder.close(namespace);
        }
        tags_builder.close(TagType::Namespaces);
    }
    if !attributes.is_empty() {
        tags_builder.open(TagType::Attributes);
        for (tag_type, value) in attributes {
            tags_builder.open(tag_type.clone());
            text_builder.text_node(&value);
            tags_builder.close(tag_type);
        }
        tags_builder.close(TagType::Attributes);
    }
    Ok(())
}

// TODO: this is an ugly conversion, it'd be nicer if we just stored the u8 vecs
fn to_string(bytes: impl AsRef<[u8]>) -> String {
    std::str::from_utf8(bytes.as_ref()).unwrap().to_string()
}

fn tag_name(r: (ResolveResult, LocalName)) -> Result<TagName, ParseError> {
    let (resolved, local_name) = r;
    Ok(match resolved {
        ResolveResult::Unbound => TagName {
            namespace: "".to_string(),
            local_name: to_string(local_name),
        },
        ResolveResult::Bound(namespace) => TagName {
            namespace: to_string(namespace),
            local_name: to_string(local_name),
        },
        ResolveResult::Unknown(prefix) => {
            let prefix = to_string(prefix);
            return Err(ParseError::UnknownPrefix(prefix));
        }
    })
}
