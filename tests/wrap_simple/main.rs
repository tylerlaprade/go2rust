use std::sync::{Arc, Mutex};

fn main() {
        // Simplest possible test
    let mut x = Arc::new(Mutex::new(Some(42)));
    println!("{}", (*x.lock().unwrap().as_mut().unwrap()));
}