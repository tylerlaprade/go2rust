use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut a = (*decimal.borrow_mut().as_mut().unwrap()).new_from_int(Rc::new(RefCell::new(Some(10))));
    let mut b = (*decimal.borrow_mut().as_mut().unwrap()).new_from_int(Rc::new(RefCell::new(Some(3))));
    let mut result = (*a.borrow_mut().as_mut().unwrap()).div(Rc::new(RefCell::new(Some((*b.borrow_mut().as_mut().unwrap())))));
    print!("10 / 3 = {}\n", (*(*result.borrow_mut().as_mut().unwrap()).string().borrow().as_ref().unwrap()));
    let mut c = (*decimal.borrow_mut().as_mut().unwrap()).new_from_float(Rc::new(RefCell::new(Some(3.14159))));
    let mut d = (*decimal.borrow_mut().as_mut().unwrap()).new_from_float(Rc::new(RefCell::new(Some(2.0))));
    let mut product = (*c.borrow_mut().as_mut().unwrap()).mul(Rc::new(RefCell::new(Some((*d.borrow_mut().as_mut().unwrap())))));
    print!("3.14159 * 2 = {}\n", (*(*product.borrow_mut().as_mut().unwrap()).string().borrow().as_ref().unwrap()));
}