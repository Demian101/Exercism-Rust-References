pub fn sum_of_multiples(level: u32, items: &[u32]) -> u32 {
    (1..level).filter(|n|
        items.iter()
            .filter(|&&x| x > 0)
            .any(|&item| n % item == 0
        )
    )
    .sum()
}