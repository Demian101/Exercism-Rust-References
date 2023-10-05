fn square(x: i32) -> i32 {
    x * x
}

#[test]
fn func_single() {
    let input = vec![2];
    let expected = vec![4];
    assert_eq!(accumulate::map_custom(input, square), expected);
}

#[test]
fn func_multi() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(accumulate::map_custom(input, square), expected);
}

#[test]
fn closure() {
    let input = vec![2, 3, 4, 5];
    let expected = vec![4, 9, 16, 25];
    assert_eq!(accumulate::map_custom(input, |x| x * x), expected);
}

#[test]
fn closure_floats() {
    let input = vec![2.0, 3.0, 4.0, 5.0];
    let expected = vec![4.0, 9.0, 16.0, 25.0];
    assert_eq!(accumulate::map_custom(input, |x| x * x), expected);
}

#[test]
fn strings() {
    let input = vec!["1".to_string(), "2".into(), "3".into()];
    let expected = vec!["11".to_string(), "22".into(), "33".into()];
    assert_eq!(accumulate::map_custom(input, |s| s.repeat(2)), expected);
}

#[test]
fn change_in_type() {
    let input: Vec<&str> = vec!["1", "2", "3"];
    let expected: Vec<String> = vec!["1".into(), "2".into(), "3".into()];
    assert_eq!(accumulate::map_custom(input, |s| s.to_string()), expected);
}

#[test]
fn mutating_closure() {
    let mut counter = 0;
    let input = vec![-2, 3, 4, -5];
    let expected = vec![2, 3, 4, 5];
    let result = accumulate::map_custom(input, |x: i64| {
        counter += 1;
        x.abs()
    });
    assert_eq!(result, expected);
    assert_eq!(counter, 4);
}

#[test]
fn minimal_bounds_on_input_and_output() {
    // must be able to accept arbitrary input and output types
    struct Foo;
    struct Bar;
    accumulate::map_custom(vec![Foo], |_| Bar);
}
