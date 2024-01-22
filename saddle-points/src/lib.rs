pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = vec![];
    for (y, row) in input.iter().enumerate() {
        for (x, n) in row.iter().enumerate() {
            let col: Vec<u64> = input.iter().map(|v| v[x]).collect();
            if row.iter().all(|m| n>=m) && col.iter().all(|m| n<=m){
                points.push((y, x));
            }
        }
    }
    points
}
