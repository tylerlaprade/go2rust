use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut n: Rc<RefCell<Option<u8>>> = Rc::new(RefCell::new(Some(250)));
    let mut v = Rc::new(RefCell::new(Some(((*n.borrow().as_ref().unwrap())).wrapping_neg())));
    let mut w = Rc::new(RefCell::new(Some((((*n.borrow().as_ref().unwrap()) & ! 1 as u8)).wrapping_neg() >> 1 as u8)));

    let mut small: Rc<RefCell<Option<u16>>> = Rc::new(RefCell::new(Some(2)));
    println!("{} {} {}", { let __v = (*v.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*w.borrow().as_ref().unwrap()).clone(); __v }, ((*small.borrow().as_ref().unwrap())).wrapping_neg());
}