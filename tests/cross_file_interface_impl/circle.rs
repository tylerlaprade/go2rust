use crate::shape::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Circle {
    pub r: Rc<RefCell<Option<i32>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        Self { r: { let __guard = self.r.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Circle {
    fn default() -> Self {
        Self { r: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.r.borrow().as_ref().unwrap()))
    }
}


impl Circle {
    pub fn area(&self) -> i32 {
        return (*self.r.borrow().as_ref().unwrap()) * (*self.r.borrow().as_ref().unwrap());
    }
}

impl Shape for Circle {
    fn area(&self) -> i32 {
        self.area()
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            false
        } else {
            false
        }
    }
}

pub fn new_circle(r: Rc<RefCell<Option<i32>>>) -> Rc<RefCell<Option<Box<dyn Shape>>>> {
    Rc::new(RefCell::new(Some(Box::new(Circle { r: r.clone(), ..Default::default() }) as Box<dyn Shape>)))
}