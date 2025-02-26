use xoz::Xoz;

#[test]
fn test_advanced_compare_text() {
    let mut xoz = Xoz::new();
    let root1 = xoz.parse_str(r#"<a>text</a>"#).unwrap();
    let root2 = xoz.parse_str(r#"<a>TEXT</a>"#).unwrap();
    // case insensitive compare
    assert!(xoz.advanced_deep_equal(
        root1,
        root2,
        |_| true,
        |a, b| a.to_lowercase() == b.to_lowercase()
    ));
}

#[test]
fn test_advanced_compare_element() {
    let mut xoz = Xoz::new();
    let root1 = xoz.parse_str(r#"<a>text</a>"#).unwrap();
    let root2 = xoz.parse_str(r#"<b>text</b>"#).unwrap();

    assert!(!xoz.advanced_deep_equal(root1, root2, |_| true, |a, b| a == b));
}

#[test]
fn test_advanced_compare_text2() {
    let mut xoz = Xoz::new();
    let root1 = xoz.parse_str(r#"<a>text</a>"#).unwrap();
    let root2 = xoz.parse_str(r#"<a>different</a>"#).unwrap();
    // case insensitive compare doesn't matter, it's still different
    assert!(!xoz.advanced_deep_equal(
        root1,
        root2,
        |_| true,
        |a, b| a.to_lowercase() == b.to_lowercase()
    ));
}

#[test]
fn test_advanced_compare_attribute_text() {
    let mut xoz = Xoz::new();
    let root1 = xoz.parse_str(r#"<a alpha="alpha">text</a>"#).unwrap();
    let root2 = xoz.parse_str(r#"<a alpha="ALPHA">text</a>"#).unwrap();
    // case insensitive compare for attributes too
    assert!(xoz.advanced_deep_equal(
        root1,
        root2,
        |_| true,
        |a, b| a.to_lowercase() == b.to_lowercase()
    ));
}

#[test]
fn test_advanced_compare_filter() {
    let mut xoz = Xoz::new();
    let root1 = xoz.parse_str(r#"<a>text<!--comment--></a>"#).unwrap();
    let root2 = xoz.parse_str(r#"<a>text</a>"#).unwrap();
    // compare, disregarding comments
    assert!(xoz.advanced_deep_equal(root1, root2, |node| !xoz.is_comment(node), |a, b| a == b));
}
