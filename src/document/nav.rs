use vers_vecs::{trees::Tree, IsAncestor};

use crate::{node_info_vec::NodeInfoId, NodeType};

use super::{Document, Node};

impl Document {
    /// Give the document node of the XML document
    pub fn root(&self) -> Node {
        Node::new(
            self.structure
                .tree()
                .root()
                .expect("XML document always has a root"),
        )
    }

    /// Obtain the document element.
    ///
    /// ```rust
    /// use xoz::Document;
    /// let doc = Document::parse_str("<p>Example</p>").unwrap();
    ///
    /// let doc_el = doc.document_element();
    ///
    /// assert!(doc.is_element(doc_el));
    /// assert_eq!(doc.parent(doc_el), Some(doc.root()));
    /// ```
    pub fn document_element(&self) -> Node {
        for child in self.children(self.root()) {
            if let NodeType::Element { .. } = self.node_type(child) {
                return child;
            }
        }
        unreachable!()
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
    /// use xoz::Document;
    /// let doc = Document::parse_str("<p>Example</p>").unwrap();
    /// let root = doc.root();
    /// let p = doc.document_element();
    /// let text = doc.first_child(p).unwrap();
    ///
    /// assert_eq!(doc.parent(text), Some(p));
    /// assert_eq!(doc.parent(p), Some(root));
    /// assert_eq!(doc.parent(root), None);
    /// ```
    pub fn parent(&self, node: Node) -> Option<Node> {
        // two strategies are possible: skipping the attributes and namespaces nodes
        // if found, or checking whether we are an attribute or namespace node before
        // we even try. I've chosen the first strategy.
        let parent = self.primitive_parent(node)?;
        if self.node_info_id_for_node(parent).is_special() {
            // if the parent is an attribute or namespace node, we skip it
            self.primitive_parent(parent)
        } else {
            // if it's not, then it's a parent
            Some(parent)
        }
    }

    /// Get first child.
    ///
    /// Returns [`None`] if there are no children.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<p>Example</p>").unwrap();
    /// let root = doc.root();
    /// let p = doc.document_element();
    /// let text = doc.first_child(p).unwrap();
    /// assert_eq!(doc.first_child(root), Some(p));
    /// assert_eq!(doc.first_child(p), Some(text));
    /// assert_eq!(doc.first_child(text), None);
    /// ```
    pub fn first_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node)?;
        let node_info_id = self.node_info_id_for_node(node);
        if node_info_id.is_attributes() {
            // the first child is the attributes node, skip it
            self.next_sibling(node)
        } else if node_info_id.is_namespaces() {
            // the first child is the namespaces node
            // check if the next sibling is the attributes node
            let next = self.next_sibling(node)?;
            // if so, the first child is the next sibling
            if self.node_info_id_for_node(next).is_attributes() {
                self.next_sibling(next)
            } else {
                // if not, the first child is this sibling
                Some(next)
            }
        } else {
            // if it's not a special node, then it's definitely a first child
            Some(node)
        }
    }

    /// Get last child.
    ///
    /// Returns [`None`] if there are no children.
    pub fn last_child(&self, node: Node) -> Option<Node> {
        let child = self.primitive_last_child(node)?;
        if self.node_info_id_for_node(child).is_special() {
            None
        } else {
            Some(child)
        }
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
    /// let doc = xoz::Document::parse_str("<p><a/><b/></p>").unwrap();
    /// let p = doc.document_element();
    /// let a = doc.first_child(p).unwrap();
    /// let b = doc.next_sibling(a).unwrap();
    /// assert_eq!(doc.next_sibling(b), None);
    /// ```
    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure
            .tree()
            .next_sibling(node.get())
            .map(Node::new)
    }

    /// Get previous sibling.
    ///
    /// Returns [`None`] if there is no previous sibling.
    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        let prev = self.primitive_previous_sibling(node)?;
        if self.node_info_id_for_node(prev).is_special() {
            // attributes and namespaces nodes are not siblings
            None
        } else {
            Some(prev)
        }
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// The ancestor node is not considered a descendant of itself.
    pub fn is_ancestor(&self, ancestor: Node, descendant: Node) -> bool {
        if ancestor == descendant {
            return false;
        }
        self.is_ancestor_or_self(ancestor, descendant)
    }

    /// If ancestor is an ancestor of descendant, return true.
    /// A node is considered a descendant of itself.
    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        self.structure
            .tree()
            .is_ancestor(ancestor.get(), descendant.get())
            .expect("Illegal tree structure or node not in tree")
    }

    /// Obtain top node, given node anywhere in a tree
    ///
    /// In an XML document this is the document element.
    pub fn top_element(&self, node: Node) -> Node {
        if self.is_document(node) {
            return self.document_element();
        }
        let mut top = node;
        for ancestor in self.ancestors_or_self(node) {
            if self.is_element(ancestor) {
                top = ancestor;
            }
        }
        top
    }

    /// Return true if node is directly under the document node.
    ///
    /// This means it's either the document element or a comment or processing
    /// instruction.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<!--foo--><p>Example</p><?bar?>").unwrap();
    ///
    /// let root = doc.root();
    /// let comment = doc.first_child(root).unwrap();
    /// let p = doc.next_sibling(comment).unwrap();
    /// let pi = doc.next_sibling(p).unwrap();
    /// let text = doc.first_child(p).unwrap();
    ///
    /// assert!(doc.is_directly_under_document(comment));
    /// assert!(doc.is_directly_under_document(pi));
    /// assert!(doc.is_directly_under_document(p));
    /// assert!(!doc.is_directly_under_document(text));
    /// ```
    pub fn is_directly_under_document(&self, node: Node) -> bool {
        self.parent(node) == Some(self.root())
    }

    /// Get index of child.
    ///
    /// Returns [`None`] if the node is not a child of this node.
    ///
    /// Namespace and attribute nodes aren't considered children.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<p><a/><b/></p>").unwrap();
    /// let p = doc.document_element();
    /// let a = doc.first_child(p).unwrap();
    /// let b = doc.next_sibling(a).unwrap();
    /// assert_eq!(doc.child_index(p, a), Some(0));
    /// assert_eq!(doc.child_index(p, b), Some(1));
    /// assert_eq!(doc.child_index(a, b), None);
    /// ```
    pub fn child_index(&self, parent: Node, node: Node) -> Option<usize> {
        for (i, child) in self.children(parent).enumerate() {
            if child == node {
                return Some(i);
            }
        }
        None
    }

    /// Descendant of node type
    ///
    /// Look for the first descendant of node in document order that has NodeType.
    pub fn typed_descendant(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let node_info_id = self.node_info_id(node_type)?;
        self.typed_descendant_by_node_info_id(node, node_info_id)
    }

    pub(crate) fn typed_descendant_by_node_info_id(
        &self,
        node: Node,
        node_info_id: NodeInfoId,
    ) -> Option<Node> {
        self.structure
            .typed_descendant(node.get(), node_info_id)
            .map(Node::new)
    }

    /// Following node of node type.
    ///
    /// Look for the first following node (after node) in document order that
    /// has node type.
    pub fn typed_foll(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let node_info_id = self.node_info_id(node_type)?;
        self.typed_foll_by_node_info_id(node, node_info_id)
    }

    pub(crate) fn typed_foll_by_node_info_id(
        &self,
        node: Node,
        node_info_id: NodeInfoId,
    ) -> Option<Node> {
        self.structure
            .typed_following(node.get(), node_info_id)
            .map(Node::new)
    }
}
