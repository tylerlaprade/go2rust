use go2rust_stdlib_stubs::*;

use crate::r#mod::*;
use crate::iter::*;
use crate::reader::*;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SMALL_BUFFER_SIZE: i32 = 64;


pub(crate) const OP_READ: i8 = -1;
pub(crate) const OP_INVALID: i8 = 0;
pub(crate) const OP_READ_RUNE1: i8 = 1;
pub(crate) const OP_READ_RUNE2: i8 = 2;
pub(crate) const OP_READ_RUNE3: i8 = 3;
pub(crate) const OP_READ_RUNE4: i8 = 4;


pub(crate) const MAX_INT: i32 = i32::MAX;


pub const MIN_READ: i32 = 512;


/// The readOp constants describe the last action performed on
/// the buffer, so that UnreadRune and UnreadByte can check for
/// invalid usage. opReadRuneX constants are chosen such that
/// converted to int they correspond to the rune size that was read.
#[derive(Debug, Clone, Default)]
pub struct readOp(pub Arc<Mutex<Option<i8>>>);

impl Display for readOp {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for readOp {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i8> for readOp {
    fn eq(&self, other: &i8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for readOp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i8> for readOp {
    fn partial_cmp(&self, other: &i8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<readOp> for i8 {
    fn eq(&self, other: &readOp) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<readOp> for i8 {
    fn partial_cmp(&self, other: &readOp) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for readOp {
    type Output = readOp;
    fn add(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i8> for readOp {
    type Output = readOp;
    fn add(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<readOp> for i8 {
    type Output = readOp;
    fn add(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for readOp {
    type Output = readOp;
    fn sub(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i8> for readOp {
    type Output = readOp;
    fn sub(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<readOp> for i8 {
    type Output = readOp;
    fn sub(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for readOp {
    type Output = readOp;
    fn mul(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i8> for readOp {
    type Output = readOp;
    fn mul(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<readOp> for i8 {
    type Output = readOp;
    fn mul(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for readOp {
    type Output = readOp;
    fn div(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i8> for readOp {
    type Output = readOp;
    fn div(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<readOp> for i8 {
    type Output = readOp;
    fn div(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for readOp {
    type Output = readOp;
    fn neg(self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for readOp {
    type Output = readOp;
    fn rem(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i8> for readOp {
    type Output = readOp;
    fn rem(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<readOp> for i8 {
    type Output = readOp;
    fn rem(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for readOp {
    type Output = readOp;
    fn bitand(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i8> for readOp {
    type Output = readOp;
    fn bitand(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<readOp> for i8 {
    type Output = readOp;
    fn bitand(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for readOp {
    type Output = readOp;
    fn bitor(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i8> for readOp {
    type Output = readOp;
    fn bitor(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<readOp> for i8 {
    type Output = readOp;
    fn bitor(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for readOp {
    type Output = readOp;
    fn bitxor(self, other: Self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i8> for readOp {
    type Output = readOp;
    fn bitxor(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<readOp> for i8 {
    type Output = readOp;
    fn bitxor(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for readOp {
    type Output = readOp;
    fn not(self) -> readOp {
        readOp(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for readOp {
    type Output = readOp;
    fn shl(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for readOp {
    type Output = readOp;
    fn shl(self, other: i32) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for readOp {
    type Output = readOp;
    fn shl(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for readOp {
    type Output = readOp;
    fn shl(self, other: i16) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for readOp {
    type Output = readOp;
    fn shl(self, other: i64) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for readOp {
    type Output = readOp;
    fn shl(self, other: u32) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for readOp {
    type Output = readOp;
    fn shl(self, other: u8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for readOp {
    type Output = readOp;
    fn shl(self, other: u16) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for readOp {
    type Output = readOp;
    fn shl(self, other: u64) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for readOp {
    type Output = readOp;
    fn shl(self, other: usize) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for readOp {
    type Output = readOp;
    fn shr(self, other: readOp) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for readOp {
    type Output = readOp;
    fn shr(self, other: i32) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for readOp {
    type Output = readOp;
    fn shr(self, other: i8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for readOp {
    type Output = readOp;
    fn shr(self, other: i16) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for readOp {
    type Output = readOp;
    fn shr(self, other: i64) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for readOp {
    type Output = readOp;
    fn shr(self, other: u32) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for readOp {
    type Output = readOp;
    fn shr(self, other: u8) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for readOp {
    type Output = readOp;
    fn shr(self, other: u16) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for readOp {
    type Output = readOp;
    fn shr(self, other: u64) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for readOp {
    type Output = readOp;
    fn shr(self, other: usize) -> readOp {
        readOp(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for readOp {}

impl Ord for readOp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub static ErrTooLarge: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errNegativeRead: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errUnreadByte: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrTooLarge.lock().unwrap() = None;
    *errNegativeRead.lock().unwrap() = None;
    *errUnreadByte.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: too large".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrTooLarge.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: reader returned negative count from Read".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNegativeRead.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: UnreadByte: previous operation was not a successful read".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errUnreadByte.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrTooLarge.lock().unwrap() = None;
    *errNegativeRead.lock().unwrap() = None;
    *errUnreadByte.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: too large".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrTooLarge.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: reader returned negative count from Read".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errNegativeRead.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: UnreadByte: previous operation was not a successful read".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errUnreadByte.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
