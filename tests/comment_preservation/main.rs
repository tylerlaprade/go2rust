use std::cell::{RefCell};
use std::rc::{Rc};

/// This is a doc comment for the main function
fn main() {
        // Initialize a variable
    let mut x = Rc::new(RefCell::new(Some(42)));

        // Print the value
    println!("{} {}", "Value:".to_string(), (*x.borrow_mut().as_mut().unwrap()));

        // Do some math
    let mut y = Rc::new(RefCell::new(Some((*x.borrow_mut().as_mut().unwrap()) * 2)));

        // Another comment
        // spanning multiple lines
        // to test preservation
    println!("{} {}", "Double:".to_string(), (*y.borrow_mut().as_mut().unwrap()));
}