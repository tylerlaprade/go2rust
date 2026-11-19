use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct A {
    pub x: Rc<RefCell<Option<i32>>>,
}

impl A {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for A {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for A {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct B {
    pub a: Rc<RefCell<Option<A>>>,
    pub y: Rc<RefCell<Option<i32>>>,
}

impl B {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.a.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            a: __go_clone_0_0,
            y: __go_clone_1_0,
        }
    }
}


impl Default for B {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(A::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            a: __go_default_0_0,
            y: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.a.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct C {
    pub b: Rc<RefCell<Option<B>>>,
    pub z: Rc<RefCell<Option<i32>>>,
}

impl C {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.b.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.z.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            b: __go_clone_0_0,
            z: __go_clone_1_0,
        }
    }
}


impl Default for C {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(B::default())));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            b: __go_default_0_0,
            z: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for C {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.b.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.z.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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