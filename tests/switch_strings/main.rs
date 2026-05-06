use std::cell::{RefCell};
use std::rc::{Rc};

pub fn classify(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    { let _switch_val = (*value.borrow().as_ref().unwrap()).to_lowercase();
    if _switch_val == ("go".to_string()) || _switch_val == ("rust".to_string()) {
            return Rc::new(RefCell::new(Some("systems".to_string())));
        } else if _switch_val == ("python".to_string()) {
            return Rc::new(RefCell::new(Some("scripting".to_string())));
        } else {
            return Rc::new(RefCell::new(Some("other".to_string())));
        }
    }
}

fn main() {
    println!("{}", (*classify(Rc::new(RefCell::new(Some("Go".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*classify(Rc::new(RefCell::new(Some("python".to_string())))).borrow().as_ref().unwrap()));
    println!("{}", (*classify(Rc::new(RefCell::new(Some("zig".to_string())))).borrow().as_ref().unwrap()));
}