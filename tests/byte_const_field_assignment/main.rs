use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub color: Rc<RefCell<Option<u8>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.color.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            color: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            color: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.color.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    const white: i64 = 0;
const grey: i64 = 1;
const black: i64 = 2;

    let mut n = Rc::new(RefCell::new(Some(node { color: Rc::new(RefCell::new(Some(white as u8))), ..Default::default() })));
    if (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap()) == white as u8 {
        { let new_val = grey as u8; *(*n.borrow().as_ref().unwrap()).color.borrow_mut() = Some(new_val); };
    }
    println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap()) == grey as u8));
    { let new_val = black as u8; *(*n.borrow().as_ref().unwrap()).color.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).color.borrow().as_ref().unwrap())));
}