use vers_vecs::{trees::Tree, IsAncestor};

use crate::{node_info_vec::NodeInfoId, NodeType};

use super::{Document, Node};

impl Document {
    pub fn root(&self) -> Node {
        Node::new(
            self.structure
                .tree()
                .root()
                .expect("XML document always has a root"),
        )
    }

    pub fn document_element(&self) -> Node {
        for child in self.children(self.root()) {
            if let NodeType::Element { .. } = self.node_type(child) {
                return child;
            }
        }
        unreachable!()
    }

    pub fn parent(&self, node: Node) -> Option<Node> {
        // two strategies are possible: skipping the attributes and namespaces nodes
        // if found, or checking whether we are an attribute or namespace node before
        // we even try. I've chosen the first strategy.
        let parent = self.primitive_parent(node)?;
        if self.node_info_id_for_node(parent).is_special() {
            // if the parent is an attribute or namespace node, we skip it
            self.primitive_parent(parent)
        } else {
            // if it's not, then it's a parent
            Some(parent)
        }
    }

    pub fn first_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node)?;
        let node_info_id = self.node_info_id_for_node(node);
        if node_info_id.is_attributes() {
            // the first child is the attributes node, skip it
            self.next_sibling(node)
        } else if node_info_id.is_namespaces() {
            // the first child is the namespaces node
            // check if the next sibling is the attributes node
            let next = self.next_sibling(node)?;
            // if so, the first child is the next sibling
            if self.node_info_id_for_node(next).is_attributes() {
                self.next_sibling(next)
            } else {
                // if not, the first child is this sibling
                Some(next)
            }
        } else {
            // if it's not a special node, then it's definitely a first child
            Some(node)
        }
    }

    pub fn last_child(&self, node: Node) -> Option<Node> {
        let child = self.primitive_last_child(node)?;
        if self.node_info_id_for_node(child).is_special() {
            None
        } else {
            Some(child)
        }
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure
            .tree()
            .next_sibling(node.get())
            .map(Node::new)
    }

    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        let prev = self.primitive_previous_sibling(node)?;
        if self.node_info_id_for_node(prev).is_special() {
            // attributes and namespaces nodes are not siblings
            None
        } else {
            Some(prev)
        }
    }

    pub fn is_ancestor(&self, ancestor: Node, descendant: Node) -> bool {
        if ancestor == descendant {
            return false;
        }
        self.is_ancestor_or_self(ancestor, descendant)
    }

    pub fn is_ancestor_or_self(&self, ancestor: Node, descendant: Node) -> bool {
        self.structure
            .tree()
            .is_ancestor(ancestor.get(), descendant.get())
            .expect("Illegal tree structure or node not in tree")
    }

    pub fn top_element(&self, node: Node) -> Node {
        if self.is_document(node) {
            return self.document_element();
        }
        let mut top = node;
        for ancestor in self.ancestors_or_self(node) {
            if self.is_element(ancestor) {
                top = ancestor;
            }
        }
        top
    }

    pub fn is_directly_under_document(&self, node: Node) -> bool {
        self.parent(node) == Some(self.root())
    }

    pub fn is_document_element(&self, node: Node) -> bool {
        self.is_element(node) && self.is_directly_under_document(node)
    }

    pub fn child_index(&self, parent: Node, node: Node) -> Option<usize> {
        for (i, child) in self.children(parent).enumerate() {
            if child == node {
                return Some(i);
            }
        }
        None
    }

    pub fn typed_descendant(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let node_info_id = self.node_info_id(node_type)?;
        self.typed_descendant_by_node_info_id(node, node_info_id)
    }

    pub(crate) fn typed_descendant_by_node_info_id(
        &self,
        node: Node,
        node_info_id: NodeInfoId,
    ) -> Option<Node> {
        self.structure
            .typed_descendant(node.get(), node_info_id)
            .map(Node::new)
    }

    pub fn typed_foll(&self, node: Node, node_type: NodeType) -> Option<Node> {
        let node_info_id = self.node_info_id(node_type)?;
        self.typed_foll_by_node_info_id(node, node_info_id)
    }

    pub(crate) fn typed_foll_by_node_info_id(
        &self,
        node: Node,
        node_info_id: NodeInfoId,
    ) -> Option<Node> {
        self.structure
            .typed_following(node.get(), node_info_id)
            .map(Node::new)
    }
}
