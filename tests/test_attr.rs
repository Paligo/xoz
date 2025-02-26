use xoz::{NodeName, Xoz};

#[test]
fn test_attribute_names() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_node(doc_el, "a").unwrap();
    let b = xoz.attribute_node(doc_el, "b").unwrap();

    let a_name = xoz.node_name(a).unwrap();
    let b_name = xoz.node_name(b).unwrap();

    assert_eq!(a_name.local_name(), b"a");
    assert_eq!(b_name.local_name(), b"b");
}

#[test]
fn test_attribute_value() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_value(doc_el, "a");
    let b = xoz.attribute_value(doc_el, "b");
    let c = xoz.attribute_value(doc_el, "c");
    assert_eq!(a, Some("A"));
    assert_eq!(b, Some("B"));
    assert_eq!(c, None);
}

#[test]
fn test_attribute_entries() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let entries = xoz.attribute_entries(doc_el).collect::<Vec<_>>();
    let node_name_a = NodeName::new("", "a");
    let node_name_b = NodeName::new("", "b");
    assert_eq!(entries, vec![(&node_name_a, "A"), (&node_name_b, "B")]);
}

#[test]
fn test_text_and_attribute_value() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A">text</doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let text = xoz.first_child(doc_el).unwrap();
    let a = xoz.attribute_value(doc_el, "a");
    assert_eq!(xoz.text_str(text), Some("text"));
    assert_eq!(a, Some("A"));
}
