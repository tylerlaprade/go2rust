use go2rust_stdlib_stubs::*;

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const NEED_NAME: i32 = 1 << 0;
pub const NEED_FILES: i32 = 1 << 1;
pub const NEED_TYPES: i32 = 1 << 2;


#[derive(Debug, Clone, Default)]
pub struct LoadMode(pub Rc<RefCell<Option<i32>>>);

impl Display for LoadMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for LoadMode {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for LoadMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for LoadMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for LoadMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<LoadMode> for i32 {
    fn eq(&self, other: &LoadMode) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<LoadMode> for i32 {
    fn partial_cmp(&self, other: &LoadMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for LoadMode {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for LoadMode {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<LoadMode> for i32 {
    type Output = i32;
    fn add(self, other: LoadMode) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for LoadMode {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for LoadMode {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<LoadMode> for i32 {
    type Output = i32;
    fn sub(self, other: LoadMode) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for LoadMode {
    type Output = LoadMode;
    fn bitand(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for LoadMode {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<LoadMode> for i32 {
    type Output = i32;
    fn bitand(self, other: LoadMode) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for LoadMode {
    type Output = LoadMode;
    fn bitor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for LoadMode {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<LoadMode> for i32 {
    type Output = i32;
    fn bitor(self, other: LoadMode) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for LoadMode {
    type Output = LoadMode;
    fn bitxor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for LoadMode {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<LoadMode> for i32 {
    type Output = i32;
    fn bitxor(self, other: LoadMode) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for LoadMode {}

impl Ord for LoadMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Config {
    pub mode: Rc<RefCell<Option<LoadMode>>>,
}

impl Config {
    pub fn __go_value_clone(&self) -> Self {
        Self { mode: { let __guard = self.mode.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Config {
    fn default() -> Self {
        Self { mode: Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.mode.borrow().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Config {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub fn enabled(cfg: Rc<RefCell<Option<Config>>>, bit: Rc<RefCell<Option<LoadMode>>>) -> Rc<RefCell<Option<bool>>> {

    return Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(((*(*(*cfg.borrow().as_ref().unwrap()).mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) & (*(*bit.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap())))))) != LoadMode(Rc::new(RefCell::new(Some(0 as i32)))))));
}