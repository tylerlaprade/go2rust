use std::cell::{RefCell};
use std::rc::{Rc};

pub fn take_index(i: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*i.borrow().as_ref().unwrap());
            let __tmp_y = 1;
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

fn main() {
    let mut total = Rc::new(RefCell::new(Some(0)));
    let mut values = Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string(), "c".to_string()])));
    for i in 0..({ let __range_holder = values.clone(); let __range_guard = __range_holder.borrow(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) {
        { let __rhs = (*take_index(Rc::new(RefCell::new(Some(i as i32)))).borrow().as_ref().unwrap()); let mut guard = total.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    println!("{}", format!("{}", { let __v = (*total.borrow().as_ref().unwrap()).clone(); __v }));
}