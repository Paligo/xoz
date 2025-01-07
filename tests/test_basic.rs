use xoz::parse_document;

#[test]
fn test_elements() {
    let doc = parse_document(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    // dbg!(doc.node_value(doc_el));
    // dbg!(doc.node_value(a));
    // dbg!(doc.node_value(b));

    // let doc_el_name = doc.node_name(doc_el).unwrap();
    // let a_name = doc.node_name(a).unwrap();
    // let b_name = doc.node_name(b).unwrap();
    // assert_eq!(doc_el_name.local_name(), "doc");
    // assert_eq!(a_name.local_name(), "a");
    // assert_eq!(b_name.local_name(), "b");
}
