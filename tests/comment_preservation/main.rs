use std::sync::{Arc, Mutex};

/// This is a doc comment for the main function
fn main() {
        // Initialize a variable
    let mut x = Arc::new(Mutex::new(Some(42)));

        // Print the value
    println!("{} {}", "Value:".to_string(), (*x.lock().unwrap().as_mut().unwrap()));

        // Do some math
    let mut y = Arc::new(Mutex::new(Some((*x.lock().unwrap().as_mut().unwrap()) * 2)));

        // Another comment
        // spanning multiple lines
        // to test preservation
    println!("{} {}", "Double:".to_string(), (*y.lock().unwrap().as_mut().unwrap()));
}