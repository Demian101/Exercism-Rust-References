#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}
pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 { return Ok(1); }

    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    let mut max_product = 0;
    for i in 0..=string_digits.len() - span {
        let product: u64 = string_digits[i..i + span]
            .chars()
            .map(|c| 
                c.to_digit(10).ok_or(Error::InvalidDigit(c))
            )
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .fold(1, |acc, &x| 
                acc * x as u64
            );
        if product > max_product {
            max_product = product;
        }
    }
    Ok(max_product)
}

