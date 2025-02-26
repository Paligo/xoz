use crate::{
    document::{Document, Node},
    NodeType,
};

/// The state of a node when we traverse through it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraverseState {
    /// Open [`NodeType::Document`] or [`NodeType::Element`], when those have children.
    /// Example: `<a>`
    Open,
    /// Close [`NodeType::Document`] or [`NodeType::Element`], when those have children.
    /// Example: `</a>`
    Close,
    /// A node without children. Any other node type that can be traversed, as well
    /// as empty elements like `<a/>`.
    Empty,
}

pub(crate) struct TraverseIter<'a> {
    doc: &'a Document,
    node: Option<Node>,
    stack: Vec<Node>,
}

impl<'a> TraverseIter<'a> {
    pub(crate) fn new(doc: &'a Document, node: Node) -> Self {
        Self {
            doc,
            node: Some(node),
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for TraverseIter<'a> {
    type Item = (&'a NodeType<'a>, TraverseState, Node);
    fn next(&mut self) -> Option<Self::Item> {
        // we traverse down the tree, taking the first child when we can,
        // putting the parent on the stack when we do so. This is an open tag.
        // When we cannot go further down, we take the next sibling of the
        // current node. When we cannot go further with next sibling, we pop
        // the stack. We know a child is empty if there is no first child.
        match self.node {
            None => {
                if let Some(node) = self.stack.pop() {
                    self.node = self.doc.next_sibling(node);
                    Some((self.doc.node_type(node), TraverseState::Close, node))
                } else {
                    None
                }
            }
            Some(node) => {
                let open_close = if let Some(child) = self.doc.first_child(node) {
                    self.stack.push(node);
                    self.node = Some(child);
                    TraverseState::Open
                } else {
                    self.node = self.doc.next_sibling(node);
                    TraverseState::Empty
                };
                Some((self.doc.node_type(node), open_close, node))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::parse_document, NodeName};

    use super::*;

    #[test]
    fn test_single_element() {
        let doc = parse_document("<a/>").unwrap();
        let a = doc.document_element();
        let mut traverse = TraverseIter::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Empty,
                a
            ))
        );
        assert_eq!(traverse.next(), None);
    }

    #[test]
    fn test_one_child() {
        let doc = parse_document("<a><b/></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let mut traverse = TraverseIter::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Open,
                a
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "b")),
                TraverseState::Empty,
                b
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Close,
                a
            ))
        );
        assert_eq!(traverse.next(), None);
    }

    #[test]
    fn test_multiple_children() {
        let doc = parse_document("<a><b/><c/><d/></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let c = doc.next_sibling(b).unwrap();
        let d = doc.next_sibling(c).unwrap();

        let mut traverse = TraverseIter::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Open,
                a
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "b")),
                TraverseState::Empty,
                b
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "c")),
                TraverseState::Empty,
                c
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "d")),
                TraverseState::Empty,
                d
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Close,
                a
            ))
        );
        assert_eq!(traverse.next(), None);
    }

    #[test]
    fn test_multiple_children2() {
        let doc = parse_document("<a><b/><c/><d/></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let c = doc.next_sibling(b).unwrap();
        let d = doc.next_sibling(c).unwrap();

        let traverse = TraverseIter::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Open,
                    a
                ),
                (
                    &NodeType::Element(NodeName::new("", "b")),
                    TraverseState::Empty,
                    b
                ),
                (
                    &NodeType::Element(NodeName::new("", "c")),
                    TraverseState::Empty,
                    c
                ),
                (
                    &NodeType::Element(NodeName::new("", "d")),
                    TraverseState::Empty,
                    d
                ),
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Close,
                    a
                ),
            ]
        )
    }

    #[test]
    fn test_deeper() {
        let doc = parse_document("<a><b><c/></b></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let c = doc.first_child(b).unwrap();

        let traverse = TraverseIter::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Open,
                    a
                ),
                (
                    &NodeType::Element(NodeName::new("", "b")),
                    TraverseState::Open,
                    b
                ),
                (
                    &NodeType::Element(NodeName::new("", "c")),
                    TraverseState::Empty,
                    c
                ),
                (
                    &NodeType::Element(NodeName::new("", "b")),
                    TraverseState::Close,
                    b
                ),
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Close,
                    a
                ),
            ]
        )
    }

    #[test]
    fn test_nesting() {
        let doc = parse_document("<a><b><c/><d/></b><e/></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let c = doc.first_child(b).unwrap();
        let d = doc.next_sibling(c).unwrap();
        let e = doc.next_sibling(b).unwrap();

        let traverse = TraverseIter::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Open,
                    a
                ),
                (
                    &NodeType::Element(NodeName::new("", "b")),
                    TraverseState::Open,
                    b
                ),
                (
                    &NodeType::Element(NodeName::new("", "c")),
                    TraverseState::Empty,
                    c
                ),
                (
                    &NodeType::Element(NodeName::new("", "d")),
                    TraverseState::Empty,
                    d
                ),
                (
                    &NodeType::Element(NodeName::new("", "b")),
                    TraverseState::Close,
                    b
                ),
                (
                    &NodeType::Element(NodeName::new("", "e")),
                    TraverseState::Empty,
                    e
                ),
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Close,
                    a
                ),
            ]
        )
    }

    #[test]
    fn test_text() {
        let doc = parse_document("<a>text</a>").unwrap();
        let a = doc.document_element();
        let text = doc.first_child(a).unwrap();

        let traverse = TraverseIter::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Open,
                    a
                ),
                (&NodeType::Text, TraverseState::Empty, text),
                (
                    &NodeType::Element(NodeName::new("", "a")),
                    TraverseState::Close,
                    a
                ),
            ]
        )
    }

    #[test]
    fn test_attributes() {
        let doc = parse_document(r#"<a b="B" c="C"/>"#).unwrap();
        let a = doc.document_element();

        let traverse = TraverseIter::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![(
                &NodeType::Element(NodeName::new("", "a")),
                TraverseState::Empty,
                a
            ),]
        )
    }
}
