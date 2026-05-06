use std::cell::{RefCell};
use std::rc::{Rc};

/// This is a doc comment for the main function
fn main() {
        // Initialize a variable
    let mut x = Rc::new(RefCell::new(Some(42)));

        // Print the value
    println!("{} {}", "Value:".to_string(), { let __v = (*x.borrow().as_ref().unwrap()).clone(); __v });

        // Do some math
    let mut y = Rc::new(RefCell::new(Some((*x.borrow().as_ref().unwrap()) * 2)));

        // Another comment
        // spanning multiple lines
        // to test preservation
    println!("{} {}", "Double:".to_string(), { let __v = (*y.borrow().as_ref().unwrap()).clone(); __v });
}