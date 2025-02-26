use crate::NodeName;

use super::core::{Node, Xoz};

/// ## Attribute
///
/// Various functions for accessing attributes.
impl Xoz {
    /// Get the attribute node with the given name.
    ///
    /// If this is not an element node, or there is no attribute with the given name,
    /// it returns `None`.
    ///
    /// Note that [`Xoz::attribute_value`] can be used to access the attribute
    /// value directly.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.attribute_node(p, "a").unwrap();
    /// let value = xoz.string_value(a);
    /// assert_eq!(value, "1");
    /// ```
    pub fn attribute_node<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attribute_node(node.document_node, name)
            .map(|n| document.new_node(n))
    }

    /// Get a node which contains the attributes children of this node.
    ///
    /// This node has tag type `TagType::Attributes`.
    ///
    /// If this is not an element node or there are no attributes, it returns `None`.
    pub fn attributes_child(&self, node: Node) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .attributes_child(node.document_node)
            .map(|n| document.new_node(n))
    }

    /// Get the value of the attribute with the given name.
    ///
    /// If this is not an element node, or there is no attribute with the given name,
    /// it returns `None`.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let value = xoz.attribute_value(p, "a").unwrap();
    /// assert_eq!(value, "1");
    /// ```
    pub fn attribute_value<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<&str> {
        let document = self.document(node.document_id);
        document.attribute_value(node.document_node, name)
    }

    /// Get an iterator over the name and value of all attributes of this node.
    ///
    /// If this is not an element node, it returns an empty iterator.
    pub fn attribute_entries(&self, node: Node) -> impl Iterator<Item = (&NodeName, &str)> {
        let document = self.document(node.document_id);
        document.attribute_entries(node.document_node)
    }
}
