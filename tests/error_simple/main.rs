use std::cell::{RefCell};
use std::error::Error;
use std::rc::{Rc};

pub fn divide(a: Rc<RefCell<Option<i32>>>, b: Rc<RefCell<Option<i32>>>) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn Error + Send + Sync>>>>) {

    if (*b.borrow_mut().as_mut().unwrap()) == 0 {
        return (Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("division by zero".to_string())))));
    }
    return ({
            let __tmp_x = (*a.borrow_mut().as_mut().unwrap());
            let __tmp_y = (*b.borrow_mut().as_mut().unwrap());
            Rc::new(RefCell::new(Some(__tmp_x / __tmp_y)))
        }, Rc::new(RefCell::new(None)));
}

fn main() {
        // Success case
    let (mut result, mut err) = divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(2))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.borrow_mut().as_mut().unwrap()));
    }

        // Error case
    (result, err) = divide(Rc::new(RefCell::new(Some(10))), Rc::new(RefCell::new(Some(0))));
    if (*err.borrow()).is_some() {
        println!("{} {}", "Error:".to_string(), (*err.borrow_mut().as_mut().unwrap()));
    } else {
        println!("{} {}", "Result:".to_string(), (*result.borrow_mut().as_mut().unwrap()));
    }
}