use crate::{
    document::Document,
    error::Error,
    structure::Structure,
    node::{NodeName, NodeType},
    tags_builder::TagsBuilder,
    tagvec::SArrayMatrix,
    text::TextBuilder,
    Namespace,
};

pub fn parse_document(xml: &str) -> Result<Document, xot::ParseError> {
    // TODO: for now go through Xot to parse a new XML document
    let mut xot = xot::Xot::new();
    let doc = xot.parse(xml)?;
    Ok(from_xot_node(&xot, doc).unwrap())
}

/// Given a document node, construct a new Xoz document.
fn from_xot_node(xot: &xot::Xot, node: xot::Node) -> Result<Document, Error> {
    assert!(xot.is_document(node));
    let mut tags_builder = TagsBuilder::new();
    let mut text_builder = TextBuilder::new();
    for edge in xot.traverse(node) {
        match edge {
            xot::NodeEdge::Start(node) => {
                match xot.value(node) {
                    xot::Value::Document => tags_builder.open(NodeType::Document),
                    xot::Value::Namespace(_namespace) => {
                        unreachable!("Unreachable in traverse()");
                    }
                    xot::Value::Attribute(_attribute) => {
                        unreachable!("Unreachable in traverse()");
                    }
                    xot::Value::Element(element) => {
                        tags_builder.open(element_tag_type(element, xot));
                        let namespaces = xot.namespaces(node);
                        if !namespaces.is_empty() {
                            tags_builder.open(NodeType::Namespaces);
                            for (prefix_id, namespace_id) in namespaces.iter() {
                                let prefix = xot.prefix_str(prefix_id);
                                let uri = xot.namespace_str(*namespace_id);
                                let t = NodeType::Namespace(Namespace::new(prefix, uri));
                                tags_builder.open(t.clone());
                                tags_builder.close(t);
                            }
                            tags_builder.close(NodeType::Namespaces);
                        }
                        let attributes = xot.attributes(node);
                        if !attributes.is_empty() {
                            tags_builder.open(NodeType::Attributes);
                            for (name_id, value) in attributes.iter() {
                                let (local_name, namespace) = xot.name_ns_str(name_id);
                                let t = NodeType::Attribute(NodeName::new(namespace, local_name));
                                tags_builder.open(t.clone());
                                text_builder.text_node(value);
                                tags_builder.close(t);
                            }
                            tags_builder.close(NodeType::Attributes);
                        }
                    }
                    xot::Value::Text(text) => {
                        tags_builder.open(NodeType::Text);
                        text_builder.text_node(text.get());
                    }
                    xot::Value::Comment(comment) => {
                        tags_builder.open(NodeType::Comment);
                        // additional work to represent text content
                        // TODO: but comments are not supposed to be searchable, so
                        // it shouldn't be added as a text node to the text index
                    }
                    xot::Value::ProcessingInstruction(pi) => {
                        todo!("Cannot represent processing instruction yet");
                    }
                };
            }
            xot::NodeEdge::End(node) => {
                let tag_type = match xot.value(node) {
                    xot::Value::Document => NodeType::Document,
                    xot::Value::Namespace(_namespace) => unreachable!(),
                    xot::Value::Attribute(_attribute) => unreachable!(),
                    xot::Value::Element(element) => element_tag_type(element, xot),
                    xot::Value::Text(_text) => NodeType::Text,
                    xot::Value::Comment(_comment) => NodeType::Comment,
                    xot::Value::ProcessingInstruction(pi) => {
                        todo!();
                    }
                };
                tags_builder.close(tag_type);
            }
        }
    }
    let structure = Structure::new(tags_builder, |tags_builder| {
        SArrayMatrix::new(tags_builder.usage(), tags_builder.tags_amount())
    })?;
    let text_usage = text_builder.build();
    Ok(Document {
        structure,
        text_usage,
    })
}

fn element_tag_type<'a>(element: &'a xot::Element, xot: &'a xot::Xot) -> NodeType<'a> {
    let (local_name, namespace) = xot.name_ns_str(element.name());
    NodeType::Element(NodeName::new(namespace, local_name))
}
