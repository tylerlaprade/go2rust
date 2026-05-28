use std::cell::{RefCell};
use std::rc::{Rc};

pub(crate) const BELL: i32 = ('\u{7}' as i32);


pub(crate) const MAX_RUNE: i32 = ('\u{10ffff}' as i32);


fn main() {
    let mut v = Rc::new(RefCell::new(Some(('\u{b}' as i32))));
    let mut b = Rc::new(RefCell::new(Some(('\u{8}' as i32))));
    let mut f = Rc::new(RefCell::new(Some(('\u{c}' as i32) as u8)));
    println!("{} {} {} {} {}", format!("{}", (*Rc::new(RefCell::new(Some((*v.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(BELL as i32))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(MAX_RUNE as i32))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())), format!("{}", { let __v = (*f.borrow().as_ref().unwrap()).clone(); __v }));
}