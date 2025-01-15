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
    type Item;

    // the parent of a node
    fn parent(&self, node: Node) -> Option<Node>;

    // the first matching descendant (in document order)
    fn descendant(&self, node: Node) -> Option<Node>;
    // the next matching sibling (in document order)
    fn sibling(&self, node: Node) -> Option<Node>;
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
    type Item = Node;

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
    pub(crate) fn new(root: Node, node: Option<Node>, tree_ops: T) -> Self {
        Self {
            root,
            node,
            ops: tree_ops,
        }
    }

    fn sibling_up(&self, node: Node) -> Option<Node> {
        let mut current = node;
        loop {
            if current == self.root {
                // we're done
                return None;
            }
            if let Some(sibling) = self.ops.sibling(current) {
                return Some(sibling);
            } else {
                current = self
                    .ops
                    .parent(current)
                    .expect("We should have a parent for a descendant");
            }
        }
    }
}

impl<T: TreeOps> Iterator for DescendantsIter<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;

        let descendant = self.ops.descendant(node);
        self.node = if let Some(descendant) = descendant {
            // if there is a first child, take it
            Some(descendant)
        } else {
            self.sibling_up(node)
        };
        Some(node)
    }
}

// pub(crate) struct FollowingIter<T: TreeOps>(T);

// impl<T: TreeOps> Iterator for FollowingIter<T> {
//     type Item = Node;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }

// impl<T> FollowingIter<T>
// where
//     T: TreeOps,
// {
//     pub(crate) fn new(tree_ops: T) -> Self {
//         Self(tree_ops)
//     }
// }

pub(crate) struct FollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    descendant_iter: Option<WithSelfIter<DescendantsIter<NodeTreeOps<'a>>>>,
}

impl<'a> FollowingIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
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
    type Item = Node;

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
