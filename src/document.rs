use vers_vecs::trees::Tree;

use crate::{
    structure::Structure,
    tag::{TagInfo, TagType},
    tagvec::{SArrayMatrix, TagId},
    text::TextUsage,
};

pub struct Document {
    pub(crate) structure: Structure<SArrayMatrix>,
    pub(crate) text_usage: TextUsage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node(usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name<'a> {
    local_name: &'a str,
    namespace: &'a str,
    prefix: &'a str,
}

impl<'a> Name<'a> {
    pub fn name_without_namespace(name: &'a str) -> Self {
        Self {
            local_name: name,
            namespace: "",
            prefix: "",
        }
    }

    pub fn local_name(&self) -> &str {
        self.local_name
    }
    pub fn namespace(&self) -> &str {
        self.namespace
    }
    pub fn prefix(&self) -> &str {
        self.prefix
    }
}

impl Document {
    pub fn tag(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.structure.lookup_tag_id(tag_info)
    }

    pub fn root(&self) -> Node {
        Node(
            self.structure
                .tree()
                .root()
                .expect("XML document always has a root"),
        )
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

    pub fn document_element(&self) -> Node {
        for child in self.children(self.root()) {
            if let Some(TagType::Element { .. }) = self.node_value(child) {
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
        match self.node_value(parent) {
            // if the parent is an attribute or namespace node, we skip it
            Some(TagType::Attributes) | Some(TagType::Namespaces) => self.primitive_parent(parent),
            // if it's not, then it's a parent
            _ => Some(parent),
        }
    }

    pub fn first_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node)?;
        match self.node_value(node) {
            // the first child is the attributes node, skip it
            Some(TagType::Attributes) => self.next_sibling(node),
            // the first child is the namespaces node
            Some(TagType::Namespaces) => {
                // check if the next sibling is the attributes node
                let next = self.next_sibling(node)?;
                // if so, the first child is the next sibling
                if let Some(TagType::Attributes) = self.node_value(next) {
                    self.next_sibling(next)
                } else {
                    // if not, the first child is this sibling
                    Some(next)
                }
            }
            // if it's not a special node, then it's definitely a first child
            _ => Some(node),
        }
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().next_sibling(node.0).map(Node)
    }

    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        let prev = self.primitive_previous_sibling(node)?;
        match self.node_value(prev) {
            // the previous sibling is the attributes node, we are at the beginning
            Some(TagType::Attributes) => None,
            // the previous sibling is the namespaces node, we're at the beginning too
            Some(TagType::Namespaces) => None,
            // if it's not a special node, then it's definitely a previous sibling
            _ => Some(prev),
        }
    }

    pub(crate) fn attributes_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            match self.node_value(node) {
                // the first child is the attributes node
                Some(TagType::Attributes) => Some(node),
                // the first child is the namespaces node, check for attributes node
                Some(TagType::Namespaces) => {
                    let next = self.next_sibling(node);
                    next.filter(|next| matches!(self.node_value(*next), Some(TagType::Attributes)))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn attribute_node(&self, node: Node, name: &Name) -> Option<Node> {
        let attributes = self.attributes_child(node)?;
        for child in self.primitive_children(attributes) {
            if let Some(TagType::Attribute {
                namespace,
                local_name,
            }) = self.node_value(child)
            {
                if namespace == name.namespace && local_name == name.local_name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn attribute_value(&self, node: Node, name: &Name) -> Option<&str> {
        let attribute_node = self.attribute_node(node, name)?;
        let text_id = self.structure.text_id(attribute_node.0);
        Some(self.text_usage.text_value(text_id))
    }

    pub fn node_name(&self, node: Node) -> Option<Name> {
        if let Some(value) = self.node_value(node) {
            match value {
                TagType::Element {
                    namespace,
                    local_name,
                } => Some(Name {
                    local_name,
                    namespace,
                    // TODO: proper prefix lookup
                    prefix: "",
                }),
                TagType::Attribute {
                    namespace,
                    local_name,
                } => Some(Name {
                    local_name,
                    namespace,
                    // TODO: proper prefix lookup
                    prefix: "",
                }),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn node_value(&self, node: Node) -> Option<&TagType> {
        self.structure.get_tag(node.0).map(|tag_info| {
            assert!(tag_info.is_open_tag());
            tag_info.tag_type()
        })
    }

    pub fn children(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter {
            doc: self,
            node: self.first_child(node),
        }
    }

    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter {
            doc: self,
            node: Some(node),
        }
    }

    pub fn text_str(&self, node: Node) -> Option<&str> {
        if matches!(self.node_value(node)?, TagType::Text) {
            let text_id = self.structure.text_id(node.0);
            Some(self.text_usage.text_value(text_id))
        } else {
            None
        }
    }

    pub fn subtree_tags(&self, node: Node, tag_id: TagId) -> usize {
        self.structure.subtree_tags(node.0, tag_id).unwrap_or(0)
    }

    pub fn tagged_descendant(&self, node: Node, tag_id: TagId) -> Option<Node> {
        self.structure.tagged_descendant(node.0, tag_id).map(Node)
    }

    pub(crate) fn primitive_parent(&self, node: Node) -> Option<Node> {
        self.structure.tree().parent(node.0).map(Node)
    }

    pub(crate) fn primitive_first_child(&self, node: Node) -> Option<Node> {
        self.structure.tree().first_child(node.0).map(Node)
    }

    pub(crate) fn primitive_previous_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().previous_sibling(node.0).map(Node)
    }

    // next_sibling is itself already primitive in behavior

    pub(crate) fn primitive_children(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter {
            doc: self,
            node: self.primitive_first_child(node),
        }
    }
}

struct NextSiblingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl Iterator for NextSiblingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.next_sibling(node);
        Some(node)
    }
}
