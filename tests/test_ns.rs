use xoz::NodeName;

#[test]
fn test_simple_empty_prefix_for_namespace() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"");
}

#[test]
fn test_simple_named_prefix_for_namespace() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"ns");
}

#[test]
fn test_simple_named_prefix_for_namespace_not_found() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc.prefix_for_namespace(doc_el, b"http://example.com/example2");
    assert!(prefix.is_none());
}

#[test]
fn test_simple_named_prefix_for_namespace_prefer_empty1() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns="http://example.com" xmlns:ns="http://example.com" />"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"");
}

#[test]
fn test_simple_named_prefix_for_namespace_prefer_empty2() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns:ns="http://example.com" xmlns="http://example.com" />"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"");
}

#[test]
fn test_nested_prefix() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns:ns1="http://example.com"><p xmlns:ns2="http://example.com"/></doc>"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"ns1");
    let prefix = doc.prefix_for_namespace(p, b"http://example.com").unwrap();
    assert_eq!(prefix, b"ns2");
}

#[test]
fn test_nested_prefix_go_up() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com"><p/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(prefix, b"ns");
    let ns = doc.prefix_for_namespace(p, b"http://example.com").unwrap();
    assert_eq!(ns, b"ns");
}

#[test]
fn test_nested_prefix_not_found() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com"><p/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let prefix = doc.prefix_for_namespace(p, b"http://example.com/example2");
    assert!(prefix.is_none());
}

#[test]
fn test_xml_prefix() {
    let doc = xoz::Document::parse_str(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc
        .prefix_for_namespace(doc_el, b"http://www.w3.org/XML/1998/namespace")
        .unwrap();
    assert_eq!(prefix, b"xml");
}

#[test]
fn test_prefix_for_default() {
    let doc = xoz::Document::parse_str(r#"<doc />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc.prefix_for_namespace(doc_el, b"").unwrap();
    assert_eq!(prefix, b"");
}

#[test]
fn test_node_prefix_no_declaration() {
    let doc = xoz::Document::parse_str(r#"<doc />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc.node_prefix(doc_el).unwrap();
    assert!(prefix.is_empty());
}

#[test]
fn test_node_prefix_non_element_or_attribute() {
    let doc = xoz::Document::parse_str(r#"<doc>text</doc>"#).unwrap();
    let doc_el = doc.document_element();
    let text = doc.first_child(doc_el).unwrap();
    let prefix = doc.node_prefix(text);
    assert!(prefix.is_none());
}

#[test]
fn test_node_prefix_default() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc.node_prefix(doc_el).unwrap();
    assert_eq!(prefix, b"");
}

#[test]
fn test_node_prefix_explicit() {
    let doc = xoz::Document::parse_str(r#"<ns:doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let prefix = doc.node_prefix(doc_el).unwrap();
    assert_eq!(prefix, b"ns");
}

#[test]
fn test_node_prefix_explicit_inherited() {
    let doc = xoz::Document::parse_str(
        r#"<ns:doc xmlns:ns="http://example.com"><ns:p>Text</ns:p></ns:doc>"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let prefix = doc.node_prefix(p).unwrap();
    assert_eq!(prefix, b"ns");
}

#[test]
fn test_node_prefix_attribute() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let prefix = doc.node_prefix(a).unwrap();
    assert!(prefix.is_empty());
}

#[test]
fn test_node_prefix_attribute_default_ns() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns="http://example.com" a="A" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let prefix = doc.node_prefix(a).unwrap();
    assert!(prefix.is_empty());
}

#[test]
fn test_node_prefix_attribute_explicit() {
    let doc =
        xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" ns:a="A" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, NodeName::new("http://example.com", "a"))
        .unwrap();
    let prefix = doc.node_prefix(a).unwrap();
    assert_eq!(prefix, b"ns");
}

#[test]
fn test_node_full_name_no_ns() {
    let doc = xoz::Document::parse_str(r#"<doc />"#).unwrap();
    let doc_el = doc.document_element();
    let full_name = doc.node_full_name(doc_el).unwrap();
    assert_eq!(full_name, "doc");
}

#[test]
fn test_node_full_name_default() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let full_name = doc.node_full_name(doc_el).unwrap();
    assert_eq!(full_name, "doc");
}

#[test]
fn test_node_full_name_explicit() {
    let doc = xoz::Document::parse_str(r#"<ns:doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let full_name = doc.node_full_name(doc_el).unwrap();
    assert_eq!(full_name, "ns:doc");
}

#[test]
fn test_node_full_name_attribute_empty() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let full_name = doc.node_full_name(a).unwrap();
    assert_eq!(full_name, "a");
}

#[test]
fn test_node_full_name_attribute_explicit() {
    let doc =
        xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" ns:a="A" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, NodeName::new("http://example.com", "a"))
        .unwrap();
    let full_name = doc.node_full_name(a).unwrap();
    assert_eq!(full_name, "ns:a");
}
