use vers_vecs::{trees::Tree, IsAncestor};

use crate::{node_info_vec::NodeInfoId, NodeName, NodeType};

use super::{Document, Node};

impl Document {
    /// Given node type, return the node info id, if it exists.
    pub(crate) fn node_info_id(&self, node_type: NodeType) -> Option<NodeInfoId> {
        self.structure.lookup_node_info_id_for_node_type(node_type)
    }

    pub(crate) fn node_info_id_for_node(&self, node: Node) -> NodeInfoId {
        self.structure.node_info_id(node.get())
    }

    /// Preorder number of node
    ///
    /// This can be used to sort nodes by preorder.
    ///
    /// Note that since attributes and namespaces are also nodes in the tree,
    /// as well as the nodes that hold them, the preorder may have gaps.
    pub fn preorder(&self, node: Node) -> usize {
        self.structure.tree().node_index(node.get())
    }

    /// Given a node, give back the [`NodeName`] of this node.
    ///
    /// For elements and attribute that is their name, for processing
    /// instructions this is a name based on the target attribute.
    ///
    /// For anything else, it's `None`.
    ///
    /// ```rust
    /// use xoz::{Document, NodeName};
    /// let doc = Document::parse_str(r#"<ex:doc xmlns:ex="http://example.com" ex:b="B"><a/></ex:doc>"#).unwrap();
    /// let doc_el = doc.document_element();
    /// let a_el = doc.first_child(doc_el).unwrap();
    ///
    /// let doc_name = doc.node_name(doc_el).unwrap();
    /// assert_eq!(doc_name.local_name(), b"doc");
    /// assert_eq!(doc_name.namespace(), b"http://example.com");
    ///
    /// let a_name = doc.node_name(a_el).unwrap();
    /// assert_eq!(a_name.local_name(), b"a");
    /// assert_eq!(a_name.namespace(), b"");
    ///
    /// // it also works on attribute nodes
    /// let b_attribute = doc.attribute_node(doc_el, NodeName::new("http://example.com", "b")).unwrap();
    /// let b_name = doc.node_name(b_attribute).unwrap();
    /// assert_eq!(b_name.local_name(), b"b");
    /// assert_eq!(b_name.namespace(), b"http://example.com");
    /// ```
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
            .is_ancestor(ancestor.get(), descendant.get())
            .expect("Illegal tree structure or node not in tree")
    }
}
