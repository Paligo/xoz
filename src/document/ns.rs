use vers_vecs::trees::Tree;

use crate::{TagName, TagType};

use super::{Document, Node};

impl Document {
    /// Get a node which contains the namespace declarations ("xmlns") children of
    /// of this node.
    ///
    /// This node has tag type `TagType::Namespaces`.
    ///
    /// If this is not an element node, or there are no namespace declarations,
    /// it returns `None`.
    pub(crate) fn namespaces_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            let tag_id = self.tag_id(node);
            if tag_id.is_namespaces() {
                // the first child is the namespaces node
                Some(node)
            } else {
                None
            }
        } else {
            None
        }
    }
}
