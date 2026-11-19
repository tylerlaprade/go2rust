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
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl Rectangle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            width: __go_clone_0_0,
            height: __go_clone_0_1,
        }
    }
}


impl Default for Rectangle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        let __go_default_0_1 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            width: __go_default_0_0,
            height: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.width.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.height.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pub radius: Rc<RefCell<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.radius.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            radius: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0.0)));
        Self {
            radius: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.radius.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Rectangle {
    pub fn area(&self) -> f64 {
        return (*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap());
    }

    pub fn perimeter(&self) -> f64 {
        2.0 * ((*self.width.borrow().as_ref().unwrap()) + (*self.height.borrow().as_ref().unwrap()))
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        Rectangle::area(self)
    }
    fn perimeter(&self) -> f64 {
        Rectangle::perimeter(self)
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

#[derive(Clone)]
pub struct RectanglePtr(pub Rc<RefCell<Option<Rectangle>>>);

impl std::fmt::Display for RectanglePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectanglePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rectangle::area(__recv)
    }
    fn perimeter(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Rectangle::perimeter(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectanglePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        3.14159 * (*self.radius.borrow().as_ref().unwrap()) * (*self.radius.borrow().as_ref().unwrap())
    }

    pub fn perimeter(&self) -> f64 {
        6.28318 * (*self.radius.borrow().as_ref().unwrap())
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        Circle::area(self)
    }
    fn perimeter(&self) -> f64 {
        Circle::perimeter(self)
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

#[derive(Clone)]
pub struct CirclePtr(pub Rc<RefCell<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.borrow();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for CirclePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::area(__recv)
    }
    fn perimeter(&self) -> f64 {
        let __recv_guard = self.0.borrow();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::perimeter(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape> {
        Box::new(self.clone()) as Box<dyn Shape>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &dyn Shape) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Rc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn print_shape_info(s: Rc<RefCell<Option<Box<dyn Shape>>>>) {
    print!("Area: {:.2}, Perimeter: {:.2}\n", (*s.borrow().as_ref().unwrap()).area(), (*s.borrow().as_ref().unwrap()).perimeter());
}

fn main() {
    let mut rect = Rc::new(RefCell::new(Some(Rectangle { width: Rc::new(RefCell::new(Some(10.0))), height: Rc::new(RefCell::new(Some(5.0))), ..Default::default() })));
    let mut circle = Rc::new(RefCell::new(Some(Circle { radius: Rc::new(RefCell::new(Some(3.0))), ..Default::default() })));

    println!("{}", format!("{}", "Rectangle:".to_string()));
    print_shape_info(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = rect.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>))));

    println!("{}", format!("{}", "Circle:".to_string()));
    print_shape_info(Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = circle.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>))));

        // Interface slice
    let mut shapes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = rect.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>))), Rc::new(RefCell::new(Some(Box::new({ let __arg_holder = circle.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Shape>)))])));
    println!("{}", format!("{}", "All shapes:".to_string()));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, shape) in __range_values.iter().enumerate() {
        print!("Shape {}: ", i as i32 + 1);
        print_shape_info(shape.clone());
    } }
}