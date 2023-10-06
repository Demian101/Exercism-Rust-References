use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashSet;

thread_local!(static NAMES: RefCell<HashSet<String>> = RefCell::new(HashSet::new()));

pub struct Robot(String);

fn generate_name() -> String {
    let mut rng = thread_rng();
    let name = format!(
        "{}{}{:03}",
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(b'A'..=b'Z') as char,
        rng.gen_range(0..1000)
    );
    if NAMES.with(|n| 
        n.borrow_mut().insert(name.clone())
    ) { name }
    else { generate_name() }
}

impl Robot {
    pub fn new() -> Self { Robot(generate_name()) }

    pub fn name(&self) -> &str { &self.0 }

    pub fn reset_name(&mut self) {
        NAMES.with(|c| 
            c.borrow_mut().remove(&self.0)
        );
        self.0 = generate_name();
    }
}


