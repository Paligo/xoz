use crate::{
    document::{Document, Node},
    TagInfo, TagType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum OpenClose {
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
    type Item = (OpenClose, &'a TagType<'a>, Node);
    fn next(&mut self) -> Option<Self::Item> {
        // we traverse down the tree, taking the first child when we can,
        // putting the parent on the stack when we do so. This is an open tag.
        // when we cannot go further down, we take the next sibling of the
        // current node. when we cannot go further with next sibling, we pop
        // the stack. if we can pop the stack immediately after going down,
        // this node is empty. otherwise we pop a close for this node.

        match self.node {
            None => {
                if let Some(node) = self.stack.pop() {
                    self.node = self.doc.next_sibling(node);
                    Some((OpenClose::Close, self.doc.value(node), node))
                } else {
                    None
                }
            }
            Some(node) => {
                let open_close = if let Some(child) = self.doc.first_child(node) {
                    self.stack.push(node);
                    self.node = Some(child);
                    OpenClose::Open
                } else if let Some(sibling) = self.doc.next_sibling(node) {
                    self.node = Some(sibling);
                    OpenClose::Empty
                } else {
                    self.node = None;
                    OpenClose::Empty
                };
                Some((open_close, self.doc.value(node), node))
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
            Some((
                OpenClose::Empty,
                &TagType::Element(TagName::new("", "a")),
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
        let mut traverse = Traverse::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((OpenClose::Open, &TagType::Element(TagName::new("", "a")), a))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Empty,
                &TagType::Element(TagName::new("", "b")),
                b
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Close,
                &TagType::Element(TagName::new("", "a")),
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

        let mut traverse = Traverse::new(&doc, a);
        assert_eq!(
            traverse.next(),
            Some((OpenClose::Open, &TagType::Element(TagName::new("", "a")), a))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Empty,
                &TagType::Element(TagName::new("", "b")),
                b
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Empty,
                &TagType::Element(TagName::new("", "c")),
                c
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Empty,
                &TagType::Element(TagName::new("", "d")),
                d
            ))
        );
        assert_eq!(
            traverse.next(),
            Some((
                OpenClose::Close,
                &TagType::Element(TagName::new("", "a")),
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

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (OpenClose::Open, &TagType::Element(TagName::new("", "a")), a),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "b")),
                    b
                ),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "c")),
                    c
                ),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "d")),
                    d
                ),
                (
                    OpenClose::Close,
                    &TagType::Element(TagName::new("", "a")),
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

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (OpenClose::Open, &TagType::Element(TagName::new("", "a")), a),
                (OpenClose::Open, &TagType::Element(TagName::new("", "b")), b),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "c")),
                    c
                ),
                (
                    OpenClose::Close,
                    &TagType::Element(TagName::new("", "b")),
                    b
                ),
                (
                    OpenClose::Close,
                    &TagType::Element(TagName::new("", "a")),
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

        let traverse = Traverse::new(&doc, a).collect::<Vec<_>>();
        assert_eq!(
            traverse,
            vec![
                (OpenClose::Open, &TagType::Element(TagName::new("", "a")), a),
                (OpenClose::Open, &TagType::Element(TagName::new("", "b")), b),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "c")),
                    c
                ),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "d")),
                    d
                ),
                (
                    OpenClose::Close,
                    &TagType::Element(TagName::new("", "b")),
                    b
                ),
                (
                    OpenClose::Empty,
                    &TagType::Element(TagName::new("", "e")),
                    e
                ),
                (
                    OpenClose::Close,
                    &TagType::Element(TagName::new("", "a")),
                    a
                ),
            ]
        )
    }
}
