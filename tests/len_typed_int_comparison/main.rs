use std::cell::{RefCell};
use std::rc::{Rc};

pub fn consumed_all(values: Rc<RefCell<Option<Vec<i32>>>>) -> bool {

    let mut i: Rc<RefCell<Option<i32>>> = Rc::new(RefCell::new(Some(0)));
    { let new_val = 0; *i.borrow_mut() = Some(new_val); };
    while ((*i.borrow().as_ref().unwrap()) as i32) < ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return ((*i.borrow().as_ref().unwrap()) as i32) == ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32);
}

pub fn last_index(values: Rc<RefCell<Option<Vec<i32>>>>) -> i32 {

    return ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) - (1 as i32);
}

pub fn within_double(values: Rc<RefCell<Option<Vec<i32>>>>, i: Rc<RefCell<Option<i32>>>) -> bool {

    return ((*i.borrow().as_ref().unwrap()) as i32) < (((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) + ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) as i32);
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![1, 2, 3])));
    println!("{}", format!("{}", consumed_all(values.clone())));
    println!("{}", format!("{}", last_index(values.clone())));
    println!("{}", format!("{}", within_double(values.clone(), Rc::new(RefCell::new(Some(4))))));
}