use xoz::NodeType;

#[test]
fn test_preorder() {
    let doc = xoz::Document::parse_str(r#"<doc><a/><b><c/><d/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(c).unwrap();
    // now put nodes in arbitrary order
    let mut nodes = [b, d, c, a, doc_el];
    // sort them by preorder
    nodes.sort_by_key(|&node| doc.preorder(node));
    assert_eq!(nodes, [doc_el, a, b, c, d]);
}

#[test]
fn test_subtree_tags() {
    let doc = xoz::Document::parse_str(r#"<doc><a/><a/></doc>"#).unwrap();
    assert_eq!(
        doc.subtree_count(doc.document_element(), NodeType::element("a")),
        2
    );
}

#[test]
fn test_subtree_tags_root() {
    let doc = xoz::Document::parse_str(r#"<doc><a/><a/></doc>"#).unwrap();
    assert_eq!(doc.subtree_count(doc.root(), NodeType::element("a")), 2);
}

#[test]
fn test_subtree_tags_deeper() {
    let doc = xoz::Document::parse_str(r#"<doc><b><a/></b><a/></doc>"#).unwrap();
    assert_eq!(
        doc.subtree_count(doc.document_element(), NodeType::element("a")),
        2
    );
}
