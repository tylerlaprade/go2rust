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
    type Output = RelocKind;
    fn add(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<RelocKind> for i32 {
    type Output = RelocKind;
    fn add(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for RelocKind {
    type Output = RelocKind;
    fn sub(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for RelocKind {
    type Output = RelocKind;
    fn sub(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<RelocKind> for i32 {
    type Output = RelocKind;
    fn sub(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for RelocKind {
    type Output = RelocKind;
    fn bitand(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for RelocKind {
    type Output = RelocKind;
    fn bitand(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<RelocKind> for i32 {
    type Output = RelocKind;
    fn bitand(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for RelocKind {
    type Output = RelocKind;
    fn bitor(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for RelocKind {
    type Output = RelocKind;
    fn bitor(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<RelocKind> for i32 {
    type Output = RelocKind;
    fn bitor(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for RelocKind {
    type Output = RelocKind;
    fn bitxor(self, other: Self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for RelocKind {
    type Output = RelocKind;
    fn bitxor(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<RelocKind> for i32 {
    type Output = RelocKind;
    fn bitxor(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for RelocKind {
    type Output = RelocKind;
    fn not(self) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: i8) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: i16) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: i64) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: u32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: u8) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: u16) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: u64) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for RelocKind {
    type Output = RelocKind;
    fn shl(self, other: usize) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: RelocKind) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: i32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: i8) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: i16) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: i64) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: u32) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: u8) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: u16) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: u64) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for RelocKind {
    type Output = RelocKind;
    fn shr(self, other: usize) -> RelocKind {
        RelocKind(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
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
    type Output = SyncMarker;
    fn add(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<SyncMarker> for i32 {
    type Output = SyncMarker;
    fn add(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(self + *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for SyncMarker {
    type Output = SyncMarker;
    fn sub(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for SyncMarker {
    type Output = SyncMarker;
    fn sub(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<SyncMarker> for i32 {
    type Output = SyncMarker;
    fn sub(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(self - *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for SyncMarker {
    type Output = SyncMarker;
    fn bitand(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for SyncMarker {
    type Output = SyncMarker;
    fn bitand(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<SyncMarker> for i32 {
    type Output = SyncMarker;
    fn bitand(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(self & *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for SyncMarker {
    type Output = SyncMarker;
    fn bitor(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for SyncMarker {
    type Output = SyncMarker;
    fn bitor(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<SyncMarker> for i32 {
    type Output = SyncMarker;
    fn bitor(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(self | *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for SyncMarker {
    type Output = SyncMarker;
    fn bitxor(self, other: Self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for SyncMarker {
    type Output = SyncMarker;
    fn bitxor(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<SyncMarker> for i32 {
    type Output = SyncMarker;
    fn bitxor(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(self ^ *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Not for SyncMarker {
    type Output = SyncMarker;
    fn not(self) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(!*self.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: i8) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: i16) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: i64) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: u32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: u8) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: u16) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: u64) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for SyncMarker {
    type Output = SyncMarker;
    fn shl(self, other: usize) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: SyncMarker) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> *other.0.borrow().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: i32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: i8) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: i16) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: i64) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: u32) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: u8) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: u16) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: u64) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for SyncMarker {
    type Output = SyncMarker;
    fn shr(self, other: usize) -> SyncMarker {
        SyncMarker(Rc::new(RefCell::new(Some(*self.0.borrow().as_ref().unwrap() >> other))))
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
        (*Rc::new(RefCell::new(Some((*(*m.borrow().as_ref().unwrap()).0.borrow().as_ref().unwrap()) as i32))).borrow().as_ref().unwrap())
    }

    pub fn call_sync(&self) -> i32 {
        self.sync(Rc::new(RefCell::new(Some(SYNC_BOOL))))
    }
}