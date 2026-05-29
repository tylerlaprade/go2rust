use crate::counter_value::*;
use crate::iface::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Counter {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl Counter {
    pub fn step(&mut self) {
        { let __target = self.n.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

impl Stepper for Counter {
    fn step(&mut self) {
        self.step()
    }
    fn value(&self) -> i32 {
        self.value()
    }
    fn __go_clone_box_stepper(&self) -> Box<dyn Stepper> {
        Box::new(self.clone()) as Box<dyn Stepper>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stepper(&self, other: &dyn Stepper) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Counter>() {
            false
        } else {
            false
        }
    }
}