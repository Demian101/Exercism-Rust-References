/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        let v1 = value.to_string();
        let half = v1.len() / 2;
        match v1
               .bytes()
               .take(half)
               .eq(v1.bytes().rev().take(half)) {
            true => Some(Palindrome(value)),
            _ => None
        }
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palind_vec: Vec<u64> = Vec::new();

    for i in min..max {
        for j in min..=max {
            if let Some(v) = Palindrome::new(i * j){
                if !palind_vec.contains(&v.into_inner()){
                    palind_vec.push(v.into_inner());
                }
            }
        }
    }
    match palind_vec.len() {
        0 => return None,
        _ => return Some(
            ( Palindrome(*palind_vec.iter().min().unwrap()), 
            Palindrome(*palind_vec.iter().max().unwrap()) )
        ),
    };
}
