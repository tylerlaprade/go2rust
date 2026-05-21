use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct parsed {
    pub kind: Rc<RefCell<Option<String>>>,
    pub rest: Rc<RefCell<Option<String>>>,
}

impl parsed {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.borrow(); Rc::new(RefCell::new((*__guard).clone())) }, rest: { let __guard = self.rest.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for parsed {
    fn default() -> Self {
        Self { kind: Rc::new(RefCell::new(Some(String::new()))), rest: Rc::new(RefCell::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for parsed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.kind.borrow().as_ref().unwrap()), (*self.rest.borrow().as_ref().unwrap()))
    }
}


pub fn split(mut x: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<parsed>>> {

    let mut p: Rc<RefCell<Option<parsed>>> = Rc::new(RefCell::new(Some(Default::default())));
    { let __tmp_0 = Rc::new(RefCell::new(Some({ let __s = (*x.borrow().as_ref().unwrap()).clone(); __s[..(1) as usize].to_string() }))); let __tmp_1 = Rc::new(RefCell::new(Some({ let __s = (*x.borrow().as_ref().unwrap()).clone(); __s[(1) as usize..].to_string() }))); *(*p.borrow().as_ref().unwrap()).kind.borrow_mut() = __tmp_0.borrow_mut().take(); *x.borrow_mut() = __tmp_1.borrow_mut().take(); };
    { let new_val = x.borrow().as_ref().unwrap().clone(); *(*p.borrow().as_ref().unwrap()).rest.borrow_mut() = Some(new_val); };
    return Rc::new(RefCell::new(Some(p.borrow().as_ref().unwrap().clone())));
}

fn main() {
    let mut p = split(Rc::new(RefCell::new(Some("abc".to_string()))));
    println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()).clone()));
    println!("{}", format!("{}", (*(*p.borrow().as_ref().unwrap()).rest.borrow().as_ref().unwrap()).clone()));
}