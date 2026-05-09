use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct token_Pos(pub i32);

impl PartialEq<i32> for token_Pos {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl PartialEq<token_Pos> for i32 {
    fn eq(&self, other: &token_Pos) -> bool {
        *self == other.0
    }
}

impl std::ops::BitAnd for token_Pos {
    type Output = token_Pos;
    fn bitand(self, other: Self) -> token_Pos {
        token_Pos(self.0 & other.0)
    }
}

impl std::ops::BitOr for token_Pos {
    type Output = token_Pos;
    fn bitor(self, other: Self) -> token_Pos {
        token_Pos(self.0 | other.0)
    }
}

impl std::fmt::Display for token_Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<token_Pos>")
    }
}


impl token_Pos {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


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


pub fn kind() -> Arc<Mutex<Option<types_BasicKind>>> {

    return Arc::new(Mutex::new(Some(types::INT.clone())));
}

pub fn dir() -> Arc<Mutex<Option<types_ChanDir>>> {

    return Arc::new(Mutex::new(Some(types::SEND_RECV.clone())));
}

pub fn zero_kind() -> Arc<Mutex<Option<types_BasicKind>>> {

    return Arc::new(Mutex::new(Some(types_BasicKind(0.0 as i32))));
}

pub fn pos_from_int(n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<token_Pos>>> {

    return Arc::new(Mutex::new(Some(token_Pos((*n.lock().unwrap().as_ref().unwrap()) as i32))));
}

pub fn int_value() -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some(1)));
}

pub fn kind_name() -> Arc<Mutex<Option<String>>> {

    { let _switch_val = { let __v = kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (types::INT.clone()) {
            return Arc::new(Mutex::new(Some("int".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("other".to_string())));
        }
    }
}

pub fn int_name() -> Arc<Mutex<Option<String>>> {

    { let _switch_val = { let __v = int_value(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (1) {
            return Arc::new(Mutex::new(Some("one".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("other".to_string())));
        }
    }
}

fn main() {
    if false {
        println!("{} {} {} {} {} {}", (*Arc::new(Mutex::new(Some((*kind().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some((*dir().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some((*zero_kind().lock().unwrap().as_ref().unwrap()).0 as u32))).lock().unwrap().as_ref().unwrap()), (*pos_from_int(Arc::new(Mutex::new(Some(1)))).lock().unwrap().as_ref().unwrap()), (*kind_name().lock().unwrap().as_ref().unwrap()), (*int_name().lock().unwrap().as_ref().unwrap()));
    }
    println!("{}", "ok".to_string());
}