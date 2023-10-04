const WORDS: [(usize, &'static str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong")
];

pub fn raindrops(n: usize) -> String {
    let res: String = WORDS
        .iter()
        .filter_map(|(d, w)| if n % d == 0 { Some(*w) } else { None })
        .collect();

    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
