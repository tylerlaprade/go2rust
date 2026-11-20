use std::any::Any;
use std::fmt::{Display};

/// Stepper is implemented by Counter, whose methods are split across two files.
pub trait Stepper: std::fmt::Display + Any {
    fn __go_clone_box_stepper(&self) -> Box<dyn Stepper>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_stepper(&self, other: &dyn Stepper) -> bool;
    fn step(&mut self);
    fn value(&self) -> i64;
}

impl Clone for Box<dyn Stepper> {
    fn clone(&self) -> Self {
        Stepper::__go_clone_box_stepper(self.as_ref())
    }
}