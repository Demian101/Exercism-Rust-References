// 模数，表示字母表的长度
const M: i32 = 26;

// 错误类型，表示非互质错误
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// 使用仿射密码（Affine Cipher）对明文进行加密，返回Result类型，其中Ok包含加密后的结果，Err包含错误类型（非互质错误）
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // 检查是否互质，如果不是，则返回错误
    match find_mmi(a, M) {
        Some(_) => Ok(encoded(plaintext, a, b)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

// 加密函数，返回加密后的字符串
fn encoded(plaintext: &str, a: i32, b: i32) -> String {
    // 将明文转换为小写，过滤非字母数字字符，应用仿射密码加密，并按照规定的分组长度进行分组
    plaintext
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| encode_char(c, a, b))
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

// 加密字符处理函数，返回加密后的字符
fn encode_char(c: char, a: i32, b: i32) -> char {
    match c {
        '0'..='9' => c,
        _ => {
            let x = c as i32 - 'a' as i32;
            // 计算仿射密码加密后的数值，使用rem_euclid确保结果在合适范围内
            let num = (a * x + b).rem_euclid(M);
            char::from(num as u8 + 'a' as u8)
        }
    }
}

/// 使用仿射密码（Affine Cipher）对密文进行解密，返回Result类型，其中Ok包含解密后的结果，Err包含错误类型（非互质错误）
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // 检查是否互质，如果不是，则返回错误
    match find_mmi(a, M) {
        Some(mmi) => Ok(decoded(ciphertext, b, mmi)),
        None => Err(AffineCipherError::NotCoprime(a)),
    }
}

// 解密函数，返回解密后的字符串
fn decoded(ciphertext: &str, b: i32, mmi: i32) -> String {
    // 过滤非字母数字字符，应用仿射密码解密
    ciphertext
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| decode_char(c, b, mmi))
        .collect()
}

// 解密字符处理函数，返回解密后的字符
fn decode_char(c: char, b: i32, mmi: i32) -> char {
    match c {
        '0'..='9' => c,
        _ => {
            let y = c as i32 - 'a' as i32;
            // 计算仿射密码解密后的数值，使用rem_euclid确保结果在合适范围内
            let num = (mmi * (y - b)).rem_euclid(M);
            char::from(num as u8 + 'a' as u8)
        }
    }
}

// 查找模数乘法逆元，返回Option类型，其中Some包含逆元，返回 None 表示逆元不存在
fn find_mmi(a: i32, m: i32) -> Option<i32> {
    (1..m).into_iter().find(|n| a * n % m == 1)
}