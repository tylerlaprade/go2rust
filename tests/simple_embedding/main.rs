use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Inner {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Inner {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Inner {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Outer {
    pub inner: Rc<RefCell<Option<Inner>>>,
    pub name: Rc<RefCell<Option<String>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.inner.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            inner: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for Outer {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Inner::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            inner: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.inner.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Inner {
    pub fn get_value(&self) -> i32 {
        return (*self.value.borrow().as_ref().unwrap());
    }
}

impl Outer {
    pub fn get_value(&self) -> i32 {
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
    println!("{} {}", format!("{}", "Value:".to_string()), format!("{}", (*(*(*o.borrow().as_ref().unwrap()).inner.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "Name:".to_string()), format!("{}", (*(*o.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone()));

        // Method call
    println!("{} {}", format!("{}", "GetValue:".to_string()), format!("{}", (*o.borrow().as_ref().unwrap()).get_value()));
}