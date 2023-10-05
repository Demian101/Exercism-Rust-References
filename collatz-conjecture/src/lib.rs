// pub fn collatz(mut n: u64) -> Option<u64> {
//     if n == 0 { return None }
//     let mut count = 0;
//     while n != 1 {
//        match n % 2  {
//             0 => n /= 2,
//             1 => n = n * 3 + 1,

//         }
//         count += 1;
//     }
//     Some(count)
// }

pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 { return None }
    let mut count = 0;
    while n != 1 {
        match n % 2  {   // 在匹配的同时, 这步代码也被执行了
            0 => n /= 2,
            _ => n = n.checked_mul(3)?.checked_add(1)?,
        }
        count += 1;
    }
    Some(count)
}
