use std::cell::{RefCell};
use std::rc::{Rc};

pub fn limit(values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap()).len() as i32)));
}

pub fn count_until_limit(values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {

    let mut count = Rc::new(RefCell::new(Some(0)));
    for i in 0..({ let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        if i as i32 >= (*limit(values.clone()).borrow().as_ref().unwrap()) {
        break
    }
        { let mut guard = count.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return count.clone();
}

fn main() {
    println!("{}", (*count_until_limit(Rc::new(RefCell::new(Some(vec![1, 2, 3])))).borrow().as_ref().unwrap()));
}