use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![1, 2, 3, 4])));

    let mut n = Rc::new(RefCell::new(Some((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    { let _switch_val = (*n.borrow().as_ref().unwrap());
    if _switch_val == (0) {
            println!("{}", format!("{}", "empty".to_string()));
        } else if _switch_val == (4) {
            println!("{}", format!("{}", "len is four".to_string()));
        } else {
            println!("{}", format!("{}", "other".to_string()));
        }
    }

    let mut x = Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap())[(1) as usize].clone() * 10)));
    if (*x.borrow().as_ref().unwrap()) > 30 {
            println!("{}", format!("{}", "large".to_string()));
        } else {
            println!("{}", format!("{}", "small".to_string()));
        }
}