use xoz::{parse_document, Name, TagInfo, TagType};

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

#[test]
fn test_text() {
    let doc = parse_document(r#"<doc>text</doc>"#).unwrap();
    let doc_el = doc.document_element();
    let text = doc.first_child(doc_el).unwrap();
    assert_eq!(doc.text_str(text), Some("text"));
    assert_eq!(doc.text_str(doc_el), None);
}

#[test]
fn test_multiple_text() {
    let doc = parse_document(r#"<doc><a>A</a><b>B</b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let a_text = doc.first_child(a).unwrap();
    let b_text = doc.first_child(b).unwrap();
    assert_eq!(doc.text_str(a_text), Some("A"));
    assert_eq!(doc.text_str(b_text), Some("B"));
}

#[test]
fn test_attribute_value() {
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.attribute_value(doc_el, &Name::name_without_namespace("a"));
    let b = doc.attribute_value(doc_el, &Name::name_without_namespace("b"));
    let c = doc.attribute_value(doc_el, &Name::name_without_namespace("c"));
    assert_eq!(a, Some("A"));
    assert_eq!(b, Some("B"));
    assert_eq!(c, None);
}

#[test]
fn test_text_and_attribute_value() {
    let doc = parse_document(r#"<doc a="A">text</doc>"#).unwrap();
    let doc_el = doc.document_element();
    let text = doc.first_child(doc_el).unwrap();
    let a = doc.attribute_value(doc_el, &Name::name_without_namespace("a"));
    assert_eq!(doc.text_str(text), Some("text"));
    assert_eq!(a, Some("A"));
}

#[test]
fn test_following_siblings() {
    let doc = parse_document(r#"<doc><a/><b/><c/></doc>"#).unwrap();
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
    let doc = parse_document(r#"<doc><a/><b/><c/></doc>"#).unwrap();
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
fn test_preorder() {
    let doc = parse_document(r#"<doc><a/><b><c/><d/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(c).unwrap();
    // now put nodes in arbitrary order
    let mut nodes = [b, d, c, a, doc_el];
    // sort them by preorder
    nodes.sort_by_key(|&node| doc.preorder(node));
    assert_eq!(nodes, [doc_el, a, b, c, d]);
}

#[test]
fn test_subtree_tags() {
    let doc = parse_document(r#"<doc><a/><a/></doc>"#).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        }))
        .unwrap();
    assert_eq!(doc.subtree_tags(doc.document_element(), tag_id), 2);
}

#[test]
fn test_subtree_tags_root() {
    let doc = parse_document(r#"<doc><a/><a/></doc>"#).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        }))
        .unwrap();
    assert_eq!(doc.subtree_tags(doc.root(), tag_id), 2);
}

#[test]
fn test_subtree_tags_deeper() {
    let doc = parse_document(r#"<doc><b><a/></b><a/></doc>"#).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "a".to_string(),
        }))
        .unwrap();
    assert_eq!(doc.subtree_tags(doc.document_element(), tag_id), 2);
}

#[test]
fn test_tagged_descendant() {
    let doc = parse_document(r#"<doc><a><b/></a></doc>"#).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let b = doc
        .tagged_descendant(doc.document_element(), tag_id)
        .unwrap();
    assert_eq!(doc.node_name(b).unwrap().local_name(), "b");
}

#[test]
fn test_tagged_descendant_node_itself() {
    let doc = parse_document(r#"<doc><a><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();

    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let found = doc.tagged_descendant(b, tag_id);
    assert!(found.is_none());
}

#[test]
fn test_tagged_descendant2() {
    let doc = parse_document(r#"<doc><a><b><a><b/></a></b></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let first_b = doc.first_child(a).unwrap();
    let a2 = doc.first_child(first_b).unwrap();
    let second_b = doc.first_child(a2).unwrap();

    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();

    let b = doc
        .tagged_descendant(doc.document_element(), tag_id)
        .unwrap();
    assert_eq!(b, first_b);
    let b = doc.tagged_descendant(b, tag_id).unwrap();
    assert_eq!(b, second_b);
}

#[test]
fn test_last_child() {
    let doc = parse_document(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let c_last = doc.last_child(doc_el).unwrap();
    assert_eq!(c, c_last);
}

#[test]
fn test_last_child_with_attributes() {
    let doc = parse_document(r#"<doc a="A"><a/><b/><c/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.next_sibling(b).unwrap();
    let c_last = doc.last_child(doc_el).unwrap();
    assert_eq!(c, c_last);
}

#[test]
fn test_no_last_child_if_only_attributes() {
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let doc_el = doc.document_element();
    let last = doc.last_child(doc_el);
    assert_eq!(last, None);
}

#[test]
fn test_ancestors() {
    let doc = parse_document(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
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
    let doc = parse_document(r#"<doc><a><b><c/></b></a></doc>"#).unwrap();
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
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, &Name::name_without_namespace("a"))
        .unwrap();
    let ancestors: Vec<_> = doc.ancestors(a).collect();
    assert_eq!(ancestors, vec![doc_el, root]);
}

#[test]
fn test_ancestors_or_self_of_attribute() {
    let doc = parse_document(r#"<doc a="A" b="B" />"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, &Name::name_without_namespace("a"))
        .unwrap();
    let ancestors: Vec<_> = doc.ancestors_or_self(a).collect();
    assert_eq!(ancestors, vec![a, doc_el, root]);
}

#[test]
fn test_child_index() {
    let doc = parse_document(r#"<doc><a/><b/><c/></doc>"#).unwrap();
    let root = doc.root();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.next_sibling(a).unwrap();
    let c = doc.next_sibling(b).unwrap();
    assert_eq!(doc.child_index(a), Some(0));
    assert_eq!(doc.child_index(b), Some(1));
    assert_eq!(doc.child_index(c), Some(2));
    assert_eq!(doc.child_index(root), None);
}

#[test]
fn test_descendants() {
    let doc = parse_document(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
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
    let doc = parse_document(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
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
    let doc = parse_document(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let descendants: Vec<_> = doc.descendants(doc_el).collect();

    assert_eq!(descendants, vec![]);
}

#[test]
fn test_descendants_or_self_one_node() {
    let doc = parse_document(r#"<doc/>"#).unwrap();
    let doc_el = doc.document_element();
    let descendants: Vec<_> = doc.descendants_or_self(doc_el).collect();

    assert_eq!(descendants, vec![doc_el]);
}

#[test]
fn test_following() {
    let doc = parse_document(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
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
fn test_following_two() {
    let doc = parse_document(r#"<x><a><b><d/><e/></b><c><f/><g/></c></a><y/></x>"#).unwrap();
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
fn test_tagged_descendants() {
    let doc = parse_document(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants(doc_el, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_next_sibling() {
    let doc = parse_document(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants(doc_el, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self() {
    let doc = parse_document(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants(outer_b, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_including_self2() {
    let doc = parse_document(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants(outer_b, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 0);
}

#[test]
fn test_tagged_descendants_or_self() {
    let doc = parse_document(r#"<doc><a><b/><b/></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants_or_self(doc_el, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_next_sibling() {
    let doc = parse_document(r#"<doc><a><b/></a><c><b/></c></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants_or_self(doc_el, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 2);
}

#[test]
fn test_tagged_descendants_or_self_including_self() {
    let doc = parse_document(r#"<doc><b><b/><b/></b></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants_or_self(outer_b, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 3);
}

#[test]
fn test_tagged_descendants_or_self_including_self2() {
    let doc = parse_document(r#"<doc><b/></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let outer_b = doc.first_child(doc_el).unwrap();
    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "b".to_string(),
        }))
        .unwrap();
    let tagged_descendants: Vec<_> = doc.tagged_descendants_or_self(outer_b, tag_id).collect();
    assert_eq!(tagged_descendants.len(), 1);
}

#[test]
fn test_tagged_following() {
    let doc = parse_document(r#"<doc><a><b><c/></b><d><e/><f/></d></a></doc>"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc.first_child(doc_el).unwrap();
    let b = doc.first_child(a).unwrap();
    let c = doc.first_child(b).unwrap();
    let d = doc.next_sibling(b).unwrap();
    let e = doc.first_child(d).unwrap();
    let f = doc.next_sibling(e).unwrap();

    let tag_id = doc
        .tag(&TagInfo::open(TagType::Element {
            namespace: "".to_string(),
            local_name: "f".to_string(),
        }))
        .unwrap();
    let following: Vec<_> = doc.tagged_following(c, tag_id).collect();
    assert_eq!(following, vec![f]);
}

#[test]
fn test_attributes_axis() {
    let doc = parse_document(r#"<doc a="A" b="B" c="C" />"#).unwrap();
    let doc_el = doc.document_element();
    let a = doc
        .attribute_node(doc_el, &Name::name_without_namespace("a"))
        .unwrap();
    let b = doc
        .attribute_node(doc_el, &Name::name_without_namespace("b"))
        .unwrap();
    let c = doc
        .attribute_node(doc_el, &Name::name_without_namespace("c"))
        .unwrap();

    let attributes: Vec<_> = doc.attributes(doc_el).collect();
    assert_eq!(attributes, vec![a, b, c]);
}
