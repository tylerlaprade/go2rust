use std::cell::{RefCell};
use std::rc::{Rc};

pub const FUTURE: &'static str = "";


pub fn fallback(ok: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<String>>> {

    if (*ok.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some("ok".to_string())));
    }
    return Rc::new(RefCell::new(Some(FUTURE.to_string())));
}

fn main() {
    println!("{}", (*fallback(Rc::new(RefCell::new(Some(true)))).borrow().as_ref().unwrap()));
    println!("{}", (*fallback(Rc::new(RefCell::new(Some(false)))).borrow().as_ref().unwrap()));
}