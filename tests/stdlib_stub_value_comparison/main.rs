use std::cell::{RefCell};
use std::rc::{Rc};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct types_Chan;

impl std::fmt::Display for types_Chan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<types_Chan>")
    }
}


impl types_Chan {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
    pub fn dir(&self) -> Rc<RefCell<Option<types_ChanDir>>> {
        Rc::new(RefCell::new(Some::<types_ChanDir>(Default::default())))
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
    pub const SEND_RECV: types_ChanDir = types_ChanDir(0);
}


pub fn is_bidirectional(ch: Rc<RefCell<Option<types_Chan>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*(*ch.borrow_mut().as_mut().unwrap()).dir().borrow().as_ref().unwrap()).clone() == types_ChanDir(types::SEND_RECV.0 as i32))));
}

pub fn has_direction(ch: Rc<RefCell<Option<types_Chan>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some((*(*ch.borrow_mut().as_mut().unwrap()).dir().borrow().as_ref().unwrap()).clone() & types_ChanDir(types::SEND_RECV.0 as i32) != types_ChanDir(0 as i32))));
}

fn main() {
    if false {
        println!("{}", (*is_bidirectional(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap()));
        println!("{}", (*has_direction(Rc::new(RefCell::new(None))).borrow().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}