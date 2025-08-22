use std::cell::{RefCell};
use std::rc::{Rc};

/// Add adds two numbers
pub fn add(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

/// Multiply multiplies two numbers
pub fn multiply(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}