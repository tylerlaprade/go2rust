use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Key: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Key>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn Key) -> bool;
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Key> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct namedKey {
    pub name: Rc<RefCell<Option<String>>>,
}

impl namedKey {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for namedKey {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for namedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub trait Finder: std::fmt::Display + Any {
    fn __go_clone_box(&self) -> Box<dyn Finder>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq(&self, other: &dyn Finder) -> bool;
    fn find(&self, key: &dyn Key) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn Finder> {
    fn clone(&self) -> Self {
        self.__go_clone_box()
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct finder {
}

impl finder {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for finder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl namedKey {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

impl Key for namedKey {
    fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box(&self) -> Box<dyn Key> {
        Box::new(self.clone()) as Box<dyn Key>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn Key) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<namedKey>() {
            self == __other
        } else {
            false
        }
    }
}

impl finder {
    pub fn find(&self, key: &dyn Key) -> Rc<RefCell<Option<String>>> {
        return key.name();
    }
}

impl Finder for finder {
    fn find(&self, key: &dyn Key) -> Rc<RefCell<Option<String>>> {
        return key.name();
    }
    fn __go_clone_box(&self) -> Box<dyn Finder> {
        Box::new(self.clone()) as Box<dyn Finder>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &dyn Finder) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<finder>() {
            self == __other
        } else {
            false
        }
    }
}

fn main() {
    let mut key = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut finder = Rc::new(RefCell::new(Some(finder {  })));
    let mut found = (*finder.borrow().as_ref().unwrap()).find(key.borrow().as_ref().unwrap());
    println!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v });
}