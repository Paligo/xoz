use xoz::{NodeType, Xoz};

#[test]
fn test_preorder() {
    let mut xoz = Xoz::new();

    let root = xoz.parse_str(r#"<doc><a/><b><c/><d/></b></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(c).unwrap();
    // now put nodes in arbitrary order
    let mut nodes = [b, d, c, a, doc_el];
    // sort them by preorder
    nodes.sort_by_key(|&node| xoz.preorder(node));
    assert_eq!(nodes, [doc_el, a, b, c, d]);
}

#[test]
fn test_subtree_tags() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><a/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    assert_eq!(
        xoz.subtree_count(doc_el, NodeType::element("a")),
        2
    );
}

#[test]
fn test_subtree_tags_root() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><a/></doc>"#).unwrap();
    assert_eq!(xoz.subtree_count(root, NodeType::element("a")), 2);
}

#[test]
fn test_subtree_tags_deeper() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><b><a/></b><a/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    assert_eq!(
        xoz.subtree_count(doc_el, NodeType::element("a")),
        2
    );
}
