use quick_xml::{events::BytesPI, Error};
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

impl Document {
    pub fn parse_str(xml: &str) -> Result<Self, QuickXMLError> {
        parse_document(xml)
    }

    /// Given a tag info, return the tag id, if it exists.
    pub fn tag(&self, tag_info: &TagInfo) -> Option<TagId> {
        self.structure.lookup_tag_id(tag_info)
    }

    /// Give the document node of the XML document
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

    /// Obtain the document element.
    ///
    /// ```rust
    /// use xoz::Document;
    /// let doc = Document::parse_str("<p>Example</p>").unwrap();
    ///
    /// let doc_el = doc.document_element();
    ///
    /// assert!(doc.is_element(doc_el));
    /// assert_eq!(doc.parent(doc_el), Some(doc.root()));
    /// ```
    pub fn document_element(&self) -> Node {
        for child in self.children(self.root()) {
            if let TagType::Element { .. } = self.value(child) {
                return child;
            }
        }
        unreachable!()
    }

    /// Get parent node.
    ///
    /// Returns [`None`] if this is the document node or if the node is
    /// unattached to a document.
    ///
    /// Attribute and namespace nodes have a parent, even though they aren't
    /// children of the element they are in.
    ///
    /// ```rust
    /// use xoz::Document;
    /// let doc = Document::parse_str("<p>Example</p>").unwrap();
    /// let root = doc.root();
    /// let p = doc.document_element();
    /// let text = doc.first_child(p).unwrap();
    ///
    /// assert_eq!(doc.parent(text), Some(p));
    /// assert_eq!(doc.parent(p), Some(root));
    /// assert_eq!(doc.parent(root), None);
    /// ```
    pub fn parent(&self, node: Node) -> Option<Node> {
        // two strategies are possible: skipping the attributes and namespaces nodes
        // if found, or checking whether we are an attribute or namespace node before
        // we even try. I've chosen the first strategy.
        let parent = self.primitive_parent(node)?;
        if self.tag_id(parent).is_special() {
            // if the parent is an attribute or namespace node, we skip it
            self.primitive_parent(parent)
        } else {
            // if it's not, then it's a parent
            Some(parent)
        }
    }

    /// Get first child.
    ///
    /// Returns [`None`] if there are no children.
    ///
    /// ```rust
    /// let doc = xoz::Document::parse_str("<p>Example</p>").unwrap();
    /// let root = doc.root();
    /// let p = doc.document_element();
    /// let text = doc.first_child(p).unwrap();
    /// assert_eq!(doc.first_child(root), Some(p));
    /// assert_eq!(doc.first_child(p), Some(text));
    /// assert_eq!(doc.first_child(text), None);
    /// ```
    pub fn first_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node)?;
        let tag_id = self.tag_id(node);
        if tag_id.is_attributes() {
            // the first child is the attributes node, skip it
            self.next_sibling(node)
        } else if tag_id.is_namespaces() {
            // the first child is the namespaces node
            // check if the next sibling is the attributes node
            let next = self.next_sibling(node)?;
            // if so, the first child is the next sibling
            if self.tag_id(next).is_attributes() {
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
        if self.tag_id(child).is_special() {
            None
        } else {
            Some(child)
        }
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().next_sibling(node.0).map(Node)
    }

    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        let prev = self.primitive_previous_sibling(node)?;
        if self.tag_id(prev).is_special() {
            // attributes and namespaces nodes are not siblings
            None
        } else {
            Some(prev)
        }
    }

    pub(crate) fn attributes_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            let tag_id = self.tag_id(node);
            if tag_id.is_attributes() {
                // the first child is the attributes node
                Some(node)
            } else if tag_id.is_namespaces() {
                // the first child is the namespaces node, check for attributes node
                let next = self.next_sibling(node);
                next.filter(|next| self.tag_id(*next).is_attributes())
            } else {
                None
            }
        } else {
            None
        }
    }

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

    pub fn attribute_node(&self, node: Node, name: &TagName) -> Option<Node> {
        let attributes = self.attributes_child(node)?;
        for child in self.primitive_children(attributes) {
            if let TagType::Attribute(tag_name) = self.value(child) {
                if tag_name == name {
                    return Some(child);
                }
            }
        }
        None
    }

    pub fn attribute_value(&self, node: Node, name: &TagName) -> Option<&str> {
        let attribute_node = self.attribute_node(node, name)?;
        let text_id = self.structure.text_id(attribute_node.0);
        Some(self.text_usage.text_value(text_id))
    }

    pub fn node_name(&self, node: Node) -> Option<&TagName> {
        match self.value(node) {
            TagType::Element(tag_name) => Some(tag_name),
            TagType::Attribute(tag_name) => Some(tag_name),
            _ => None,
        }
    }

    pub fn value(&self, node: Node) -> &TagType {
        let tag_info = self.structure.get_tag(node.0);
        debug_assert!(tag_info.is_open_tag());
        tag_info.tag_type()
    }

    pub fn tag_id(&self, node: Node) -> TagId {
        self.structure.tag_id(node.0)
    }

    pub fn is_document(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Document)
    }

    pub fn is_element(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Element { .. })
    }

    pub fn is_text(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Text)
    }

    pub fn is_comment(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Comment)
    }

    pub fn is_processing_instruction(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::ProcessingInstruction)
    }

    pub fn is_attribute(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Attribute { .. })
    }

    pub fn is_namespace(&self, node: Node) -> bool {
        matches!(self.value(node), TagType::Namespace { .. })
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
            let tag_name = match self.value(n) {
                TagType::Attribute(tag_name) => tag_name,
                _ => unreachable!(),
            };
            (tag_name, value)
        })
    }

    pub fn namespace_entries(&self, node: Node) -> impl Iterator<Item = (&[u8], &[u8])> + use<'_> {
        NamespacesIter::new(self, node).map(move |n| match self.value(n) {
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
        if matches!(self.value(node), TagType::Text) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn comment_str(&self, node: Node) -> Option<&str> {
        if matches!(self.value(node), TagType::Comment) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction_str(&self, node: Node) -> Option<&str> {
        if matches!(self.value(node), TagType::ProcessingInstruction) {
            self.node_str(node)
        } else {
            None
        }
    }

    pub fn processing_instruction(&self, node: Node) -> Option<ProcessingInstruction> {
        if matches!(self.value(node), TagType::ProcessingInstruction) {
            let s = self.node_str(node).expect("Missing PI data");
            Some(ProcessingInstruction { data: s })
        } else {
            None
        }
    }

    pub fn string_value(&self, node: Node) -> String {
        match self.value(node) {
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
