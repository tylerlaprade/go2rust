use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
struct A {
    x: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.x.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct B {
    a: Rc<RefCell<Option<A>>>,
    y: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.y.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
struct C {
    b: Rc<RefCell<Option<B>>>,
    z: Rc<RefCell<Option<i32>>>,
}

impl std::fmt::Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.z.borrow().as_ref().unwrap()))
    }
}


impl C {
    pub fn show_x(&self) {
        print!("X = {}\n", (*self.x.borrow().as_ref().unwrap()));
    }
}

impl B {
}

fn main() {
    let mut c = Rc::new(RefCell::new(Some(C { b: Rc::new(RefCell::new(Some(B { a: Rc::new(RefCell::new(Some(A { x: Rc::new(RefCell::new(Some(10))) }))), y: Rc::new(RefCell::new(Some(20))) }))), z: Rc::new(RefCell::new(Some(30))) })));

        // Direct access to nested promoted field
    print!("c.X = {}\n", (*(*(*c.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap().a.borrow().as_ref().unwrap()).x.borrow().as_ref().unwrap()));
    print!("c.Y = {}\n", (*(*(*c.borrow().as_ref().unwrap()).b.borrow().as_ref().unwrap()).y.borrow().as_ref().unwrap()));
    print!("c.Z = {}\n", (*(*c.borrow().as_ref().unwrap()).z.borrow().as_ref().unwrap()));

        // Method accessing promoted field
    (*c.borrow_mut().as_mut().unwrap()).show_x();
}