#[test]
fn test_text() {
    let doc = xoz::Document::parse_str(r#"<doc>text</doc>"#).unwrap();
    let doc_el = doc.document_element();
    let text = doc.first_child(doc_el).unwrap();
    assert_eq!(doc.text_str(text), Some("text"));
    assert_eq!(doc.text_str(doc_el), None);
}

#[test]
fn test_multiple_text() {
    let doc = xoz::Document::parse_str(r#"<doc><a>A</a><b>B</b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let a_text = doc.first_child(a).unwrap();
    let b_text = doc.first_child(b).unwrap();
    assert_eq!(doc.text_str(a_text), Some("A"));
    assert_eq!(doc.text_str(b_text), Some("B"));
}

#[test]
fn test_processing_instruction() {
    let doc = xoz::Document::parse_str(r#"<doc><?target content?></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();

    let pi = doc.processing_instruction(p).unwrap();
    assert_eq!(pi.target(), "target".to_string());
    assert_eq!(pi.content(), " content".to_string());
}
