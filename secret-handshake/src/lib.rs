static ACTIONS: &[&str] = &["wink","double blink","close your eyes","jump"];

pub fn actions(n: u8) -> Vec<&'static str> {
    let n_binary = format!("{:b}", n);
    let mut actions = Vec::<&'static str>::new();
    for i in 0..n_binary.len() {
        if n_binary.chars().nth(n_binary.len() - i -1).unwrap() == '1' {
            if i == 4 {
                actions.reverse();
                return actions;
            }
            actions.push(ACTIONS[i]);
        }

    }
    actions
}
