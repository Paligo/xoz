use std::num::NonZeroI64;

use vers_vecs::trees::Tree;

use crate::{
    iter::NextSiblingIter, node_info_vec::SArrayMatrix, parser::parse_document,
    structure::Structure, text::TextUsage, QuickXMLError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// we start counting at 1 so the Option of a Node is the same size as a Node
pub(crate) struct DocumentId(NonZeroI64);

impl DocumentId {
    pub(crate) fn new(index: usize) -> Self {
        DocumentId(NonZeroI64::new(index as i64 + 1).unwrap())
    }

    pub(crate) fn index(self) -> usize {
        self.0.get() as usize - 1
    }
}

pub(crate) struct Document {
    pub(crate) id: DocumentId,
    pub(crate) structure: Structure<SArrayMatrix>,
    pub(crate) text_usage: TextUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct Node(usize);

impl Node {
    pub(crate) fn new(index: usize) -> Self {
        Node(index)
    }

    pub(crate) fn get(self) -> usize {
        self.0
    }
}

impl Document {
    pub fn parse_str(xml: &str) -> Result<Self, QuickXMLError> {
        parse_document(xml)
    }

    pub(crate) fn primitive_parent(&self, node: Node) -> Option<Node> {
        self.structure.tree().parent(node.0).map(Node)
    }

    pub(crate) fn primitive_first_child(&self, node: Node) -> Option<Node> {
        self.structure.tree().first_child(node.0).map(Node)
    }

    pub(crate) fn primitive_last_child(&self, node: Node) -> Option<Node> {
        self.structure.tree().last_child(node.0).map(Node)
    }

    pub(crate) fn primitive_previous_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().previous_sibling(node.0).map(Node)
    }

    // next_sibling is itself already primitive in behavior

    pub(crate) fn primitive_children(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter::new(self, self.primitive_first_child(node))
    }
}
