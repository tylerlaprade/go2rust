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
        return Rc::new(RefCell::new(Some(self.clone())));
    }

    pub fn r#use(&self, other: Rc<RefCell<Option<Box_>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = (*other.borrow().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }
}

fn main() {
    let mut r#box = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(7))), ..Default::default() })));
    let mut holder = Rc::new(RefCell::new(Some(Box_ { value: Rc::new(RefCell::new(Some(0))) })));
    println!("{}", format!("{}", (*(*holder.borrow_mut().as_mut().unwrap()).r#use((*r#box.borrow_mut().as_mut().unwrap()).inner()).borrow().as_ref().unwrap())));
}