use crate::{GoChannel};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Kind(pub Arc<Mutex<Option<i32>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Kind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Kind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for i32 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for i32 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Kind {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Kind> for i32 {
    type Output = i32;
    fn add(self, other: Kind) -> i32 {
        self + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub for Kind {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Kind {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Kind> for i32 {
    type Output = i32;
    fn sub(self, other: Kind) -> i32 {
        self - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Kind {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Kind> for i32 {
    type Output = i32;
    fn bitand(self, other: Kind) -> i32 {
        self & *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Kind {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Kind> for i32 {
    type Output = i32;
    fn bitor(self, other: Kind) -> i32 {
        self | *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Kind {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Kind> for i32 {
    type Output = i32;
    fn bitxor(self, other: Kind) -> i32 {
        self ^ *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Index(pub Arc<Mutex<Option<i32>>>);

impl Display for Index {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Index {
    fn eq(&self, other: &Self) -> bool {
        self.0.lock().unwrap().as_ref().unwrap() == other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialEq<i32> for Index {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Index {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for Index {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Index> for i32 {
    fn eq(&self, other: &Index) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Index> for i32 {
    fn partial_cmp(&self, other: &Index) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Index {
    type Output = i32;
    fn add(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Add<i32> for Index {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() + other
    }
}

impl std::ops::Add<Index> for i32 {
    type Output = i32;
    fn add(self, other: Index) -> i32 {
        self + *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub for Index {
    type Output = i32;
    fn sub(self, other: Self) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::Sub<i32> for Index {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<Index> for i32 {
    type Output = i32;
    fn sub(self, other: Index) -> i32 {
        self - *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for Index {
    type Output = Index;
    fn bitand(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Index {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<Index> for i32 {
    type Output = i32;
    fn bitand(self, other: Index) -> i32 {
        self & *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitOr for Index {
    type Output = Index;
    fn bitor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Index {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<Index> for i32 {
    type Output = i32;
    fn bitor(self, other: Index) -> i32 {
        self | *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl std::ops::BitXor for Index {
    type Output = Index;
    fn bitxor(self, other: Self) -> Index {
        Index(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Index {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.lock().unwrap().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<Index> for i32 {
    type Output = i32;
    fn bitxor(self, other: Index) -> i32 {
        self ^ *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl Eq for Index {}

impl Ord for Index {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub kind: Arc<Mutex<Option<Kind>>>,
    pub index: Arc<Mutex<Option<Index>>>,
}

impl Entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.kind.lock().unwrap(); let __right = other.kind.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.index.lock().unwrap(); let __right = other.index.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl Eq for Entry {}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            let __left = { self.kind.lock().unwrap().as_ref().cloned() };
            let __right = { other.kind.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        {
            let __left = { self.index.lock().unwrap().as_ref().cloned() };
            let __right = { other.index.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}
