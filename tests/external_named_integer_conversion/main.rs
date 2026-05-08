use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicKind(pub i32);

impl std::fmt::Display for types_BasicKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_BasicKind>")
    }
}


impl types_BasicKind {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_ChanDir(pub i32);

impl std::fmt::Display for types_ChanDir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_ChanDir>")
    }
}


impl types_ChanDir {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod types {
    use super::*;
    pub const int: types_BasicKind = types_BasicKind(0);
    pub const send_recv: types_ChanDir = types_ChanDir(0);
}


pub fn kind() -> Rc<RefCell<Option<types_BasicKind>>> {

    return Rc::new(RefCell::new(Some(types::int.clone())));
}

pub fn dir() -> Rc<RefCell<Option<types_ChanDir>>> {

    return Rc::new(RefCell::new(Some(types::send_recv.clone())));
}

fn main() {
    if false {
        println!("{} {}", (*Rc::new(RefCell::new(Some((*kind().borrow().as_ref().unwrap()).0 as u32))).borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some((*dir().borrow().as_ref().unwrap()).0 as u32))).borrow().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}