use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone, Default)]
pub struct parseValue {
    pub err: Rc<RefCell<Option<Box<dyn StdError>>>>,
}

impl parseValue {
    pub fn __go_value_clone(&self) -> Self {
        Self { err: self.err.clone() }
    }
}

impl std::fmt::Display for parseValue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.err.borrow().as_ref().unwrap()))
    }
}


pub fn parse(v: Rc<RefCell<Option<parseValue>>>) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
    (*v.borrow().as_ref().unwrap()).err.clone()
}

pub fn parse_pair(v: Rc<RefCell<Option<parseValue>>>) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
    (7, { let __return_value_1 = (*v.borrow().as_ref().unwrap()).err.clone(); __return_value_1 })
}

fn main() {
    let mut v = Rc::new(RefCell::new(Some(parseValue { err: Rc::new(RefCell::new(Some(Box::<dyn std::error::Error>::from("bad".to_string())))), ..Default::default() })));
    let mut err = parse(v.clone());
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));

    let (mut n, __tmp_1) = parse_pair(v.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *err.borrow_mut() = __moved_tmp_1;;
    println!("{}", format!("{}", n));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", err.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
}