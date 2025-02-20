use vers_vecs::trees::Tree;

use crate::{NodeName, NodeType};

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
        if self.node_info_id(parent).is_special() {
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
        let node_info_id = self.node_info_id(node);
        if node_info_id.is_attributes() {
            // the first child is the attributes node, skip it
            self.next_sibling(node)
        } else if node_info_id.is_namespaces() {
            // the first child is the namespaces node
            // check if the next sibling is the attributes node
            let next = self.next_sibling(node)?;
            // if so, the first child is the next sibling
            if self.node_info_id(next).is_attributes() {
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
        if self.node_info_id(child).is_special() {
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
        if self.node_info_id(prev).is_special() {
            // attributes and namespaces nodes are not siblings
            None
        } else {
            Some(prev)
        }
    }
}
