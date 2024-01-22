pub fn encode(source: &str) -> String {
    if source.len() == 0 { return "".to_string(); }

    let mut iter = source.chars();

    let mut encoded = String::new();
    let mut cur_char = iter.next().unwrap();
    let mut count = 1;
    
    while let Some(c) = iter.next() {
        if cur_char == c { count += 1; } 
        else {
            if count > 1 {
                encoded.push_str(count.to_string().as_str());
            }
            encoded.push(cur_char);

            count = 1;
        }
        cur_char = c;
    }
    if count > 1 { encoded.push_str(count.to_string().as_str()); }
    encoded.push(cur_char);
    encoded
}

pub fn decode(source: &str) -> String {
    let mut decoded = String::new();
    let mut iter = source.chars().peekable();
    let mut count = 0;

    while let Some(c) = iter.peek() {
        match c.is_digit(10) {
            true => {
                count = count * 10 + c.to_digit(10).unwrap();
                iter.next(); // consume the digit
            },
            false => {
                if count == 0 { count = 1; }
                for _ in 0..count { decoded.push(*c);}
                count = 0;
                iter.next(); // consume the character
            },
        }
    }
    decoded
}