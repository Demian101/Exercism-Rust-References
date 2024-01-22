const VOWELS: &str = "aeiou";

pub fn translate(input: &str) -> String {
    let mut output = vec![];

    for word in input.split_whitespace() {
        output.push(translate_word(word));
    }

    output.join(" ")
}

fn translate_word(input: &str) -> String {
    let mut skipped = String::new();

    if !starts_with_vowel(input) {
        let mut chars = input.chars().peekable();

        while let Some(curren_char) = chars.next() {
            skipped.push(curren_char);
            let next_char = chars.peek();
            if curren_char == 'q' && next_char == Some(&'u') {
                skipped.push('u');
                break;
            }
            if let Some(&next_char) = next_char {
                if next_char == 'y' || VOWELS.contains(next_char) {
                    break;
                }
            }
        }
    }

    input.chars().skip(skipped.len()).collect::<String>() + &skipped + "ay"
}

fn starts_with_vowel(input: &str) -> bool {
    input.starts_with("xr")
        || input.starts_with("yt")
        || VOWELS.contains(input.chars().next().unwrap())
}