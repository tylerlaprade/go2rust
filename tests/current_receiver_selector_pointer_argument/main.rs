use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct pkg {
    pub name: Rc<RefCell<Option<String>>>,
}

impl pkg {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for pkg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct reader {
    pub current: Rc<RefCell<Option<pkg>>>,
}

impl reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { current: self.current.clone() }
    }
}

impl std::fmt::Display for reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.current.borrow().as_ref().unwrap()))
    }
}


impl reader {
    pub fn run(&self) {
        r#use(self.current.clone());
    }
}

pub fn r#use(p: Rc<RefCell<Option<pkg>>>) {
    println!("{}", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
}

fn main() {
    let mut p = Rc::new(RefCell::new(Some(pkg { name: Rc::new(RefCell::new(Some("alpha".to_string()))), ..Default::default() })));
    let mut r = Rc::new(RefCell::new(Some(reader { current: p.clone(), ..Default::default() })));
    (*r.borrow_mut().as_mut().unwrap()).run();
}