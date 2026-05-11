use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct item {
    pub name: Rc<RefCell<Option<String>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut items: Rc<RefCell<Option<Vec<item>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); (2) as usize])));
    let mut ptrs: Rc<RefCell<Option<Vec<Rc<RefCell<Option<item>>>>>>> = Rc::new(RefCell::new(Some(vec![Default::default(); ((*items.borrow().as_ref().unwrap()).len()) as usize])));
    println!("{} {}", (*items.borrow().as_ref().unwrap()).len(), (*ptrs.borrow().as_ref().unwrap()).len());
}