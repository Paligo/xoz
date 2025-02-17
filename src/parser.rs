use quick_xml::errors::Error as QuickXMLError;
use quick_xml::events::attributes::AttrError as QuickXMLAttrError;
use quick_xml::events::Event;
use quick_xml::name::{PrefixDeclaration, QName, ResolveResult};
use quick_xml::reader::NsReader;
use quick_xml::Reader;

use crate::document::Document;
use crate::tag::TagName;
use crate::tags_builder::TagsBuilder;
use crate::text::TextBuilder;
use crate::TagType;

#[derive(Debug, thiserror::Error)]
enum ParseError {
    #[error("prefix not found during parse: {0}")]
    UnknownPrefix(String),
    #[error("quick-xml attribute error: {0}")]
    QuickXMLAttr(#[from] QuickXMLAttrError),
    #[error("quick-xml error: {0}")]
    QuickXML(#[from] QuickXMLError),
}

fn parse(xml: &str) -> Result<Document, ParseError> {
    let mut reader = NsReader::from_str(xml);
    reader.config_mut().enable_all_checks(true);
    let mut tags_builder = TagsBuilder::new();
    let mut text_builder = TextBuilder::new();
    loop {
        match reader.read_event() {
            Err(e) => return Err(ParseError::QuickXML(e)),
            Ok(event) => match event {
                Event::Start(start) => {
                    let qname = start.name();
                    let (resolved, local_name) = reader.resolve_element(qname);
                    let tag_type = match resolved {
                        ResolveResult::Unbound => TagType::Element(TagName {
                            namespace: "".to_string(),
                            local_name: to_string(local_name),
                        }),
                        ResolveResult::Bound(namespace) => TagType::Element(TagName {
                            namespace: to_string(namespace),
                            local_name: to_string(local_name),
                        }),
                        ResolveResult::Unknown(prefix) => {
                            let prefix = to_string(prefix);
                            return Err(ParseError::UnknownPrefix(prefix));
                        }
                    };

                    tags_builder.open(tag_type);
                    let mut namespaces = Vec::new();
                    let mut attributes = Vec::new();
                    for attribute in start.attributes() {
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
                            let (resolved, local_name) = reader.resolve_attribute(qname);
                            let tag_type = match resolved {
                                ResolveResult::Unbound => TagType::Attribute(TagName {
                                    namespace: "".to_string(),
                                    local_name: to_string(local_name),
                                }),
                                ResolveResult::Bound(namespace) => TagType::Attribute(TagName {
                                    namespace: to_string(namespace),
                                    local_name: to_string(local_name),
                                }),
                                ResolveResult::Unknown(prefix) => {
                                    let prefix = to_string(prefix);
                                    return Err(ParseError::UnknownPrefix(prefix));
                                }
                            };
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
                }
                Event::End(end) => {}
                Event::Empty(empty) => {}
                Event::Text(text) => {}
                Event::CData(text) => {}
                Event::Comment(comment) => {}
                Event::PI(pi) => {}
                Event::Decl(_decl) => {}
                Event::DocType(doctype) => {
                    todo!()
                }
                Event::Eof => {}
            },
        }
    }
    Ok(todo!())
}

// TODO: this is an ugly conversion, it'd be nicer if we just stored the u8 vecs
fn to_string(bytes: impl AsRef<[u8]>) -> String {
    std::str::from_utf8(bytes.as_ref()).unwrap().to_string()
}

fn element_tag_type(reader: &NsReader<&[u8]>, qname: QName) -> Result<TagType, ParseError> {
    let (resolved, local_name) = reader.resolve_element(qname);
    Ok(match resolved {
        ResolveResult::Unbound => TagType::Element(TagName {
            namespace: "".to_string(),
            local_name: to_string(local_name),
        }),
        ResolveResult::Bound(namespace) => TagType::Element(TagName {
            namespace: to_string(namespace),
            local_name: to_string(local_name),
        }),
        ResolveResult::Unknown(prefix) => {
            let prefix = to_string(prefix);
            return Err(ParseError::UnknownPrefix(prefix));
        }
    })
}
