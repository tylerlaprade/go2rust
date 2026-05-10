use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Parts {
    pub left: Rc<RefCell<Option<String>>>,
    pub right: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for Parts {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.left.borrow().as_ref().unwrap()), (*self.right.borrow().as_ref().unwrap()))
    }
}


pub fn left_of(parts: Rc<RefCell<Option<Parts>>>) -> Rc<RefCell<Option<String>>> {

    return Rc::new(RefCell::new(Some((*(*parts.borrow().as_ref().unwrap()).left.borrow().as_ref().unwrap()).clone())));
}

pub fn echo(value: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {

    return value.clone();
}

fn main() {
    let mut parts = Rc::new(RefCell::new(Some(Parts { left: Rc::new(RefCell::new(Some("go".to_string()))), right: Rc::new(RefCell::new(Some("rust".to_string()))), ..Default::default() })));
    println!("{}", (*left_of(Rc::new(RefCell::new(Some((*parts.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*echo((*parts.borrow().as_ref().unwrap()).right.clone()).borrow().as_ref().unwrap()));
}