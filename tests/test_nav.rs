use xoz::{NodeType, Xoz};

#[test]
fn test_elements() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();

    let doc_el_name = xoz.node_name(doc_el).unwrap();
    let a_name = xoz.node_name(a).unwrap();
    let b_name = xoz.node_name(b).unwrap();

    assert_eq!(doc_el_name.local_name(), b"doc");
    assert_eq!(a_name.local_name(), b"a");
    assert_eq!(b_name.local_name(), b"b");
}

#[test]
fn test_elements_multiple_a() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><a/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a1 = xoz.first_child(doc_el).unwrap();
    let a2 = xoz.next_sibling(a1).unwrap();

    let doc_el_name = xoz.node_name(doc_el).unwrap();
    let a1_name = xoz.node_name(a1).unwrap();
    let a2_name = xoz.node_name(a2).unwrap();

    assert_eq!(doc_el_name.local_name(), b"doc");
    assert_eq!(a1_name.local_name(), b"a");
    assert_eq!(a2_name.local_name(), b"a");
}

#[test]
fn test_attributes_and_children() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc c="C"><a/><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();

    let a_name = xoz.node_name(a).unwrap();
    let b_name = xoz.node_name(b).unwrap();

    assert_eq!(a_name.local_name(), b"a");
    assert_eq!(b_name.local_name(), b"b");
}

#[test]
fn test_previous_sibling_without_attributes() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();

    let a_prev = xoz.previous_sibling(b).unwrap();
    assert_eq!(a, a_prev);
    assert_eq!(xoz.previous_sibling(a), None);
}

#[test]
fn test_previous_sibling_with_attributes() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc c="C"><a/><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();

    let a_prev = xoz.previous_sibling(b).unwrap();
    assert_eq!(a, a_prev);
    assert_eq!(xoz.previous_sibling(a), None);
}

#[test]
fn test_element_parent() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    assert_eq!(xoz.parent(a), Some(doc_el));
    assert_eq!(xoz.parent(b), Some(doc_el));
    assert_eq!(xoz.parent(doc_el), Some(root));
    assert_eq!(xoz.parent(root), None);
}

#[test]
fn test_attribute_parent() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_node(doc_el, "a").unwrap();
    let b = xoz.attribute_node(doc_el, "b").unwrap();
    assert_eq!(xoz.parent(a), Some(doc_el));
    assert_eq!(xoz.parent(b), Some(doc_el));
}

#[test]
fn test_top_node() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str("<a><b/></a>").unwrap();
    let a = xoz.document_element(root);
    let b = xoz.first_child(a).unwrap();
    assert_eq!(xoz.top_element(b), a);
    assert_eq!(xoz.top_element(a), a);
    assert_eq!(xoz.top_element(root), a);
}

#[test]
fn test_tagged_descendant() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let b = xoz
        .typed_descendant(xoz.document_element(root), NodeType::element("b"))
        .unwrap();
    assert_eq!(xoz.node_name(b).unwrap().local_name(), b"b");
}

#[test]
fn test_tagged_descendant_node_itself() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();

    let found = xoz.typed_descendant(b, NodeType::element("b"));
    assert!(found.is_none());
}

#[test]
fn test_tagged_descendant2() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><a><b/></a></b></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let first_b = xoz.first_child(a).unwrap();
    let a2 = xoz.first_child(first_b).unwrap();
    let second_b = xoz.first_child(a2).unwrap();

    let b = xoz
        .typed_descendant(xoz.document_element(root), NodeType::element("b"))
        .unwrap();
    assert_eq!(b, first_b);
    let b = xoz.typed_descendant(b, NodeType::element("b")).unwrap();
    assert_eq!(b, second_b);
}

#[test]
fn test_last_child() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let c_last = xoz.last_child(doc_el).unwrap();
    assert_eq!(c, c_last);
}

#[test]
fn test_last_child_with_attributes() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A"><a/><b/><c/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let c_last = xoz.last_child(doc_el).unwrap();
    assert_eq!(c, c_last);
}

#[test]
fn test_no_last_child_if_only_attributes() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let last = xoz.last_child(doc_el);
    assert_eq!(last, None);
}

#[test]
fn test_child_index() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    assert_eq!(xoz.child_index(doc_el, a), Some(0));
    assert_eq!(xoz.child_index(doc_el, b), Some(1));
    assert_eq!(xoz.child_index(doc_el, c), Some(2));
    assert_eq!(xoz.child_index(root, root), None);
}

#[test]
fn test_typed_foll1() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b1 = xoz.first_child(a).unwrap();
    let b2 = xoz.next_sibling(b1).unwrap();

    let found = xoz.typed_foll(b1, NodeType::element("b"));
    assert_eq!(found, Some(b2));
}

#[test]
fn test_typed_foll2() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><f/><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let f1 = xoz.first_child(doc_el).unwrap();
    let a = xoz.next_sibling(f1).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f2 = xoz.next_sibling(e).unwrap();

    let found = xoz.typed_foll(f1, NodeType::element("f"));
    assert_eq!(found, Some(f2));
}
