use std::cell::{RefCell};
use std::rc::{Rc};

pub fn accept(n: Rc<RefCell<Option<i64>>>) -> i64 {
    (*n.borrow().as_ref().unwrap())
}

fn main() {
    let mut last = Rc::new(RefCell::new(Some(0i64)));
    let mut values = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])));
    for i in 0..(({ let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let new_val = i as i64; *last.borrow_mut() = Some(new_val); };
    }
    println!("{}", format!("{}", accept(Rc::new(RefCell::new(Some({ let __arg_holder = last.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))));
}