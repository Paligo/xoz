use vers_vecs::trees::Tree;

use crate::{node_info_vec::NodeInfoId, NodeName, NodeType};

use super::{Document, Node};

impl Document {
    pub(crate) fn node_info_id(&self, node_type: NodeType) -> Option<NodeInfoId> {
        self.structure.lookup_node_info_id_for_node_type(node_type)
    }

    pub(crate) fn node_info_id_for_node(&self, node: Node) -> NodeInfoId {
        self.structure.node_info_id(node.get())
    }

    pub fn preorder(&self, node: Node) -> usize {
        self.structure.tree().node_index(node.get())
    }

    pub fn node_name(&self, node: Node) -> Option<&NodeName> {
        match self.node_type(node) {
            NodeType::Element(node_name) => Some(node_name),
            NodeType::Attribute(node_name) => Some(node_name),
            NodeType::ProcessingInstruction => {
                todo!()
            }
            _ => None,
        }
    }

    pub fn node_type(&self, node: Node) -> &NodeType {
        let node_info = self.structure.get_node_info(node.get());
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

    pub fn subtree_count(&self, node: Node, node_type: NodeType) -> usize {
        let node_info_id = self.node_info_id(node_type);
        if let Some(node_info_id) = node_info_id {
            self.structure
                .subtree_tags(node.get(), node_info_id)
                .unwrap_or(0)
        } else {
            0
        }
    }

    pub fn subtree_size(&self, node: Node) -> usize {
        self.structure.subtree_size(node.get())
    }
}
