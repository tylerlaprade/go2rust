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
        panic!("types_Chan.dir bridge: generic stub method body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
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


pub fn is_bidirectional(ch: Rc<RefCell<Option<types_Chan>>>) -> bool {
    (*(*ch.borrow_mut().as_mut().unwrap()).dir().borrow().as_ref().unwrap()).clone() == types::SEND_RECV
}

pub fn has_direction(ch: Rc<RefCell<Option<types_Chan>>>) -> bool {
    (*(*ch.borrow_mut().as_mut().unwrap()).dir().borrow().as_ref().unwrap()).clone() & types::SEND_RECV != types_ChanDir(0 as i32)
}

fn main() {
    if false {
        println!("{}", format!("{}", is_bidirectional(Rc::new(RefCell::new(None)))));
        println!("{}", format!("{}", has_direction(Rc::new(RefCell::new(None)))));
    }
    println!("{}", format!("{}", "ok".to_string()));
}