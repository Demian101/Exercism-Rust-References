// // /// Determines whether the supplied string is a valid ISBN number
// pub fn is_valid_isbn(isbn: &str) -> bool {
//     let isbn = isbn.replace("-", "");
//     if isbn.len() != 9+1 {return false }
    
//     let check_sum: u32 = isbn.chars().take(9).zip(
//         (1..9).rev()
//     ).map(|(ch, num)| {
//         ch.to_digit(10).unwrap() * num // Error..
//     }).sum();

//     let check_digit = match isbn.chars().last().unwrap() {
//         'X' | 'x' => 10,
//         ch => ch.to_digit(10).unwrap_or(0),
//     };
//     (check_sum + check_digit) % 11 == 0
// }

// pub fn is_valid_isbn(isbn: &str) -> Result<bool, &'static str> {
//     let isbn = isbn.replace("-", "");

//     if isbn.len() != 10 {
//         return Err("ISBN must be 10 characters in length");
//     }

//     let check_sum: u32 = isbn
//         .chars()
//         .take(9)
//         .enumerate()
//         .map(|(i, ch)| {
//             ch.to_digit(10).ok_or("Non-numeric character found")? * (10 - i as u32)
//         })
//         .sum::<Result<u32, &'static str>>()?;

//     let check_digit = match isbn.chars().last().unwrap() {
//         'X' | 'x' => 10,
//         ch => ch.to_digit(10).ok_or("Invalid check digit")?,
//     };

//     Ok((check_sum + check_digit) % 11 == 0)
// }

// pub fn is_valid_isbn(isbn: &str) -> bool {
//     let isbn = isbn.chars().filter(|&c| c.is_alphanumeric()).collect::<String>();
//     if isbn.len() != 10 { return false; }
//     isbn.chars()
//         .enumerate()
//         .filter_map(digit_value)
//         .zip((1..=10).rev())
//         .fold(0, |checksum, (v, i)| checksum + v * i) % 11 == 0
// }

// fn digit_value((index, c): (usize, char)) -> Option<u32> {
//     if c == 'X' && index == 9 {
//         Some(10)
//     } else {
//         c.to_digit(10) // return char to 10 进制
//     }
// }

pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn = isbn.replace("-", "");
    if isbn.len() != 10 { return false; }

    let check_sum: Option<u32> = isbn
        .chars()
        .take(9)
        .enumerate()
        .map(|(i, ch)| ch.to_digit(10).map(|digit| 
            digit * (10 - i as u32)))
        .sum();

    let check_digit: Option<u32> = match isbn.chars().last().unwrap() {
        'X' | 'x' => Some(10),
        ch => ch.to_digit(10),
    };

    match (check_sum, check_digit) {
        (Some(sum), Some(digit)) => (sum + digit) % 11 == 0,
        _ => false,
    }
}
