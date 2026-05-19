use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct recorder {
}

impl recorder {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for recorder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl recorder {
    pub fn r#use(&self, record: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> ()>>>>) {
        { let __f_ptr: *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> ()> = { let mut __f_guard = record.borrow_mut(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> ()> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Rc::new(RefCell::new(Some("method".to_string())))) };
    }
}

pub fn relay(record: Rc<RefCell<Option<Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> ()>>>>) {
    let mut r: Rc<RefCell<Option<recorder>>> = Rc::new(RefCell::new(Some(Default::default())));
    (*r.borrow().as_ref().unwrap()).r#use(record.clone());
}

fn main() {
    let mut out = Rc::new(RefCell::new(Some("".to_string())));
    let out_closure_clone = out.clone(); relay(Rc::new(RefCell::new(Some(Box::new(move |s: Rc<RefCell<Option<String>>>| {
        { let new_val = s.borrow().as_ref().unwrap().clone(); *out_closure_clone.borrow_mut() = Some(new_val); };
    }) as Box<dyn FnMut(Rc<RefCell<Option<String>>>) -> ()>))));
    println!("{}", format!("{}", { let __v = (*out.borrow().as_ref().unwrap()).clone(); __v }));
}