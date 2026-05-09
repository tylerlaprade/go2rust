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

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl Kind {
    pub fn method_pick(&self, values: Rc<RefCell<Option<Vec<i32>>>>) -> Rc<RefCell<Option<i32>>> {
        return Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap())[(*self.0.borrow().as_ref().unwrap()) as usize].clone())));
    }
}

pub fn pick(values: Rc<RefCell<Option<Vec<i32>>>>, k: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap())[(*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as usize].clone())));
}

pub fn previous(values: Rc<RefCell<Option<Vec<i32>>>>, k: Rc<RefCell<Option<Kind>>>) -> Rc<RefCell<Option<i32>>> {

    return Rc::new(RefCell::new(Some((*values.borrow().as_ref().unwrap())[((*(*k.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) - 1) as usize].clone())));
}

fn main() {
    let mut values = Rc::new(RefCell::new(Some(vec![10, 20, 30])));
    let mut k: Rc<RefCell<Option<Kind>>> = Rc::new(RefCell::new(Some(Kind(Rc::new(RefCell::new(Some(1)))))));
    println!("{}", (*pick(values.clone(), Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*previous(values.clone(), Rc::new(RefCell::new(Some((*k.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap()));
    println!("{}", (*(*k.borrow().as_ref().unwrap()).method_pick(values.clone()).borrow().as_ref().unwrap()));
}