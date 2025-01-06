use crate::{
    data::{TagType, TagsBuilder},
    text::TextBuilder,
};

/// Given a document node, construct a new Xoz document.
fn from_xot_node(xot: &xot::Xot, node: xot::Node) {
    assert!(xot.is_document(node));
    let mut tags_builder = TagsBuilder::new();
    let mut text_builder = TextBuilder::new();
    for edge in xot.traverse(node) {
        match edge {
            xot::NodeEdge::Start(node) => {
                match xot.value(node) {
                    xot::Value::Document => tags_builder.open(TagType::Document),
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
                            tags_builder.open(TagType::Namespaces);
                            for (prefix_id, namespace_id) in namespaces.iter() {
                                let prefix = xot.prefix_str(prefix_id);
                                let uri = xot.namespace_str(*namespace_id);
                                let t = TagType::Namespace {
                                    prefix: prefix.to_string(),
                                    uri: uri.to_string(),
                                };
                                tags_builder.open(t.clone());
                                tags_builder.close(t);
                            }
                            tags_builder.close(TagType::Namespaces);
                        }
                        let attributes = xot.attributes(node);
                        if !attributes.is_empty() {
                            tags_builder.open(TagType::Attributes);
                            for (name_id, value) in attributes.iter() {
                                let (name, uri) = xot.name_ns_str(name_id);
                                let t = TagType::Attribute {
                                    namespace: name.to_string(),
                                    name: uri.to_string(),
                                };
                                tags_builder.open(t.clone());
                                tags_builder.open(TagType::Content);
                                text_builder.text_node(value);
                                tags_builder.close(TagType::Content);
                                tags_builder.close(t);
                            }
                        }
                    }
                    xot::Value::Text(text) => {
                        tags_builder.open(TagType::Text);
                        tags_builder.open(TagType::Content);
                        text_builder.text_node(text.get());
                        tags_builder.close(TagType::Content);
                    }
                    xot::Value::Comment(comment) => {
                        tags_builder.open(TagType::Comment);
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
                    xot::Value::Document => TagType::Document,
                    xot::Value::Namespace(_namespace) => unreachable!(),
                    xot::Value::Attribute(_attribute) => unreachable!(),
                    xot::Value::Element(element) => element_tag_type(element, xot),
                    xot::Value::Text(_text) => TagType::Text,
                    xot::Value::Comment(_comment) => TagType::Comment,
                    xot::Value::ProcessingInstruction(pi) => {
                        todo!();
                    }
                };
                tags_builder.close(tag_type);
            }
        }
    }
}

fn element_tag_type(element: &xot::Element, xot: &xot::Xot) -> TagType {
    let (name, uri) = xot.name_ns_str(element.name());
    TagType::Element {
        namespace: name.to_string(),
        name: uri.to_string(),
    }
}
