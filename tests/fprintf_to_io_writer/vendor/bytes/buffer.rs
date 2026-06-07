use go2rust_stdlib_stubs::*;

use crate::{go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::r#mod::*;
use crate::iter::*;
use crate::reader::*;

use std::any::Any;
use std::cell::{RefCell};
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


/// A Buffer is a variable-sized buffer of bytes with [Buffer.Read] and [Buffer.Write] methods.
/// The zero value for Buffer is an empty buffer ready to use.
#[derive(Debug, Clone)]
pub struct Buffer {
    pub buf: Arc<Mutex<Option<Vec<u8>>>>,
    pub off: Arc<Mutex<Option<i32>>>,
    pub last_read: Arc<Mutex<Option<readOp>>>,
}

impl Buffer {
    pub fn __go_value_clone(&self) -> Self {
        Self { buf: self.buf.clone(), off: { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, last_read: { let __guard = self.last_read.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Buffer {
    fn default() -> Self {
        Self { buf: Arc::new(Mutex::new(None)), off: Arc::new(Mutex::new(Some(0))), last_read: Arc::new(Mutex::new(Some(readOp(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Buffer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


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


impl Buffer {
    /// Bytes returns a slice of length b.Len() holding the unread portion of the buffer.
    /// The slice is valid for use only until the next buffer modification (that is,
    /// only until the next call to a method like [Buffer.Read], [Buffer.Write], [Buffer.Reset], or [Buffer.Truncate]).
    /// The slice aliases the buffer content at least until the next buffer modification,
    /// so immediate changes to the slice will affect the result of future reads.
    pub fn bytes(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }

    /// AvailableBuffer returns an empty buffer with b.Available() capacity.
    /// This buffer is intended to be appended to and
    /// passed to an immediately succeeding [Buffer.Write] call.
    /// The buffer is only valid until the next write operation on b.
    pub fn available_buffer(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }

    /// String returns the contents of the unread portion of the buffer
    /// as a string. If the [Buffer] is a nil pointer, it returns "<nil>".
    ///
    /// To build strings more efficiently, see the [strings.Builder] type.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if false {
                // Special case, useful in debugging.
        return Arc::new(Mutex::new(Some("<nil>".to_string())));
    }
                // Special case, useful in debugging.
        Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())))
    }

    /// empty reports whether the unread portion of the buffer is empty.
    pub fn empty(&self) -> bool {
        return { let __tmp_x = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ((*self.off.clone().lock().unwrap().as_ref().unwrap()) as i32); __tmp_x <= __tmp_y };
    }

    /// Len returns the number of bytes of the unread portion of the buffer;
    /// b.Len() == len(b.Bytes()).
    pub fn len(&self) -> i32 {
        return { let __tmp_x = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ((*self.off.clone().lock().unwrap().as_ref().unwrap()) as i32); __tmp_x - __tmp_y };
    }

    /// Cap returns the capacity of the buffer's underlying byte slice, that is, the
    /// total space allocated for the buffer's data.
    pub fn cap(&self) -> i32 {
        ({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32
    }

    /// Available returns how many bytes are unused in the buffer.
    pub fn available(&self) -> i32 {
        return { let __tmp_x = (({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x - __tmp_y };
    }

    /// Truncate discards all but the first n unread bytes from the buffer
    /// but continues to use the same allocated storage.
    /// It panics if n is negative or greater than the length of the buffer.
    pub fn truncate(&mut self, n: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.reset();
        return;
    }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = self.len(); __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("bytes.Buffer: truncation out of range".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
    }

    /// Reset resets the buffer to be empty,
    /// but it retains the underlying storage for use by future writes.
    /// Reset is the same as [Buffer.Truncate](0).
    pub fn reset(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (0) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
        { let new_val = 0; *self.off.lock().unwrap() = Some(new_val); };
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
    }

    /// tryGrowByReslice is an inlineable version of grow for the fast-case where the
    /// internal buffer only needs to be resliced.
    /// It returns the index where bytes should be written and whether it succeeded.
    pub fn try_grow_by_reslice(&mut self, n: Arc<Mutex<Option<i32>>>) -> (i32, bool) {
        {
        let mut l = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = (({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y } as i32); __tmp_x <= __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };;
            return ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);;
        }
    }
        (0, false)
    }

    /// grow grows the buffer to guarantee space for n more bytes.
    /// It returns the index where bytes should be written.
    /// If the buffer can't grow it will panic with ErrTooLarge.
    pub fn grow_1(&mut self, n: Arc<Mutex<Option<i32>>>) -> i32 {
        let mut m = self.len();
                // If buffer is empty, reset to recover space.
        if { let __tmp_x = m; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        self.reset();
    }
                // Try to grow by means of a reslice.
        {
        let (mut i, mut ok) = self.try_grow_by_reslice(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if ok {
            return i;;
        }
    }
        if { let __nil_target = self.buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x <= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut v = Vec::with_capacity((SMALL_BUFFER_SIZE) as usize); v.resize(({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize, 0); v }))); self.buf = new_val; };
        return 0;
    }
        let mut c = Arc::new(Mutex::new(Some(({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32)));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x / __tmp_y }; let __tmp_y = m; __tmp_x - __tmp_y }; __tmp_x <= __tmp_y } {
                // We can slide things down instead of allocating a new
                // slice. We only need m+n <= c to slide, but
                // we instead let capacity get twice as large so we
                // don't spend all our time copying.
        { let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min((*self.buf.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*self.buf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = i32::MAX; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }; __tmp_x > __tmp_y } {
        std::panic::panic_any({ let __err_holder = ErrTooLarge.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
    } else {
        { let new_val = grow_slice(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))); self.buf = new_val; };
    }
                // We can slide things down instead of allocating a new
                // slice. We only need m+n <= c to slide, but
                // we instead let capacity get twice as large so we
                // don't spend all our time copying.
                // Add b.off to account for b.buf[:b.off] being sliced off the front.
                // Restore b.off and len(b.buf).
        { let new_val = 0; *self.off.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = m; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
        m
    }

    /// Grow grows the buffer's capacity, if necessary, to guarantee space for
    /// another n bytes. After Grow(n), at least n bytes can be written to the
    /// buffer without another allocation.
    /// If n is negative, Grow will panic.
    /// If the buffer can't grow it will panic with [ErrTooLarge].
    pub fn grow(&mut self, n: Arc<Mutex<Option<i32>>>) {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("bytes.Buffer.Grow: negative count".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut m = self.grow_1(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (m) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
    }

    /// Write appends the contents of p to the buffer, growing the buffer as
    /// needed. The return value n is the length of p; err is always nil. If the
    /// buffer becomes too large, Write will panic with [ErrTooLarge].
    pub fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        let (mut m, mut ok) = self.try_grow_by_reslice(Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
        if !ok {
        { let new_val = self.grow_1(Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))); m = new_val; };
    }
        ((*{ let _dst_start = (m) as usize; let _dst_len = (*self.buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = { let __copy_src_holder = p.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)))
    }

    /// WriteString appends the contents of s to the buffer, growing the buffer as
    /// needed. The return value n is the length of s; err is always nil. If the
    /// buffer becomes too large, WriteString will panic with [ErrTooLarge].
    pub fn write_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        let (mut m, mut ok) = self.try_grow_by_reslice(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32))));
        if !ok {
        { let new_val = self.grow_1(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)))); m = new_val; };
    }
        ((*{ let _dst_start = (m) as usize; let _dst_len = (*self.buf.lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*self.buf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)))
    }

    /// ReadFrom reads data from r until EOF and appends it to the buffer, growing
    /// the buffer as needed. The return value n is the number of bytes read. Any
    /// error except io.EOF encountered during the read is also returned. If the
    /// buffer becomes too large, ReadFrom will panic with [ErrTooLarge].
    pub fn read_from(&mut self, r: Arc<Mutex<Option<io_Reader>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        loop {
        let mut i = self.grow_1(Arc::new(Mutex::new(Some(512))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
        let (mut m, mut e) = (*r.lock().unwrap().as_ref().unwrap()).read(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = (({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) })) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        if { let __tmp_x = m; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any({ let __err_holder = errNegativeRead.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
    }

        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = i; let __tmp_y = m; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.buf = new_val; };
        { let __rhs = (*Arc::new(Mutex::new(Some(m as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __left = e.clone(); let __right = io::EOF().clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }
                // e is EOF, so return nil explicitly
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, e.clone());
    }
    }
    }

    /// WriteTo writes data to w until the buffer is drained or an error occurs.
    /// The return value n is the number of bytes written; it always fits into an
    /// int, but it is int64 to match the [io.WriterTo] interface. Any error
    /// encountered during the write is also returned.
    pub fn write_to(&mut self, w: Arc<Mutex<Option<io_Writer>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        {
        let mut nBytes = self.len();;
        if { let __tmp_x = nBytes; let __tmp_y = 0; __tmp_x > __tmp_y } {
            let (mut m, mut e) = (*w.lock().unwrap().as_ref().unwrap()).write(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));;
            if { let __tmp_x = m; let __tmp_y = nBytes; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("bytes.Buffer.WriteTo: invalid Write count".to_string()) as Box<dyn Any + Send + Sync>);
    };
            { let __target = self.off.clone(); let __rhs = m; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
            { let new_val = Arc::new(Mutex::new(Some(m as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };;
            if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, e.clone());
    };
            if { let __tmp_x = m; let __tmp_y = nBytes; __tmp_x != __tmp_y } {
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __return_value_1 = io::ErrShortWrite().clone(); __return_value_1 });
    };
        }
    }
                // all bytes should have been written, by definition of
                // Write method in io.Writer
                // Buffer is now empty; reset.
        self.reset();
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// WriteByte appends the byte c to the buffer, growing the buffer as needed.
    /// The returned error is always nil, but is included to match [bufio.Writer]'s
    /// WriteByte. If the buffer becomes too large, WriteByte will panic with
    /// [ErrTooLarge].
    pub fn write_byte(&mut self, c: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        let (mut m, mut ok) = self.try_grow_by_reslice(Arc::new(Mutex::new(Some(1))));
        if !ok {
        { let new_val = self.grow_1(Arc::new(Mutex::new(Some(1)))); m = new_val; };
    }
        (*self.buf.lock().unwrap().as_mut().unwrap())[(m) as usize] = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
        return Arc::new(Mutex::new(None));
    }

    /// WriteRune appends the UTF-8 encoding of Unicode code point r to the
    /// buffer, returning its length and an error, which is always nil but is
    /// included to match [bufio.Writer]'s WriteRune. The buffer is grown as needed;
    /// if it becomes too large, WriteRune will panic with [ErrTooLarge].
    pub fn write_rune(&mut self, r: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // Compare as uint32 to correctly handle negative runes.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = unicode_utf8::RUNE_SELF as u32; __tmp_x < __tmp_y } {
        self.write_byte(Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))));
        return (1, Arc::new(Mutex::new(None)));
    }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        let (mut m, mut ok) = self.try_grow_by_reslice(Arc::new(Mutex::new(Some(unicode_utf8::U_T_F_MAX))));
        if !ok {
        { let new_val = self.grow_1(Arc::new(Mutex::new(Some(unicode_utf8::U_T_F_MAX)))); m = new_val; };
    }
        { let new_val = unicode_utf8::append_rune(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (m) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); self.buf = new_val; };
        return ({ let __tmp_x = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (m as i32); __tmp_x - __tmp_y }, Arc::new(Mutex::new(None)));
    }

    /// Read reads the next len(p) bytes from the buffer or until the buffer
    /// is drained. The return value n is the number of bytes read. If the
    /// buffer has no data to return, err is [io.EOF] (unless len(p) is zero);
    /// otherwise it is nil.
    pub fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        if self.empty() {
                // Buffer is empty, reset to recover space.
        self.reset();
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (0, Arc::new(Mutex::new(None)));
    }
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
                // Buffer is empty, reset to recover space.
        { let new_val = { let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min((*p.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*p.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        { let __target = self.off.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_READ as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
    }
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// Next returns a slice containing the next n bytes from the buffer,
    /// advancing the buffer as if the bytes had been returned by [Buffer.Read].
    /// If there are fewer than n bytes in the buffer, Next returns the entire buffer.
    /// The slice is only valid until the next call to a read or write method.
    pub fn next(&mut self, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        let mut m = self.len();
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = m; __tmp_x > __tmp_y } {
        { let new_val = m; *n.lock().unwrap() = Some(new_val); };
    }
        let mut data = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = ({ let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        { let __target = self.off.clone(); let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_READ as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
    }
        return data.clone();
    }

    /// ReadByte reads and returns the next byte from the buffer.
    /// If no byte is available, it returns error [io.EOF].
    pub fn read_byte(&mut self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if self.empty() {
                // Buffer is empty, reset to recover space.
        self.reset();
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
                // Buffer is empty, reset to recover space.
        let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        { let __target = self.off.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_READ as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        return ({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// ReadRune reads and returns the next UTF-8-encoded
    /// Unicode code point from the buffer.
    /// If no bytes are available, the error returned is io.EOF.
    /// If the bytes are an erroneous UTF-8 encoding, it
    /// consumes one byte and returns U+FFFD, 1.
    pub fn read_rune(&mut self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if self.empty() {
                // Buffer is empty, reset to recover space.
        self.reset();
        return (0, 0, { let __return_value_2 = io::EOF().clone(); __return_value_2 });
    }
                // Buffer is empty, reset to recover space.
        let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } {
        { let __target = self.off.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_READ_RUNE1 as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        return ((*Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()), 1, Arc::new(Mutex::new(None)));
    }
        let (__tmp_0, mut n) = unicode_utf8::decode_rune(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *r.lock().unwrap() = Some(__tmp_0);;
        { let __target = self.off.clone(); let __rhs = n; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = readOp(Arc::new(Mutex::new(Some(n as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        return ({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }, n, Arc::new(Mutex::new(None)));
    }

    /// UnreadRune unreads the last rune returned by [Buffer.ReadRune].
    /// If the most recent read or write operation on the buffer was
    /// not a successful [Buffer.ReadRune], UnreadRune returns an error.  (In this regard
    /// it is stricter than [Buffer.UnreadByte], which will unread the last byte
    /// from any read operation.)
    pub fn unread_rune(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = { let __selector_holder = self.last_read.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Buffer: UnreadRune: previous operation was not a successful ReadRune".to_string()))));
    }
        if { let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*self.last_read.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        { let __target = self.off.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*(*self.last_read.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }

    /// UnreadByte unreads the last byte returned by the most recent successful
    /// read operation that read at least one byte. If a write has happened since
    /// the last read, if the last read returned an error, or if the read read zero
    /// bytes, UnreadByte returns an error.
    pub fn unread_byte(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = { let __selector_holder = self.last_read.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); __tmp_x == __tmp_y } {
        return errUnreadByte.clone();
    }
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_INVALID as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __target = self.off.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Arc::new(Mutex::new(None));
    }

    /// ReadBytes reads until the first occurrence of delim in the input,
    /// returning a slice containing the data up to and including the delimiter.
    /// If ReadBytes encounters an error before finding a delimiter,
    /// it returns the data read before the error and the error itself (often [io.EOF]).
    /// ReadBytes returns err != nil if and only if the returned data does not end in
    /// delim.
    pub fn read_bytes(&mut self, delim: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut line: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let (mut slice, __tmp_1) = self.read_slice(Arc::new(Mutex::new(Some({ let __arg_holder = delim.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
                // return a copy of slice. The buffer's backing array may
                // be overwritten by later calls.
        { let new_val = { let __append_target = line.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = slice.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; line = new_val; };
        return (line.clone(), err.clone());
    }

    /// readSlice is like ReadBytes but returns a reference to internal buffer data.
    pub fn read_slice(&mut self, delim: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut line: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut i = index_byte(Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = delim.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut end = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*self.off.lock().unwrap().as_ref().unwrap()); let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y })));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = ({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *end.lock().unwrap() = Some(new_val); };
        { let __rhs_holder = io::EOF().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.off.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = ({ let __v = (*end.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); line = new_val; };
        { let new_val = end.lock().unwrap().as_ref().unwrap().clone(); *self.off.lock().unwrap() = Some(new_val); };
        { let new_val = readOp(Arc::new(Mutex::new(Some(OP_READ as i8)))); *self.last_read.lock().unwrap() = Some(new_val); };
        return (line.clone(), err.clone());
    }

    /// ReadString reads until the first occurrence of delim in the input,
    /// returning a string containing the data up to and including the delimiter.
    /// If ReadString encounters an error before finding a delimiter,
    /// it returns the data read before the error and the error itself (often [io.EOF]).
    /// ReadString returns err != nil if and only if the returned data does not end
    /// in delim.
    pub fn read_string(&mut self, delim: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut line: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let (mut slice, __tmp_1) = self.read_slice(Arc::new(Mutex::new(Some({ let __arg_holder = delim.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        return (Arc::new(Mutex::new(Some(String::from_utf8((*slice.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), err.clone());
    }
}

/// growSlice grows b by n, preserving the original content of b.
/// If the allocation fails, it panics with ErrTooLarge.
pub fn grow_slice(b: Arc<Mutex<Option<Vec<u8>>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        if { let __nil_result = (*go_recover().lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any({ let __err_holder = ErrTooLarge.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));

                // TODO(http://golang.org/issue/51462): We should rely on the append-make
                // pattern so that the compiler can call runtime.growslice. For example:
                //	return append(b, make([]byte, n)...)
                // This avoids unnecessary zero-ing of the first len(b) bytes of the
                // allocated slice, but this pattern causes b to escape onto the heap.
                //
                // Instead use the append-make pattern with a nil slice to ensure that
                // we allocate buffers rounded up to the closest size class.
        let mut c = Arc::new(Mutex::new(Some({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y })));
        if { let __tmp_x = ({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = 2; let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x * __tmp_y } as i32); __tmp_x < __tmp_y } {
                // The growth rate has historically always been 2x. In the future,
                // we could rely purely on append to determine the growth rate.
        { let new_val = { let __tmp_x = 2; let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x * __tmp_y }; *c.lock().unwrap() = Some(new_val); };
    }
                // The growth rate has historically always been 2x. In the future,
                // we could rely purely on append to determine the growth rate.
        let mut b2 = { let __append_target = Arc::new(Mutex::new(None)).clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))).clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() };
        let mut i = { let _src = { let __copy_src_holder = b.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*b2.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b2.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some({ let __seq_holder = b2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            Arc::new(Mutex::new(None))
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Buffer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
