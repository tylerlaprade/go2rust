use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct cache {
    pub child: Rc<RefCell<Option<node>>>,
}

impl cache {
    pub fn __go_value_clone(&self) -> Self {
        Self { child: self.child.clone() }
    }
}

impl std::fmt::Display for cache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.child.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct AnonymousStruct1 {
    child: Rc<RefCell<Option<node>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { child: self.child.clone() }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.child.borrow().as_ref().unwrap()))
    }
}


impl cache {
    pub fn lookup(&self) -> Rc<RefCell<Option<AnonymousStruct1>>> {
        return Rc::new(RefCell::new(Some(AnonymousStruct1 { child: { let __field = self.child.clone(); __field } })));
    }

    pub fn get(&self) -> Rc<RefCell<Option<node>>> {
        return (*self.lookup().borrow().as_ref().unwrap()).child.clone();
    }
}

fn main() {
    if false {
        let mut c = Rc::new(RefCell::new(Some(cache { child: Rc::new(RefCell::new(Some(Default::default()))) })));
        println!("{}", format!("{}", format!("&{}", (*(*c.borrow().as_ref().unwrap()).get().borrow().as_ref().unwrap()))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}