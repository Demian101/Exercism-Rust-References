// use std::collections::HashSet;

// pub fn check(candidate: &str) -> bool {
//     let mut candidate = candidate.replace(" ","").replace("-","").to_lowercase();
//     let length = candidate.len();
//     let mut seen = HashSet::new();
    
//     candidate.retain(|c| {
//         let is_first = !seen.contains(&c);
//         seen.insert(c);
//         is_first
//     });
//     candidate.len() == length
// }

use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut set = HashSet::new();
    candidate
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .all(|c| set.insert(c))
}