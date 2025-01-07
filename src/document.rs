use vers_vecs::trees::Tree;

use crate::{data::Structure, tag::TagType, tagvec::SArrayMatrix, text::TextUsage};

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

impl Name<'_> {
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

pub enum Value<'a> {
    Element(Name<'a>),
    Text(&'a str),
}

impl Document {
    pub fn root(&self) -> Node {
        Node(
            self.structure
                .tree()
                .root()
                .expect("XML document always has a root"),
        )
    }

    pub fn document_element(&self) -> Node {
        for child in self.children(self.root()) {
            if let Some(TagType::Element { .. }) = self.node_value(child) {
                return child;
            }
        }
        unreachable!()
    }

    pub fn first_child(&self, node: Node) -> Option<Node> {
        self.structure.tree().first_child(node.0).map(Node)
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().next_sibling(node.0).map(Node)
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
