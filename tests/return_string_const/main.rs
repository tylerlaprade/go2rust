use std::cell::{RefCell};
use std::rc::{Rc};

pub const FUTURE: &'static str = "";


pub const GREETING: &'static str = "hello";


pub fn fallback(ok: Rc<RefCell<Option<bool>>>) -> Rc<RefCell<Option<String>>> {

    if (*ok.borrow().as_ref().unwrap()) {
        return Rc::new(RefCell::new(Some("ok".to_string())));
    }
    return Rc::new(RefCell::new(Some(FUTURE.to_string())));
}

pub fn middle() -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some({ let __s = &(GREETING); __s[(1) as usize..(4) as usize].to_string() })));
}

fn main() {
    println!("{}", format!("{}", (*fallback(Rc::new(RefCell::new(Some(true)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*fallback(Rc::new(RefCell::new(Some(false)))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*middle().borrow().as_ref().unwrap())));
}