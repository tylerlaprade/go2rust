use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub trait Key: std::fmt::Display {
    fn name(&self) -> Rc<RefCell<Option<String>>>;
}

#[derive(Debug, Clone, Default)]
pub struct namedKey {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for namedKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub trait Finder: std::fmt::Display {
    fn find(&self, key: &dyn Key) -> Rc<RefCell<Option<String>>>;
}

#[derive(Debug, Clone, Default)]
pub struct finder {
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
}

fn main() {
    let mut key = Rc::new(RefCell::new(Some(namedKey { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut finder = Rc::new(RefCell::new(Some(finder {  })));
    let mut found = (*finder.borrow().as_ref().unwrap()).find(key.borrow().as_ref().unwrap());
    println!("{}", { let __v = (*found.borrow().as_ref().unwrap()).clone(); __v });
}