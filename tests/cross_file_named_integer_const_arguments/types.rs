use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

pub const RELOC_STRING: i32 = 0;
pub const RELOC_META: i32 = 1;


pub const SYNC_E_O_F: i32 = 0 + 1;
pub const SYNC_BOOL: i32 = 1 + 1;


#[derive(Debug, Clone, Default)]
pub struct RelocKind(pub Rc<RefCell<Option<i32>>>);

impl Display for RelocKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for RelocKind {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for RelocKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for RelocKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for RelocKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<RelocKind> for i32 {
    fn eq(&self, other: &RelocKind) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<RelocKind> for i32 {
    fn partial_cmp(&self, other: &RelocKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for RelocKind {
    type Output = RelocKind;
    fn add(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for RelocKind {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<RelocKind> for i32 {
    type Output = i32;
    fn add(self, other: RelocKind) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for RelocKind {
    type Output = RelocKind;
    fn sub(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for RelocKind {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<RelocKind> for i32 {
    type Output = i32;
    fn sub(self, other: RelocKind) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for RelocKind {
    type Output = RelocKind;
    fn bitand(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for RelocKind {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<RelocKind> for i32 {
    type Output = i32;
    fn bitand(self, other: RelocKind) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for RelocKind {
    type Output = RelocKind;
    fn bitor(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for RelocKind {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<RelocKind> for i32 {
    type Output = i32;
    fn bitor(self, other: RelocKind) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for RelocKind {
    type Output = RelocKind;
    fn bitxor(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for RelocKind {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<RelocKind> for i32 {
    type Output = i32;
    fn bitxor(self, other: RelocKind) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for RelocKind {}

impl Ord for RelocKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct SyncMarker(pub Rc<RefCell<Option<i32>>>);

impl Display for SyncMarker {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.borrow().as_ref().unwrap())
    }
}

impl PartialEq for SyncMarker {
    fn eq(&self, other: &Self) -> bool {
        self.0.borrow().as_ref().unwrap() == other.0.borrow().as_ref().unwrap()
    }
}

impl PartialEq<i32> for SyncMarker {
    fn eq(&self, other: &i32) -> bool {
        *self.0.borrow().as_ref().unwrap() == *other
    }
}

impl PartialOrd for SyncMarker {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl PartialOrd<i32> for SyncMarker {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.borrow().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<SyncMarker> for i32 {
    fn eq(&self, other: &SyncMarker) -> bool {
        *self == *other.0.borrow().as_ref().unwrap()
    }
}

impl PartialOrd<SyncMarker> for i32 {
    fn partial_cmp(&self, other: &SyncMarker) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.borrow().as_ref().unwrap())
    }
}

impl std::ops::Add for SyncMarker {
    type Output = SyncMarker;
    fn add(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for SyncMarker {
    type Output = i32;
    fn add(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() + other
    }
}

impl std::ops::Add<SyncMarker> for i32 {
    type Output = i32;
    fn add(self, other: SyncMarker) -> i32 {
        self + *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::Sub for SyncMarker {
    type Output = SyncMarker;
    fn sub(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for SyncMarker {
    type Output = i32;
    fn sub(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() - other
    }
}

impl std::ops::Sub<SyncMarker> for i32 {
    type Output = i32;
    fn sub(self, other: SyncMarker) -> i32 {
        self - *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitAnd for SyncMarker {
    type Output = SyncMarker;
    fn bitand(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for SyncMarker {
    type Output = i32;
    fn bitand(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() & other
    }
}

impl std::ops::BitAnd<SyncMarker> for i32 {
    type Output = i32;
    fn bitand(self, other: SyncMarker) -> i32 {
        self & *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitOr for SyncMarker {
    type Output = SyncMarker;
    fn bitor(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for SyncMarker {
    type Output = i32;
    fn bitor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() | other
    }
}

impl std::ops::BitOr<SyncMarker> for i32 {
    type Output = i32;
    fn bitor(self, other: SyncMarker) -> i32 {
        self | *other.0.borrow().as_ref().unwrap()
    }
}

impl std::ops::BitXor for SyncMarker {
    type Output = SyncMarker;
    fn bitxor(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for SyncMarker {
    type Output = i32;
    fn bitxor(self, other: i32) -> i32 {
        *self.0.borrow().as_ref().unwrap() ^ other
    }
}

impl std::ops::BitXor<SyncMarker> for i32 {
    type Output = i32;
    fn bitxor(self, other: SyncMarker) -> i32 {
        self ^ *other.0.borrow().as_ref().unwrap()
    }
}

impl Eq for SyncMarker {}

impl Ord for SyncMarker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.borrow().as_ref().cloned() };
        let __right = { other.0.borrow().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Encoder {
}

impl Encoder {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for Encoder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl Encoder {
    pub fn sync(&self, m: Rc<RefCell<Option<SyncMarker>>>) -> i32 {
        return (*Rc::new(RefCell::new(Some((*(*m.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap());
    }

    pub fn call_sync(&self) -> i32 {
        return self.sync(Rc::new(RefCell::new(Some(SyncMarker(Rc::new(RefCell::new(Some(SYNC_BOOL as i32))))))));
    }
}