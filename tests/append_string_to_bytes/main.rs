use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct label {
    pub name: Rc<RefCell<Option<String>>>,
}

impl std::fmt::Display for label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.borrow().as_ref().unwrap()))
    }
}


impl label {
    pub fn name(&self) -> Rc<RefCell<Option<String>>> {
        return self.name.clone();
    }
}

pub fn append_string(dst: Rc<RefCell<Option<Vec<u8>>>>, s: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<Vec<u8>>>> {

    return {(*dst.borrow_mut()).get_or_insert_with(Vec::new).extend((*s.borrow().as_ref().unwrap()).clone().as_bytes().iter().cloned()); dst.clone()};
}

pub fn append_method(dst: Rc<RefCell<Option<Vec<u8>>>>, l: Rc<RefCell<Option<label>>>) -> Rc<RefCell<Option<Vec<u8>>>> {

    return {(*dst.borrow_mut()).get_or_insert_with(Vec::new).extend((*(*l.borrow().as_ref().unwrap()).name().borrow().as_ref().unwrap()).clone().as_bytes().iter().cloned()); dst.clone()};
}

fn main() {
    println!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*append_string(Rc::new(RefCell::new(Some(Vec::<u8>::new()))), Rc::new(RefCell::new(Some("go".to_string())))).borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
    println!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8((*append_method(Rc::new(RefCell::new(Some(Vec::<u8>::new()))), Rc::new(RefCell::new(Some(label { name: Rc::new(RefCell::new(Some("rust".to_string()))), ..Default::default() })))).borrow().as_ref().unwrap()).clone()).unwrap()))).borrow().as_ref().unwrap()));
}