use xoz::{parse_document, Name};

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

#[test]
fn test_attribute_names() {
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, &Name::name_without_namespace("a"))
        .unwrap();
    let b = doc
        .attribute_node(doc_el, &Name::name_without_namespace("b"))
        .unwrap();

    let a_name = doc.node_name(a).unwrap();
    let b_name = doc.node_name(b).unwrap();

    assert_eq!(a_name.local_name(), "a");
    assert_eq!(b_name.local_name(), "b");
}

#[test]
fn test_attributes_and_children() {
    let doc = parse_document(r#"<doc c="C"><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();

    let a_name = doc.node_name(a).unwrap();
    let b_name = doc.node_name(b).unwrap();

    assert_eq!(a_name.local_name(), "a");
    assert_eq!(b_name.local_name(), "b");
}

#[test]
fn test_previous_sibling_without_attributes() {
    let doc = parse_document(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();

    let a_prev = doc.previous_sibling(b).unwrap();
    assert_eq!(a, a_prev);
    assert_eq!(doc.previous_sibling(a), None);
}

#[test]
fn test_previous_sibling_with_attributes() {
    let doc = parse_document(r#"<doc c="C"><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();

    let a_prev = doc.previous_sibling(b).unwrap();
    assert_eq!(a, a_prev);
    assert_eq!(doc.previous_sibling(a), None);
}

#[test]
fn test_element_parent() {
    let doc = parse_document(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    assert_eq!(doc.parent(a), Some(doc_el));
    assert_eq!(doc.parent(b), Some(doc_el));
    assert_eq!(doc.parent(doc_el), Some(doc.root()));
    assert_eq!(doc.parent(doc.root()), None);
}

#[test]
fn test_attribute_parent() {
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, &Name::name_without_namespace("a"))
        .unwrap();
    let b = doc
        .attribute_node(doc_el, &Name::name_without_namespace("b"))
        .unwrap();
    assert_eq!(doc.parent(a), Some(doc_el));
    assert_eq!(doc.parent(b), Some(doc_el));
}
