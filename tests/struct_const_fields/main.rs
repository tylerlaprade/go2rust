use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const INVALID: i8 = 0;
pub const TYPE: i8 = 1;
pub const FUNC: i8 = 2;
pub const FIELD: i8 = 3;


#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<i8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i8> for Kind {
    fn eq(&self, other: &i8) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i8> for Kind {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i8 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i8 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = i8;
    fn add(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i8> for Kind {
    type Output = i8;
    fn add(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Kind> for i8 {
    type Output = i8;
    fn add(self, other: Kind) -> i8 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Kind {
    type Output = i8;
    fn sub(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i8> for Kind {
    type Output = i8;
    fn sub(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Kind> for i8 {
    type Output = i8;
    fn sub(self, other: Kind) -> i8 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Kind {
    type Output = i8;
    fn bitand(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Kind> for i8 {
    type Output = i8;
    fn bitand(self, other: Kind) -> i8 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Kind {
    type Output = i8;
    fn bitor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Kind> for i8 {
    type Output = i8;
    fn bitor(self, other: Kind) -> i8 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Kind {
    type Output = i8;
    fn bitxor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Kind> for i8 {
    type Output = i8;
    fn bitxor(self, other: Kind) -> i8 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Version(pub Rc<RefCell<Option<i8>>>);

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

impl PartialEq<i8> for Version {
    fn eq(&self, other: &i8) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i8> for Version {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Version> for i8 {
    fn eq(&self, other: &Version) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Version> for i8 {
    fn partial_cmp(&self, other: &Version) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Version {
    type Output = i8;
    fn add(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i8> for Version {
    type Output = i8;
    fn add(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Version> for i8 {
    type Output = i8;
    fn add(self, other: Version) -> i8 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Version {
    type Output = i8;
    fn sub(self, other: Self) -> i8 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i8> for Version {
    type Output = i8;
    fn sub(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Version> for i8 {
    type Output = i8;
    fn sub(self, other: Version) -> i8 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Version {
    type Output = Version;
    fn bitand(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for Version {
    type Output = i8;
    fn bitand(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Version> for i8 {
    type Output = i8;
    fn bitand(self, other: Version) -> i8 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Version {
    type Output = Version;
    fn bitor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for Version {
    type Output = i8;
    fn bitor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Version> for i8 {
    type Output = i8;
    fn bitor(self, other: Version) -> i8 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Version {
    type Output = Version;
    fn bitxor(self, other: Self) -> Version {
        Version(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for Version {
    type Output = i8;
    fn bitxor(self, other: i8) -> i8 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Version> for i8 {
    type Output = i8;
    fn bitxor(self, other: Version) -> i8 {
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


#[derive(Debug, Clone, Default)]
pub struct Symbol {
    pub name: Rc<RefCell<Option<String>>>,
    pub kind: Rc<RefCell<Option<Kind>>>,
    pub version: Rc<RefCell<Option<Version>>>,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.borrow().as_ref().unwrap()), (*self.kind.borrow().as_ref().unwrap()), (*self.version.borrow().as_ref().unwrap()))
    }
}


fn main() {
    let mut sym = Rc::new(RefCell::new(Some(Symbol { name: Rc::new(RefCell::new(Some("Println".to_string()))), kind: Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(FUNC as i8))))))), version: Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(1 as i8))))))), ..Default::default() })));
    let mut field = Rc::new(RefCell::new(Some(Symbol { name: Rc::new(RefCell::new(Some("Point.X".to_string()))), kind: Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(FIELD as i8))))))), version: Rc::new(RefCell::new(Some(Version(Rc::new(RefCell::new(Some(0 as i8))))))), ..Default::default() })));

    println!("{} {} {}", (*(*sym.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*sym.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()), (*(*sym.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
    println!("{} {} {}", (*(*field.borrow().as_ref().unwrap()).name.borrow().as_ref().unwrap()), (*(*field.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()), (*(*field.borrow().as_ref().unwrap()).version.borrow().as_ref().unwrap()));
}