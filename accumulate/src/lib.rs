pub fn map_custom<A, B, F>(input: Vec<A>, mut func: F) -> Vec<B>
where
    F: FnMut(A) -> B,
{
    let mut result = Vec::new();
    for item in input {
        result.push(func(item));
    }
    result
}
