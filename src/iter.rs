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

pub(crate) struct Descendants<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
}

impl<'a> Descendants<'a> {
    pub(crate) fn new(doc: &'a Document, root: Node) -> Self {
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

pub(crate) struct DescenderWrapper<T: Descender>(T);

impl<T> DescenderWrapper<T>
where
    T: Descender,
{
    pub(crate) fn new(descender: T) -> Self {
        Self(descender)
    }
}

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

pub(crate) struct FollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    descendant_iter: Option<WithSelfIter<DescenderWrapper<Descendants<'a>>>>,
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

pub(crate) struct TaggedDescendants<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
    tag_id: TagId,
}

impl<'a> TaggedDescendants<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node, tag_id: TagId) -> Self {
        Self {
            doc,
            root: node,
            node: Some(node),
            tag_id,
        }
    }
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
