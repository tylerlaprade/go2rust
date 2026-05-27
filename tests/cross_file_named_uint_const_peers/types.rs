use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const V0: u32 = 0;
pub const V1: u32 = 1;
pub const V2: u32 = 2;
pub(crate) const NUM_VERSIONS: i32 = 3;


pub(crate) const FLAG_SYNC_MARKERS: i32 = 1 << 0;


#[derive(Debug, Clone, Default)]
pub struct Version(pub Rc<RefCell<Option<u32>>>);

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<u32> for Version {
    fn eq(&self, other: &u32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
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
    type Output = u32;
    fn add(self, other: u32) -> u32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Version> for u32 {
    type Output = u32;
    fn add(self, other: Version) -> u32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Version {
    type Output = Version;
    fn sub(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for Version {
    type Output = u32;
    fn sub(self, other: u32) -> u32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Version> for u32 {
    type Output = u32;
    fn sub(self, other: Version) -> u32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Version {
    type Output = Version;
    fn bitand(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for Version {
    type Output = u32;
    fn bitand(self, other: u32) -> u32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Version> for u32 {
    type Output = u32;
    fn bitand(self, other: Version) -> u32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Version {
    type Output = Version;
    fn bitor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for Version {
    type Output = u32;
    fn bitor(self, other: u32) -> u32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Version> for u32 {
    type Output = u32;
    fn bitor(self, other: Version) -> u32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Version {
    type Output = Version;
    fn bitxor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for Version {
    type Output = u32;
    fn bitxor(self, other: u32) -> u32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Version> for u32 {
    type Output = u32;
    fn bitxor(self, other: Version) -> u32 {
        self ^ *other.0.borrow().as_ref().unwrap()
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
        Self { version: { let __guard = self.version.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for Header {
    fn default() -> Self {
        Self { version: Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.version.borrow().as_ref().unwrap()))
    }
}


impl Version {
    pub fn has(&self, f: Rc<RefCell<Option<Version>>>) -> bool {
        Version(Rc::new(RefCell::new(Some(V0 as u32)))) <= (*self.0.borrow().as_ref().unwrap()) && ((*self.0.borrow().as_ref().unwrap()) < Version(Rc::new(RefCell::new(Some(V2 as u32)))) || (*f.borrow().as_ref().unwrap()) == Version(Rc::new(RefCell::new(Some(V0 as u32)))))
    }
}