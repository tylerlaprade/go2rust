use go2rust_stdlib_stubs::*;

use crate::map::*;
use crate::runtime::*;
use crate::runtime_fast32_swiss::*;
use crate::runtime_fast64_swiss::*;
use crate::runtime_faststr_swiss::*;
use crate::runtime_swiss::*;
use crate::table::*;
use crate::table_debug::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_AVG_GROUP_LOAD: i32 = 7;
pub(crate) const CTRL_EMPTY: u8 = 0b10000000;
pub(crate) const CTRL_DELETED: u8 = 0b11111110;
pub(crate) const BITSET_L_S_B: i64 = 0x0101010101010101;
pub(crate) const BITSET_M_S_B: u64 = 0x8080808080808080;
pub(crate) const BITSET_EMPTY: u64 = BITSET_L_S_B as u64 * (CTRL_EMPTY as u64);
pub(crate) const BITSET_DELETED: u64 = BITSET_L_S_B as u64 * (CTRL_DELETED as u64);


pub(crate) const CTRL_GROUPS_SIZE: usize = std::mem::size_of::<ctrlGroup>();
pub(crate) const GROUP_SLOTS_OFFSET: usize = CTRL_GROUPS_SIZE;


/// bitset represents a set of slots within a group.
///
/// The underlying representation depends on GOARCH.
///
/// On AMD64, bitset uses one bit per slot, where the bit is set if the slot is
/// part of the set. All of the ctrlGroup.match* methods are replaced with
/// intrinsics that return this packed representation.
///
/// On other architectures, bitset uses one byte per slot, where each byte is
/// either 0x80 if the slot is part of the set or 0x00 otherwise. This makes it
/// convenient to calculate for an entire group at once using standard
/// arithemetic instructions.
#[derive(Debug, Clone, Default)]
pub struct bitset(pub Arc<Mutex<Option<u64>>>);

impl Display for bitset {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for bitset {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for bitset {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for bitset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for bitset {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<bitset> for u64 {
    fn eq(&self, other: &bitset) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<bitset> for u64 {
    fn partial_cmp(&self, other: &bitset) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for bitset {
    type Output = bitset;
    fn add(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for bitset {
    type Output = bitset;
    fn add(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<bitset> for u64 {
    type Output = bitset;
    fn add(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for bitset {
    type Output = bitset;
    fn sub(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for bitset {
    type Output = bitset;
    fn sub(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<bitset> for u64 {
    type Output = bitset;
    fn sub(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for bitset {
    type Output = bitset;
    fn mul(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for bitset {
    type Output = bitset;
    fn mul(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<bitset> for u64 {
    type Output = bitset;
    fn mul(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for bitset {
    type Output = bitset;
    fn div(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for bitset {
    type Output = bitset;
    fn div(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<bitset> for u64 {
    type Output = bitset;
    fn div(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for bitset {
    type Output = bitset;
    fn rem(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for bitset {
    type Output = bitset;
    fn rem(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<bitset> for u64 {
    type Output = bitset;
    fn rem(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for bitset {
    type Output = bitset;
    fn bitand(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for bitset {
    type Output = bitset;
    fn bitand(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<bitset> for u64 {
    type Output = bitset;
    fn bitand(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for bitset {
    type Output = bitset;
    fn bitor(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for bitset {
    type Output = bitset;
    fn bitor(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<bitset> for u64 {
    type Output = bitset;
    fn bitor(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for bitset {
    type Output = bitset;
    fn bitxor(self, other: Self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for bitset {
    type Output = bitset;
    fn bitxor(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<bitset> for u64 {
    type Output = bitset;
    fn bitxor(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for bitset {
    type Output = bitset;
    fn not(self) -> bitset {
        bitset(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for bitset {
    type Output = bitset;
    fn shl(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for bitset {
    type Output = bitset;
    fn shl(self, other: i32) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for bitset {
    type Output = bitset;
    fn shl(self, other: i8) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for bitset {
    type Output = bitset;
    fn shl(self, other: i16) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for bitset {
    type Output = bitset;
    fn shl(self, other: i64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for bitset {
    type Output = bitset;
    fn shl(self, other: u32) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for bitset {
    type Output = bitset;
    fn shl(self, other: u8) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for bitset {
    type Output = bitset;
    fn shl(self, other: u16) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for bitset {
    type Output = bitset;
    fn shl(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for bitset {
    type Output = bitset;
    fn shl(self, other: usize) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for bitset {
    type Output = bitset;
    fn shr(self, other: bitset) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for bitset {
    type Output = bitset;
    fn shr(self, other: i32) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for bitset {
    type Output = bitset;
    fn shr(self, other: i8) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for bitset {
    type Output = bitset;
    fn shr(self, other: i16) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for bitset {
    type Output = bitset;
    fn shr(self, other: i64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for bitset {
    type Output = bitset;
    fn shr(self, other: u32) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for bitset {
    type Output = bitset;
    fn shr(self, other: u8) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for bitset {
    type Output = bitset;
    fn shr(self, other: u16) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for bitset {
    type Output = bitset;
    fn shr(self, other: u64) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for bitset {
    type Output = bitset;
    fn shr(self, other: usize) -> bitset {
        bitset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for bitset {}

impl Ord for bitset {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// Each slot in the hash table has a control byte which can have one of three
/// states: empty, deleted, and full. They have the following bit patterns:
///
///	  empty: 1 0 0 0 0 0 0 0
///	deleted: 1 1 1 1 1 1 1 0
///	   full: 0 h h h h h h h  // h represents the H1 hash bits
///
/// TODO(prattmic): Consider inverting the top bit so that the zero value is empty.
#[derive(Debug, Clone, Default)]
pub struct ctrl(pub Arc<Mutex<Option<u8>>>);

impl Display for ctrl {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ctrl {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for ctrl {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ctrl {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for ctrl {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ctrl> for u8 {
    fn eq(&self, other: &ctrl) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ctrl> for u8 {
    fn partial_cmp(&self, other: &ctrl) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ctrl {
    type Output = ctrl;
    fn add(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for ctrl {
    type Output = ctrl;
    fn add(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ctrl> for u8 {
    type Output = ctrl;
    fn add(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ctrl {
    type Output = ctrl;
    fn sub(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for ctrl {
    type Output = ctrl;
    fn sub(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ctrl> for u8 {
    type Output = ctrl;
    fn sub(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ctrl {
    type Output = ctrl;
    fn mul(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for ctrl {
    type Output = ctrl;
    fn mul(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ctrl> for u8 {
    type Output = ctrl;
    fn mul(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ctrl {
    type Output = ctrl;
    fn div(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for ctrl {
    type Output = ctrl;
    fn div(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ctrl> for u8 {
    type Output = ctrl;
    fn div(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ctrl {
    type Output = ctrl;
    fn rem(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for ctrl {
    type Output = ctrl;
    fn rem(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ctrl> for u8 {
    type Output = ctrl;
    fn rem(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ctrl {
    type Output = ctrl;
    fn bitand(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for ctrl {
    type Output = ctrl;
    fn bitand(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ctrl> for u8 {
    type Output = ctrl;
    fn bitand(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ctrl {
    type Output = ctrl;
    fn bitor(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for ctrl {
    type Output = ctrl;
    fn bitor(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ctrl> for u8 {
    type Output = ctrl;
    fn bitor(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ctrl {
    type Output = ctrl;
    fn bitxor(self, other: Self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for ctrl {
    type Output = ctrl;
    fn bitxor(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ctrl> for u8 {
    type Output = ctrl;
    fn bitxor(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ctrl {
    type Output = ctrl;
    fn not(self) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ctrl {
    type Output = ctrl;
    fn shl(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ctrl {
    type Output = ctrl;
    fn shl(self, other: i32) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ctrl {
    type Output = ctrl;
    fn shl(self, other: i8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ctrl {
    type Output = ctrl;
    fn shl(self, other: i16) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ctrl {
    type Output = ctrl;
    fn shl(self, other: i64) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ctrl {
    type Output = ctrl;
    fn shl(self, other: u32) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ctrl {
    type Output = ctrl;
    fn shl(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ctrl {
    type Output = ctrl;
    fn shl(self, other: u16) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ctrl {
    type Output = ctrl;
    fn shl(self, other: u64) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ctrl {
    type Output = ctrl;
    fn shl(self, other: usize) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ctrl {
    type Output = ctrl;
    fn shr(self, other: ctrl) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ctrl {
    type Output = ctrl;
    fn shr(self, other: i32) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ctrl {
    type Output = ctrl;
    fn shr(self, other: i8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ctrl {
    type Output = ctrl;
    fn shr(self, other: i16) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ctrl {
    type Output = ctrl;
    fn shr(self, other: i64) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ctrl {
    type Output = ctrl;
    fn shr(self, other: u32) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ctrl {
    type Output = ctrl;
    fn shr(self, other: u8) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ctrl {
    type Output = ctrl;
    fn shr(self, other: u16) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ctrl {
    type Output = ctrl;
    fn shr(self, other: u64) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ctrl {
    type Output = ctrl;
    fn shr(self, other: usize) -> ctrl {
        ctrl(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ctrl {}

impl Ord for ctrl {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// ctrlGroup is a fixed size array of abi.SwissMapGroupSlots control bytes
/// stored in a uint64.
#[derive(Debug, Clone, Default)]
pub struct ctrlGroup(pub Arc<Mutex<Option<u64>>>);

impl Display for ctrlGroup {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ctrlGroup {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for ctrlGroup {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ctrlGroup {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for ctrlGroup {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ctrlGroup> for u64 {
    fn eq(&self, other: &ctrlGroup) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ctrlGroup> for u64 {
    fn partial_cmp(&self, other: &ctrlGroup) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ctrlGroup {
    type Output = ctrlGroup;
    fn add(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn add(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn add(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ctrlGroup {
    type Output = ctrlGroup;
    fn sub(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn sub(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn sub(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ctrlGroup {
    type Output = ctrlGroup;
    fn mul(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn mul(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn mul(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ctrlGroup {
    type Output = ctrlGroup;
    fn div(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn div(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn div(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ctrlGroup {
    type Output = ctrlGroup;
    fn rem(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn rem(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn rem(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ctrlGroup {
    type Output = ctrlGroup;
    fn bitand(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn bitand(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn bitand(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ctrlGroup {
    type Output = ctrlGroup;
    fn bitor(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn bitor(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn bitor(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ctrlGroup {
    type Output = ctrlGroup;
    fn bitxor(self, other: Self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn bitxor(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ctrlGroup> for u64 {
    type Output = ctrlGroup;
    fn bitxor(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ctrlGroup {
    type Output = ctrlGroup;
    fn not(self) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: i32) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: i8) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: i16) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: i64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: u32) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: u8) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: u16) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ctrlGroup {
    type Output = ctrlGroup;
    fn shl(self, other: usize) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: ctrlGroup) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: i32) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: i8) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: i16) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: i64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: u32) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: u8) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: u16) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: u64) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ctrlGroup {
    type Output = ctrlGroup;
    fn shr(self, other: usize) -> ctrlGroup {
        ctrlGroup(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ctrlGroup {}

impl Ord for ctrlGroup {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl bitset {
    /// first returns the relative index of the first control byte in the group that
    /// is in the set.
    ///
    /// Preconditions: b is not 0 (empty).
    pub fn first(&self) -> usize {
        bitset_first(Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// removeFirst clears the first set bit (that is, resets the least significant
    /// set bit to 0).
    pub fn remove_first(&self) -> Arc<Mutex<Option<bitset>>> {
        return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & (((*self.0.lock().unwrap().as_ref().unwrap()) - 1))))))))));
    }

    /// removeBelow clears all set bits below slot i (non-inclusive).
    pub fn remove_below(&self, i: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitset>>> {
        bitset_remove_below(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// lowestSet returns true if the bit is set for the lowest index in the bitset.
    ///
    /// This is intended for use with shiftOutLowest to loop over all entries in the
    /// bitset regardless of whether they are set.
    pub fn lowest_set(&self) -> bool {
        bitset_lowest_set(Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// shiftOutLowest shifts the lowest entry out of the bitset. Afterwards, the
    /// lowest entry in the bitset corresponds to the next slot.
    pub fn shift_out_lowest(&self) -> Arc<Mutex<Option<bitset>>> {
        bitset_shift_out_lowest(Arc::new(Mutex::new(Some(self.clone()))))
    }
}

impl ctrlGroup {
    /// get returns the i-th control byte.
    pub fn get(&self, i: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<ctrl>>> {
        if internal_goarch::BIG_ENDIAN {
        return Arc::new(Mutex::new(Some(ctrl(Arc::new(Mutex::new(Some((*{ let __v = (*Arc::new(Mutex::new({ let __ptr = { let __go_unsafe_result: Arc<Mutex<Option<usize>>> = unimplemented!("unsafe.Add requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<ctrl>(unimplemented!("unsafe.Pointer conversion to ctrl")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))));
    }
        Arc::new(Mutex::new(Some(ctrl(Arc::new(Mutex::new(Some((*{ let __v = (*Arc::new(Mutex::new({ let __ptr = { let __go_unsafe_result: Arc<Mutex<Option<usize>>> = unimplemented!("unsafe.Add requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<ctrl>(unimplemented!("unsafe.Pointer conversion to ctrl")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()))))))))
    }

    /// set sets the i-th control byte.
    pub fn set(&self, i: Arc<Mutex<Option<usize>>>, c: Arc<Mutex<Option<ctrl>>>) {
        if internal_goarch::BIG_ENDIAN {
        { unimplemented!("unsafe.Pointer dereference assignment"); };
        return;
    }
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }

    /// setEmpty sets all the control bytes to empty.
    pub fn set_empty(&mut self) {
        { let new_val = ctrlGroup(Arc::new(Mutex::new(Some(BITSET_EMPTY as u64)))); *self = new_val; };
    }

    /// matchH2 returns the set of slots which are full and for which the 7-bit hash
    /// matches the given value. May return false positives.
    pub fn match_h2(&self, h: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitset>>> {
        ctrl_group_match_h2(Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// matchEmpty returns the set of slots in the group that are empty.
    pub fn match_empty(&self) -> Arc<Mutex<Option<bitset>>> {
        ctrl_group_match_empty(Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// matchEmptyOrDeleted returns the set of slots in the group that are empty or
    /// deleted.
    pub fn match_empty_or_deleted(&self) -> Arc<Mutex<Option<bitset>>> {
        ctrl_group_match_empty_or_deleted(Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// matchFull returns the set of slots in the group that are full.
    pub fn match_full(&self) -> Arc<Mutex<Option<bitset>>> {
        ctrl_group_match_full(Arc::new(Mutex::new(Some(self.clone()))))
    }
}

/// Portable implementation of first.
///
/// On AMD64, this is replaced with an intrisic that simply does
/// TrailingZeros64. There is no need to shift as the bitset is packed.
pub fn bitset_first(b: Arc<Mutex<Option<bitset>>>) -> usize {
    return { let __tmp_x = (*Arc::new(Mutex::new(Some(internal_runtime_sys::trailing_zeros64(Arc::new(Mutex::new(Some((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3; __tmp_x >> __tmp_y };
}

/// Portable implementation of removeBelow.
///
/// On AMD64, this is replaced with an intrisic that clears the lower i bits.
pub fn bitset_remove_below(b: Arc<Mutex<Option<bitset>>>, i: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitset>>> {
        // Clear all bits below slot i's byte.
    let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (1 as u64); let __tmp_y = ({ let __tmp_x = 8 as u64; let __tmp_y = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }); __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some(((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ! (*mask.lock().unwrap().as_ref().unwrap()) as u64))))))));
}

/// Portable implementation of lowestSet.
///
/// On AMD64, this is replaced with an intrisic that checks the lowest bit.
pub fn bitset_lowest_set(b: Arc<Mutex<Option<bitset>>>) -> bool {
    return { let __tmp_x = bitset(Arc::new(Mutex::new(Some(((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ((1 << 7i32))))))); let __tmp_y = bitset(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y };
}

/// Portable implementation of shiftOutLowest.
///
/// On AMD64, this is replaced with an intrisic that shifts a single bit.
pub fn bitset_shift_out_lowest(b: Arc<Mutex<Option<bitset>>>) -> Arc<Mutex<Option<bitset>>> {
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some(((*{ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) >> 8i32))))))));
}

/// Portable implementation of matchH2.
///
/// Note: On AMD64, this is an intrinsic implemented with SIMD instructions. See
/// note on bitset about the packed instrinsified return value.
pub fn ctrl_group_match_h2(g: Arc<Mutex<Option<ctrlGroup>>>, h: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<bitset>>> {
        // NB: This generic matching routine produces false positive matches when
        // h is 2^N and the control bytes have a seq of 2^N followed by 2^N+1. For
        // example: if ctrls==0x0302 and h=02, we'll compute v as 0x0100. When we
        // subtract off 0x0101 the first 2 bytes we'll become 0xffff and both be
        // considered matches of h. The false positive matches are not a problem,
        // just a rare inefficiency. Note that they only occur if there is a real
        // match and never occur on ctrlEmpty, or ctrlDeleted. The subsequent key
        // comparisons ensure that there is no correctness issue.
    let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*{ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = BITSET_L_S_B as u64; let __tmp_y = (*Arc::new(Mutex::new(Some((*h.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }); __tmp_x ^ __tmp_y })));
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = BITSET_L_S_B as u64; __tmp_x - __tmp_y }); let __tmp_y = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & ! __tmp_y }); let __tmp_y = BITSET_M_S_B as u64; __tmp_x & __tmp_y } as u64)))))));
}

/// Portable implementation of matchEmpty.
///
/// Note: On AMD64, this is an intrinsic implemented with SIMD instructions. See
/// note on bitset about the packed instrinsified return value.
pub fn ctrl_group_match_empty(g: Arc<Mutex<Option<ctrlGroup>>>) -> Arc<Mutex<Option<bitset>>> {
        // An empty slot is   1000 0000
        // A deleted slot is  1111 1110
        // A full slot is     0??? ????
        //
        // A slot is empty iff bit 7 is set and bit 1 is not. We could select any
        // of the other bits here (e.g. v << 1 would also work).
    let mut v = Arc::new(Mutex::new(Some((*{ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)));
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x << __tmp_y }); __tmp_x & ! __tmp_y }); let __tmp_y = BITSET_M_S_B as u64; __tmp_x & __tmp_y } as u64)))))));
}

/// Portable implementation of matchEmptyOrDeleted.
///
/// Note: On AMD64, this is an intrinsic implemented with SIMD instructions. See
/// note on bitset about the packed instrinsified return value.
pub fn ctrl_group_match_empty_or_deleted(g: Arc<Mutex<Option<ctrlGroup>>>) -> Arc<Mutex<Option<bitset>>> {
        // An empty slot is  1000 0000
        // A deleted slot is 1111 1110
        // A full slot is    0??? ????
        //
        // A slot is empty or deleted iff bit 7 is set.
    let mut v = Arc::new(Mutex::new(Some((*{ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)));
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = BITSET_M_S_B as u64; __tmp_x & __tmp_y } as u64)))))));
}

/// Portable implementation of matchFull.
///
/// Note: On AMD64, this is an intrinsic implemented with SIMD instructions. See
/// note on bitset about the packed instrinsified return value.
pub fn ctrl_group_match_full(g: Arc<Mutex<Option<ctrlGroup>>>) -> Arc<Mutex<Option<bitset>>> {
        // An empty slot is  1000 0000
        // A deleted slot is 1111 1110
        // A full slot is    0??? ????
        //
        // A slot is full iff bit 7 is unset.
    let mut v = Arc::new(Mutex::new(Some((*{ let __v = (*g.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)));
    return Arc::new(Mutex::new(Some(bitset(Arc::new(Mutex::new(Some({ let __tmp_x = !(*v.lock().unwrap().as_ref().unwrap()); let __tmp_y = BITSET_M_S_B as u64; __tmp_x & __tmp_y } as u64)))))));
}