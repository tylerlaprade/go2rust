use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Module {
    pub path: Rc<RefCell<Option<String>>>,
}

impl Module {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.path.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Source {
    pub module: Rc<RefCell<Option<Module>>>,
}

impl Source {
    pub fn __go_value_clone(&self) -> Self {
        Self { module: { let __guard = self.module.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Source {
    fn default() -> Self {
        Self { module: Rc::new(RefCell::new(Some(Module::default()))) }
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.module.borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct Dest {
    pub module: Rc<RefCell<Option<Module>>>,
}

impl Dest {
    pub fn __go_value_clone(&self) -> Self {
        Self { module: { let __guard = self.module.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Dest {
    fn default() -> Self {
        Self { module: Rc::new(RefCell::new(Some(Module::default()))) }
    }
}

impl std::fmt::Display for Dest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.module.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut src = Rc::new(RefCell::new(Some(Source { module: Rc::new(RefCell::new(Some(Module { path: Rc::new(RefCell::new(Some("old".to_string()))), ..Default::default() }))), ..Default::default() })));
    let mut dst = Rc::new(RefCell::new(Some(Dest { module: Rc::new(RefCell::new(Some({ let __selector_holder = (*src.borrow().as_ref().unwrap()).module.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
    { let new_val = "new".to_string(); *(*(*src.borrow().as_ref().unwrap()).module.borrow().as_ref().unwrap()).path.borrow_mut() = Some(new_val); };
    println!("{}", (*(*(*dst.borrow().as_ref().unwrap()).module.borrow().as_ref().unwrap()).path.borrow().as_ref().unwrap()));
    println!("{}", (*(*(*src.borrow().as_ref().unwrap()).module.borrow().as_ref().unwrap()).path.borrow().as_ref().unwrap()));
}