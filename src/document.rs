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
        DescendantIter::new(self, node)
    }

    pub fn following(&self, node: Node) -> impl Iterator<Item = Node> + use<'_> {
        FollowingIter {
            doc: self,
            node: Some(node),
            descendant_iter: None,
        }
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

struct DescendantIter<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
}

impl<'a> DescendantIter<'a> {
    fn new(doc: &'a Document, root: Node) -> Self {
        Self {
            doc,
            root,
            node: Some(root),
        }
    }
}

impl Iterator for DescendantIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        // get the current node
        let node = self.node?;

        let first_child = self.doc.first_child(node);
        if let Some(first_child) = first_child {
            // if there is a first child, take it
            self.node = Some(first_child);
        } else {
            // if there is no first child, try to look for next sibling. if
            // it doesn't exist for current, go up the ancestor chain
            let mut current = node;
            loop {
                if current == self.root {
                    // we're done
                    self.node = None;
                    break;
                }
                if let Some(next_sibling) = self.doc.next_sibling(current) {
                    self.node = Some(next_sibling);
                    break;
                } else {
                    current = self
                        .doc
                        .parent(current)
                        .expect("We should have a parent for a descendant");
                }
            }
        }
        Some(node)
    }
}

struct FollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    descendant_iter: Option<DescendantIter<'a>>,
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
                    self.descendant_iter = Some(DescendantIter::new(self.doc, next_sibling));
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
