use crate::{iter::AttributesIter, NodeName, NodeType};

use super::{Document, Node};

impl Document {
    pub(crate) fn attributes_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            let node_info_id = self.node_info_id_for_node(node);
            if node_info_id.is_attributes() {
                // the first child is the attributes node
                Some(node)
            } else if node_info_id.is_namespaces() {
                // the first child is the namespaces node, check for attributes node
                let next = self.next_sibling(node);
                next.filter(|next| self.node_info_id_for_node(*next).is_attributes())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn attribute_node<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<Node> {
        let attributes = self.attributes_child(node)?;
        let name = name.into();
        for child in self.primitive_children(attributes) {
            if let NodeType::Attribute(tag_name) = self.node_type(child) {
                if tag_name == &name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn attribute_value<'a>(&self, node: Node, name: impl Into<NodeName<'a>>) -> Option<&str> {
        let attribute_node = self.attribute_node(node, name)?;
        let text_id = self.structure.text_id(attribute_node.get());
        Some(self.text_usage.text_value(text_id))
    }

    pub fn attribute_entries(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&NodeName, &str)> + use<'_> {
        AttributesIter::new(self, node).map(move |n| {
            let text_id = self.structure.text_id(n.get());
            let value = self.text_usage.text_value(text_id);
            let tag_name = match self.node_type(n) {
                NodeType::Attribute(tag_name) => tag_name,
                _ => unreachable!(),
            };
            (tag_name, value)
        })
    }
}
