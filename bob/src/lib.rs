pub fn reply(message: &str) -> &str {
    match message.trim() {
        s if am_yelling(s) && am_asking(s) => "Calm down, I know what I'm doing!",
        s if am_yelling(s) => "Whoa, chill out!",
        s if am_asking(s) => "Sure.",
        s if said_nothing(s) => "Fine. Be that way!",
        _ => "Whatever."
    }
}

fn am_yelling(m: &str) -> bool {
    m.to_uppercase() == m && m.chars().any(|c| c.is_uppercase())
}
fn am_asking(m: &str) -> bool {
    m.ends_with('?')
}

fn said_nothing(m: &str) -> bool {
    m.is_empty()
}