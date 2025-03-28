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

    pub(crate) fn following(&self, node: Node) -> Option<Node> {
        // otherwise, go up parent chain until we find a next sibling
        let mut current = node;
        while let Some(parent) = self.doc.parent(current) {
            if parent == self.root {
                return None;
            }
            let sibling = self.doc.next_sibling(parent);
            if let Some(sibling) = sibling {
                return Some(sibling);
            }
            current = parent;
        }
        None
    }
}

impl Iterator for DescendantsIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        let node = self.node?;
        self.node = if let Some(first_child) = self.doc.first_child(node) {
            Some(first_child)
        } else if let Some(sibling) = self.doc.next_sibling(node) {
            Some(sibling)
        } else {
            self.following(node)
        };
        Some(node)
    }
}

pub(crate) struct FollowingIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
}

impl<'a> FollowingIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
        Self {
            node: Self::following(doc, node),
            doc,
        }
    }

    fn following(doc: &Document, node: Node) -> Option<Node> {
        if let Some(next_sibling) = doc.next_sibling(node) {
            // if we have a next sibling, go there
            Some(next_sibling)
        } else {
            // otherwise, go up parent chain until we find a next sibling
            let mut current = node;
            while let Some(parent) = doc.parent(current) {
                let sibling = doc.next_sibling(parent);
                if let Some(sibling) = sibling {
                    return Some(sibling);
                }
                current = parent;
            }
            None
        }
    }
}

impl Iterator for FollowingIter<'_> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.node?;

        self.node = if let Some(first_child) = self.doc.first_child(node) {
            Some(first_child)
        } else {
            Self::following(self.doc, node)
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
