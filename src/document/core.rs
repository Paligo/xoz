use quick_xml::events::BytesPI;
use vers_vecs::trees::Tree;

use crate::{
    iter::{
        AncestorIter, AttributesIter, ChildrenIter, DescendantsIter, FollowingIter, NamespacesIter,
        NextSiblingIter, NodeTreeOps, PreviousSiblingIter, TaggedTreeOps, WithSelfIter,
        WithTaggedSelfIter,
    },
    parser::parse_document,
    structure::Structure,
    tag::{TagInfo, TagType},
    tagvec::{SArrayMatrix, TagId},
    text::TextUsage,
    traverse::{TagState, TraverseIter},
    QuickXMLError, TagName,
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

    /// Given a tag info, return the tag id, if it exists.
    pub fn tag(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.structure.lookup_tag_id(tag_info)
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

    pub fn attribute_value<'a>(&self, node: Node, name: impl Into<TagName<'a>>) -> Option<&str> {
        let attribute_node = self.attribute_node(node, name)?;
        let text_id = self.structure.text_id(attribute_node.0);
        Some(self.text_usage.text_value(text_id))
    }

    pub fn node_name(&self, node: Node) -> Option<&TagName> {
        match self.tag_type(node) {
            TagType::Element(tag_name) => Some(tag_name),
            TagType::Attribute(tag_name) => Some(tag_name),
            _ => None,
        }
    }

    pub fn tag_type(&self, node: Node) -> &TagType {
        let tag_info = self.structure.get_tag(node.0);
        debug_assert!(tag_info.is_open_tag());
        tag_info.tag_type()
    }

    pub fn tag_id(&self, node: Node) -> TagId {
        self.structure.tag_id(node.0)
    }

    pub fn is_document(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Document)
    }

    pub fn is_element(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Element { .. })
    }

    pub fn is_text(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Text)
    }

    pub fn is_comment(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Comment)
    }

    pub fn is_processing_instruction(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::ProcessingInstruction)
    }

    pub fn is_attribute(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Attribute { .. })
    }

    pub fn is_namespace(&self, node: Node) -> bool {
        matches!(self.tag_type(node), TagType::Namespace { .. })
    }

    pub fn is_ancestor(&self, node: Node, descendant: Node) -> bool {
        // TODO: replace with bp tree is_ancestor once that exists
        self.ancestors(descendant).any(|n| n == node)
    }

    pub fn child_index(&self, node: Node) -> Option<usize> {
        let parent = self.parent(node)?;
        for (i, child) in self.children(parent).enumerate() {
            if child == node {
                return Some(i);
            }
        }
        None
    }

    pub fn children(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        ChildrenIter::new(self, node)
    }

    pub fn axis_child(&self, node: Node) -> impl DoubleEndedIterator<Item = Node> + use<'_> {
        self.children(node)
    }

    pub fn following_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        NextSiblingIter::new(self, self.next_sibling(node))
    }

    pub fn axis_following_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.following_siblings(node)
    }

    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        PreviousSiblingIter::new(self, self.previous_sibling(node))
    }

    pub fn axis_preceding_sibling(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let siblings: Vec<_> = self.preceding_siblings(node).collect();
        siblings.into_iter().rev()
    }

    pub fn ancestors_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.ancestors(node))
    }

    pub fn axis_ancestor_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors_or_self(node).collect();
        ancestors.into_iter().rev()
    }

    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AncestorIter::new(node, NodeTreeOps::new(self))
    }

    pub fn axis_ancestor(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        let ancestors: Vec<_> = self.ancestors(node).collect();
        ancestors.into_iter().rev()
    }

    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        DescendantsIter::new(node, NodeTreeOps::new(self))
    }

    pub fn axis_descendant(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(node)
    }

    pub fn descendants_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        WithSelfIter::new(node, self.descendants(node))
    }

    pub fn axis_descendant_or_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants_or_self(node)
    }

    pub fn attributes(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AttributesIter::new(self, node)
    }

    pub fn attribute_entries(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&TagName, &str)> + use<'_> {
        AttributesIter::new(self, node).map(move |n| {
            let text_id = self.structure.text_id(n.0);
            let value = self.text_usage.text_value(text_id);
            let tag_name = match self.tag_type(n) {
                TagType::Attribute(tag_name) => tag_name,
                _ => unreachable!(),
            };
            (tag_name, value)
        })
    }

    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + use<'_> {
        NamespacesIter::new(self, node).map(move |n| match self.tag_type(n) {
            TagType::Namespace(namespace) => (namespace.prefix(), namespace.uri()),
            _ => unreachable!(),
        })
    }

    pub fn axis_attribute(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.attributes(node)
    }

    pub fn axis_parent(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.parent(node).into_iter()
    }

    pub fn axis_self(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        std::iter::once(node)
    }

    pub fn tagged_descendants(
        &self,
        node: Node,
        tag_id: TagId,
    ) -> impl Iterator<Item = Node> + use<'_> {
        DescendantsIter::new(node, TaggedTreeOps::new(self, tag_id))
    }

    pub fn tagged_descendants_or_self(
        &self,
        node: Node,
        tag_id: TagId,
    ) -> impl Iterator<Item = Node> + use<'_> {
        WithTaggedSelfIter::new(self, node, self.tagged_descendants(node, tag_id), tag_id)
    }

    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter::new(node, NodeTreeOps::new(self))
    }

    pub fn preceding(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants(self.root())
            .take_while(move |n| *n != node)
            .filter(move |n| !self.is_ancestor(*n, node))
    }

    pub fn tagged_following(
        &self,
        node: Node,
        tag_id: TagId,
    ) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter::new(node, TaggedTreeOps::new(self, tag_id))
    }

    pub fn traverse(
        &self,
        node: Node,
    ) -> impl Iterator<Item = (&TagType, TagState, Node)> + use<'_> {
        TraverseIter::new(self, node)
    }

    pub fn text_str(&self, node: Node) -> Option<&str> {
        if matches!(self.tag_type(node), TagType::Text) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn comment_str(&self, node: Node) -> Option<&str> {
        if matches!(self.tag_type(node), TagType::Comment) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        if matches!(self.tag_type(node), TagType::ProcessingInstruction) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        if matches!(self.tag_type(node), TagType::ProcessingInstruction) {
            let s = self.node_str(node).expect("Missing PI data");
            Some(ProcessingInstruction { data: s })
        } else {
            None
        }
    }

    pub fn string_value(&self, node: Node) -> String {
        match self.tag_type(node) {
            TagType::Document | TagType::Element(_) => self.descendants_to_string(node),
            TagType::Text | TagType::Comment | TagType::Attribute(_) => {
                self.node_str(node).unwrap().to_string()
            }
            TagType::ProcessingInstruction => self.processing_instruction(node).unwrap().content(),
            TagType::Namespace(namespace) => {
                let uri = namespace.uri();
                String::from_utf8(uri.to_vec()).expect("Namespace URI is not utf8")
            }
            TagType::Namespaces | TagType::Attributes => {
                panic!("Cannot use this with namespaces or attribute node")
            }
        }
    }

    fn node_str(&self, node: Node) -> Option<&str> {
        let text_id = self.structure.text_id(node.0);
        Some(self.text_usage.text_value(text_id))
    }

    fn descendants_to_string(&self, node: Node) -> String {
        let texts = self.descendants(node).filter_map(|n| self.text_str(n));
        let (lower_bound, _) = texts.size_hint();
        let mut r = String::with_capacity(lower_bound);
        for text in texts {
            r.push_str(text);
        }
        r
    }

    pub fn subtree_tags(&self, node: Node, tag_id: TagId) -> usize {
        self.structure.subtree_tags(node.0, tag_id).unwrap_or(0)
    }

    pub fn tagged_descendant(&self, node: Node, tag_id: TagId) -> Option<Node> {
        self.structure.tagged_descendant(node.0, tag_id).map(Node)
    }

    pub fn tagged_foll(&self, node: Node, tag_id: TagId) -> Option<Node> {
        self.structure.tagged_following(node.0, tag_id).map(Node)
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

pub struct ProcessingInstruction<'a> {
    data: &'a str,
}

impl ProcessingInstruction<'_> {
    pub fn target(&self) -> String {
        let bytes_pi = BytesPI::new(self.data);
        let target = std::str::from_utf8(bytes_pi.target()).expect("PI target is not utf8");
        target.to_string()
    }

    pub fn content(&self) -> String {
        let bytes_pi = BytesPI::new(self.data);
        let content = std::str::from_utf8(bytes_pi.content()).expect("PI content is not utf8");
        content.to_string()
    }
}
