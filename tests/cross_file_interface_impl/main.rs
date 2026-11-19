mod circle;
mod shape;
use circle::*;
use shape::*;

use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

pub trait measure: std::fmt::Display + Any {
    fn __go_clone_box_measure(&self) -> Box<dyn measure>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_measure(&self, other: &dyn measure) -> bool;
    fn area(&self) -> i32;
}

impl Clone for Box<dyn measure> {
    fn clone(&self) -> Self {
        measure::__go_clone_box_measure(self.as_ref())
    }
}

fn main() {
    let mut shapes = Rc::new(RefCell::new(Some(vec![new_circle(Rc::new(RefCell::new(Some(2)))).clone(), new_circle(Rc::new(RefCell::new(Some(3)))).clone()])));
    println!("{}", format!("{}", total_area(shapes.clone())));
    
    let mut m = Rc::new(RefCell::new(Some(Box::new(Circle { r: Rc::new(RefCell::new(Some(4))), ..Default::default() }) as Box<dyn measure>)));
    println!("{}", format!("{}", (*m.borrow().as_ref().unwrap()).area()));
}