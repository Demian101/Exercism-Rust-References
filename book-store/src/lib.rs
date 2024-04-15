use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};
const DISCOUNTS: [u32; 6] = [0, 0, 40, 80, 160, 200];
const COST_OF_ONE: u32 = 800;


pub fn lowest_price(books: &[u32]) -> u32 {
    let mut freq = vec![0; 6];
    for b in books {
        freq[*b as usize] += 1;
    }
    // for input = &[1, 1, 2, 2, 3, 3, 4, 5];
    // freq be like: [0, 2, 2, 2, 1, 1]
    // 第一/第二/第三 本数量都是 2, 第四/五本数量都是 1 
    helper(&mut freq, &mut HashMap::new())
}

// books: [0, 2, 2, 2, 1, 1]
fn helper(books: &mut Vec<u32>, memo: &mut HashMap<Vec<u32>, u32>) -> u32 {
    // books=[0, 1, 2] and books=[2, 0, 1] have the same cost
    let key = find_non_zero(books, |(_, &count)| count);
    println!("key  {:?}", key); // [1, 1, 2, 2, 2]

    // contains_key can accept `&Vec` type !
    if memo.contains_key(&key) {
        return memo[&key];
    }
    // Find the books that can be added to the basket
    let remaining = find_non_zero(books, |(b, _)| b);
    println!("remaining  {:?}", remaining); //  [1, 2, 3, 4, 5]

    if remaining.is_empty() {
        // println!("call me ");
        memo.insert(key, 0);
        return 0;
    }

    let mut cost = u32::MAX;
    let mut basket = HashSet::new();
    let books_copy = books.to_vec();

    // Try adding all combinations of the remaining books to the basket
    for i in 1..=remaining.len() {
        for choice in remaining
            .iter()
            .combinations(i)
            // Dedup combinations that have the same counts.
            // For example, books=[1, 1] has three combinations [[1], [1], [1, 1]],
            // where the second combination is redundant.
            .unique_by(|c| c.iter().map(|&x| books_copy[*x]).collect::<Vec<u32>>())
        {
            for book in &choice {
                basket.insert(**book);
                books[**book] -= 1;
            }
            // Check the cost of the current basket, and start a new basket
            // 递归计算剩余书籍的最低价格并与当前 cost 对比，选择更小的一个
            cost = cmp::min(cost, get_cost(basket.len()) + helper(books, memo));
            // Undo the current combination and try another
            for book in &choice {
                basket.remove(book);
                books[**book] += 1;
            }
        }
    }
    memo.insert(key, cost);
    cost
}

fn find_non_zero<T: Ord, F>(books: &[u32], f: F) -> Vec<T>
where
    F: FnMut((usize, &u32)) -> T,
{
    let mut result: Vec<T> = books
        .iter()
        .enumerate() // 在 map 过程中保留每个元素的索引
        .filter(|(_, &count)| count > 0)
        .map(f)
        .collect();

    result.sort();
    result

}

fn get_cost(n: usize) -> u32 {
    (COST_OF_ONE - DISCOUNTS[n]) * n as u32
}