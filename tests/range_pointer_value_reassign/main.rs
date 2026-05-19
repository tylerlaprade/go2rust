use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct node {
    pub value: Rc<RefCell<Option<i32>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for node {
    fn default() -> Self {
        Self { value: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut nodes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(2))), ..Default::default() })))])));
    { let __range_holder = nodes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for mut n in __range_values.iter().cloned() {
        if (*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()) == 1 {
        { let new_val = (*nodes.borrow().as_ref().unwrap())[(1) as usize].clone().clone(); n = new_val; };
    }
        println!("{}", format!("{}", (*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap())));
    } }
}