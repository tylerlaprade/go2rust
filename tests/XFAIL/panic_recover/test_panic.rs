use std::sync::{Arc, Mutex};

fn main() {
    eprintln!("{}", "Before panic".to_string());
    panic!("test panic");
    eprintln!("{}", "After panic".to_string());
}