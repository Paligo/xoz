use xoz::parse_document;

#[test]
fn test_elements() {
    let doc = parse_document(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();

    let doc_el_name = doc.node_name(doc_el).unwrap();
    let a_name = doc.node_name(a).unwrap();
    let b_name = doc.node_name(b).unwrap();

    assert_eq!(doc_el_name.local_name(), "doc");
    assert_eq!(a_name.local_name(), "a");
    assert_eq!(b_name.local_name(), "b");
}

#[test]
fn test_elements_multiple_a() {
    let doc = parse_document(r#"<doc><a/><a/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a1 = doc.first_child(doc_el).unwrap();
    let a2 = doc.next_sibling(a1).unwrap();

    let doc_el_name = doc.node_name(doc_el).unwrap();
    let a1_name = doc.node_name(a1).unwrap();
    let a2_name = doc.node_name(a2).unwrap();

    assert_eq!(doc_el_name.local_name(), "doc");
    assert_eq!(a1_name.local_name(), "a");
    assert_eq!(a2_name.local_name(), "a");
}
