use crate::{NodeName, NodeType};

use super::core::{Node, Xoz};

/// ## Information
///
/// Node information such as node type, and name.
impl Xoz {
    /// Preorder number of node
    ///
    /// This can be used to sort nodes by preorder.
    ///
    /// Note that since attributes and namespaces are also nodes in the tree,
    /// as well as the nodes that hold them, the preorder may have gaps.
    pub fn preorder(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.preorder(node.document_node)
    }

    /// Sort key for node.
    ///
    /// This can be used to sort nodes in a stable way: nodes in the
    /// same document sort together, and within the same document
    /// sort in preorder.
    pub fn sort_key(&self, node: Node) -> (usize, usize) {
        let document = self.document(node.document_id);
        (document.id.index(), document.preorder(node.document_node))
    }

    /// Given a node, give back the [`NodeName`] of this node.
    ///
    /// For elements and attribute that is their name, for processing
    /// instructions this is a name based on the target attribute.
    ///
    /// For anything else, it's `None`.
    ///
    /// ```rust
    /// use xoz::{Xoz, NodeName};
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<ex:doc xmlns:ex="http://example.com" ex:b="B"><a/></ex:doc>"#).unwrap();
    /// let doc_el = xoz.document_element(root);
    /// let a_el = xoz.first_child(doc_el).unwrap();
    ///
    /// let doc_name = xoz.node_name(doc_el).unwrap();
    /// assert_eq!(doc_name.local_name(), b"doc");
    /// assert_eq!(doc_name.namespace(), b"http://example.com");
    ///
    /// let a_name = xoz.node_name(a_el).unwrap();
    /// assert_eq!(a_name.local_name(), b"a");
    /// assert_eq!(a_name.namespace(), b"");
    ///
    /// // it also works on attribute nodes
    /// let b_attribute = xoz.attribute_node(doc_el, NodeName::new("http://example.com", "b")).unwrap();
    /// let b_name = xoz.node_name(b_attribute).unwrap();
    /// assert_eq!(b_name.local_name(), b"b");
    /// assert_eq!(b_name.namespace(), b"http://example.com");
    /// ```
    pub fn node_name(&self, node: Node) -> Option<&NodeName> {
        let document = self.document(node.document_id);
        document.node_name(node.document_node)
    }

    /// Get the [`NodeType`] for a node.
    pub fn node_type(&self, node: Node) -> &NodeType {
        let document = self.document(node.document_id);
        document.node_type(node.document_node)
    }

    /// Check whether this node is a document node.
    pub fn is_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document(node.document_node)
    }

    /// Check whether this node is an element node.
    pub fn is_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_element(node.document_node)
    }

    /// Check whether this node is a text node.
    pub fn is_text(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_text(node.document_node)
    }

    /// Check whether this node is a comment node.
    pub fn is_comment(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_comment(node.document_node)
    }

    /// Check whether this node is a processing instruction node.
    pub fn is_processing_instruction(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_processing_instruction(node.document_node)
    }

    /// Check whether this node is an attribute node.
    pub fn is_attribute(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_attribute(node.document_node)
    }

    /// Check whether this node is a namespace node.
    pub fn is_namespace(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_namespace(node.document_node)
    }

    /// Count how many nodes of a certain type are in the subtree of this node.
    pub fn subtree_count(&self, node: Node, node_type: NodeType) -> usize {
        let document = self.document(node.document_id);
        document.subtree_count(node.document_node, node_type)
    }

    /// Count how many nodes there are in a subtree of this node.
    pub fn subtree_size(&self, node: Node) -> usize {
        let document = self.document(node.document_id);
        document.subtree_size(node.document_node)
    }
}
