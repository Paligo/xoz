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
            if let TagType::Element { .. } = self.value(child) {
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
        match self.value(parent) {
            // if the parent is an attribute or namespace node, we skip it
            TagType::Attributes | TagType::Namespaces => self.primitive_parent(parent),
            // if it's not, then it's a parent
            _ => Some(parent),
        }
    }

    pub fn first_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node)?;
        match self.value(node) {
            // the first child is the attributes node, skip it
            TagType::Attributes => self.next_sibling(node),
            // the first child is the namespaces node
            TagType::Namespaces => {
                // check if the next sibling is the attributes node
                let next = self.next_sibling(node)?;
                // if so, the first child is the next sibling
                if let TagType::Attributes = self.value(next) {
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

    pub fn last_child(&self, node: Node) -> Option<Node> {
        let child = self.primitive_last_child(node)?;
        match self.value(child) {
            TagType::Attributes | TagType::Namespaces => None,
            _ => Some(child),
        }
    }

    pub fn next_sibling(&self, node: Node) -> Option<Node> {
        self.structure.tree().next_sibling(node.0).map(Node)
    }

    pub fn previous_sibling(&self, node: Node) -> Option<Node> {
        let prev = self.primitive_previous_sibling(node)?;
        match self.value(prev) {
            // the previous sibling is the attributes node, we are at the beginning
            TagType::Attributes => None,
            // the previous sibling is the namespaces node, we're at the beginning too
            TagType::Namespaces => None,
            // if it's not a special node, then it's definitely a previous sibling
            _ => Some(prev),
        }
    }

    pub(crate) fn attributes_child(&self, node: Node) -> Option<Node> {
        let node = self.primitive_first_child(node);
        if let Some(node) = node {
            match self.value(node) {
                // the first child is the attributes node
                TagType::Attributes => Some(node),
                // the first child is the namespaces node, check for attributes node
                TagType::Namespaces => {
                    let next = self.next_sibling(node);
                    next.filter(|next| matches!(self.value(*next), TagType::Attributes))
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
            if let TagType::Attribute {
                namespace,
                local_name,
            } = self.value(child)
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
        match self.value(node) {
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

    pub fn child_index(&self, node: Node) -> Option<usize> {
        let parent = self.parent(node)?;
        for (i, child) in self.children(parent).enumerate() {
            if child == node {
                return Some(i);
            }
        }
        None
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

    pub fn preceding_siblings(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        PreviousSiblingIter {
            doc: self,
            node: Some(node),
        }
    }

    pub fn ancestors(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        AncestorIter {
            doc: self,
            node: Some(node),
        }
    }

    pub fn descendants(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        self.descendants_iter(node)
    }

    fn descendants_iter(&self, node: Node) -> WithSelfIter<DescenderWrapper<Descendants>> {
        WithSelfIter::new(node, DescenderWrapper(Descendants::new(self, node)))
    }

    pub fn tagged_descendants(
        &self,
        node: Node,
        tag_id: TagId,
    ) -> impl Iterator<Item = Node> + use<'_> {
        WithTaggedSelfIter::new(
            self,
            node,
            DescenderWrapper(TaggedDescendants::new(self, node, tag_id)),
            tag_id,
        )
    }

    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter::new(self, node)
    }

    pub fn text_str(&self, node: Node) -> Option<&str> {
        if matches!(self.value(node), TagType::Text) {
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

    pub(crate) fn primitive_last_child(&self, node: Node) -> Option<Node> {
        self.structure.tree().last_child(node.0).map(Node)
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

struct PreviousSiblingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl Iterator for PreviousSiblingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.previous_sibling(node);
        Some(node)
    }
}

struct AncestorIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl Iterator for AncestorIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        let new_node = self.doc.parent(node);
        if let Some(new_node) = new_node {
            match self.doc.value(new_node) {
                TagType::Attributes | TagType::Namespaces => {
                    // skip the attributes and namespaces nodes
                    self.node = self.doc.parent(new_node);
                }
                _ => {
                    // if it's not a special node, then it's a parent
                    self.node = Some(new_node);
                }
            }
        } else {
            self.node = None;
        }
        Some(node)
    }
}

struct WithSelfIter<I: Iterator<Item = Node>> {
    node: Option<Node>,
    iter: I,
}

impl<I> WithSelfIter<I>
where
    I: Iterator<Item = Node>,
{
    fn new(node: Node, iter: I) -> Self {
        Self {
            node: Some(node),
            iter,
        }
    }
}

impl<I> Iterator for WithSelfIter<I>
where
    I: Iterator<Item = Node>,
{
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            Some(node)
        } else {
            self.iter.next()
        }
    }
}

struct Descendants<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
}

impl<'a> Descendants<'a> {
    fn new(doc: &'a Document, root: Node) -> Self {
        Self {
            doc,
            root,
            node: Some(root),
        }
    }
}

// a descender defines how we descend in the tree
trait Descender {
    type Item;

    // the root node
    fn root(&self) -> Node;

    // the current node
    fn node(&self) -> Option<Node>;
    // update the current node
    fn set_node(&mut self, node: Option<Node>);

    // the parent of a node
    fn parent(&self, node: Node) -> Option<Node>;

    // the first matching descendant (in document order)
    fn descendant(&self, node: Node) -> Option<Node>;
    // the next matching sibling (in document order)
    fn sibling(&self, node: Node) -> Option<Node>;
}

impl Descender for Descendants<'_> {
    type Item = Node;

    fn root(&self) -> Self::Item {
        self.root
    }

    fn node(&self) -> Option<Node> {
        self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn parent(&self, node: Node) -> Option<Node> {
        self.doc.parent(node)
    }

    fn descendant(&self, node: Node) -> Option<Node> {
        self.doc.first_child(node)
    }

    fn sibling(&self, node: Node) -> Option<Node> {
        self.doc.next_sibling(node)
    }
}

struct DescenderWrapper<T: Descender>(T);

impl<T: Descender> Iterator for DescenderWrapper<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let s = &mut self.0;
        // get the current node
        let node = s.node()?;

        let first_child = s.descendant(node);
        if let Some(first_child) = first_child {
            // if there is a first child, take it
            self.0.set_node(Some(first_child));
            Some(first_child)
        } else {
            // if there is no first child, try to look for next sibling. if
            // it doesn't exist for current, go up the ancestor chain
            let mut current = node;
            loop {
                if current == s.root() {
                    // we're done
                    s.set_node(None);
                    return None;
                }
                if let Some(next_sibling) = s.sibling(current) {
                    s.set_node(Some(next_sibling));
                    return Some(next_sibling);
                } else {
                    current = s
                        .parent(current)
                        .expect("We should have a parent for a descendant");
                }
            }
        }
    }
}

struct FollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    descendant_iter: Option<WithSelfIter<DescenderWrapper<Descendants<'a>>>>,
}

impl<'a> FollowingIter<'a> {
    fn new(doc: &'a Document, node: Node) -> Self {
        Self {
            doc,
            node: Some(node),
            descendant_iter: None,
        }
    }
}

impl Iterator for FollowingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(descendant_iter) = &mut self.descendant_iter {
            // we if we have a descendant iter, keep getting nodes from it until
            // it's empty
            let next = descendant_iter.next();
            if let Some(next) = next {
                Some(next)
            } else {
                // if it's empty, get the next item using the normal strategy
                self.descendant_iter = None;
                self.next()
            }
        } else if let Some(node) = self.node {
            // if there is no descendant iter, try to look for next sibling. if
            // it doesn't exist for current, go up the ancestor chain
            let mut current = node;
            loop {
                if let Some(next_sibling) = self.doc.next_sibling(current) {
                    self.node = Some(next_sibling);
                    self.descendant_iter = Some(self.doc.descendants_iter(next_sibling));
                    return self.next();
                } else {
                    let parent = self.doc.parent(current);
                    if let Some(parent) = parent {
                        current = parent;
                    } else {
                        self.node = None;
                        return None;
                    }
                }
            }
        } else {
            // if there is no more parent, we're done
            None
        }
    }
}

struct TaggedDescendants<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
    tag_id: TagId,
}

impl Descender for TaggedDescendants<'_> {
    type Item = Node;

    fn root(&self) -> Self::Item {
        self.root
    }

    fn node(&self) -> Option<Node> {
        self.node
    }

    fn set_node(&mut self, node: Option<Node>) {
        self.node = node;
    }

    fn parent(&self, node: Node) -> Option<Node> {
        self.doc.parent(node)
    }

    fn descendant(&self, node: Node) -> Option<Node> {
        self.doc.tagged_descendant(node, self.tag_id)
    }

    fn sibling(&self, node: Node) -> Option<Node> {
        while let Some(next_sibling) = self.doc.next_sibling(node) {
            if self.doc.tag_id(next_sibling) == self.tag_id {
                return Some(next_sibling);
            }
        }
        None
    }
}

impl<'a> TaggedDescendants<'a> {
    fn new(doc: &'a Document, node: Node, tag_id: TagId) -> Self {
        Self {
            doc,
            root: node,
            node: Some(node),
            tag_id,
        }
    }
}

struct WithTaggedSelfIter<'a, I: Iterator<Item = Node>> {
    doc: &'a Document,
    node: Option<Node>,
    iter: I,
    tag_id: TagId,
}

impl<'a, I> WithTaggedSelfIter<'a, I>
where
    I: Iterator<Item = Node>,
{
    fn new(doc: &'a Document, node: Node, iter: I, tag_id: TagId) -> Self {
        Self {
            doc,
            node: Some(node),
            iter,
            tag_id,
        }
    }
}

impl<I> Iterator for WithTaggedSelfIter<'_, I>
where
    I: Iterator<Item = Node>,
{
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if self.doc.tag_id(node) == self.tag_id {
                Some(node)
            } else {
                self.next()
            }
        } else {
            self.iter.next()
        }
    }
}
