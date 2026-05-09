use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Kind(pub Rc<RefCell<Option<i32>>>);

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

impl PartialEq<i32> for Kind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Kind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i32 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i32 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Kind {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Kind> for i32 {
    type Output = i32;
    fn add(self, other: Kind) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Kind {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Kind {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Kind> for i32 {
    type Output = i32;
    fn sub(self, other: Kind) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Kind {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Kind> for i32 {
    type Output = i32;
    fn bitand(self, other: Kind) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Kind {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Kind> for i32 {
    type Output = i32;
    fn bitor(self, other: Kind) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}


#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub kind: Rc<RefCell<Option<Kind>>>,
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.kind.borrow().as_ref().unwrap()))
    }
}


impl Kind {
    pub fn method_int(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*self.0.borrow().as_ref().unwrap()) as i32)));
    }

    pub fn method_plus(&self) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some(((*self.0.borrow().as_ref().unwrap()) + 1) as i32)));
    }
}

pub fn as_int(k: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32)));
}

pub fn as_uint64(k: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<u64>>> {

    return Rc::new(RefCell::new(Some((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as u64)));
}

pub fn field_as_int(e: Rc<RefCell<Option<Entry>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*(*(*e.borrow().as_ref().unwrap()).kind.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32)));
}

pub fn plus_as_int(k: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some(((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) + 1) as i32)));
}

fn main() {
    let mut k: Rc<RefCell<Option<Kind>>> = Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(3)))))));
    let mut entry = Rc::new(RefCell::new(Some(Entry { kind: k.clone(), ..Default::default() })));
    println!("{}", (*as_int(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*as_uint64(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*field_as_int(Rc::new(RefCell::new(Some((*entry.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*plus_as_int(Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*(*k.borrow().as_ref().unwrap()).method_int().borrow().as_ref().unwrap()));
    println!("{}", (*(*k.borrow().as_ref().unwrap()).method_plus().borrow().as_ref().unwrap()));
}