use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct Inner {
    value: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct Outer {
    inner: Rc<RefCell<Option<Inner>>>,
    name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
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
        let mut guard = embedded.borrow_mut();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.get_value()
    }
}

fn main() {
    let mut o = Rc::new(RefCell::new(Some(Outer { inner: Rc::new(RefCell::new(Some(Inner { value: Rc::new(RefCell::new(Some(42))) }))), name: Rc::new(RefCell::new(Some("test".to_string()))) })));

        // Direct field access
    println!("{} {}", "Value:".to_string(), (*(*(*o.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()));
    println!("{} {}", "Name:".to_string(), (*(*o.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));

        // Method call
    println!("{} {}", "GetValue:".to_string(), (*(*o.borrow_mut().as_mut().unwrap()).get_value().borrow().as_ref().unwrap()));
}