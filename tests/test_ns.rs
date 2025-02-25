#[test]
fn test_simple_empty_prefix_for_namespace() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"");
}

#[test]
fn test_simple_named_prefix_for_namespace() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"ns");
}

#[test]
fn test_simple_named_prefix_for_namespace_not_found() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com" />"#).unwrap();
    let doc_el = doc.document_element();
    let ns = doc.prefix_for_namespace(doc_el, b"http://example.com/example2");
    assert!(ns.is_none());
}

#[test]
fn test_simple_named_prefix_for_namespace_prefer_empty1() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns="http://example.com" xmlns:ns="http://example.com" />"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"");
}

#[test]
fn test_simple_named_prefix_for_namespace_prefer_empty2() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns:ns="http://example.com" xmlns="http://example.com" />"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"");
}

#[test]
fn test_nested_prefix() {
    let doc = xoz::Document::parse_str(
        r#"<doc xmlns:ns1="http://example.com"><p xmlns:ns2="http://example.com"/></doc>"#,
    )
    .unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"ns1");
    let ns = doc.prefix_for_namespace(p, b"http://example.com").unwrap();
    assert_eq!(ns, b"ns2");
}

#[test]
fn test_nested_prefix_go_up() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com"><p/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://example.com")
        .unwrap();
    assert_eq!(ns, b"ns");
    let ns = doc.prefix_for_namespace(p, b"http://example.com").unwrap();
    assert_eq!(ns, b"ns");
}

#[test]
fn test_nested_prefix_not_found() {
    let doc = xoz::Document::parse_str(r#"<doc xmlns:ns="http://example.com"><p/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let p = doc.first_child(doc_el).unwrap();
    let ns = doc.prefix_for_namespace(p, b"http://example.com/example2");
    assert!(ns.is_none());
}

#[test]
fn test_xml_prefix() {
    let doc = xoz::Document::parse_str(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let ns = doc
        .prefix_for_namespace(doc_el, b"http://www.w3.org/XML/1998/namespace")
        .unwrap();
    assert_eq!(ns, b"xml");
}
