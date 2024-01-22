pub struct WordProblem;

pub fn answer(command: &str) -> Option<i32> {
    if command.starts_with("What is ") {
        let (n, rest) = num(&command[8..])?;
        op(n, rest)
    } else {
        None
    }
}

fn op(n: i32, command: &str) -> Option<i32> {
    if command == "?" {
        Some(n)
    } else if command.starts_with(" plus ") {
        let (m, rest) = num(&command[6..])?;
        op(n + m, rest)
    } else if command.starts_with(" minus ") {
        let (m, rest) = num(&command[7..])?;
        op(n - m, rest)
    } else if command.starts_with(" multiplied by ") {
        let (m, rest) = num(&command[15..])?;
        op(n * m, rest)
    } else if command.starts_with(" divided by ") {
        let (m, rest) = num(&command[12..])?;
        op(n / m, rest)
    } else {
        None
    }
}

fn num(command: &str) -> Option<(i32, &str)> {
    let num_digits: String = command.chars().take_while(|x| x.is_digit(10) || *x == '-').collect();
    let n = num_digits.parse::<i32>().ok()?;
    Some((n, &command[num_digits.len()..]))
}