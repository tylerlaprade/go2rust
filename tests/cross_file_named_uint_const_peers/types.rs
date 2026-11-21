use crate::{GoSlice, GoSliceAppendBoxed, GoSliceFromSlice, GoSliceHeader, GoSliceView, Vec};

use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
pub const V0: u32 = 0;
pub const V1: u32 = 1;
pub const V2: u32 = 2;
pub(crate) const NUM_VERSIONS: i64 = 3;


pub(crate) const FLAG_SYNC_MARKERS: i64 = 1 << 0;


#[derive(Debug)]
pub struct Version(pub Rc<RefCell<Option<u32>>>);

impl Clone for Version {
    fn clone(&self) -> Self {
        let __value = (*self.0.borrow()).clone();
        Version(Rc::new(RefCell::new(__value)))
    }
}

impl Default for Version {
    fn default() -> Self {
        Self(Rc::new(RefCell::new(Some(Default::default()))))
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for Version {
    fn eq(&self, other: &u32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for Version {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Version> for u32 {
    fn eq(&self, other: &Version) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Version> for u32 {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Version {
    type Output = Version;
    fn add(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for Version {
    type Output = Version;
    fn add(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Version> for u32 {
    type Output = Version;
    fn add(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Version {
    type Output = Version;
    fn sub(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for Version {
    type Output = Version;
    fn sub(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Version> for u32 {
    type Output = Version;
    fn sub(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Version {
    type Output = Version;
    fn mul(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for Version {
    type Output = Version;
    fn mul(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Version> for u32 {
    type Output = Version;
    fn mul(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self * *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Version {
    type Output = Version;
    fn div(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for Version {
    type Output = Version;
    fn div(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Version> for u32 {
    type Output = Version;
    fn div(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self / *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Version {
    type Output = Version;
    fn rem(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for Version {
    type Output = Version;
    fn rem(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Version> for u32 {
    type Output = Version;
    fn rem(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self % *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Version {
    type Output = Version;
    fn bitand(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for Version {
    type Output = Version;
    fn bitand(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Version> for u32 {
    type Output = Version;
    fn bitand(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Version {
    type Output = Version;
    fn bitor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for Version {
    type Output = Version;
    fn bitor(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Version> for u32 {
    type Output = Version;
    fn bitor(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Version {
    type Output = Version;
    fn bitxor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for Version {
    type Output = Version;
    fn bitxor(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Version> for u32 {
    type Output = Version;
    fn bitxor(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Version {
    type Output = Version;
    fn not(self) -> Version {
        Version(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Version {
    type Output = Version;
    fn shl(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Version {
    type Output = Version;
    fn shl(self, other: i32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Version {
    type Output = Version;
    fn shl(self, other: i8) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Version {
    type Output = Version;
    fn shl(self, other: i16) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Version {
    type Output = Version;
    fn shl(self, other: i64) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Version {
    type Output = Version;
    fn shl(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Version {
    type Output = Version;
    fn shl(self, other: u8) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Version {
    type Output = Version;
    fn shl(self, other: u16) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Version {
    type Output = Version;
    fn shl(self, other: u64) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Version {
    type Output = Version;
    fn shl(self, other: usize) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Version {
    type Output = Version;
    fn shr(self, other: Version) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Version {
    type Output = Version;
    fn shr(self, other: i32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Version {
    type Output = Version;
    fn shr(self, other: i8) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Version {
    type Output = Version;
    fn shr(self, other: i16) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Version {
    type Output = Version;
    fn shr(self, other: i64) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Version {
    type Output = Version;
    fn shr(self, other: u32) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Version {
    type Output = Version;
    fn shr(self, other: u8) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Version {
    type Output = Version;
    fn shr(self, other: u16) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Version {
    type Output = Version;
    fn shr(self, other: u64) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Version {
    type Output = Version;
    fn shr(self, other: usize) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl Eq for Version {}

impl Ord for Version {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct Header {
    pub version: Rc<RefCell<Option<Version>>>,
}

impl Header {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.version.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            version: __go_clone_0_0,
        }
    }
}


impl Default for Header {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(0)))))));
        Self {
            version: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.version.borrow().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Version {
    pub fn has(&self, f: Rc<RefCell<Option<Version>>>) -> bool {
        (Version(Rc::new(RefCell::new(Some(V0 as u32))))) <= (*self.0.borrow().as_ref().unwrap()) && ((*self.0.borrow().as_ref().unwrap()) < Version(Rc::new(RefCell::new(Some(V2 as u32)))) || (*f.borrow().as_ref().unwrap()) == Version(Rc::new(RefCell::new(Some(V0 as u32)))))
    }
}