use vers_vecs::trees::Tree;

use crate::{
    iter::NextSiblingIter, node::NodeType, node_info_vec::SArrayMatrix, parser::parse_document,
    structure::Structure, text::TextUsage, QuickXMLError,
};

pub struct Document {
    pub(crate) structure: Structure<SArrayMatrix>,
    pub(crate) text_usage: TextUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(usize);

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

    pub fn subtree_tags(&self, node: Node, node_type: NodeType) -> usize {
        let node_info_id = self.node_info_id(node_type);
        if let Some(node_info_id) = node_info_id {
            self.structure
                .subtree_tags(node.0, node_info_id)
                .unwrap_or(0)
        } else {
            0
        }
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
