use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    let mut a = github_com_shopspring_decimal::new_from_int(Rc::new(RefCell::new(Some(10))));
    let mut b = github_com_shopspring_decimal::new_from_int(Rc::new(RefCell::new(Some(3))));
    let mut result = (*a.borrow().as_ref().unwrap()).div(Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap())))));

    print!("10 / 3 = {}\n", /* ERROR: Type information not available for print argument */ (*(*result.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap()));

    let mut c = github_com_shopspring_decimal::new_from_float(Rc::new(RefCell::new(Some(3.14159))));
    let mut d = github_com_shopspring_decimal::new_from_float(Rc::new(RefCell::new(Some(2.0))));
    let mut product = (*c.borrow().as_ref().unwrap()).mul(Rc::new(RefCell::new(Some((*d.borrow().as_ref().unwrap())))));

    print!("3.14159 * 2 = {}\n", /* ERROR: Type information not available for print argument */ (*(*product.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap()));
}