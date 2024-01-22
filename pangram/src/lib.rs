// use std::collections::HashSet;
// const ALPHA_BETIC: &str = "abcdefghijklmnopqrstuvwxyz";

// pub fn is_pangram(sentence: &str) -> bool {
//     let mut set = HashSet::new();
//     sentence.to_lowercase().chars().filter(|ch| ch.is_ascii_alphabetic())
//         .for_each (|ch| 
//             if !set.contains(&ch) { set.insert(ch); });
//     set.len() == 26 // or ALPHA_BETIC.len()
// }


use std::collections::HashSet;
use std::iter::FromIterator;
const ALPHA_BETIC: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn is_pangram(sentence: &str) -> bool {
    let all:  HashSet<char> = HashSet::from_iter(ALPHA_BETIC.chars());
    let used: HashSet<char> = HashSet::from_iter(sentence.to_lowercase().chars());
    all.is_subset(&used)
}