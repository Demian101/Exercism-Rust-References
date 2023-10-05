//phrase: "Complementary metal-oxide semiconductor";
pub fn abbreviate(phrase: &str) -> String {
    phrase.split(&[' ', '_', '-'])
    .flat_map( |word| {
        word.chars().take(1).chain(
            word.chars()
                .skip_while(|ch| ch.is_uppercase())
                .filter(|ch| ch.is_uppercase()) 
        )
    }).collect::<String>()
    .to_uppercase()
}