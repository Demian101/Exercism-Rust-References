pub fn is_leap_year(year: u64) -> bool {
    match year {
        x if x % 400 == 0 => true,
        y if y % 100 == 0 => false,
        z if z % 4 == 0 => true,
        _ => false
    }
}

// match year {
//     if year % 4 == 0 {
//         div_4_100 if div_4 % 100 == 0 => return true
//         div_4_not_100 if if div_4 % 100 != 0 => return false
//     }
//     _ => false
// }
