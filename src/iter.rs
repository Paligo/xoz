use crate::{
    document::{Document, Node},
    TagId, TagType,
};

pub(crate) struct NextSiblingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> NextSiblingIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Option<Node>) -> Self {
        Self { doc, node }
    }
}

impl Iterator for NextSiblingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.next_sibling(node);
        Some(node)
    }
}

pub(crate) struct PreviousSiblingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> PreviousSiblingIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Option<Node>) -> Self {
        Self { doc, node }
    }
}

impl Iterator for PreviousSiblingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.previous_sibling(node);
        Some(node)
    }
}

pub(crate) struct AncestorIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> AncestorIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
        Self {
            doc,
            node: Some(node),
        }
    }
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

pub(crate) struct WithSelfIter<I: Iterator<Item = Node>> {
    node: Option<Node>,
    iter: I,
}

impl<I> WithSelfIter<I>
where
    I: Iterator<Item = Node>,
{
    pub(crate) fn new(node: Node, iter: I) -> Self {
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

pub(crate) trait TreeOps {
    // the parent of a node
    fn parent(&self, node: Node) -> Option<Node>;

    // the first matching descendant (in document order)
    fn descendant(&self, node: Node) -> Option<Node>;
    // the next matching sibling (in document order)
    fn sibling(&self, node: Node) -> Option<Node>;

    fn sibling_up(&self, node: Node) -> Option<Node> {
        let mut current = node;
        loop {
            if let Some(sibling) = self.sibling(current) {
                return Some(sibling);
            } else if let Some(parent) = self.parent(current) {
                current = parent;
            } else {
                return None;
            }
        }
    }

    fn rooted_sibling_up(&self, node: Node, root: Node) -> Option<Node> {
        let mut current = node;
        loop {
            if current == root {
                // we're done
                return None;
            }
            if let Some(sibling) = self.sibling(current) {
                return Some(sibling);
            } else {
                current = self
                    .parent(current)
                    .expect("We should have a parent for a descendant");
            }
        }
    }
}

pub(crate) struct NodeTreeOps<'a> {
    doc: &'a Document,
}

impl<'a> NodeTreeOps<'a> {
    pub(crate) fn new(doc: &'a Document) -> Self {
        Self { doc }
    }
}

impl TreeOps for NodeTreeOps<'_> {
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

pub(crate) struct DescendantsIter<T: TreeOps> {
    root: Node,
    node: Option<Node>,
    ops: T,
}

impl<T> DescendantsIter<T>
where
    T: TreeOps,
{
    pub(crate) fn new(root: Node, tree_ops: T) -> Self {
        Self {
            root,
            node: tree_ops.descendant(root),
            ops: tree_ops,
        }
    }
}

impl<T: TreeOps> Iterator for DescendantsIter<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        let node = self.node?;
        self.node = if let Some(descendant) = self.ops.descendant(node) {
            Some(descendant)
        } else {
            self.ops.rooted_sibling_up(node, self.root)
        };
        Some(node)
    }
}

pub(crate) struct FollowingIter<T: TreeOps> {
    node: Option<Node>,
    ops: T,
}

impl<T> FollowingIter<T>
where
    T: TreeOps,
{
    pub(crate) fn new(node: Node, tree_ops: T) -> Self {
        Self {
            node: tree_ops.sibling_up(node),
            ops: tree_ops,
        }
    }
}

impl<T: TreeOps> Iterator for FollowingIter<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;

        self.node = if let Some(descendant) = self.ops.descendant(node) {
            Some(descendant)
        } else {
            self.ops.sibling_up(node)
        };
        Some(node)
    }
}

pub(crate) struct TaggedTreeOps<'a> {
    doc: &'a Document,
    tag_id: TagId,
}

impl<'a> TaggedTreeOps<'a> {
    pub(crate) fn new(doc: &'a Document, tag_id: TagId) -> Self {
        Self { doc, tag_id }
    }
}

impl TreeOps for TaggedTreeOps<'_> {
    fn parent(&self, node: Node) -> Option<Node> {
        self.doc.parent(node)
    }

    fn descendant(&self, node: Node) -> Option<Node> {
        self.doc.tagged_descendant(node, self.tag_id)
    }

    fn sibling(&self, node: Node) -> Option<Node> {
        // TODO: does a tagged_sibling exist?
        while let Some(next_sibling) = self.doc.next_sibling(node) {
            if self.doc.tag_id(next_sibling) == self.tag_id {
                return Some(next_sibling);
            }
        }
        None
    }
}

pub(crate) struct WithTaggedSelfIter<'a, I: Iterator<Item = Node>> {
    doc: &'a Document,
    node: Option<Node>,
    iter: I,
    tag_id: TagId,
}

impl<'a, I> WithTaggedSelfIter<'a, I>
where
    I: Iterator<Item = Node>,
{
    pub(crate) fn new(doc: &'a Document, node: Node, iter: I, tag_id: TagId) -> Self {
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
