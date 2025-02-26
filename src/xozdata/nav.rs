use crate::NodeType;

use super::core::{Node, Xoz};

/// ## Navigation
///
/// Core navigation functions for navigating the tree.
impl Xoz {
    /// Obtain the document element.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    ///
    /// let doc_el = xoz.document_element(root);
    ///
    /// assert!(xoz.is_element(doc_el));
    /// assert_eq!(xoz.parent(doc_el), Some(root));
    /// ```
    pub fn document_element(&self, root: Node) -> Node {
        let document = self.document(root.document_id);
        document.new_node(document.document_element())
    }

    /// Get parent node.
    ///
    /// Returns [`None`] if this is the document node or if the node is
    /// unattached to a document.
    ///
    /// Attribute and namespace nodes have a parent, even though they aren't
    /// children of the element they are in.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let text = xoz.first_child(p).unwrap();
    ///
    /// assert_eq!(xoz.parent(text), Some(p));
    /// assert_eq!(xoz.parent(p), Some(root));
    /// assert_eq!(xoz.parent(root), None);
    /// ```
    pub fn parent(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.parent(n))
    }

    /// Get first child.
    ///
    /// Returns [`None`] if there are no children.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p>Example</p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let text = xoz.first_child(p).unwrap();
    /// assert_eq!(xoz.first_child(root), Some(p));
    /// assert_eq!(xoz.first_child(p), Some(text));
    /// assert_eq!(xoz.first_child(text), None);
    /// ```
    pub fn first_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.first_child(n))
    }

    /// Get last child.
    ///
    /// Returns [`None`] if there are no children.
    pub fn last_child(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.last_child(n))
    }

    /// Get next sibling.
    ///
    /// Returns [`None`] if there is no next sibling.
    ///
    /// For normal child nodes, gives the next child.
    ///
    /// For namespace and attribute nodes, gives the next namespace or
    /// attribute in definition order.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// assert_eq!(xoz.next_sibling(b), None);
    /// ```
    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.next_sibling(n))
    }

    /// Get previous sibling.
    ///
    /// Returns [`None`] if there is no previous sibling.
    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        self.wrap_option(node, |doc, n| doc.previous_sibling(n))
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// The ancestor node is not considered a descendant of itself.
    pub fn is_ancestor(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor(ancestor.document_node, descendant.document_node)
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// A node is considered a descendant of itself.
    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        let ancestor_document_id = ancestor.document_id;
        let descendant_document_id = descendant.document_id;
        if ancestor_document_id != descendant_document_id {
            return false;
        }
        let document = self.document(ancestor_document_id);
        document.is_ancestor_or_self(ancestor.document_node, descendant.document_node)
    }

    /// Obtain top node, given node anywhere in a tree
    ///
    /// In an XML document this is the document element.
    pub fn top_element(&self, node: Node) -> Node {
        self.wrap(node, |doc, n| doc.top_element(n))
    }

    /// Return true if node is directly under the document node.
    ///
    /// This means it's either the document element or a comment or processing
    /// instruction.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<!--foo--><p>Example</p><?bar?>").unwrap();
    ///
    /// let comment = xoz.first_child(root).unwrap();
    /// let p = xoz.next_sibling(comment).unwrap();
    /// let pi = xoz.next_sibling(p).unwrap();
    /// let text = xoz.first_child(p).unwrap();
    ///
    /// assert!(xoz.is_directly_under_document(comment));
    /// assert!(xoz.is_directly_under_document(pi));
    /// assert!(xoz.is_directly_under_document(p));
    /// assert!(!xoz.is_directly_under_document(text));
    /// ```
    pub fn is_directly_under_document(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_directly_under_document(node.document_node)
    }

    /// Returns true if the node is the document element
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<!--foo--><p>Example<em>Em</em></p>").unwrap();
    /// let comment = xoz.first_child(root).unwrap();
    /// let p = xoz.next_sibling(comment).unwrap();
    /// let text = xoz.first_child(p).unwrap();
    /// let em = xoz.next_sibling(text).unwrap();
    /// assert!(!xoz.is_document_element(comment));
    /// assert!(xoz.is_document_element(p));
    /// assert!(!xoz.is_document_element(text));
    /// assert!(!xoz.is_document_element(em));
    /// ```
    pub fn is_document_element(&self, node: Node) -> bool {
        let document = self.document(node.document_id);
        document.is_document_element(node.document_node)
    }

    /// Get index of child.
    ///
    /// Returns [`None`] if the node is not a child of this node.
    ///
    /// Namespace and attribute nodes aren't considered children.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// assert_eq!(xoz.child_index(p, a), Some(0));
    /// assert_eq!(xoz.child_index(p, b), Some(1));
    /// assert_eq!(xoz.child_index(a, b), None);
    /// ```
    pub fn child_index(&self, parent: Node, node: Node) -> Option<usize> {
        let parent_document_id = parent.document_id;
        let node_document_id = node.document_id;
        if parent_document_id != node_document_id {
            return None;
        }
        let document = self.document(node_document_id);
        document.child_index(parent.document_node, node.document_node)
    }

    /// Descendant of node type
    ///
    /// Look for the first descendant of node in document order that has NodeType.
    pub fn typed_descendant(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_descendant(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }

    /// Following node of node type.
    ///
    /// Look for the first following node (after node) in document order that
    /// has node type.
    pub fn typed_foll(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let document = self.document(node.document_id);
        document
            .typed_foll(node.document_node, node_type)
            .map(|n| document.new_node(n))
    }
}
