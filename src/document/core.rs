use vers_vecs::trees::{IsAncestor, Tree};

use crate::{
    iter::{AttributesIter, NamespacesIter, NextSiblingIter},
    node::NodeType,
    node_info_vec::{NodeInfoId, SArrayMatrix},
    parser::parse_document,
    structure::Structure,
    text::TextUsage,
    NodeName, QuickXMLError,
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

    /// Given node info, return the node info id, if it exists.
    /// TODO: owned node type creation is not ideal
    pub(crate) fn node_info_id(&self, node_type: NodeType) -> Option<NodeInfoId> {
        self.structure.lookup_node_info_id_for_node_type(node_type)
    }

    pub(crate) fn node_info_id_for_node(&self, node: Node) -> NodeInfoId {
        self.structure.node_info_id(node.0)
    }

    /// Preorder number of node
    ///
    /// This can be used to sort nodes by preorder.
    ///
    /// Note that since attributes and namespaces are also nodes in the tree,
    /// as well as the nodes that hold them, the preorder may have gaps.
    pub fn preorder(&self, node: Node) -> usize {
        self.structure.tree().node_index(node.0)
    }

    pub fn node_name(&self, node: Node) -> Option<&NodeName> {
        match self.node_type(node) {
            NodeType::Element(node_name) => Some(node_name),
            NodeType::Attribute(node_name) => Some(node_name),
            _ => None,
        }
    }

    pub fn node_type(&self, node: Node) -> &NodeType {
        let node_info = self.structure.get_node_info(node.0);
        debug_assert!(node_info.is_open_tag());
        node_info.node_type()
    }

    pub fn is_document(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Document)
    }

    pub fn is_element(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Element { .. })
    }

    pub fn is_text(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Text)
    }

    pub fn is_comment(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Comment)
    }

    pub fn is_processing_instruction(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::ProcessingInstruction)
    }

    pub fn is_attribute(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Attribute { .. })
    }

    pub fn is_namespace(&self, node: Node) -> bool {
        matches!(self.node_type(node), NodeType::Namespace { .. })
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
    /// Node that a node is considered a descendant of itself.
    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        self.structure
            .tree()
            .is_ancestor(ancestor.0, descendant.0)
            .expect("Illegal tree structure or node not in tree")
    }

    pub fn attribute_entries(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&NodeName, &str)> + use<'_> {
        AttributesIter::new(self, node).map(move |n| {
            let text_id = self.structure.text_id(n.0);
            let value = self.text_usage.text_value(text_id);
            let tag_name = match self.node_type(n) {
                NodeType::Attribute(tag_name) => tag_name,
                _ => unreachable!(),
            };
            (tag_name, value)
        })
    }

    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + use<'_> {
        NamespacesIter::new(self, node).map(move |n| match self.node_type(n) {
            NodeType::Namespace(namespace) => (namespace.prefix(), namespace.uri()),
            _ => unreachable!(),
        })
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
