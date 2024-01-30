fn get_char(order: i8, x: i8, y: i8) -> char {
    match x.abs() + y.abs() {
        // order => (x.abs() as u8 + 'A' as u8) as char, 
        _m if _m == order => (x.abs() as u8 + 'A' as u8) as char,
        _ =>' '
    }
}

pub fn get_diamond(ch: char) -> Vec<String> {
    // 阶数, i8 是为了配合 `-order`
    let order = ch as i8 - 'A' as i8;
    (-order..=order).map(|y|
        (-order..=order).map(|x|
            get_char(order, x, y)
        ).collect::<String>()
    ).collect::<Vec<String>>()
}