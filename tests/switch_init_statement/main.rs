use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4])));

    let mut n = (*values.borrow().as_ref().unwrap()).len();
    { let _switch_val = n;
    if _switch_val == (0) {
            println!("{}", "empty".to_string());
        } else if _switch_val == (4) {
            println!("{}", "len is four".to_string());
        } else {
            println!("{}", "other".to_string());
        }
    }

    let mut x = Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap())[1 as usize].clone() * 10)));
    if (*x.borrow().as_ref().unwrap()) > 30 {
            println!("{}", "large".to_string());
        } else {
            println!("{}", "small".to_string());
        }
}