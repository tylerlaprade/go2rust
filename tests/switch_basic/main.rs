use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut i = Rc::new(RefCell::new(Some(2)));
    { let _switch_val = (*i.borrow().as_ref().unwrap());
    if _switch_val == (1) {
            println!("{}", format!("{}", "one".to_string()));
        } else if _switch_val == (2) {
            println!("{}", format!("{}", "two".to_string()));
        } else if _switch_val == (3) {
            println!("{}", format!("{}", "three".to_string()));
        }
    }

    if (*i.borrow().as_ref().unwrap()) < 2 {
            println!("{}", format!("{}", "less than 2".to_string()));
        } else if (*i.borrow().as_ref().unwrap()) > 2 {
            println!("{}", format!("{}", "greater than 2".to_string()));
        } else {
            println!("{}", format!("{}", "equal to 2".to_string()));
        }
}