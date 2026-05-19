use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct A {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl A {
    pub fn __go_value_clone(&self) -> Self {
        Self { x: { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for A {
    fn default() -> Self {
        Self { x: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct B {
    pub a: Rc<RefCell<Option<A>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}

impl B {
    pub fn __go_value_clone(&self) -> Self {
        Self { a: { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, y: { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for B {
    fn default() -> Self {
        Self { a: Rc::new(RefCell::new(Some(A::default()))), y: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.a.borrow().as_ref().unwrap()), (*self.y.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct C {
    pub b: Rc<RefCell<Option<B>>>,
    pub z: Rc<RefCell<Option<i32>>>,
}

impl C {
    pub fn __go_value_clone(&self) -> Self {
        Self { b: { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, z: { let __guard = self.z.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for C {
    fn default() -> Self {
        Self { b: Rc::new(RefCell::new(Some(B::default()))), z: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.b.borrow().as_ref().unwrap()), (*self.z.borrow().as_ref().unwrap()))
    }
}


impl C {
    pub fn show_x(&self) {
        print!("X = {}\n", (*self.b.borrow().as_ref().unwrap().a.borrow().as_ref().unwrap().x.borrow().as_ref().unwrap()));
    }
}

impl B {
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(C { b: Rc::new(RefCell::new(Some(B { a: Rc::new(RefCell::new(Some(A { x: Rc::new(RefCell::new(Some(10))), ..Default::default() }))), y: Rc::new(RefCell::new(Some(20))), ..Default::default() }))), z: Rc::new(RefCell::new(Some(30))), ..Default::default() })));

        // Direct access to nested promoted field
    print!("c.X = {}\n", (*(*(*c.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap().a.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()));
    print!("c.Y = {}\n", (*(*(*c.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));
    print!("c.Z = {}\n", (*(*c.borrow().as_ref().unwrap()).z.borrow().as_ref().unwrap()));

        // Method accessing promoted field
    (*c.borrow().as_ref().unwrap()).show_x();
}