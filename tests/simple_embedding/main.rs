use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Inner {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Inner {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Inner {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Outer {
    pub inner: Rc<RefCell<Option<Inner>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        Self { inner: { let __guard = self.inner.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Outer {
    fn default() -> Self {
        Self { inner: Rc::new(RefCell::new(Some(Inner::default()))), name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.inner.borrow().as_ref().unwrap()), (*self.name.borrow().as_ref().unwrap()))
    }
}


impl Inner {
    pub fn get_value(&self) -> Rc<RefCell<Option<i32>>> {
        return self.value.clone();
    }
}

impl Outer {
    pub fn get_value(&self) -> Rc<RefCell<Option<i32>>> {
        // Forward to embedded type's method
        let embedded = self.inner.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.get_value()
    }
}

fn main() {
    let mut o = Rc::new(RefCell::new(Some(Outer { inner: Rc::new(RefCell::new(Some(Inner { value: Rc::new(RefCell::new(Some(42))), ..Default::default() }))), name: Rc::new(RefCell::new(Some("test".to_string()))), ..Default::default() })));

        // Direct field access
    println!("{} {}", "Value:".to_string(), (*(*(*o.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()));
    println!("{} {}", "Name:".to_string(), (*(*o.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());

        // Method call
    println!("{} {}", "GetValue:".to_string(), (*(*o.borrow().as_ref().unwrap()).get_value().borrow().as_ref().unwrap()));
}