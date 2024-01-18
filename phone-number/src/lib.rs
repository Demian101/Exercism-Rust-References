pub fn number(user_number: &str) -> Option<String> {
    // get all numbers.
    let number: String = user_number
        .chars()
        .filter(|c|c.is_numeric())
        .collect();

    // 长度为 10，且符合 NANP 的格式 -- 直接返回
    if number.len() == 10 && check_format(&number) {
        Some(number)

    // 长度为11，检查第一位是否为'1'，并且剩下的数字符合 NANP 的格式
    } else if number.len() == 11  && &number[0..1] == "1"  && check_format(&number[1..]) {
        Some(String::from(&number[1..]))
    } else {
        None
    }
}


fn check_format(number: &str) -> bool {
    number
        .char_indices()
        // `all` 方法检查迭代器中的所有项是否都满足某个条件。
        .all(|(index, char)|
          // 第一个数字（index == 0）和第四个数字（index == 3）必须在'2'到'9'的范围内 
          (index == 0 || index == 3) && char >= '2'
          // // 其他位置的数字可以在'0'到'9'的范围内
          || index != 0 && index != 3) 
}

