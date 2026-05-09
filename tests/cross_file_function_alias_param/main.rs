mod aa_use;
mod zz_types;
use aa_use::*;
use zz_types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{}", (*apply(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i32>>>| -> Rc<RefCell<Option<i32>>> { double(__arg0) }) as Box<dyn Fn(Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>>>))), Rc::new(RefCell::new(Some(21)))).borrow().as_ref().unwrap()));
}

pub fn double(x: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<i32>>> {

    return {
            let __tmp_x = (*x.borrow().as_ref().unwrap());
            let __tmp_y = 2;
            Rc::new(RefCell::new(Some(__tmp_x * __tmp_y)))
        };
}