use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Source {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Source {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone, Default)]
pub struct Dest {
    pub name: Rc<RefCell<Option<String>>>,
}

impl Dest {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut src = Rc::new(RefCell::new(Some(Source { name: Rc::new(RefCell::new(Some("original".to_string()))), ..Default::default() })));
    let mut dst = Rc::new(RefCell::new(Some(Dest { name: Rc::new(RefCell::new(Some({ let __selector_holder = (*src.borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
    { let new_val = "changed".to_string(); *(*src.borrow().as_ref().unwrap()).name.borrow_mut() = Some(new_val); };

    println!("{}", (*(*dst.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
    println!("{}", (*(*src.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()).clone());
}