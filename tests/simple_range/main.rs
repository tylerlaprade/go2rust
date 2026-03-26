use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
        // Simple slice range
    let mut numbers = Rc::new(RefCell::new(Some(vec![10, 20, 30])));

        // Index and value
    for (i, num) in (*numbers.borrow().as_ref().unwrap()).iter().copied().enumerate() {
        println!("{} {} {} {}", "Index:".to_string(), i, "Value:".to_string(), num);
    }

        // Value only
    for num in (*numbers.borrow().as_ref().unwrap()).iter().copied() {
        println!("{} {}", "Value:".to_string(), num);
    }

        // Index only
    for i in 0..(*numbers.borrow().as_ref().unwrap()).len() {
        println!("{} {}", "Index:".to_string(), i);
    }
}