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


pub fn read(n: Rc<RefCell<Option<node>>>) -> Rc<RefCell<Option<i32>>> {

    if (*n.borrow()).is_none() {
        return Rc::new(RefCell::new(Some(-1)));
    }
    return Rc::new(RefCell::new(Some({ let __selector_holder = (*n.borrow().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

fn main() {
    let mut nodes = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(1))), ..Default::default() }))), Rc::new(RefCell::new(Some(node { value: Rc::new(RefCell::new(Some(3))), ..Default::default() })))])));
    let mut sum = Rc::new(RefCell::new(Some(0)));
    { let __range_holder = nodes.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        if (*n.borrow()).is_some() {
        { let __rhs = (*(*n.borrow().as_ref().unwrap()).value.borrow().as_ref().unwrap()); let mut guard = sum.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        println!("{}", format!("{}", (*read((*n).clone()).borrow().as_ref().unwrap())));
    }
    } }
    println!("{}", format!("{}", { let __v = (*sum.borrow().as_ref().unwrap()).clone(); __v }));
}