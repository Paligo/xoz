use crate::iter::{
    AncestorIter, AttributesIter, ChildrenIter, DescendantsIter, NextSiblingIter, NodeTreeOps,
    PreviousSiblingIter, WithSelfIter,
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
}
