mod aa_use;
mod zz_types;
use aa_use::*;
use zz_types::*;

use std::cell::{RefCell};
use std::rc::{Rc};

fn main() {
    println!("{}", format!("{}", apply(Rc::new(RefCell::new(Some(Box::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { double(__arg0) }) as Box<dyn FnMut(Rc<RefCell<Option<i64>>>) -> i64>))), Rc::new(RefCell::new(Some(21))))));
}

pub fn double(x: Rc<RefCell<Option<i64>>>) -> i64 {
    (*x.borrow().as_ref().unwrap()) * 2
}