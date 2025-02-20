pub use quick_xml::errors::Error as QuickXMLError;
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event;
use quick_xml::name::{LocalName, NamespaceError, PrefixDeclaration, ResolveResult};
use quick_xml::reader::NsReader;

use crate::document::Document;
use crate::node::NodeName;
use crate::node_info_vec::SArrayMatrix;
use crate::structure::Structure;
use crate::text::TextBuilder;
use crate::tree_builder::TreeBuilder;
use crate::{Namespace, NodeType};

pub fn parse_document(xml: &str) -> Result<Document, QuickXMLError> {
    let mut reader = NsReader::from_str(xml);
    reader.config_mut().enable_all_checks(true);
    let mut tags_builder = TreeBuilder::new();
    let mut text_builder = TextBuilder::new();
    tags_builder.open(NodeType::Document);
    loop {
        match reader.read_event() {
            Err(e) => return Err(e),
            Ok(event) => match event {
                Event::Start(start) => {
                    let qname = start.name();
                    let name = node_name(reader.resolve_element(qname))?;
                    let node_type = NodeType::Element(name);
                    tags_builder.open(node_type);
                    build_element_attributes(
                        &reader,
                        &mut tags_builder,
                        &mut text_builder,
                        start.attributes(),
                    )?;
                }
                Event::End(end) => {
                    let qname = end.name();
                    let name = node_name(reader.resolve_element(qname))?;
                    let node_type = NodeType::Element(name);
                    tags_builder.close(node_type);
                }
                Event::Empty(empty) => {
                    let qname = empty.name();
                    let name = node_name(reader.resolve_element(qname))?;
                    let node_type = NodeType::Element(name);
                    tags_builder.open(node_type.clone());
                    build_element_attributes(
                        &reader,
                        &mut tags_builder,
                        &mut text_builder,
                        empty.attributes(),
                    )?;
                    tags_builder.close(node_type);
                }
                Event::Text(text) => {
                    tags_builder.open(NodeType::Text);
                    text_builder.text_node(&text.unescape()?);
                    tags_builder.close(NodeType::Text);
                }
                Event::CData(text) => {
                    tags_builder.open(NodeType::Text);
                    text_builder.text_node(&text.minimal_escape()?.unescape()?);
                    tags_builder.close(NodeType::Text);
                }
                Event::Comment(comment) => {
                    tags_builder.open(NodeType::Comment);
                    text_builder.text_node(&comment.unescape()?);
                    tags_builder.close(NodeType::Comment);
                }
                Event::PI(pi) => {
                    tags_builder.open(NodeType::ProcessingInstruction);
                    let pi = std::str::from_utf8(&pi).expect("PI is not utf8");
                    text_builder.text_node(pi);
                    tags_builder.close(NodeType::ProcessingInstruction);
                }
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
    tags_builder.close(NodeType::Document);
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
    tags_builder: &mut TreeBuilder,
    text_builder: &mut TextBuilder,
    attributes_iter: Attributes<'_>,
) -> Result<(), QuickXMLError> {
    let mut namespaces = Vec::new();
    let mut attributes = Vec::new();
    for attribute in attributes_iter {
        let attribute = attribute?;
        let qname = attribute.key;
        let value = attribute.decode_and_unescape_value(reader.decoder())?;
        if let Some(prefix_declaration) = qname.as_namespace_binding() {
            let node_type = match prefix_declaration {
                PrefixDeclaration::Default => NodeType::Namespace(Namespace::new("", &*value)),
                PrefixDeclaration::Named(prefix) => {
                    NodeType::Namespace(Namespace::new(prefix, &*value))
                }
            };
            namespaces.push(node_type);
        } else {
            let name = node_name(reader.resolve_attribute(qname))?;
            let node_type = NodeType::Attribute(name);
            attributes.push((node_type, value));
        }
    }
    if !namespaces.is_empty() {
        tags_builder.open(NodeType::Namespaces);
        for namespace in namespaces {
            tags_builder.open(namespace.clone());
            tags_builder.close(namespace);
        }
        tags_builder.close(NodeType::Namespaces);
    }
    if !attributes.is_empty() {
        tags_builder.open(NodeType::Attributes);
        for (node_type, value) in attributes {
            tags_builder.open(node_type.clone());
            text_builder.text_node(&value);
            tags_builder.close(node_type);
        }
        tags_builder.close(NodeType::Attributes);
    }
    Ok(())
}

fn node_name<'a>(r: (ResolveResult<'a>, LocalName<'a>)) -> Result<NodeName<'a>, QuickXMLError> {
    let (resolved, local_name) = r;
    Ok(match resolved {
        ResolveResult::Unbound => NodeName::from_u8(b"", local_name.into_inner()),
        ResolveResult::Bound(namespace) => {
            NodeName::from_u8(namespace.into_inner(), local_name.into_inner())
        }
        ResolveResult::Unknown(prefix) => {
            return Err(QuickXMLError::Namespace(NamespaceError::UnknownPrefix(
                prefix,
            )));
        }
    })
}
