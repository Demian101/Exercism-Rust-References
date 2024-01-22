/// Compute the Scrabble score for a word.

pub fn score(word: &str) -> u64 {
    let score_sym  = |c| match c {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G' => 2,
        'B' | 'C' | 'M' | 'P' => 3,
        'F' | 'H' | 'V' | 'W' | 'Y' => 4,
        'K' => 5,
        'J' | 'X' => 8,
        'Q' | 'Z' => 10,
        _ => 0,
    };
    word.chars().map(|c| c.to_ascii_uppercase())
        .map(score_sym)
        .fold(0, |acc, b| acc + b)

    // word.chars()
    // .filter(char::is_ascii) // (|&c| c.is_ascii()) // char::is_ascii)
    // .map(|x| x.to_ascii_uppercase())
    // .map(score_sym)
    // .sum()
}
