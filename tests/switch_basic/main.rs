use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(2)));
    { let _switch_val = (*i.borrow().as_ref().unwrap());
    if _switch_val == (1) {
            println!("{}", "one".to_string());
        } else if _switch_val == (2) {
            println!("{}", "two".to_string());
        } else if _switch_val == (3) {
            println!("{}", "three".to_string());
        }
    }

    if (*i.borrow().as_ref().unwrap()) < 2 {
            println!("{}", "less than 2".to_string());
        } else if (*i.borrow().as_ref().unwrap()) > 2 {
            println!("{}", "greater than 2".to_string());
        } else {
            println!("{}", "equal to 2".to_string());
        }
}