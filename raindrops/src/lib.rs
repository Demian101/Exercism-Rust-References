const WORDS: [(usize, &'static str); 3] = [
    (3, "Pling"),
    (5, "Plang"),
    (7, "Plong"),
];

pub fn raindrops(n: usize) -> String {
    let res: String = WORDS.iter()
        .filter_map(|(d, s)|  
          if n % d == 0  {Some(s.to_string())} else {None}
        ).collect();
    if res.is_empty(){ return n.to_string() }
    else {
        return res
    }
}
