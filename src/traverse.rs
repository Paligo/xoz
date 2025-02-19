use crate::{
    document::{Document, Node},
    TagInfo, TagType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Open,
    Close,
    Empty,
}

struct Traverse<'a> {
    doc: &'a Document,
    node: Option<Node>,
    stack: Vec<Node>,
}

impl<'a> Traverse<'a> {
    fn new(doc: &'a Document, node: Node) -> Self {
        Self {
            doc,
            node: Some(node),
            stack: Vec::new(),
        }
    }
}

impl<'a> Iterator for Traverse<'a> {
    type Item = (&'a TagType<'a>, State, Node);
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
                    Some((self.doc.value(node), State::Close, node))
                } else {
                    None
                }
            }
            Some(node) => {
                let open_close = if let Some(child) = self.doc.first_child(node) {
                    self.stack.push(node);
                    self.node = Some(child);
                    State::Open
                } else {
                    self.node = self.doc.next_sibling(node);
                    State::Empty
                };
                Some((self.doc.value(node), open_close, node))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{builder::parse_document, TagName};

    use super::*;

    #[test]
    fn test_single_element() {
        let doc = parse_document("<a/>").unwrap();
        let a = doc.document_element();
        let mut traverse = Traverse::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "a")), State::Empty, a))
        );
        assert_eq!(traverse.next(), None);
    }

    #[test]
    fn test_one_child() {
        let doc = parse_document("<a><b/></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let mut traverse = Traverse::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "a")), State::Open, a))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "b")), State::Empty, b))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "a")), State::Close, a))
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

        let mut traverse = Traverse::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "a")), State::Open, a))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "b")), State::Empty, b))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "c")), State::Empty, c))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "d")), State::Empty, d))
        );
        assert_eq!(
            traverse.next(),
            Some((&TagType::Element(TagName::new("", "a")), State::Close, a))
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

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (&TagType::Element(TagName::new("", "a")), State::Open, a),
                (&TagType::Element(TagName::new("", "b")), State::Empty, b),
                (&TagType::Element(TagName::new("", "c")), State::Empty, c),
                (&TagType::Element(TagName::new("", "d")), State::Empty, d),
                (&TagType::Element(TagName::new("", "a")), State::Close, a),
            ]
        )
    }

    #[test]
    fn test_deeper() {
        let doc = parse_document("<a><b><c/></b></a>").unwrap();
        let a = doc.document_element();
        let b = doc.first_child(a).unwrap();
        let c = doc.first_child(b).unwrap();

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (&TagType::Element(TagName::new("", "a")), State::Open, a),
                (&TagType::Element(TagName::new("", "b")), State::Open, b),
                (&TagType::Element(TagName::new("", "c")), State::Empty, c),
                (&TagType::Element(TagName::new("", "b")), State::Close, b),
                (&TagType::Element(TagName::new("", "a")), State::Close, a),
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

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (&TagType::Element(TagName::new("", "a")), State::Open, a),
                (&TagType::Element(TagName::new("", "b")), State::Open, b),
                (&TagType::Element(TagName::new("", "c")), State::Empty, c),
                (&TagType::Element(TagName::new("", "d")), State::Empty, d),
                (&TagType::Element(TagName::new("", "b")), State::Close, b),
                (&TagType::Element(TagName::new("", "e")), State::Empty, e),
                (&TagType::Element(TagName::new("", "a")), State::Close, a),
            ]
        )
    }

    #[test]
    fn test_text() {
        let doc = parse_document("<a>text</a>").unwrap();
        let a = doc.document_element();
        let text = doc.first_child(a).unwrap();

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (&TagType::Element(TagName::new("", "a")), State::Open, a),
                (&TagType::Text, State::Empty, text),
                (&TagType::Element(TagName::new("", "a")), State::Close, a),
            ]
        )
    }

    #[test]
    fn test_attributes() {
        let doc = parse_document(r#"<a b="B" c="C"/>"#).unwrap();
        let a = doc.document_element();

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![(&TagType::Element(TagName::new("", "a")), State::Empty, a),]
        )
    }
}
