use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

pub fn regular(v: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<String>>> {

    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    { let _switch_val = (*v.borrow().as_ref().unwrap());
    if _switch_val == (1) {
            { let new_val = "one".to_string(); *result.borrow_mut() = Some(new_val); };
        } else {
            { let new_val = "other".to_string(); *result.borrow_mut() = Some(new_val); };
        }
    }
    return result.clone();
}

pub fn typed(v: Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<String>>> {

    let mut result = Rc::new(RefCell::new(Some("start".to_string())));
    // ERROR: Invalid type switch format
    return result.clone();
}

fn main() {
    println!("{}", (*regular(Rc::new(RefCell::new(Some(1)))).borrow().as_ref().unwrap()));
    println!("{}", (*regular(Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap()));
    if false {
        println!("{}", (*typed(Rc::new(RefCell::new(Some(Box::new(1) as Box<dyn Any>)))).borrow().as_ref().unwrap()));
    }
}