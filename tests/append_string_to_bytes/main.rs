use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct label {
    pub name: Rc<RefCell<Option<String>>>,
}

impl label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for label {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl label {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

pub fn append_string(dst: Rc<RefCell<Option<Vec<u8>>>>, s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    { let __append_target = dst.clone(); let __append_values: Vec<_> = (*s.borrow().as_ref().unwrap()).clone().as_bytes().iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }
}

pub fn append_method(dst: Rc<RefCell<Option<Vec<u8>>>>, l: Rc<RefCell<Option<label>>>) -> Rc<RefCell<Option<Vec<u8>>>> {
    { let __append_target = dst.clone(); let __append_values: Vec<_> = (*(*l.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap()).clone().as_bytes().iter().cloned().collect(); (*__append_target.borrow_mut()).get_or_insert_with(Vec::new).extend(__append_values); __append_target.clone() }
}

fn main() {
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*append_string(Rc::new(RefCell::new(Some(Vec::<u8>::new()))), Rc::new(RefCell::new(Some("go".to_string())))).borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*append_method(Rc::new(RefCell::new(Some(Vec::<u8>::new()))), Rc::new(RefCell::new(Some(label { name: Rc::new(RefCell::new(Some("rust".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap())));
}