use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    if 7 % 2 == 0 {
        println!("{}", format!("{}", "7 is even".to_string()));
    } else {
        println!("{}", format!("{}", "7 is odd".to_string()));
    }

    if 8 % 4 == 0 {
        println!("{}", format!("{}", "8 is divisible by 4".to_string()));
    }

    let mut num = Rc::new(RefCell::new(Some(9)));
    if (*num.borrow().as_ref().unwrap()) < 0 {
        println!("{} {}", format!("{}", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "is negative".to_string()));
    } else if (*num.borrow().as_ref().unwrap()) < 10 {
        println!("{} {}", format!("{}", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "has 1 digit".to_string()));
    } else {
        println!("{} {}", format!("{}", { let __v = (*num.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "has multiple digits".to_string()));
    }
}