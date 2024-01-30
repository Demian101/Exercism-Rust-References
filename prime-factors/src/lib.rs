
pub fn factors(mut n: u64) -> Vec<u64> {
    use std::time::Instant;
    let now = Instant::now();

    let mut prime_factors = vec![];
     while n > 1 {
        let i = (2..n+1).find(|x| n % x == 0 ).unwrap(); 
        prime_factors.push(i);
        n /= i;
    } 

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    prime_factors
}


// // 参考解法 1 :
// // 低效原因： candidates.next() 每次只能 + 1
// pub fn factors(mut n: u64) -> Vec<u64> {
//     let mut factors = Vec::new();
//     let mut candidates = 2..;  // 无穷 自然数 迭代器
//     while n > 1 {
//         let x = candidates.next().unwrap();
//         while n % x == 0 {
//             n /= x;
//             factors.push(x);
//             println!("{} - {:?}", n,x);
//         }
//     }
//     factors
// }