use crate::methods::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Counter holds a numeric value
#[derive(Debug, Clone)]
pub struct Counter {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Counter {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


/// Point represents a 2D point
#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<f64>>>,
    pub y: Rc<RefCell<Option<f64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, y: { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Point {
    fn default() -> Self {
        Self { x: Rc::new(RefCell::new(Some(0.0))), y: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}
