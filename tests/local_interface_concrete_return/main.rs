use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Reader: std::fmt::Display {
    fn read(&self) -> Rc<RefCell<Option<i32>>>;
}

#[derive(Debug, Clone, Default)]
pub struct counter {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


pub trait Valuer: std::fmt::Display {
    fn value(&self) -> Rc<RefCell<Option<i32>>>;
}

#[derive(Debug, Clone, Default)]
pub struct number {
    pub n: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for number {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.n.borrow().as_ref().unwrap()))
    }
}


impl counter {
    pub fn read(&mut self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
    }
}

impl Reader for counter {
    fn read(&self) -> Rc<RefCell<Option<i32>>> {
        return self.n.clone();
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
}

pub fn new_reader() -> Rc<RefCell<Option<Box<dyn Reader>>>> {

    return Rc::new(RefCell::new(Some(Box::new(counter { n: Rc::new(RefCell::new(Some(7))), ..Default::default() }) as Box<dyn Reader>)));
}

pub fn new_valuer() -> Rc<RefCell<Option<Box<dyn Valuer>>>> {

    return Rc::new(RefCell::new(Some(Box::new(number { n: Rc::new(RefCell::new(Some(11))), ..Default::default() }) as Box<dyn Valuer>)));
}

fn main() {
    let mut reader = new_reader();
    let mut valuer = new_valuer();
    println!("{} {}", (*(*reader.borrow().as_ref().unwrap()).read().borrow().as_ref().unwrap()), (*(*valuer.borrow().as_ref().unwrap()).value().borrow().as_ref().unwrap()));
}