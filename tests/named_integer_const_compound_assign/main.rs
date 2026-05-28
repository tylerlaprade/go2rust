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
    type Output = LoadMode;
    fn add(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for LoadMode {
    type Output = LoadMode;
    fn add(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<LoadMode> for i32 {
    type Output = LoadMode;
    fn add(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for LoadMode {
    type Output = LoadMode;
    fn sub(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for LoadMode {
    type Output = LoadMode;
    fn sub(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<LoadMode> for i32 {
    type Output = LoadMode;
    fn sub(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for LoadMode {
    type Output = LoadMode;
    fn bitand(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for LoadMode {
    type Output = LoadMode;
    fn bitand(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<LoadMode> for i32 {
    type Output = LoadMode;
    fn bitand(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for LoadMode {
    type Output = LoadMode;
    fn bitor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for LoadMode {
    type Output = LoadMode;
    fn bitor(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<LoadMode> for i32 {
    type Output = LoadMode;
    fn bitor(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for LoadMode {
    type Output = LoadMode;
    fn bitxor(self, other: Self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for LoadMode {
    type Output = LoadMode;
    fn bitxor(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<LoadMode> for i32 {
    type Output = LoadMode;
    fn bitxor(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for LoadMode {
    type Output = LoadMode;
    fn not(self) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: i8) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: i16) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: i64) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: u32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: u8) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: u16) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: u64) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for LoadMode {
    type Output = LoadMode;
    fn shl(self, other: usize) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: LoadMode) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: i32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: i8) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: i16) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: i64) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: u32) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: u8) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: u16) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: u64) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for LoadMode {
    type Output = LoadMode;
    fn shr(self, other: usize) -> LoadMode {
        LoadMode(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
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
    { let __rhs = LoadMode(Rc::new(RefCell::new(Some(NEED_IMPORTS as i32)))); let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    { let __rhs = LoadMode(Rc::new(RefCell::new(Some(8 as i32)))); let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    { let __rhs = LoadMode(Rc::new(RefCell::new(Some(NEED_FILES as i32 | NEED_NAME as i32 as i32)))); let mut guard = mode.borrow_mut(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some((*(*mode.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())));
}