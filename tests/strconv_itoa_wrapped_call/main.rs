use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub index: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.index.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            index: __go_clone_0_0,
        }
    }
}


impl Default for item {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            index: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.index.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl item {
    pub fn index(&self) -> i32 {
        return (*self.index.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut v = Rc::new(RefCell::new(Some(item { index: Rc::new(RefCell::new(Some(3))), ..Default::default() })));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(((*v.borrow().as_ref().unwrap()).index()).to_string()))).borrow().as_ref().unwrap())));
}