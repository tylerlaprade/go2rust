use std::cell::{RefCell};
use std::rc::{Rc};

pub fn consumed_all(values: Rc<RefCell<Option<Vec<i64>>>>) -> bool {
    let mut i: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(Some(0)));
    { let new_val = 0; *i.borrow_mut() = Some(new_val); };
    while ((*i.borrow().as_ref().unwrap()) as i64) < ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) {
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return ((*i.borrow().as_ref().unwrap()) as i64) == ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64);
}

pub fn last_index(values: Rc<RefCell<Option<Vec<i64>>>>) -> i64 {
    ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) - (1 as i64)
}

pub fn within_double(values: Rc<RefCell<Option<Vec<i64>>>>, i: Rc<RefCell<Option<i64>>>) -> bool {
    ((*i.borrow().as_ref().unwrap()) as i64) < (((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) + ((*values.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64) as i64)
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![1i64, 2i64, 3i64])));
    println!("{}", format!("{}", consumed_all(values.clone())));
    println!("{}", format!("{}", last_index(values.clone())));
    println!("{}", format!("{}", within_double(values.clone(), Rc::new(RefCell::new(Some(4))))));
}