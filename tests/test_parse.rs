use xoz::{error::quickxml, Xoz};

// TODO: This doesn't work yet
// #[test]
// fn test_unclosed_tag() {
//     let mut xoz = Xoz::new();
//     let root = xoz.parse_str(r#"<doc>"#);
//     assert!(root.is_err());
// }

#[test]
fn test_unmatched_end() {
    let mut xoz = Xoz::new();
    let err = xoz.parse_str(r#"</doc>"#).unwrap_err();

    assert!(matches!(
        err,
        quickxml::Error::IllFormed(quickxml::IllFormedError::UnmatchedEndTag(_))
    ));
}

#[test]
fn test_mismatched_end() {
    let mut xoz = Xoz::new();
    let err = xoz.parse_str(r#"<doc></doc2>"#).unwrap_err();
    assert!(matches!(
        err,
        quickxml::Error::IllFormed(quickxml::IllFormedError::MismatchedEndTag { .. })
    ));
}

#[test]
fn test_attributes_in_end() {
    let mut xot = Xoz::new();
    let err = xot.parse_str(r#"<doc></doc attr="value">"#).unwrap_err();
    assert!(matches!(
        err,
        quickxml::Error::IllFormed(quickxml::IllFormedError::MismatchedEndTag { .. })
    ));
}

#[test]
fn test_attributes_in_end2() {
    let mut xot = Xoz::new();
    let err = xot
        .parse_str(r#"<doc attr="value"></doc attr="value">"#)
        .unwrap_err();
    assert!(matches!(
        err,
        quickxml::Error::IllFormed(quickxml::IllFormedError::MismatchedEndTag { .. })
    ));
}
