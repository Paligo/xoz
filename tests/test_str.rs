use xoz::Xoz;

#[test]
fn test_text() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc>text</doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let text = xoz.first_child(doc_el).unwrap();
    assert_eq!(xoz.text_str(text), Some("text"));
    assert_eq!(xoz.text_str(doc_el), None);
}

#[test]
fn test_multiple_text() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a>A</a><b>B</b></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let a_text = xoz.first_child(a).unwrap();
    let b_text = xoz.first_child(b).unwrap();
    assert_eq!(xoz.text_str(a_text), Some("A"));
    assert_eq!(xoz.text_str(b_text), Some("B"));
}

#[test]
fn test_processing_instruction() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><?target content?></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let p = xoz.first_child(doc_el).unwrap();

    let pi = xoz.processing_instruction(p).unwrap();
    assert_eq!(pi.target(), b"target");
    assert_eq!(pi.content(), b" content");
}
