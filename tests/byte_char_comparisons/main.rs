use std::cell::{RefCell};
use std::rc::{Rc};

pub fn is_digit(c: Rc<RefCell<Option<u8>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some(('0' as u8) <= (*c.borrow().as_ref().unwrap()) && (*c.borrow().as_ref().unwrap()) <= ('9' as u8))));
}

pub fn starts_with_v(s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*s.borrow().as_ref().unwrap()).len() > 0 && { let __s = (*s.borrow().as_ref().unwrap()).clone(); __s.as_bytes()[(0) as usize] } == ('v' as u8))));
}

fn main() {
    println!("{} {}", "digit 5:".to_string(), (*is_digit(Rc::new(RefCell::new(Some(('5' as u8))))).borrow().as_ref().unwrap()));
    println!("{} {}", "digit x:".to_string(), (*is_digit(Rc::new(RefCell::new(Some(('x' as u8))))).borrow().as_ref().unwrap()));
    println!("{} {}", "version v1:".to_string(), (*starts_with_v(Rc::new(RefCell::new(Some("v1.0.0".to_string())))).borrow().as_ref().unwrap()));
    println!("{} {}", "version x1:".to_string(), (*starts_with_v(Rc::new(RefCell::new(Some("x1.0.0".to_string())))).borrow().as_ref().unwrap()));
}