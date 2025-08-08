use std::sync::{Arc, Mutex};

fn main() {
    eprintln!("{}", "This goes to stderr".to_string());

    let mut s = Arc::new(Mutex::new(Some("hello".to_string())));
    eprintln!("{}", (*(*s.lock().unwrap().as_mut().unwrap()).lock().unwrap().as_ref().unwrap()).len());
}