pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // 孩子们的名字，按字母顺序排列
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", 
        "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"
    ];
    // 找到请求的学生在名单中的位置
    let student_index = students.iter().position(|&s| s == student).unwrap();

    // 用来将字符转化为植物名的映射
    let plant_mapping: Vec<(&str, char)> = vec![
        ("grass", 'G'), 
        ("clover", 'C'), 
        ("radishes", 'R'), 
        ("violets", 'V')
    ];

    // 从输入的 diagram 中提取每个孩子的植物字符
    let diagram_lines: Vec<&str> = diagram.split('\n').collect();
    let mut student_plants = String::new();
    for line in &diagram_lines {
        // 注意：每个学生在每一行上占据 2 个位置
        student_plants.push(line.chars().nth(student_index * 2).unwrap());
        student_plants.push(line.chars().nth(student_index * 2 + 1).unwrap());
    }

    // 将字符映射为植物名
    student_plants
        .chars()
        .map(|c| plant_mapping.iter().find(|&&(_, plant_char)| plant_char == c).unwrap().0)
        .collect()
}
