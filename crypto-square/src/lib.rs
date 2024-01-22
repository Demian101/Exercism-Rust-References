pub fn encrypt(input: &str) -> String {
    if input.is_empty() {
        return "".to_string();
    }
    // normalization
    let mut norm = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase());

    // 获取矩形的行数和列数
    let (c, r) = dim(norm.clone().count());
    // // 初始化一个二维向量，表示矩形的每一行 
    let mut result = vec![String::new(); c];
    // 遍历编码消息的每个字符，并按照规则填充到二维向量中
    for i in 0..c * r {
        match norm.next() {
            Some(x) => result[i % c].push(x), // 将字符添加到对应列的行中
            None => result[i % c].push(' '),  // 如果没有字符了，则添加空格
        }
    }
    // 将二维向量连接成一个字符串，每个块之间用空格分隔
    result.join(" ")
}

fn dim(str_length: usize) -> (usize, usize) {
    let mut c = 1;
    let mut r = 1;
    // 计算列数和行数，使得列数乘以行数大于等于字符串长度
    while c * r < str_length {
        match c > r {
            true => r += 1,
            false => c += 1,
        }
    }
    (c, r)
}