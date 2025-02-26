// #[test]
// fn test_advanced_compare_text() {
//     let doc1 = xoz::Document::parse_str(r#"<a>text</a>"#).unwrap();
//     let doc2 = xoz::Document::parse_str(r#"<a>TEXT</a>"#).unwrap();
//     let root1 = doc1.root();
//     let root2 = doc2.root();
//     // case insensitive compare
//     assert!(doc1.advanced_deep_equal(
//         root1,
//         root2,
//         |_| true,
//         |a, b| a.to_lowercase() == b.to_lowercase()
//     ));
// }

// #[test]
// fn test_advanced_compare_text2() {
//     let doc1 = xoz::Document::parse_str(r#"<a>text</a>"#).unwrap();
//     let doc2 = xoz::Document::parse_str(r#"<a>different</a>"#).unwrap();
//     let root1 = doc1.root();
//     let root2 = doc2.root();
//     // case insensitive compare doesn't matter, it's still different
//     assert!(!doc1.advanced_deep_equal(
//         root1,
//         root2,
//         |_| true,
//         |a, b| a.to_lowercase() == b.to_lowercase()
//     ));
// }

// #[test]
// fn test_advanced_compare_attribute_text() {
//     let doc1 = xoz::Document::parse_str(r#"<a alpha="alpha">text</a>"#).unwrap();
//     let doc2 = xoz::Document::parse_str(r#"<a alpha="ALPHA">text</a>"#).unwrap();
//     let root1 = doc1.root();
//     let root2 = doc2.root();
//     // no text compares as equal, so this is not equal
//     assert!(doc1.advanced_deep_equal(
//         root1,
//         root2,
//         |_| true,
//         |a, b| a.to_lowercase() == b.to_lowercase()
//     ));
// }

// #[test]
// fn test_advanced_compare_filter() {
//     let doc1 = xoz::Document::parse_str(r#"<a>text<!--comment--></a>"#).unwrap();
//     let doc2 = xoz::Document::parse_str(r#"<a>text</a>"#).unwrap();
//     let root1 = doc1.root();
//     let root2 = doc2.root();
//     // compare, disregarding comments
//     assert!(doc1.advanced_deep_equal(root1, root2, |node| !doc1.is_comment(node), |a, b| a == b));
// }
