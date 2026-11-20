mod counter_step;
mod counter_value;
mod iface;
use counter_step::*;
use counter_value::*;
use iface::*;

use std::cell::{RefCell};
use std::rc::{Rc};

pub fn run(s: Rc<RefCell<Option<Box<dyn Stepper>>>>) -> i64 {
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow_mut().as_mut().unwrap()).step();
    (*s.borrow().as_ref().unwrap()).value()
}

fn main() {
    println!("{}", format!("{}", run(Rc::new(RefCell::new(Some(Box::new(crate::counter_step::CounterPtr(Rc::new(RefCell::new(Some(Counter { n: Rc::new(RefCell::new(Some(0))) }))).clone())) as Box<dyn Stepper>))))));
}