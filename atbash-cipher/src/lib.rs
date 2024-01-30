// `mindblowingly.` --> `nrmwy oldrm tob` (5 个一组)
pub fn encode(plaintext: &str) -> String {
    plaintext
        .to_lowercase()
        .chars()
        .filter_map(mapping).collect::<Vec<_>>()
        .chunks(5)
        // Convert each chunk to a String
        .map(|chunk| chunk.iter().collect::<String>()) 
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode(ciphertext: &str) -> String {
    ciphertext.to_lowercase().chars()
        .filter_map(mapping).collect()
}

fn mapping(c: char) -> Option<char> {
    match c {
        '0'..='9' => Some(c),
        'a'..='z' => Some((b'a' + b'z' - c as u8) as char),
        _ => None
    }
}

