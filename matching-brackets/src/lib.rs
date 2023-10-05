// 我的代码:
// pub fn brackets_are_balanced(s: &str) -> bool {
//   let mut brackets = Vec::<char>::new();
//   for ch in s.chars() {
//     if ch == '(' || ch == '{' || ch == '[' { brackets.push(ch); }
//     match ch {
//       if c == ')' && brackets.pop() != '(' => return false ,
//       if c == '}' && brackets.pop() != '{' => return false ,
//       if c == ']' && brackets.pop().unwrap() != '[' => return false ,
//     }  
//   }
//   return true;
// }

// solution
pub fn brackets_are_balanced(s: &str) -> bool {
  let mut brackets = Vec::<char>::new(); // or bra: Vec<char> = Vec::new()
  for ch in s.chars() {
    match ch {
      '(' | '{' | '[' => brackets.push(ch),
      ')' => if brackets.pop() != Some('(') { return false },
      '}' => if brackets.pop() != Some('{') { return false },
      ']' => if brackets.pop() != Some('[') { return false },
      _ => (),
    }
  }
  brackets.is_empty()
}
