use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MODE_P_I_D: u8 = 0;
pub(crate) const MODE_HANDLE: u8 = 1;


pub(crate) const STATUS_O_K: u64 = 0;
pub(crate) const STATUS_DONE: u64 = ((1 as u64) << (62 as u64));
pub(crate) const STATUS_RELEASED: u64 = ((1 as u64) << (63 as u64));
pub(crate) const PROCESS_STATUS_MASK: u64 = 0x3 << 62;


#[derive(Debug, Clone, Default)]
pub struct processMode(pub Arc<Mutex<Option<u8>>>);

impl Display for processMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for processMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for processMode {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for processMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for processMode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<processMode> for u8 {
    fn eq(&self, other: &processMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<processMode> for u8 {
    fn partial_cmp(&self, other: &processMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for processMode {
    type Output = processMode;
    fn add(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for processMode {
    type Output = processMode;
    fn add(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<processMode> for u8 {
    type Output = processMode;
    fn add(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for processMode {
    type Output = processMode;
    fn sub(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for processMode {
    type Output = processMode;
    fn sub(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<processMode> for u8 {
    type Output = processMode;
    fn sub(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for processMode {
    type Output = processMode;
    fn mul(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for processMode {
    type Output = processMode;
    fn mul(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<processMode> for u8 {
    type Output = processMode;
    fn mul(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for processMode {
    type Output = processMode;
    fn div(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for processMode {
    type Output = processMode;
    fn div(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<processMode> for u8 {
    type Output = processMode;
    fn div(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for processMode {
    type Output = processMode;
    fn rem(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for processMode {
    type Output = processMode;
    fn rem(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<processMode> for u8 {
    type Output = processMode;
    fn rem(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for processMode {
    type Output = processMode;
    fn bitand(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for processMode {
    type Output = processMode;
    fn bitand(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<processMode> for u8 {
    type Output = processMode;
    fn bitand(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for processMode {
    type Output = processMode;
    fn bitor(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for processMode {
    type Output = processMode;
    fn bitor(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<processMode> for u8 {
    type Output = processMode;
    fn bitor(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for processMode {
    type Output = processMode;
    fn bitxor(self, other: Self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for processMode {
    type Output = processMode;
    fn bitxor(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<processMode> for u8 {
    type Output = processMode;
    fn bitxor(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for processMode {
    type Output = processMode;
    fn not(self) -> processMode {
        processMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for processMode {
    type Output = processMode;
    fn shl(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for processMode {
    type Output = processMode;
    fn shl(self, other: i32) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for processMode {
    type Output = processMode;
    fn shl(self, other: i8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for processMode {
    type Output = processMode;
    fn shl(self, other: i16) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for processMode {
    type Output = processMode;
    fn shl(self, other: i64) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for processMode {
    type Output = processMode;
    fn shl(self, other: u32) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for processMode {
    type Output = processMode;
    fn shl(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for processMode {
    type Output = processMode;
    fn shl(self, other: u16) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for processMode {
    type Output = processMode;
    fn shl(self, other: u64) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for processMode {
    type Output = processMode;
    fn shl(self, other: usize) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for processMode {
    type Output = processMode;
    fn shr(self, other: processMode) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for processMode {
    type Output = processMode;
    fn shr(self, other: i32) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for processMode {
    type Output = processMode;
    fn shr(self, other: i8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for processMode {
    type Output = processMode;
    fn shr(self, other: i16) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for processMode {
    type Output = processMode;
    fn shr(self, other: i64) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for processMode {
    type Output = processMode;
    fn shr(self, other: u32) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for processMode {
    type Output = processMode;
    fn shr(self, other: u8) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for processMode {
    type Output = processMode;
    fn shr(self, other: u16) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for processMode {
    type Output = processMode;
    fn shr(self, other: u64) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for processMode {
    type Output = processMode;
    fn shr(self, other: usize) -> processMode {
        processMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for processMode {}

impl Ord for processMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone, Default)]
pub struct processStatus(pub Arc<Mutex<Option<u64>>>);

impl Display for processStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for processStatus {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for processStatus {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for processStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for processStatus {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<processStatus> for u64 {
    fn eq(&self, other: &processStatus) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<processStatus> for u64 {
    fn partial_cmp(&self, other: &processStatus) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for processStatus {
    type Output = processStatus;
    fn add(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for processStatus {
    type Output = processStatus;
    fn add(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<processStatus> for u64 {
    type Output = processStatus;
    fn add(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for processStatus {
    type Output = processStatus;
    fn sub(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for processStatus {
    type Output = processStatus;
    fn sub(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<processStatus> for u64 {
    type Output = processStatus;
    fn sub(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for processStatus {
    type Output = processStatus;
    fn mul(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for processStatus {
    type Output = processStatus;
    fn mul(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<processStatus> for u64 {
    type Output = processStatus;
    fn mul(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for processStatus {
    type Output = processStatus;
    fn div(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for processStatus {
    type Output = processStatus;
    fn div(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<processStatus> for u64 {
    type Output = processStatus;
    fn div(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for processStatus {
    type Output = processStatus;
    fn rem(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for processStatus {
    type Output = processStatus;
    fn rem(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<processStatus> for u64 {
    type Output = processStatus;
    fn rem(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for processStatus {
    type Output = processStatus;
    fn bitand(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for processStatus {
    type Output = processStatus;
    fn bitand(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<processStatus> for u64 {
    type Output = processStatus;
    fn bitand(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for processStatus {
    type Output = processStatus;
    fn bitor(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for processStatus {
    type Output = processStatus;
    fn bitor(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<processStatus> for u64 {
    type Output = processStatus;
    fn bitor(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for processStatus {
    type Output = processStatus;
    fn bitxor(self, other: Self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for processStatus {
    type Output = processStatus;
    fn bitxor(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<processStatus> for u64 {
    type Output = processStatus;
    fn bitxor(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for processStatus {
    type Output = processStatus;
    fn not(self) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for processStatus {
    type Output = processStatus;
    fn shl(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for processStatus {
    type Output = processStatus;
    fn shl(self, other: i32) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for processStatus {
    type Output = processStatus;
    fn shl(self, other: i8) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for processStatus {
    type Output = processStatus;
    fn shl(self, other: i16) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for processStatus {
    type Output = processStatus;
    fn shl(self, other: i64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for processStatus {
    type Output = processStatus;
    fn shl(self, other: u32) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for processStatus {
    type Output = processStatus;
    fn shl(self, other: u8) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for processStatus {
    type Output = processStatus;
    fn shl(self, other: u16) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for processStatus {
    type Output = processStatus;
    fn shl(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for processStatus {
    type Output = processStatus;
    fn shl(self, other: usize) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for processStatus {
    type Output = processStatus;
    fn shr(self, other: processStatus) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for processStatus {
    type Output = processStatus;
    fn shr(self, other: i32) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for processStatus {
    type Output = processStatus;
    fn shr(self, other: i8) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for processStatus {
    type Output = processStatus;
    fn shr(self, other: i16) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for processStatus {
    type Output = processStatus;
    fn shr(self, other: i64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for processStatus {
    type Output = processStatus;
    fn shr(self, other: u32) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for processStatus {
    type Output = processStatus;
    fn shr(self, other: u8) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for processStatus {
    type Output = processStatus;
    fn shr(self, other: u16) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for processStatus {
    type Output = processStatus;
    fn shr(self, other: u64) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for processStatus {
    type Output = processStatus;
    fn shr(self, other: usize) -> processStatus {
        processStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for processStatus {}

impl Ord for processStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Signal represents an operating system signal.
/// The usual underlying implementation is operating system-dependent:
/// on Unix it is syscall.Signal.
pub trait Signal: std::fmt::Display + Any {
    fn __go_clone_box_signal(&self) -> Box<dyn Signal + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_signal(&self, other: &(dyn Signal + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
    fn signal(&self);
}

impl Clone for Box<dyn Signal + Send + Sync> {
    fn clone(&self) -> Self {
        Signal::__go_clone_box_signal(self.as_ref())
    }
}

pub static ErrProcessDone: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrProcessDone.lock().unwrap() = None;
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("os: process already finished".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrProcessDone.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrProcessDone.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_7() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("os: process already finished".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrProcessDone.lock().unwrap() = new_val; }
}


impl Signal for syscall::syscall_unix::Signal {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        self.string()
    }
    fn signal(&self) {
        self.signal()
    }
    fn __go_clone_box_signal(&self) -> Box<dyn Signal + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Signal + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_signal(&self, other: &(dyn Signal + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<syscall::syscall_unix::Signal>() {
            false
        } else {
            false
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
