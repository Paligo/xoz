use vers_vecs::trees::Tree;

use crate::{TagName, TagType};

use super::{Document, Node};

impl Document {
    /// Get a node which contains the attributes children of this node.
    ///
    /// This node has tag type `TagType::Attributes`.
    ///
    /// If this is not an element node or there are no attributes, it returns `None`.
    pub(crate) fn attributes_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            let tag_id = self.tag_id(node);
            if tag_id.is_attributes() {
                // the first child is the attributes node
                Some(node)
            } else if tag_id.is_namespaces() {
                // the first child is the namespaces node, check for attributes node
                let next = self.next_sibling(node);
                next.filter(|next| self.tag_id(*next).is_attributes())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get the attribute node with the given name.
    ///
    /// If this is not an element node, or there is no attribute with the given name,
    /// it returns `None`.
    ///
    /// Note that [`Document::attribute_value`] can be used to access the attribute
    /// value directly.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = doc.document_element();
    /// let a = doc.attribute_node(p, "a").unwrap();
    /// let value = doc.string_value(a);
    /// assert_eq!(value, "1");
    /// ```
    pub fn attribute_node<'a>(&self, node: Node, name: impl Into<TagName<'a>>) -> Option<Node> {
        let attributes = self.attributes_child(node)?;
        let name = name.into();
        for child in self.primitive_children(attributes) {
            if let TagType::Attribute(tag_name) = self.value(child) {
                if tag_name == &name {
                    return Some(child);
                }
            }
        }
        None
    }
}
