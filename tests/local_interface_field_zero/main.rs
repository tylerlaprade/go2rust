use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Reader>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn Reader) -> bool;
    fn read(&self) -> Rc<RefCell<Option<i32>>>;
}

impl Clone for Box<dyn Reader> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Clone)]
pub struct holder {
    pub reader: Rc<RefCell<Option<Box<dyn Reader>>>>,
    pub count: Rc<RefCell<Option<i32>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { reader: self.reader.clone(), count: { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for holder {
    fn default() -> Self {
        Self { reader: Rc::new(RefCell::new(None)), count: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.reader.borrow().as_ref().unwrap()), (*self.count.borrow().as_ref().unwrap()))
    }
}


pub fn zero_holder() -> Rc<RefCell<Option<holder>>> {

    return Rc::new(RefCell::new(Some(holder { reader: Rc::new(RefCell::new(None)), count: Rc::new(RefCell::new(Some(0))) })));
}

fn main() {
    let mut holder = zero_holder();
    println!("{}", format!("{}", (*(*holder.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
}