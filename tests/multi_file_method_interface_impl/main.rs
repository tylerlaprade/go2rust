mod counter_step;
mod counter_value;
mod iface;
use counter_step::*;
use counter_value::*;
use iface::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn run(s: Rc<RefCell<Option<Box<dyn Stepper>>>>) -> i32 {
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow().as_ref().unwrap()).value()
}

fn main() {
    println!("{}", format!("{}", run(Rc::new(RefCell::new(Some(Box::new(Counter { n: Rc::new(RefCell::new(Some(0))) }) as Box<dyn Stepper>))))));
}