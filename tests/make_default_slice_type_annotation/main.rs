use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { name: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut items: Rc<RefCell<Option<Vec<item>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (2) as usize])));
    let mut ptrs = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(None)); ((*items.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    println!("{} {}", format!("{}", (*items.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*ptrs.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}