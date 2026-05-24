use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box_reader(&self) -> Box<dyn Reader>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool;
    fn read(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn Reader> {
    fn clone(&self) -> Self {
        self.__go_clone_box_reader()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct counter {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for counter {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


pub trait Valuer: std::fmt::Display + Any {
    fn __go_clone_box_valuer(&self) -> Box<dyn Valuer>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_valuer(&self, other: &dyn Valuer) -> bool;
    fn value(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn Valuer> {
    fn clone(&self) -> Self {
        self.__go_clone_box_valuer()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct number {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl number {
    pub fn __go_value_clone(&self) -> Self {
        Self { n: { let __guard = self.n.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for number {
    fn default() -> Self {
        Self { n: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl counter {
    pub fn read(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
}

impl Reader for counter {
    fn read(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader> {
        Box::new(self.clone()) as Box<dyn Reader>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &dyn Reader) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<counter>() {
            self == __other
        } else {
            false
        }
    }
}

impl number {
    pub fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
}

impl Valuer for number {
    fn value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
    fn __go_clone_box_valuer(&self) -> Box<dyn Valuer> {
        Box::new(self.clone()) as Box<dyn Valuer>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_valuer(&self, other: &dyn Valuer) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<number>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn new_reader() -> Rc<RefCell<Option<Box<dyn Reader>>>> {

    return Rc::new(RefCell::new(Some(Box::new(counter { n: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() }) as Box<dyn Reader>)));
}

pub fn new_valuer() -> Rc<RefCell<Option<Box<dyn Valuer>>>> {

    return Rc::new(RefCell::new(Some(Box::new(number { n: Rc::new(RefCell::new(Some(11 as i32))), ..Default::default() }) as Box<dyn Valuer>)));
}

fn main() {
    let mut reader = new_reader();
    let mut valuer = new_valuer();
    println!("{} {}", format!("{}", (*(*reader.borrow().as_ref().unwrap()).read().borrow().as_ref().unwrap())), format!("{}", (*(*valuer.borrow().as_ref().unwrap()).value().borrow().as_ref().unwrap())));
}