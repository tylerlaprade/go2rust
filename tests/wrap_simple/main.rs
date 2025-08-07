use std::sync::{Arc, Mutex};

fn main() {
    let mut x = Arc::new(Mutex::new(Some(42)));
    println!("{}", (*x.lock().unwrap().as_mut().unwrap()));
}