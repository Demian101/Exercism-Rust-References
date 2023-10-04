pub fn build_proverb(list: &[&str]) -> String {
  // ["nail", "shoe", "horse"];
  let words = list.to_vec(); // Vec<&str>
  words.iter().enumerate().map(|(i, v)| {
    match i < words.len()-1 {
      true => format!("For want of a {} the {} was lost.\n"
                 , v, words[i+1]),
      false => format!("And all for the want of a {}."
                 , words.first().unwrap())
    }
  }).collect()
}