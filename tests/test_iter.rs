use xoz::{NodeType, Xoz};

#[test]
fn test_following_siblings() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let a_siblings: Vec<_> = xoz.following_siblings(a).collect();
    let b_siblings: Vec<_> = xoz.following_siblings(b).collect();
    let c_siblings: Vec<_> = xoz.following_siblings(c).collect();
    assert_eq!(a_siblings, vec![b, c]);
    assert_eq!(b_siblings, vec![c]);
    assert_eq!(c_siblings, vec![]);
}

#[test]
fn test_preceding_siblings() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.next_sibling(a).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let a_siblings: Vec<_> = xoz.preceding_siblings(a).collect();
    let b_siblings: Vec<_> = xoz.preceding_siblings(b).collect();
    let c_siblings: Vec<_> = xoz.preceding_siblings(c).collect();
    assert_eq!(a_siblings, vec![]);
    assert_eq!(b_siblings, vec![a]);
    assert_eq!(c_siblings, vec![b, a]);
}

#[test]
fn test_ancestors() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let ancestors: Vec<_> = xoz.ancestors(c).collect();
    assert_eq!(ancestors, vec![b, a, doc_el, root]);
}

#[test]
fn test_ancestors_or_self() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let ancestors: Vec<_> = xoz.ancestors_or_self(c).collect();
    assert_eq!(ancestors, vec![c, b, a, doc_el, root]);
}

#[test]
fn test_ancestors_of_attribute() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_node(doc_el, "a").unwrap();
    let ancestors: Vec<_> = xoz.ancestors(a).collect();
    assert_eq!(ancestors, vec![doc_el, root]);
}

#[test]
fn test_ancestors_or_self_of_attribute() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_node(doc_el, "a").unwrap();
    let ancestors: Vec<_> = xoz.ancestors_or_self(a).collect();
    assert_eq!(ancestors, vec![a, doc_el, root]);
}

#[test]
fn test_descendants() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f = xoz.next_sibling(e).unwrap();

    let descendants: Vec<_> = xoz.descendants(doc_el).collect();
    assert_eq!(descendants, vec![a, b, c, d, e, f]);

    let descendants: Vec<_> = xoz.descendants(a).collect();
    assert_eq!(descendants, vec![b, c, d, e, f]);

    let descendants: Vec<_> = xoz.descendants(b).collect();
    assert_eq!(descendants, vec![c]);
}

#[test]
fn test_descendants_or_self() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f = xoz.next_sibling(e).unwrap();

    let descendants: Vec<_> = xoz.descendants_or_self(doc_el).collect();
    assert_eq!(descendants, vec![doc_el, a, b, c, d, e, f]);

    let descendants: Vec<_> = xoz.descendants_or_self(a).collect();
    assert_eq!(descendants, vec![a, b, c, d, e, f]);

    let descendants: Vec<_> = xoz.descendants_or_self(b).collect();
    assert_eq!(descendants, vec![b, c]);
}

#[test]
fn test_descendants_one_node() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc/>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let descendants: Vec<_> = xoz.descendants(doc_el).collect();

    assert_eq!(descendants, vec![]);
}

#[test]
fn test_descendants_or_self_one_node() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc/>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let descendants: Vec<_> = xoz.descendants_or_self(doc_el).collect();

    assert_eq!(descendants, vec![doc_el]);
}

#[test]
fn test_following() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f = xoz.next_sibling(e).unwrap();
    let following: Vec<_> = xoz.following(c).collect();
    assert_eq!(following, vec![d, e, f]);
}

#[test]
fn test_preceding() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f = xoz.next_sibling(e).unwrap();
    let preceding: Vec<_> = xoz.axis_preceding(f).collect();
    assert_eq!(preceding, vec![b, c, e]);
}

#[test]
fn test_following_two() {
    let mut xoz = Xoz::new();
    let root = xoz
        .parse_str(r#"<x><a><b><d/><e/></b><c><f/><g/></c></a><y/></x>"#).unwrap();
    let x = xoz.document_element(root);
    let a = xoz.first_child(x).unwrap();
    let b = xoz.first_child(a).unwrap();
    let d = xoz.first_child(b).unwrap();
    let e = xoz.next_sibling(d).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let f = xoz.first_child(c).unwrap();
    let g = xoz.next_sibling(f).unwrap();
    let y = xoz.next_sibling(a).unwrap();

    let following: Vec<_> = xoz.following(d).collect();
    assert_eq!(following, vec![e, c, f, g, y]);
}

#[test]
fn test_preceding_two() {
    let mut xoz = Xoz::new();
    let root = xoz
        .parse_str(r#"<x><a><b><d/><e/></b><c><f/><g/></c></a><y/></x>"#).unwrap();
    let x = xoz.document_element(root);
    let a = xoz.first_child(x).unwrap();
    let b = xoz.first_child(a).unwrap();
    let d = xoz.first_child(b).unwrap();
    let e = xoz.next_sibling(d).unwrap();
    let c = xoz.next_sibling(b).unwrap();
    let f = xoz.first_child(c).unwrap();
    let g = xoz.next_sibling(f).unwrap();
    let y = xoz.next_sibling(a).unwrap();

    let preceding: Vec<_> = xoz.axis_preceding(y).collect();
    assert_eq!(preceding, vec![a, b, d, e, c, f, g]);

    let preceding: Vec<_> = xoz.axis_preceding(f).collect();
    assert_eq!(preceding, vec![b, d, e]);
}

#[test]
fn test_tagged_descendants() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b1 = xoz.first_child(a).unwrap();
    let b2 = xoz.next_sibling(b1).unwrap();

    let tagged_descendants: Vec<_> = xoz
        .typed_descendants(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants, vec![b1, b2]);
}

#[test]
fn test_tagged_descendants_next_sibling() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let outer_b = xoz.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self2() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let outer_b = xoz.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 0);
}

#[test]
fn test_tagged_descendants_or_self() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants_or_self(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_next_sibling() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants_or_self(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_including_self() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let outer_b = xoz.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants_or_self(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 3);
}

#[test]
fn test_tagged_descendants_or_self_including_self2() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let outer_b = xoz.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = xoz
        .typed_descendants_or_self(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 1);
}

#[test]
fn test_typed_descendants_bug() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str("<d><p>foo<a/>bar</p><p>baz<a/></p></d>").unwrap();
    let d = xoz.document_element(root);
    let p = xoz.first_child(d).unwrap();
    let text = xoz.first_child(p).unwrap();
    let a1 = xoz.next_sibling(text).unwrap();
    let p2 = xoz.next_sibling(p).unwrap();
    let text = xoz.first_child(p2).unwrap();
    let a2 = xoz.next_sibling(text).unwrap();

    let p_nodes = xoz
        .typed_descendants(d, xoz::NodeType::Element("p".into()))
        .collect::<Vec<_>>();
    assert_eq!(p_nodes, vec![p, p2]);
    let a_nodes = xoz
        .typed_descendants(d, xoz::NodeType::Element("a".into()))
        .collect::<Vec<_>>();
    assert_eq!(a_nodes, vec![a1, a2]);
}

#[test]
fn test_typed_following1() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f = xoz.next_sibling(e).unwrap();

    let following: Vec<_> = xoz.typed_following(c, NodeType::element("f")).collect();
    assert_eq!(following, vec![f]);
}

#[test]
fn test_typed_following2() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><f/><a><b><c/></b><d><e/><f/></d></a><f/></doc>"#)
        .unwrap();
    let doc_el = xoz.document_element(root);
    let f1 = xoz.first_child(doc_el).unwrap();
    let a = xoz.next_sibling(f1).unwrap();
    let b = xoz.first_child(a).unwrap();
    let c = xoz.first_child(b).unwrap();
    let d = xoz.next_sibling(b).unwrap();
    let e = xoz.first_child(d).unwrap();
    let f2 = xoz.next_sibling(e).unwrap();
    let f3 = xoz.next_sibling(a).unwrap();

    let following: Vec<_> = xoz.typed_following(c, NodeType::element("f")).collect();
    assert_eq!(following, vec![f2, f3]);
}

#[test]
fn test_attributes_axis() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc a="A" b="B" c="C" />"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.attribute_node(doc_el, "a").unwrap();
    let b = xoz.attribute_node(doc_el, "b").unwrap();
    let c = xoz.attribute_node(doc_el, "c").unwrap();

    let attributes: Vec<_> = xoz.attributes(doc_el).collect();
    assert_eq!(attributes, vec![a, b, c]);
}

#[test]
fn test_parent_axis() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();

    let parents: Vec<_> = xoz.axis_parent(b).collect();
    assert_eq!(parents, vec![a]);

    let parents: Vec<_> = xoz.axis_parent(a).collect();
    assert_eq!(parents, vec![doc_el]);

    let parents: Vec<_> = xoz.axis_parent(doc_el).collect();
    assert_eq!(parents, vec![root]);

    let parents: Vec<_> = xoz.axis_parent(root).collect();
    assert_eq!(parents, vec![]);
}

#[test]
fn test_self_axis() {
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let doc_el = xoz.document_element(root);
    let a = xoz.first_child(doc_el).unwrap();
    let b = xoz.first_child(a).unwrap();

    let nodes: Vec<_> = xoz.axis_self(b).collect();
    assert_eq!(nodes, vec![b]);
    let nodes: Vec<_> = xoz.axis_self(a).collect();
    assert_eq!(nodes, vec![a]);
}
