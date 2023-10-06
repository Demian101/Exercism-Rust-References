#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}
#[rustfmt::skip]
fn require<T>(predicate: bool, error: T) -> Result<(), T> {
    if predicate { Ok(()) } else { Err(error) }
}
///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    convert_from_base(number, from_base).and_then(|n| convert_to_base(n, to_base))
}
fn convert_from_base(number: &[u32], base: u32) -> Result<u32, Error> {
    require(base >= 2, Error::InvalidInputBase)?;
    let mut num = 0;
    for &digit in number {
        require(digit < base, Error::InvalidDigit(digit))?;
        num *= base;
        num += digit;
    }
    Ok(num)
}
fn convert_to_base(mut number: u32, base: u32) -> Result<Vec<u32>, Error> {
    require(base >= 2, Error::InvalidOutputBase)?;
    if number == 0 {
        return Ok(vec![0]);
    }
    let mut ret = vec![];
    while number > 0 {
        ret.push(number % base);
        number /= base;
    }
    ret.reverse();
    Ok(ret)
}