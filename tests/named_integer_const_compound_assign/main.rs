use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const NEED_NAME: i32 = 1 << 0;
pub const NEED_FILES: i32 = 1 << 1;
pub const NEED_IMPORTS: i32 = 1 << 2;


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


fn main() {
    let mut mode: Rc<RefCell<Option<LoadMode>>> = Rc::new(RefCell::new(Some(LoadMode(Rc::new(RefCell::new(Some(0)))))));
    { let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | LoadMode(Rc::new(RefCell::new(Some(NEED_IMPORTS as i32))))); };
    { let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | LoadMode(Rc::new(RefCell::new(Some(8 as i32))))); };
    { let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | LoadMode(Rc::new(RefCell::new(Some(NEED_FILES | NEED_NAME as i32))))); };
    println!("{}", (*Rc::new(RefCell::new(Some((*(*mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap()));
}