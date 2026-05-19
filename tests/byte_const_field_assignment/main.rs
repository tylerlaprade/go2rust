use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub color: Rc<RefCell<Option<u8>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { color: { let __guard = self.color.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { color: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.color.borrow().as_ref().unwrap()))
    }
}


fn main() {
    const white: i32 = 0;
const grey: i32 = 1;
const black: i32 = 2;

    let mut n = Rc::new(RefCell::new(Some(node { color: Rc::new(RefCell::new(Some(white as u8))), ..Default::default() })));
    if (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap()) == white as u8 {
        { let new_val = grey as u8; *(*n.borrow().as_ref().unwrap()).color.borrow_mut() = Some(new_val); };
    }
    println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap()) == grey as u8));
    { let new_val = black as u8; *(*n.borrow().as_ref().unwrap()).color.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap())));
}