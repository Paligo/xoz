use super::core::{Node, Xoz};

impl Xoz {
    // ns

    /// Get a node which contains the namespace declarations ("xmlns") children of
    /// of this node.
    ///
    /// This node has tag type `TagType::Namespaces`.
    ///
    /// If this is not an element node, or there are no namespace declarations,
    /// it returns `None`.
    pub fn namespaces_child(&self, node: Node) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .namespaces_child(node.document_node)
            .map(|n| document.new_node(n))
    }

    /// Get an iterator over the namespace declarations of this node.
    ///
    /// This iterates over prefix, uri tuples.
    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + '_ {
        let document = self.document(node.document_id);
        document.namespace_entries(node.document_node)
    }

    /// Given a namespace URI, return the prefix for this node
    ///
    /// This walks up the tree to find the first namespace declaration
    /// that has the given URI. If an element declares multiple prefixes for the
    /// same URI then an empty prefix is preferred over non-empty prefix.
    ///
    /// The `xml` prefix always exists. The prefix for the empty namespace is
    /// always empty.
    pub fn prefix_for_namespace(&self, node: Node, uri: &[u8]) -> Option<&[u8]> {
        let document = self.document(node.document_id);
        document.prefix_for_namespace(node.document_node, uri)
    }

    /// Prefix for a node
    ///
    /// Only element and attributes can have prefixes.
    pub fn node_prefix(&self, node: Node) -> Option<&[u8]> {
        let document = self.document(node.document_id);
        document.node_prefix(node.document_node)
    }

    /// Full name for a node.
    ///
    /// This is the prefix and local name concatenated with a colon, if a prefix
    /// exists.
    ///
    /// If the node is not an element or attribute node, this returns `None`.
    pub fn node_full_name(&self, node: Node) -> Option<String> {
        let document = self.document(node.document_id);
        document.node_full_name(node.document_node)
    }
}
