use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(2)));
    match (*i.borrow_mut().as_mut().unwrap()) {
        1 => {
            println!("{}", "one".to_string());
        }
        2 => {
            println!("{}", "two".to_string());
        }
        3 => {
            println!("{}", "three".to_string());
        }
        _ => {}
    }

    match true {
        true if (*i.borrow_mut().as_mut().unwrap()) < 2 => {
            println!("{}", "less than 2".to_string());
        }
        true if (*i.borrow_mut().as_mut().unwrap()) > 2 => {
            println!("{}", "greater than 2".to_string());
        }
        _ => {
            println!("{}", "equal to 2".to_string());
        }
    }
}