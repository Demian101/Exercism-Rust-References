use series::*;

#[test]
fn with_length_2() {
    let expected = vec![
        "13".to_string(),
        "35".to_string(),
        "57".to_string(),
        "79".to_string(),
    ];
    assert_eq!(series("13579", 2), expected);
}

#[test]
fn with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}

#[test]
fn too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}

#[test]
fn way_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 42), expected);
}
