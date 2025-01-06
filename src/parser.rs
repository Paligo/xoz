use crate::data::{TagType, TagsBuilder};

/// Given a document node, construct a new Xoz document.
fn from_xot_node(xot: &xot::Xot, node: xot::Node) {
    assert!(xot.is_document(node));
    let mut usage = TagsBuilder::new();
    for edge in xot.all_traverse(node) {
        match edge {
            xot::NodeEdge::Start(node) => {
                match xot.value(node) {
                    xot::Value::Document => usage.open(TagType::Document),
                    xot::Value::Namespace(namespace) => {
                        usage.open(namespace_tag_type(namespace, xot))
                    }
                    xot::Value::Attribute(attribute) => {
                        usage.open(attribute_tag_type(attribute, xot));
                        // TODO: additional work to represent text content
                    }
                    xot::Value::Element(element) => usage.open(element_tag_type(element, xot)),
                    xot::Value::Text(text) => {
                        usage.open(TagType::Text);
                        // TODO: additional work to represent text content
                    }
                    xot::Value::Comment(comment) => {
                        usage.open(TagType::Comment);
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
                    xot::Value::Namespace(namespace) => namespace_tag_type(namespace, xot),
                    xot::Value::Attribute(attribute) => attribute_tag_type(attribute, xot),
                    xot::Value::Element(element) => element_tag_type(element, xot),
                    xot::Value::Text(_text) => TagType::Text,
                    xot::Value::Comment(_comment) => TagType::Comment,
                    xot::Value::ProcessingInstruction(pi) => {
                        todo!();
                    }
                };
                usage.close(tag_type);
            }
        }
    }
}

fn namespace_tag_type(namespace: &xot::Namespace, xot: &xot::Xot) -> TagType {
    let prefix = xot.prefix_str(namespace.prefix());
    let uri = xot.namespace_str(namespace.namespace());
    TagType::Namespace {
        prefix: prefix.to_string(),
        uri: uri.to_string(),
    }
}

fn attribute_tag_type(attribute: &xot::Attribute, xot: &xot::Xot) -> TagType {
    let (name, uri) = xot.name_ns_str(attribute.name());
    TagType::AttributeName {
        namespace: name.to_string(),
        name: uri.to_string(),
    }
}

fn element_tag_type(element: &xot::Element, xot: &xot::Xot) -> TagType {
    let (name, uri) = xot.name_ns_str(element.name());
    TagType::Element {
        namespace: name.to_string(),
        name: uri.to_string(),
    }
}
