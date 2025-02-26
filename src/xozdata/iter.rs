use crate::NodeType;

use super::core::{Node, Xoz};

/// ## Iteration
///
/// Iterators over the tree structure. This also supports various axes
/// as defined by XPath.
impl Xoz {
    /// Iterator over the child nodes of this node.
    ///
    /// Note that the special Namespaces and Attributes nodes are not
    /// included in the iterator, but the children of these nodes can be accessed
    /// using this way.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let children = xoz.children(p).collect::<Vec<_>>();
    ///
    /// assert_eq!(children, vec![a, b]);
    /// ```
    pub fn children(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .children(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath child axis.
    ///
    /// This is the same as [`Xoz::children`].
    pub fn axis_child(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + '_ {
        self.children(node)
    }

    /// Iterator over the following siblings of this node, not including this one.
    ///
    /// In case of namespace or attribute nodes, includes the following sibling
    /// namespace or attribute nodes.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b/><c/></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let c = xoz.next_sibling(b).unwrap();
    /// let siblings = xoz.following_siblings(a).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![b, c]);
    /// let siblings = xoz.following_siblings(b).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![c]);
    /// ```
    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .following_siblings(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath following-sibling axis.
    ///
    /// This is the same as [`Xoz::following_siblings`].
    pub fn axis_following_sibling(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.following_siblings(node)
    }

    /// Iterator over the preceding siblings of this node.
    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .preceding_siblings(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath preceding-sibling axis.
    ///
    /// This is the same as [`Xoz::preceding_siblings`] but in reverse order.
    pub fn axis_preceding_sibling(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_preceding_sibling(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over ancestor nodes, including this one.
    ///
    /// Namespace and attribute node have ancestors, even though
    /// they aren't the child of the element they are in.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    ///
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    ///
    /// let ancestors = xoz.ancestors_or_self(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![c, b, a, root]);
    /// ```
    pub fn ancestors_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .ancestors_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath ancestor-or-self axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Xoz::ancestors_or_self`].
    pub fn axis_ancestor_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_ancestor_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over ancestor nodes, not including this one.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let ancestors = xoz.ancestors(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![b, a, root]);
    /// ```
    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .ancestors(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath ancestor axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Xoz::ancestors`].
    pub fn axis_ancestor(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_ancestor(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator over of the descendants of this node,
    /// not including this one. In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let descendants = xoz.descendants(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![b, c]);
    /// ```
    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .descendants(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath descendant axis.
    ///
    /// This is the same as [`Xoz::descendants`].
    pub fn axis_descendant(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.descendants(node)
    }

    /// Iterator over of the descendants of this node, including this one.
    /// In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<a><b><c/></b></a>").unwrap();
    /// let a = xoz.document_element(root);
    /// let b = xoz.first_child(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let descendants = xoz.descendants_or_self(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![a, b, c]);
    /// ```
    pub fn descendants_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .descendants_or_self(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath descendant-or-self axis.
    ///
    /// This is the same as [`Xoz::descendants_or_self`].
    pub fn axis_descendant_or_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.descendants_or_self(node)
    }

    /// Iterator over the attribute nodes of this node.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = xoz.document_element(root);
    /// let attributes = xoz.attributes(p).collect::<Vec<_>>();
    /// assert_eq!(attributes.len(), 2);
    /// ```
    pub fn attributes(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .attributes(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath attribute axis.
    ///
    /// This is the same as [`Xoz::attributes`].
    pub fn axis_attributes(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.attributes(node)
    }

    /// Iterator representing the XPath parent axis
    pub fn axis_parent(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.parent(node).into_iter()
    }

    /// Iterator representing the XPath self axis
    pub fn axis_self(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        std::iter::once(node)
    }

    /// Following nodes in document order
    ///
    /// These are nodes that come after given node in document order,
    /// without that node itself, its ancestors, or its descendants.
    ///
    /// Does not include namespace or attribute nodes.
    ///
    /// ```rust
    /// use xoz::Xoz;
    /// let mut xoz = Xoz::new();
    /// let root = xoz.parse_str("<p><a/><b><c/><d/><e/></b><f><g/><h/></f></p>").unwrap();
    /// let p = xoz.document_element(root);
    /// let a = xoz.first_child(p).unwrap();
    /// let b = xoz.next_sibling(a).unwrap();
    /// let c = xoz.first_child(b).unwrap();
    /// let d = xoz.next_sibling(c).unwrap();
    /// let e = xoz.next_sibling(d).unwrap();
    /// let f = xoz.next_sibling(b).unwrap();
    /// let g = xoz.first_child(f).unwrap();
    /// let h = xoz.next_sibling(g).unwrap();
    /// let siblings = xoz.following(c).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![d, e, f, g, h]);
    /// ```
    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .following(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterator representing the XPath following axis.
    ///
    /// This is the same as [`Xoz::following`].
    pub fn axis_following(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        self.following(node)
    }

    /// Iterator representing the XPath preceding axis.
    ///
    /// These are nodes that come before given node in document order.
    pub fn axis_preceding(&self, node: Node) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .axis_preceding(node.document_node)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over descendants of a certain node type, using jumping.
    ///
    /// This more efficient than filtering the descendants iterator, as it
    /// only traverses the nodes that are of the given type, jumping over
    /// irrelevant ones.
    pub fn typed_descendants(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_descendants(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over descendants of a certain node type, including self if it
    /// matches, using jumping.
    pub fn typed_descendants_or_self(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_descendants_or_self(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over following nodes of a certain node type, using jumping.
    pub fn typed_following(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + '_ {
        let document = self.document(node.document_id);
        document
            .typed_following(node.document_node, node_type)
            .map(move |n| document.new_node(n))
    }

    /// Iterate over the nodes in the tree.
    ///
    /// This goes in document order. Attributes and namespace nodes are not included.
    ///
    /// The iterator yields a tuple of the node type, the tag state (open, close, empty),
    /// and the node itself. Only document and element node have an open and close state.
    pub fn traverse(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&NodeType, crate::TraverseState, Node)> + '_ {
        let document = self.document(node.document_id);
        document
            .traverse(node.document_node)
            .map(move |(node_type, tag_state, n)| (node_type, tag_state, document.new_node(n)))
    }
}
