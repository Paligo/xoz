use xoz::NodeName;

#[test]
fn test_attribute_names() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let b = doc.attribute_node(doc_el, "b").unwrap();

    let a_name = doc.node_name(a).unwrap();
    let b_name = doc.node_name(b).unwrap();

    assert_eq!(a_name.local_name(), b"a");
    assert_eq!(b_name.local_name(), b"b");
}

#[test]
fn test_attribute_value() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_value(doc_el, "a");
    let b = doc.attribute_value(doc_el, "b");
    let c = doc.attribute_value(doc_el, "c");
    assert_eq!(a, Some("A"));
    assert_eq!(b, Some("B"));
    assert_eq!(c, None);
}

#[test]
fn test_attribute_entries() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let entries = doc.attribute_entries(doc_el).collect::<Vec<_>>();
    let node_name_a = NodeName::new("", "a");
    let node_name_b = NodeName::new("", "b");
    assert_eq!(entries, vec![(&node_name_a, "A"), (&node_name_b, "B")]);
}

#[test]
fn test_text_and_attribute_value() {
    let doc = xoz::Document::parse_str(r#"<doc a="A">text</doc>"#).unwrap();
    let doc_el = doc.document_element();
    let text = doc.first_child(doc_el).unwrap();
    let a = doc.attribute_value(doc_el, "a");
    assert_eq!(doc.text_str(text), Some("text"));
    assert_eq!(a, Some("A"));
}
