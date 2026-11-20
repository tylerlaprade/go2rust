use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

pub fn divide(a: Rc<RefCell<Option<i64>>>, b: Rc<RefCell<Option<i64>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
    if (*b.borrow().as_ref().unwrap()) == 0 {
        return (0, Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("division by zero".to_string()))))));
    }
    ((*a.borrow().as_ref().unwrap()) / (*b.borrow().as_ref().unwrap()), Rc::new(RefCell::new(None)))
}

fn main() {
        // Success case
    let (mut result, mut err) = divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(2))));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "Result:".to_string()), format!("{}", result));
    }

        // Error case
    { let (__tmp_0, __tmp_1) = divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(0)))); result = __tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1; };
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "Error:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    } else {
        println!("{} {}", format!("{}", "Result:".to_string()), format!("{}", result));
    }
}