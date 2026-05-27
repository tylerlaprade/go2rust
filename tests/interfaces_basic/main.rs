use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.__go_clone_box_shape()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        Self { width: { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, height: { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        Self { width: Rc::new(RefCell::new(Some(0.0))), height: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        Self { radius: { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Circle {
    fn default() -> Self {
        Self { radius: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.radius.borrow().as_ref().unwrap()))
    }
}


impl Rectangle {
    pub fn area(&self) -> f64 {
        (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap())
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * ((*self.width.borrow().as_ref().unwrap()) + (*self.height.borrow().as_ref().unwrap()))
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap())
    }
    fn perimeter(&self) -> f64 {
        2.0 * ((*self.width.borrow().as_ref().unwrap()) + (*self.height.borrow().as_ref().unwrap()))
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rectangle>() {
            self == __other
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap())
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * 3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap())
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap())
    }
    fn perimeter(&self) -> f64 {
        2.0 * 3.14159 as f64 * (*self.radius.borrow().as_ref().unwrap())
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn print_shape_info(s: Rc<RefCell<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*s.borrow().as_ref().unwrap()).area(), (*s.borrow().as_ref().unwrap()).perimeter());
}

fn main() {
    let mut rect = Rc::new(RefCell::new(Some(Rectangle { width: Rc::new(RefCell::new(Some(10.0 as f64))), height: Rc::new(RefCell::new(Some(5.0 as f64))), ..Default::default() })));
    let mut circle = Rc::new(RefCell::new(Some(Circle { radius: Rc::new(RefCell::new(Some(3.0 as f64))), ..Default::default() })));

    println!("{}", format!("{}", "Rectangle:".to_string()));
    print_shape_info(Rc::new(RefCell::new(Some(Box::new((*rect.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))));

    println!("{}", format!("{}", "Circle:".to_string()));
    print_shape_info(Rc::new(RefCell::new(Some(Box::new((*circle.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))));

        // Interface slice
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new((*rect.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new((*circle.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>)))])));
    println!("{}", format!("{}", "All shapes:".to_string()));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: ", i as i32 + 1);
        print_shape_info(shape.clone());
    } }
}