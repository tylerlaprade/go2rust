use std::cell::{RefCell};
use std::rc::{Rc};

pub fn is_ready(flag: Rc<RefCell<Option<bool>>>) -> bool {
    (*flag.borrow().as_ref().unwrap())
}

fn main() {
    if !is_ready(Rc::new(RefCell::new(Some(false)))) {
        println!("{}", format!("{}", "not ready".to_string()));
    }
    let mut negated = Rc::new(RefCell::new(Some(!is_ready(Rc::new(RefCell::new(Some(true)))))));
    println!("{}", format!("{}", { let __v = (*negated.borrow().as_ref().unwrap()).clone(); __v }));
    if !is_ready(Rc::new(RefCell::new(Some(true)))) {
        println!("{}", format!("{}", "wrong".to_string()));
    } else {
        println!("{}", format!("{}", "ready".to_string()));
    }
}