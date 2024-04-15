pub struct RailFence {
  rails: usize
}
impl RailFence {
  pub fn new(rails: u32) -> RailFence {
    RailFence { rails: rails as usize }
  }
  pub fn encode(&self, text: &str) -> String {
    let mut res = vec![Vec::new(); self.rails];
    for (c, i) in text.chars().zip(zigzag(self.rails)) {
      res[i].push(c);  // res 分为 rails 的 3 列
    }
    // res: [['I', 'E'], ['L', 'V', 'Y', 'U'], ['O', 'O']]
    res.iter().flat_map(|c| c).collect::<String>() // "IELVYUOO"
     
  }
  pub fn decode(&self, cipher: &str) -> String {
    // indexes: [(0, 1), (1, 2), (2, 3), (1, 4), (0, 5), (1, 6), (2, 7), (1, 8)]
    let mut indexes : Vec<_> = zigzag(self.rails).zip(1..).take(cipher.len()).collect();

    indexes.sort();  // [(0,1), (0,5), (1,2), (1,4), (1,6), (1,8), (2,3), (2,7)]

    let mut char_with_index : Vec<_> = cipher.chars().zip(indexes).map(|(c, (_, i))| (i, c)).collect();
    char_with_index.sort();
    char_with_index.iter().map(|(_, c)| c).collect()
  }
}
fn zigzag(n: usize) -> impl Iterator<Item = usize> {
  // rails = 3 -> 0121|0121|0121|... 无限循环
  (0..n - 1).chain((1..n).rev()).cycle()
}