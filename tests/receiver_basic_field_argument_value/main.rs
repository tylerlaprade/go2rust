use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct Parser {
    pub offset: Rc<RefCell<Option<i32>>>,
}

impl Parser {
    pub fn __go_value_clone(&self) -> Self {
        Self { offset: { let __guard = self.offset.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Parser {
    fn default() -> Self {
        Self { offset: Rc::new(RefCell::new(Some(0))) }
    }
}

impl std::fmt::Display for Parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.offset.borrow().as_ref().unwrap()))
    }
}


impl Parser {
    pub fn add_line(&mut self, offset: Rc<RefCell<Option<i32>>>) {
        { let new_val = 99; *self.offset.borrow_mut() = Some(new_val); };
        println!("{}", format!("{}", { let __v = (*offset.borrow().as_ref().unwrap()).clone(); __v }));
    }

    pub fn parse(&mut self) {
        { let new_val = 7; *self.offset.borrow_mut() = Some(new_val); };
        { let __method_arg0 = Rc::new(RefCell::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.add_line(__method_arg0) };
    }
}

fn main() {
    let mut p: Rc<RefCell<Option<Parser>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*p.borrow_mut().as_mut().unwrap()).parse();
}