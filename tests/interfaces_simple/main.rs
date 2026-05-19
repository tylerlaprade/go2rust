use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait geometry: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn geometry>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn geometry) -> bool;
    fn area(&self) -> Rc<RefCell<Option<f64>>>;
    fn perim(&self) -> Rc<RefCell<Option<f64>>>;
}

impl Clone for Box<dyn geometry> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct rect {
    pub width: Rc<RefCell<Option<f64>>>,
    pub height: Rc<RefCell<Option<f64>>>,
}

impl rect {
    pub fn __go_value_clone(&self) -> Self {
        Self { width: { let __guard = self.width.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, height: { let __guard = self.height.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for rect {
    fn default() -> Self {
        Self { width: Rc::new(RefCell::new(Some(0.0))), height: Rc::new(RefCell::new(Some(0.0))) }
    }
}

impl std::fmt::Display for rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.width.borrow().as_ref().unwrap()), (*self.height.borrow().as_ref().unwrap()))
    }
}


impl rect {
    pub fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap()))));
    }

    pub fn perim(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * (*self.width.borrow().as_ref().unwrap()) + 2.0 * (*self.height.borrow().as_ref().unwrap()))));
    }
}

impl geometry for rect {
    fn area(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some((*self.width.borrow().as_ref().unwrap()) * (*self.height.borrow().as_ref().unwrap()))));
    }
    fn perim(&self) -> Rc<RefCell<Option<f64>>> {
        return Rc::new(RefCell::new(Some(2.0 * (*self.width.borrow().as_ref().unwrap()) + 2.0 * (*self.height.borrow().as_ref().unwrap()))));
    }
    fn __go_clone_box(&self) -> Box<dyn geometry> {
        Box::new(self.clone()) as Box<dyn geometry>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn geometry) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rect>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn measure(g: &dyn geometry) {
    println!("{}", format!("{}", g));
    println!("{}", format!("{}", (*g.area().borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*g.perim().borrow().as_ref().unwrap())));
}

fn main() {
    let mut r = Rc::new(RefCell::new(Some(rect { width: Rc::new(RefCell::new(Some(3.0))), height: Rc::new(RefCell::new(Some(4.0))), ..Default::default() })));
    measure(r.borrow().as_ref().unwrap());
}