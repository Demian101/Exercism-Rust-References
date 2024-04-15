pub fn rotate(input: &str, key: u8) -> String {
    let change = |ch: char, base: u8| {
       ((ch as u8 - base + key) % 26 + base) as char
    };

    input
        .chars()
        .map(|c| {
            match c {
                'a'..='z' => change(c, b'a'),
                'A'..='Z' => change(c, b'A'),
                _ => c,
            }
        })
        .collect()
}