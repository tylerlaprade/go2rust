use std::cell::{RefCell};
use std::rc::{Rc};

pub fn accept(n: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some(n.borrow().as_ref().unwrap().clone())));
}

fn main() {
    let mut last = Rc::new(RefCell::new(Some(0)));
    let mut values = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])));
    for i in 0..({ let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        { let new_val = i as i32; *last.borrow_mut() = Some(new_val); };
    }
    println!("{}", format!("{}", (*accept(Rc::new(RefCell::new(Some((*last.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
}