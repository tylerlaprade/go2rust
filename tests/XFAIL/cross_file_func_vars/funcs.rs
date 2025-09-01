use std::cell::{RefCell};
use std::rc::{Rc};

/// Regular function for comparison
pub fn regular_double(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}

/// Function that returns a function
pub fn make_multiplier(factor: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>>>> {

    return Rc::new(RefCell::new(Some(Box::new(move |x: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> {
        return {
            let __tmp_x = (*x.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*factor.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
    }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>)));
}

pub fn init() {
        // Assign function to variable in init
    { let new_val = Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some(format!("Dynamic: {}", (*s.borrow_mut().as_mut().unwrap())))));
    }) as Box<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>>))); *DYNAMIC_FUNC.borrow_mut() = Some(new_val); };
}