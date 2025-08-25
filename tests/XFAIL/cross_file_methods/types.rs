use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

/// Counter holds a numeric value
#[derive(Debug, Clone, Default)]
struct Counter {
    value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


/// Point represents a 2D point
#[derive(Debug, Clone, Default)]
struct Point {
    x: Rc<RefCell<Option<f64>>>,
    y: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.x.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}
