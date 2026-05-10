use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Pkg {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Pkg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


pub fn r#use(p: Rc<RefCell<Option<Pkg>>>) {
    println!("{}", (*(*p.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()));
}