use crate::{
    document::{Document, Node},
    node_info_vec::NodeInfoId,
    NodeType,
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

pub(crate) struct ChildrenIter<'a> {
    doc: &'a Document,
    head: Option<Node>,
    tail: Option<Node>,
}

impl<'a> ChildrenIter<'a> {
    pub(crate) fn new(doc: &'a Document, parent: Node) -> Self {
        Self {
            doc,
            head: doc.first_child(parent),
            tail: doc.last_child(parent),
        }
    }
}

impl Iterator for ChildrenIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.head, self.tail) {
            (Some(head), Some(tail)) if head == tail => {
                self.head = None;
                self.tail = None;
                Some(head)
            }
            (Some(head), _) => {
                self.head = self.doc.next_sibling(head);
                Some(head)
            }
            _ => None,
        }
    }
}

impl DoubleEndedIterator for ChildrenIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        match (self.head, self.tail) {
            (Some(head), Some(tail)) if head == tail => {
                self.head = None;
                self.tail = None;
                Some(head)
            }
            (_, Some(tail)) => {
                self.tail = self.doc.previous_sibling(tail);
                Some(tail)
            }
            _ => None,
        }
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
            node: doc.parent(node),
            doc,
        }
    }
}

impl Iterator for AncestorIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.parent(node);
        Some(node)
    }
}

pub(crate) struct AttributesIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> AttributesIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
        let node = doc.attributes_child(node);
        let node = if let Some(node) = node {
            doc.first_child(node)
        } else {
            None
        };
        Self { doc, node }
    }
}

impl Iterator for AttributesIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.next_sibling(node);
        Some(node)
    }
}

pub(crate) struct NamespacesIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> NamespacesIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
        let node = doc.namespaces_child(node);
        let node = if let Some(node) = node {
            doc.first_child(node)
        } else {
            None
        };
        Self { doc, node }
    }
}

impl Iterator for NamespacesIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.next_sibling(node);
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

    // the first sibling, matching or not
    fn sibling(&self, node: Node) -> Option<Node>;

    // the first matching descendant (in document order)
    fn matching_descendant(&self, node: Node) -> Option<Node>;

    // matching self or the first matching descendant (in document order)
    fn matching_descendant_or_self(&self, node: Node) -> Option<Node>;

    fn matching_sibling_up(&self, node: Node) -> Option<Node> {
        let sibling = self.sibling_up(node)?;
        // if we have one, go for this node if it matches, or a matching descendant
        self.matching_descendant_or_self(sibling)
    }

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

    fn matching_rooted_sibling_up(&self, node: Node, root: Node) -> Option<Node> {
        let sibling = self.rooted_sibling_up(node, root)?;
        // if we have one, go for this node if it matches, or a matching descendant
        self.matching_descendant_or_self(sibling)
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

    fn sibling(&self, node: Node) -> Option<Node> {
        self.doc.next_sibling(node)
    }

    fn matching_descendant(&self, node: Node) -> Option<Node> {
        self.doc.first_child(node)
    }

    fn matching_descendant_or_self(&self, node: Node) -> Option<Node> {
        Some(node)
    }
}

pub(crate) struct DescendantsIter<'a> {
    doc: &'a Document,
    root: Node,
    node: Option<Node>,
}

impl<'a> DescendantsIter<'a> {
    pub(crate) fn new(doc: &'a Document, root: Node) -> Self {
        Self {
            root,
            node: doc.first_child(root),
            doc,
        }
    }
}

impl Iterator for DescendantsIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        let node = self.node?;
        self.node = if let Some(descendant) = self.doc.first_child(node) {
            Some(descendant)
        } else if let Some(sibling) = self.doc.next_sibling(node) {
            Some(sibling)
        } else {
            // go up one to the parent and take next sibling of parent
            let parent = self.doc.parent(node)?;
            if parent != self.root {
                self.doc.next_sibling(parent)
            } else {
                // we're done
                None
            }
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
            node: tree_ops.matching_sibling_up(node),
            ops: tree_ops,
        }
    }
}

impl<T: TreeOps> Iterator for FollowingIter<T> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;

        self.node = if let Some(descendant) = self.ops.matching_descendant(node) {
            Some(descendant)
        } else {
            self.ops.matching_sibling_up(node)
        };
        Some(node)
    }
}

pub(crate) struct WithTypedSelfIter<'a, I: Iterator<Item = Node>> {
    doc: &'a Document,
    node: Option<Node>,
    iter: I,
    node_info_id: NodeInfoId,
}

impl<'a, I> WithTypedSelfIter<'a, I>
where
    I: Iterator<Item = Node>,
{
    pub(crate) fn new(doc: &'a Document, node: Node, iter: I, node_info_id: NodeInfoId) -> Self {
        Self {
            doc,
            node: Some(node),
            iter,
            node_info_id,
        }
    }
}

impl<I> Iterator for WithTypedSelfIter<'_, I>
where
    I: Iterator<Item = Node>,
{
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.take() {
            if self.doc.node_info_id_for_node(node) == self.node_info_id {
                Some(node)
            } else {
                self.next()
            }
        } else {
            self.iter.next()
        }
    }
}

pub(crate) struct TypedDescendantsIter<'a> {
    doc: &'a Document,
    parent: Node,
    node: Option<Node>,
    node_info_id: NodeInfoId,
}

impl<'a> TypedDescendantsIter<'a> {
    pub(crate) fn new(doc: &'a Document, parent: Node, node_type: NodeType) -> Self {
        if let Some(node_info_id) = doc.node_info_id(node_type) {
            Self {
                doc,
                parent,
                node: doc.typed_descendant_by_node_info_id(parent, node_info_id),
                node_info_id,
            }
        } else {
            // if this node type doesn't even exist,
            // we return an iterator doing nothing
            Self {
                doc,
                parent,
                node: None,
                // some dummy node info id
                node_info_id: NodeInfoId::new(0),
            }
        }
    }
}

impl Iterator for TypedDescendantsIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        // look for the next typed descendant
        if let Some(node) = self
            .doc
            .typed_descendant_by_node_info_id(node, self.node_info_id)
        {
            // if it's there, we have a next node
            self.node = Some(node);
        } else {
            // otherwise look for a typed following node from node onward
            let node = self.doc.typed_foll_by_node_info_id(node, self.node_info_id);
            if let Some(node) = node {
                // if we have a following node, we need to check whether parent
                // is still an ancestor of it, or if we've escaped out of the subtree
                if self.doc.is_ancestor(self.parent, node) {
                    // if we're still in the subtree, we're done
                    self.node = Some(node);
                } else {
                    // if we're out of the subtree, we're done
                    self.node = None;
                };
            } else {
                // if we don't have a following node, we're done
                self.node = None;
            }
        }
        Some(node)
    }
}

pub(crate) struct TypedFollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    node_info_id: NodeInfoId,
}

impl<'a> TypedFollowingIter<'a> {
    pub(crate) fn new(doc: &'a Document, parent: Node, node_type: NodeType) -> Self {
        if let Some(node_info_id) = doc.node_info_id(node_type) {
            Self {
                doc,
                node: doc.typed_foll_by_node_info_id(parent, node_info_id),
                node_info_id,
            }
        } else {
            // if this node type doesn't even exist,
            // we return an iterator doing nothing
            Self {
                doc,
                node: None,
                // some dummy node info id
                node_info_id: NodeInfoId::new(0),
            }
        }
    }
}

impl Iterator for TypedFollowingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;
        self.node = self.doc.typed_foll_by_node_info_id(node, self.node_info_id);
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_document;

    #[test]
    fn test_double_ended_children() {
        let doc = parse_document("<doc><a/><b/><c/><d/><e/></doc>").unwrap();
        let _root = doc.root();
        let doc_elem = doc.document_element();
        let a = doc.first_child(doc_elem).unwrap();
        let b = doc.next_sibling(a).unwrap();
        let c = doc.next_sibling(b).unwrap();
        let d = doc.next_sibling(c).unwrap();
        let e = doc.next_sibling(d).unwrap();

        let mut iter = ChildrenIter::new(&doc, doc_elem);
        assert_eq!(iter.next(), Some(a));
        assert_eq!(iter.next_back(), Some(e));
        assert_eq!(iter.next(), Some(b));
        assert_eq!(iter.next_back(), Some(d));
        assert_eq!(iter.next(), Some(c));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
