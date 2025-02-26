use crate::{
    iter::{
        AncestorIter, AttributesIter, ChildrenIter, DescendantsIter, FollowingIter,
        NextSiblingIter, NodeTreeOps, PreviousSiblingIter, TypedDescendantsIter,
        TypedFollowingIter, TypedTreeOps, WithSelfIter, WithTypedSelfIter,
    },
    traverse::TraverseIter,
    NodeType, TagState,
};

use super::{Document, Node};

impl Document {
    pub fn children(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        ChildrenIter::new(self, node)
    }

    pub fn axis_child(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        self.children(node)
    }

    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter::new(self, self.next_sibling(node))
    }

    pub fn axis_following_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.following_siblings(node)
    }

    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        PreviousSiblingIter::new(self, self.previous_sibling(node))
    }

    pub fn axis_preceding_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let siblings: Vec<_> = self.preceding_siblings(node).collect();
        siblings.into_iter().rev()
    }

    pub fn ancestors_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.ancestors(node))
    }

    pub fn axis_ancestor_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors_or_self(node).collect();
        ancestors.into_iter().rev()
    }

    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AncestorIter::new(node, NodeTreeOps::new(self))
    }

    pub fn axis_ancestor(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors(node).collect();
        ancestors.into_iter().rev()
    }

    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        DescendantsIter::new(node, NodeTreeOps::new(self))
    }

    pub fn axis_descendant(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(node)
    }

    pub fn descendants_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.descendants(node))
    }

    pub fn axis_descendant_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants_or_self(node)
    }

    pub fn attributes(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AttributesIter::new(self, node)
    }

    pub fn axis_attributes(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.attributes(node)
    }

    pub fn axis_parent(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.parent(node).into_iter()
    }

    pub fn axis_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        std::iter::once(node)
    }

    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter::new(node, NodeTreeOps::new(self))
    }

    pub fn axis_following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.following(node)
    }

    // TODO: non-xpath preceding

    pub fn axis_preceding(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(self.root())
            .take_while(move |n| *n != node)
            .filter(move |n| !self.is_ancestor(*n, node))
    }

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
    ) -> impl Iterator<Item = Node> + use<'_> {
        TypedFollowingIter::new(self, node, node_type)
    }

    pub fn traverse(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&NodeType, TagState, Node)> + use<'_> {
        TraverseIter::new(self, node)
    }
}
