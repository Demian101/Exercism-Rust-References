use std::cmp::max;
pub struct Item {
    pub weight: u32,
    pub value: u32,
}
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut dp = vec![vec![0; max_weight as usize + 1]; items.len()+1];

    for item in 1..=items.len() {
        for cap in 1..=max_weight as usize{
            let cur_weight = items[item - 1].weight as usize;
            let cur_value = items[item - 1].value;
            dp[item][cap] = if cur_weight > cap {
                dp[item - 1][cap]
            } else {
                max(dp[item - 1][cap],
                    dp[item - 1][cap - cur_weight] + cur_value)
            };
            println!("item {} cap {} : {:?}", item, cap, dp);
        }
    }
    dp[items.len()][max_weight as usize]
}


// use std::cmp::max;
// #[derive(Clone)]
// pub struct Item {
//     pub weight: u32,
//     pub value: u32,
// }
// pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
//     let mut max_value = 0;
//     let mut items = items.to_vec();
//     while let Some(item) = items.pop() {
//         let val = if item.weight <= max_weight {
//             item.value + maximum_value(max_weight - item.weight, &items)
//         } else {
//             0
//         };
//         max_value = max(val, max_value);
//     }
//     max_value
// }
