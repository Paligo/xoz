#[test]
fn test_top_node() {
    let doc = xoz::Document::parse_str("<a><b/></a>").unwrap();
    let root = doc.root();
    let a = doc.document_element();
    let b = doc.first_child(a).unwrap();
    assert_eq!(doc.top_element(b), a);
    assert_eq!(doc.top_element(a), a);
    assert_eq!(doc.top_element(root), a);
}
