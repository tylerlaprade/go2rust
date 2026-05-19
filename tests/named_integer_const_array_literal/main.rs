use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const V0: u32 = 0;
pub const V1: u32 = 1;
pub const V2: u32 = 2;


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
    type Output = u32;
    fn add(self, other: Self) -> u32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
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
    type Output = u32;
    fn sub(self, other: Self) -> u32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
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


pub(crate) struct GoGlobal<T> {
    value: std::cell::UnsafeCell<Option<T>>,
}
unsafe impl<T> Sync for GoGlobal<T> {}
impl<T> GoGlobal<T> {
    pub(crate) const fn new() -> Self {
        Self { value: std::cell::UnsafeCell::new(None) }
    }
    pub(crate) fn borrow(&'static self) -> &'static Option<T> {
        unsafe { &*self.value.get() }
    }
    pub(crate) fn borrow_mut(&'static self) -> &'static mut Option<T> {
        unsafe { &mut *self.value.get() }
    }
    pub(crate) fn clone(&'static self) -> std::rc::Rc<std::cell::RefCell<Option<T>>> where T: Clone {
        std::rc::Rc::new(std::cell::RefCell::new(self.borrow().clone()))
    }
}

pub(crate) static introduced: GoGlobal<[Version; 4]> = GoGlobal::new();


fn __go_init_globals() {
    *introduced.borrow_mut() = Some(std::array::from_fn(|_| Version(Rc::new(RefCell::new(Some(0))))));
    *introduced.borrow_mut() = Some((*Rc::new(RefCell::new(Some([Version(Rc::new(RefCell::new(Some(V1 as u32)))), Version(Rc::new(RefCell::new(Some(0 as u32)))), Version(Rc::new(RefCell::new(Some(V2 as u32)))), Version(Rc::new(RefCell::new(Some(0))))]))).borrow().as_ref().unwrap()).clone());
}


fn main() {
    __go_init_all();
    println!("{} {} {} {}", format!("{}", (*introduced.borrow().as_ref().unwrap())[(0) as usize].clone()), format!("{}", (*introduced.borrow().as_ref().unwrap())[(1) as usize].clone()), format!("{}", (*introduced.borrow().as_ref().unwrap())[(2) as usize].clone()), format!("{}", (*introduced.borrow().as_ref().unwrap())[(3) as usize].clone()));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
