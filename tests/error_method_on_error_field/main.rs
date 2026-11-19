use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct holder {
    pub err: Rc<RefCell<Option<Box<dyn StdError>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.err.clone();
        Self {
            err: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", match self.err.borrow().as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    let mut h = Rc::new(RefCell::new(Some(holder { err: Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("boom".to_string())))), ..Default::default() })));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", (*h.borrow().as_ref().unwrap()).err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}