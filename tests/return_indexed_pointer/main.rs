use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct item {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl item {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for item {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


pub fn first(items: Rc<RefCell<Option<Vec<Rc<RefCell<Option<item>>>>>>>) -> Rc<RefCell<Option<item>>> {

    return (*items.borrow().as_ref().unwrap())[(0) as usize].clone();
}

fn main() {
    let mut ptr = Rc::new(RefCell::new(Some(item { value: Rc::new(RefCell::new(Some(4))), ..Default::default() })));
    let mut items = Rc::new(RefCell::new(Some(vec![ptr.clone()])));
    let mut got = first(items.clone());

    { let new_val = 9; *(*ptr.borrow().as_ref().unwrap()).value.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*(*got.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
}