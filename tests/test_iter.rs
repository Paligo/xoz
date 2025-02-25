use xoz::NodeType;

#[test]
fn test_following_siblings() {
    let doc = xoz::Document::parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let a_siblings: Vec<_> = doc.following_siblings(a).collect();
    let b_siblings: Vec<_> = doc.following_siblings(b).collect();
    let c_siblings: Vec<_> = doc.following_siblings(c).collect();
    assert_eq!(a_siblings, vec![b, c]);
    assert_eq!(b_siblings, vec![c]);
    assert_eq!(c_siblings, vec![]);
}

#[test]
fn test_preceding_siblings() {
    let doc = xoz::Document::parse_str(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let a_siblings: Vec<_> = doc.preceding_siblings(a).collect();
    let b_siblings: Vec<_> = doc.preceding_siblings(b).collect();
    let c_siblings: Vec<_> = doc.preceding_siblings(c).collect();
    assert_eq!(a_siblings, vec![]);
    assert_eq!(b_siblings, vec![a]);
    assert_eq!(c_siblings, vec![b, a]);
}

#[test]
fn test_ancestors() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let ancestors: Vec<_> = doc.ancestors(c).collect();
    assert_eq!(ancestors, vec![b, a, doc_el, root]);
}

#[test]
fn test_ancestors_or_self() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let ancestors: Vec<_> = doc.ancestors_or_self(c).collect();
    assert_eq!(ancestors, vec![c, b, a, doc_el, root]);
}

#[test]
fn test_ancestors_of_attribute() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let ancestors: Vec<_> = doc.ancestors(a).collect();
    assert_eq!(ancestors, vec![doc_el, root]);
}

#[test]
fn test_ancestors_or_self_of_attribute() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" />"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let ancestors: Vec<_> = doc.ancestors_or_self(a).collect();
    assert_eq!(ancestors, vec![a, doc_el, root]);
}

#[test]
fn test_descendants() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();

    let descendants: Vec<_> = doc.descendants(doc_el).collect();
    assert_eq!(descendants, vec![a, b, c, d, e, f]);

    let descendants: Vec<_> = doc.descendants(a).collect();
    assert_eq!(descendants, vec![b, c, d, e, f]);

    let descendants: Vec<_> = doc.descendants(b).collect();
    assert_eq!(descendants, vec![c]);
}

#[test]
fn test_descendants_or_self() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();

    let descendants: Vec<_> = doc.descendants_or_self(doc_el).collect();
    assert_eq!(descendants, vec![doc_el, a, b, c, d, e, f]);

    let descendants: Vec<_> = doc.descendants_or_self(a).collect();
    assert_eq!(descendants, vec![a, b, c, d, e, f]);

    let descendants: Vec<_> = doc.descendants_or_self(b).collect();
    assert_eq!(descendants, vec![b, c]);
}

#[test]
fn test_descendants_one_node() {
    let doc = xoz::Document::parse_str(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let descendants: Vec<_> = doc.descendants(doc_el).collect();

    assert_eq!(descendants, vec![]);
}

#[test]
fn test_descendants_or_self_one_node() {
    let doc = xoz::Document::parse_str(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let descendants: Vec<_> = doc.descendants_or_self(doc_el).collect();

    assert_eq!(descendants, vec![doc_el]);
}

#[test]
fn test_following() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();
    let following: Vec<_> = doc.following(c).collect();
    assert_eq!(following, vec![d, e, f]);
}

#[test]
fn test_preceding() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();
    let preceding: Vec<_> = doc.axis_preceding(f).collect();
    assert_eq!(preceding, vec![b, c, e]);
}

#[test]
fn test_following_two() {
    let doc =
        xoz::Document::parse_str(r#"<x><a><b><d/><e/></b><c><f/><g/></c></a><y/></x>"#).unwrap();
    let x = doc.document_element();
    let a = doc.first_child(x).unwrap();
    let b = doc.first_child(a).unwrap();
    let d = doc.first_child(b).unwrap();
    let e = doc.next_sibling(d).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let f = doc.first_child(c).unwrap();
    let g = doc.next_sibling(f).unwrap();
    let y = doc.next_sibling(a).unwrap();

    let following: Vec<_> = doc.following(d).collect();
    assert_eq!(following, vec![e, c, f, g, y]);
}

#[test]
fn test_preceding_two() {
    let doc =
        xoz::Document::parse_str(r#"<x><a><b><d/><e/></b><c><f/><g/></c></a><y/></x>"#).unwrap();
    let x = doc.document_element();
    let a = doc.first_child(x).unwrap();
    let b = doc.first_child(a).unwrap();
    let d = doc.first_child(b).unwrap();
    let e = doc.next_sibling(d).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let f = doc.first_child(c).unwrap();
    let g = doc.next_sibling(f).unwrap();
    let y = doc.next_sibling(a).unwrap();

    let preceding: Vec<_> = doc.axis_preceding(y).collect();
    assert_eq!(preceding, vec![a, b, d, e, c, f, g]);

    let preceding: Vec<_> = doc.axis_preceding(f).collect();
    assert_eq!(preceding, vec![b, d, e]);
}

#[test]
fn test_tagged_descendants() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b1 = doc.first_child(a).unwrap();
    let b2 = doc.next_sibling(b1).unwrap();

    let tagged_descendants: Vec<_> = doc
        .typed_descendants(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants, vec![b1, b2]);
}

#[test]
fn test_tagged_descendants_next_sibling() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self() {
    let doc = xoz::Document::parse_str(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self2() {
    let doc = xoz::Document::parse_str(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 0);
}

#[test]
fn test_tagged_descendants_or_self() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants_or_self(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_next_sibling() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants_or_self(doc_el, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_including_self() {
    let doc = xoz::Document::parse_str(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants_or_self(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 3);
}

#[test]
fn test_tagged_descendants_or_self_including_self2() {
    let doc = xoz::Document::parse_str(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tagged_descendants: Vec<_> = doc
        .typed_descendants_or_self(outer_b, NodeType::element("b"))
        .collect();
    assert_eq!(tagged_descendants.len(), 1);
}

#[test]
fn test_typed_descendants_bug() {
    let doc = xoz::Document::parse_str("<d><p>foo<a/>bar</p><p>baz<a/></p></d>").unwrap();
    let d = doc.document_element();
    let p = doc.first_child(d).unwrap();
    let text = doc.first_child(p).unwrap();
    let a1 = doc.next_sibling(text).unwrap();
    let p2 = doc.next_sibling(p).unwrap();
    let text = doc.first_child(p2).unwrap();
    let a2 = doc.next_sibling(text).unwrap();

    let p_nodes = doc
        .typed_descendants(d, xoz::NodeType::Element("p".into()))
        .collect::<Vec<_>>();
    assert_eq!(p_nodes, vec![p, p2]);
    let a_nodes = doc
        .typed_descendants(d, xoz::NodeType::Element("a".into()))
        .collect::<Vec<_>>();
    assert_eq!(a_nodes, vec![a1, a2]);
}

#[test]
fn test_typed_following1() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();

    let following: Vec<_> = doc.typed_following(c, NodeType::element("f")).collect();
    assert_eq!(following, vec![f]);
}

#[test]
fn test_typed_following2() {
    let doc = xoz::Document::parse_str(r#"<doc><f/><a><b><c/></b><d><e/><f/></d></a><f/></doc>"#)
        .unwrap();
    let doc_el = doc.document_element();
    let f1 = doc.first_child(doc_el).unwrap();
    let a = doc.next_sibling(f1).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f2 = doc.next_sibling(e).unwrap();
    let f3 = doc.next_sibling(a).unwrap();

    let following: Vec<_> = doc.typed_following(c, NodeType::element("f")).collect();
    assert_eq!(following, vec![f2, f3]);
}

#[test]
fn test_attributes_axis() {
    let doc = xoz::Document::parse_str(r#"<doc a="A" b="B" c="C" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_node(doc_el, "a").unwrap();
    let b = doc.attribute_node(doc_el, "b").unwrap();
    let c = doc.attribute_node(doc_el, "c").unwrap();

    let attributes: Vec<_> = doc.attributes(doc_el).collect();
    assert_eq!(attributes, vec![a, b, c]);
}

#[test]
fn test_parent_axis() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();

    let parents: Vec<_> = doc.axis_parent(b).collect();
    assert_eq!(parents, vec![a]);

    let parents: Vec<_> = doc.axis_parent(a).collect();
    assert_eq!(parents, vec![doc_el]);

    let parents: Vec<_> = doc.axis_parent(doc_el).collect();
    assert_eq!(parents, vec![root]);

    let parents: Vec<_> = doc.axis_parent(root).collect();
    assert_eq!(parents, vec![]);
}

#[test]
fn test_self_axis() {
    let doc = xoz::Document::parse_str(r#"<doc><a><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();

    let nodes: Vec<_> = doc.axis_self(b).collect();
    assert_eq!(nodes, vec![b]);
    let nodes: Vec<_> = doc.axis_self(a).collect();
    assert_eq!(nodes, vec![a]);
}
