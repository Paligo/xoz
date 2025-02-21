use crate::{
    iter::{
        AncestorIter, AttributesIter, ChildrenIter, DescendantsIter, FollowingIter,
        NextSiblingIter, NodeTreeOps, PreviousSiblingIter, TypedDescendantsIter, TypedTreeOps,
        WithSelfIter, WithTypedSelfIter,
    },
    traverse::TraverseIter,
    NodeType, TagState,
};

use super::{Document, Node};

impl Document {
    /// Iterator over the child nodes of this node.
    ///
    /// Note that the special Namespaces and Attributes nodes are not
    /// included in the iterator, but the children of these nodes can be accessed
    /// using this way.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<p><a/><b/></p>").unwrap();
    /// let p = doc.document_element();
    /// let a = doc.first_child(p).unwrap();
    /// let b = doc.next_sibling(a).unwrap();
    /// let children = doc.children(p).collect::<Vec<_>>();
    ///
    /// assert_eq!(children, vec![a, b]);
    /// ```
    pub fn children(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        ChildrenIter::new(self, node)
    }

    /// Iterator representing the XPath child axis.
    ///
    /// This is the same as [`Document::children`].
    pub fn axis_child(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        self.children(node)
    }

    /// Iterator over the following siblings of this node, not including this one.
    ///
    /// In case of namespace or attribute nodes, includes the following sibling
    /// namespace or attribute nodes.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<p><a/><b/><c/></p>").unwrap();
    /// let p = doc.document_element();
    /// let a = doc.first_child(p).unwrap();
    /// let b = doc.next_sibling(a).unwrap();
    /// let c = doc.next_sibling(b).unwrap();
    /// let siblings = doc.following_siblings(a).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![b, c]);
    /// let siblings = doc.following_siblings(b).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![c]);
    /// ```
    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter::new(self, self.next_sibling(node))
    }

    /// Iterator representing the XPath following-sibling axis.
    ///
    /// This is the same as [`Document::following_siblings`].
    pub fn axis_following_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.following_siblings(node)
    }

    /// Iterator over the preceding siblings of this node.
    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        PreviousSiblingIter::new(self, self.previous_sibling(node))
    }

    /// Iterator representing the XPath preceding-sibling axis.
    ///
    /// This is the same as [`Document::preceding_siblings`].
    pub fn axis_preceding_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let siblings: Vec<_> = self.preceding_siblings(node).collect();
        siblings.into_iter().rev()
    }

    /// Iterator over ancestor nodes, including this one.
    ///
    /// Namespace and attribute node have ancestors, even though
    /// they aren't the child of the element they are in.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<a><b><c/></b></a>").unwrap();
    ///
    /// let root = doc.root();
    /// let a = doc.document_element();
    /// let b = doc.first_child(a).unwrap();
    /// let c = doc.first_child(b).unwrap();
    ///
    /// let ancestors = doc.ancestors_or_self(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![c, b, a, root]);
    /// ```
    pub fn ancestors_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.ancestors(node))
    }

    /// Iterator representing the XPath ancestor-or-self axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Document::ancestors_or_self`].
    pub fn axis_ancestor_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors_or_self(node).collect();
        ancestors.into_iter().rev()
    }

    /// Iterator over ancestor nodes, not including this one.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<a><b><c/></b></a>").unwrap();
    /// let root = doc.root();
    /// let a = doc.document_element();
    /// let b = doc.first_child(a).unwrap();
    /// let c = doc.first_child(b).unwrap();
    /// let ancestors = doc.ancestors(c).collect::<Vec<_>>();
    /// assert_eq!(ancestors, vec![b, a, root]);
    /// ```
    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AncestorIter::new(node, NodeTreeOps::new(self))
    }

    /// Iterator representing the XPath ancestor axis.
    ///
    /// Note that this starts at the root node, and then descends to the
    /// provided node, unlike [`Document::ancestors`].
    pub fn axis_ancestor(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors(node).collect();
        ancestors.into_iter().rev()
    }

    /// Iterator over of the descendants of this node,
    /// not including this one. In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<a><b><c/></b></a>").unwrap();
    /// let root = doc.root();
    /// let a = doc.document_element();
    /// let b = doc.first_child(a).unwrap();
    /// let c = doc.first_child(b).unwrap();
    /// let descendants = doc.descendants(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![b, c]);
    /// ```
    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        DescendantsIter::new(node, NodeTreeOps::new(self))
    }

    /// Iterator representing the XPath descendant axis.
    ///
    /// This is the same as [`Document::descendants`].
    pub fn axis_descendant(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(node)
    }

    /// Iterator over of the descendants of this node, including this one.
    /// In document order (pre-order depth-first).
    ///
    /// Namespace and attribute nodes aren't included as descendants.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<a><b><c/></b></a>").unwrap();
    /// let root = doc.root();
    /// let a = doc.document_element();
    /// let b = doc.first_child(a).unwrap();
    /// let c = doc.first_child(b).unwrap();
    /// let descendants = doc.descendants_or_self(a).collect::<Vec<_>>();
    /// assert_eq!(descendants, vec![a, b, c]);
    /// ```
    pub fn descendants_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.descendants(node))
    }

    /// Iterator representing the XPath descendant-or-self axis.
    ///
    /// This is the same as [`Document::descendants_or_self`].
    pub fn axis_descendant_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants_or_self(node)
    }

    /// Iterator over the attribute nodes of this node.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str(r#"<p a="1" b="2"/>"#).unwrap();
    /// let p = doc.document_element();
    /// let attributes = doc.attributes(p).collect::<Vec<_>>();
    /// assert_eq!(attributes.len(), 2);
    /// ```
    pub fn attributes(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AttributesIter::new(self, node)
    }

    /// Iterator representing the XPath attribute axis.
    ///
    /// This is the same as [`Document::attributes`].
    pub fn axis_attributes(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.attributes(node)
    }

    /// Iterator representing the XPath parent axis
    pub fn axis_parent(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.parent(node).into_iter()
    }

    /// Iterator representing the XPath self axis
    pub fn axis_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
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
    /// let doc = xoz::Document::parse_str("<p><a/><b><c/><d/><e/></b><f><g/><h/></f></p>").unwrap();
    /// let p = doc.document_element();
    /// let a = doc.first_child(p).unwrap();
    /// let b = doc.next_sibling(a).unwrap();
    /// let c = doc.first_child(b).unwrap();
    /// let d = doc.next_sibling(c).unwrap();
    /// let e = doc.next_sibling(d).unwrap();
    /// let f = doc.next_sibling(b).unwrap();
    /// let g = doc.first_child(f).unwrap();
    /// let h = doc.next_sibling(g).unwrap();
    /// let siblings = doc.following(c).collect::<Vec<_>>();
    /// assert_eq!(siblings, vec![d, e, f, g, h]);
    /// ```
    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter::new(node, NodeTreeOps::new(self))
    }

    /// Iterator representing the XPath following axis.
    ///
    /// This is the same as [`Document::following`].
    pub fn axis_following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.following(node)
    }

    // TODO: non-xpath preceding

    /// Iterator representing the XPath preceding axis.
    ///
    /// These are nodes that come before given node in document order.
    pub fn axis_preceding(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(self.root())
            .take_while(move |n| *n != node)
            .filter(move |n| !self.is_ancestor(*n, node))
    }

    /// Iterate over descendants of a certain node type.
    ///
    /// This more efficient than filtering the descendants iterator, as it
    /// only traverses the nodes that are of the given type, jumping over
    /// irrelevant ones.
    pub fn typed_descendants(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> impl Iterator<Item = Node> + use<'_> {
        TypedDescendantsIter::new(self, node, node_type)
    }

    pub fn typed_descendants_or_self(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> Box<dyn Iterator<Item = Node> + '_> {
        let node_info_id = self.node_info_id(node_type.clone());
        if let Some(node_info_id) = node_info_id {
            Box::new(WithTypedSelfIter::new(
                self,
                node,
                self.typed_descendants(node, node_type),
                node_info_id,
            ))
        } else {
            // since node_info_id cannot be found, self cannot be
            // matching either
            Box::new(std::iter::empty())
        }
    }

    pub fn typed_following(
        &self,
        node: Node,
        node_type: NodeType,
    ) -> Box<dyn Iterator<Item = Node> + '_> {
        let node_info_id = self.node_info_id(node_type);
        if let Some(node_info_id) = node_info_id {
            Box::new(FollowingIter::new(
                node,
                TypedTreeOps::new(self, node_info_id),
            ))
        } else {
            Box::new(std::iter::empty())
        }
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
    ) -> impl Iterator<Item = (&NodeType, TagState, Node)> + use<'_> {
        TraverseIter::new(self, node)
    }
}
