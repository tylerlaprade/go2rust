use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Box_ {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl Box_ {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Box_ {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Box_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


impl Box_ {
    pub fn inner(&self) -> Rc<RefCell<Option<Box_>>> {
        Rc::new(RefCell::new(Some(self.clone())))
    }

    pub fn r#use(&self, other: Rc<RefCell<Option<Box_>>>) -> i32 {
        return (*(*other.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap());
    }
}

fn main() {
    let mut r#box = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(7 as i32))), ..Default::default() })));
    let mut holder = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(0))) })));
    println!("{}", format!("{}", (*holder.borrow().as_ref().unwrap()).r#use((*r#box.borrow().as_ref().unwrap()).inner())));
}