use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    { let _switch_val = { let __v = Rc::new(RefCell::new(Some({ let __s = (*value.borrow().as_ref().unwrap()).clone(); __s.to_lowercase() }))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned };
    if _switch_val == ("go".to_string()) || _switch_val == ("rust".to_string()) {
            Rc::new(RefCell::new(Some("systems".to_string())))
        } else if _switch_val == ("python".to_string()) {
            Rc::new(RefCell::new(Some("scripting".to_string())))
        } else {
            Rc::new(RefCell::new(Some("other".to_string())))
        }
    }
}

fn main() {
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some("Go".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some("python".to_string())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*classify(Rc::new(RefCell::new(Some("zig".to_string())))).borrow().as_ref().unwrap())));
}