use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_BasicKind(pub i32);

impl PartialEq<i32> for types_BasicKind {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_BasicKind> for i32 {
    fn eq(&self, other: &types_BasicKind) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_BasicKind {
    type Output = types_BasicKind;
    fn bitand(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_BasicKind {
    type Output = types_BasicKind;
    fn bitor(self, other: Self) -> types_BasicKind {
        types_BasicKind(self.0 | other.0)
    }
}

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

impl PartialEq<i32> for types_ChanDir {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<types_ChanDir> for i32 {
    fn eq(&self, other: &types_ChanDir) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for types_ChanDir {
    type Output = types_ChanDir;
    fn bitand(self, other: Self) -> types_ChanDir {
        types_ChanDir(self.0 & other.0)
    }
}

impl std::ops::BitOr for types_ChanDir {
    type Output = types_ChanDir;
    fn bitor(self, other: Self) -> types_ChanDir {
        types_ChanDir(self.0 | other.0)
    }
}

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
    pub const INT: types_BasicKind = types_BasicKind(0);
    pub const SEND_RECV: types_ChanDir = types_ChanDir(0);
}


pub fn kind() -> Rc<RefCell<Option<types_BasicKind>>> {

    return Rc::new(RefCell::new(Some(types::INT.clone())));
}

pub fn dir() -> Rc<RefCell<Option<types_ChanDir>>> {

    return Rc::new(RefCell::new(Some(types::SEND_RECV.clone())));
}

fn main() {
    if false {
        println!("{} {}", (*Rc::new(RefCell::new(Some((*kind().borrow().as_ref().unwrap()).0 as u32))).borrow().as_ref().unwrap()), (*Rc::new(RefCell::new(Some((*dir().borrow().as_ref().unwrap()).0 as u32))).borrow().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}