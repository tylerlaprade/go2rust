use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

trait Shape: std::fmt::Display {
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
    fn perimeter(&self) -> Rc<RefCell<Option<f64>>>;
}

#[derive(Debug, Clone, Default)]
struct Rectangle {
    width: Rc<RefCell<Option<f64>>>,
    height: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct Circle {
    radius: Rc<RefCell<Option<f64>>>,
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.radius.borrow().as_ref().unwrap()))
    }
}


impl Rectangle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }

    pub fn perimeter(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * ((*self.width.clone().borrow().as_ref().unwrap()) + (*self.height.clone().borrow().as_ref().unwrap())))));
    }
}

impl Shape for Rectangle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.clone().borrow().as_ref().unwrap()) * (*self.height.clone().borrow().as_ref().unwrap()))));
    }
    fn perimeter(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * ((*self.width.clone().borrow().as_ref().unwrap()) + (*self.height.clone().borrow().as_ref().unwrap())))));
    }
}

impl Circle {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()) * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }

    pub fn perimeter(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * 3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }
}

impl Shape for Circle {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()) * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }
    fn perimeter(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * 3.14159 * (*self.radius.clone().borrow().as_ref().unwrap()))));
    }
}

pub fn print_shape_info(s: Rc<RefCell<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*(*s.borrow_mut().as_mut().unwrap()).area().borrow().as_ref().unwrap()), (*(*s.borrow_mut().as_mut().unwrap()).perimeter().borrow().as_ref().unwrap()));
}

fn main() {
    let mut rect = Rc::new(RefCell::new(Some(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(5.0))) })));
    let mut circle = Rc::new(RefCell::new(Some(Circle { radius: Rc::new(RefCell::new(Some(3.0))) })));

    println!("{}", "Rectangle:".to_string());
    print_shape_info(Rc::new(RefCell::new(Some(Box::new((*rect.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))));

    println!("{}", "Circle:".to_string());
    print_shape_info(Rc::new(RefCell::new(Some(Box::new((*circle.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>))));

        // Interface slice
    let mut shapes = Rc::new(RefCell::new(Some(vec![Box::new((*rect.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>, Box::new((*circle.borrow().as_ref().unwrap()).clone()) as Box<dyn Shape>])));
    println!("{}", "All shapes:".to_string());
    for (i, shape) in (*shapes.borrow_mut().as_mut().unwrap()).iter().enumerate() {
        print!("Shape {}: ", i + 1);
        print_shape_info(Rc::new(RefCell::new(Some(shape.clone()))));
    }
}