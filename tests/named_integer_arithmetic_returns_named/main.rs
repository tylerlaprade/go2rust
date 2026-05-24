use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone, Default)]
pub struct Pos(pub Rc<RefCell<Option<i32>>>);

impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Pos {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Pos {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Pos> for i32 {
    fn eq(&self, other: &Pos) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<Pos> for i32 {
    fn partial_cmp(&self, other: &Pos) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Pos {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Pos> for i32 {
    type Output = i32;
    fn add(self, other: Pos) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Pos {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Pos> for i32 {
    type Output = i32;
    fn sub(self, other: Pos) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Pos {
    type Output = Pos;
    fn bitand(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Pos {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Pos> for i32 {
    type Output = i32;
    fn bitand(self, other: Pos) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Pos {
    type Output = Pos;
    fn bitor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Pos {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Pos> for i32 {
    type Output = i32;
    fn bitor(self, other: Pos) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Pos {
    type Output = Pos;
    fn bitxor(self, other: Self) -> Pos {
        Pos(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Pos {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Pos> for i32 {
    type Output = i32;
    fn bitxor(self, other: Pos) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for Pos {}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub fn next_pos(p: Rc<RefCell<Option<Pos>>>) -> Rc<RefCell<Option<Pos>>> {

    return {
            let __tmp_x = (*p.borrow().as_ref().unwrap()).clone();
            let __tmp_y = Pos(Rc::new(RefCell::new(Some(1 as i32))));
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn sum_pos(a: Rc<RefCell<Option<Pos>>>, b: Rc<RefCell<Option<Pos>>>) -> Rc<RefCell<Option<Pos>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x + __tmp_y)))
        };
}

pub fn diff_pos(a: Rc<RefCell<Option<Pos>>>, b: Rc<RefCell<Option<Pos>>>) -> Rc<RefCell<Option<Pos>>> {

    return {
            let __tmp_x = (*a.borrow().as_ref().unwrap()).clone();
            let __tmp_y = (*b.borrow().as_ref().unwrap()).clone();
            Rc::new(RefCell::new(Some(__tmp_x - __tmp_y)))
        };
}

fn main() {
    let mut a: Rc<RefCell<Option<Pos>>> = Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(10)))))));
    let mut b: Rc<RefCell<Option<Pos>>> = Rc::new(RefCell::new(Some(Pos(Rc::new(RefCell::new(Some(5)))))));
    println!("{}", format!("{}", (*next_pos(Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*sum_pos(Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*diff_pos(Rc::new(RefCell::new(Some((*a.borrow().as_ref().unwrap()).clone()))), Rc::new(RefCell::new(Some((*b.borrow().as_ref().unwrap()).clone())))).borrow().as_ref().unwrap())));
}