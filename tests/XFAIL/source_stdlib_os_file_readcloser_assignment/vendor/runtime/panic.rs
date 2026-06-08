use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    debuglog::{print_debug_log},
    error::{errorAddressString, errorString, printindented},
    lock_spinbit::{lock, unlock},
    mfixalloc::{fixalloc},
    mheap::{mheap_},
    print::{hex, printlock, printunlock},
    proc::{freezetheworld, readgstatus, schedtrace},
    r#extern::{G_O_O_S},
    r#mod::{godebugInc},
    race0::{RACEENABLED, racereadpc},
    runtime1::{acquirem, debug, gotraceback, releasem},
    runtime2::{__GRUNNING, __GSCAN, _defer, _panic, g, gobuf, m, mutex, p, puintptr, savedOpenDeferState, sched, stack},
    security_issetugid::{is_secure_mode},
    signal_unix::{crash, signame},
    stkframe::{stkframe},
    stubs::{add, getg, gogo, mcall, noescape, systemstack},
    symtab::{findfunc, funcInfo, funcdata},
    sys_darwin::{exit},
    traceback::{goroutineheader, traceback, tracebackothers, unwindFlags, unwinder},
};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const THROW_TYPE_NONE: u32 = 0;
pub(crate) const THROW_TYPE_USER: u32 = 1;
pub(crate) const THROW_TYPE_RUNTIME: u32 = 2;


/// throwType indicates the current type of ongoing throw, which affects the
/// amount of detail printed to stderr. Higher values include more detail.
#[derive(Debug, Clone, Default)]
pub struct throwType(pub Arc<Mutex<Option<u32>>>);

impl Display for throwType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for throwType {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for throwType {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for throwType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for throwType {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<throwType> for u32 {
    fn eq(&self, other: &throwType) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<throwType> for u32 {
    fn partial_cmp(&self, other: &throwType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for throwType {
    type Output = throwType;
    fn add(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for throwType {
    type Output = throwType;
    fn add(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<throwType> for u32 {
    type Output = throwType;
    fn add(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for throwType {
    type Output = throwType;
    fn sub(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for throwType {
    type Output = throwType;
    fn sub(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<throwType> for u32 {
    type Output = throwType;
    fn sub(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for throwType {
    type Output = throwType;
    fn mul(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for throwType {
    type Output = throwType;
    fn mul(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<throwType> for u32 {
    type Output = throwType;
    fn mul(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for throwType {
    type Output = throwType;
    fn div(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for throwType {
    type Output = throwType;
    fn div(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<throwType> for u32 {
    type Output = throwType;
    fn div(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for throwType {
    type Output = throwType;
    fn rem(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for throwType {
    type Output = throwType;
    fn rem(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<throwType> for u32 {
    type Output = throwType;
    fn rem(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for throwType {
    type Output = throwType;
    fn bitand(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for throwType {
    type Output = throwType;
    fn bitand(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<throwType> for u32 {
    type Output = throwType;
    fn bitand(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for throwType {
    type Output = throwType;
    fn bitor(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for throwType {
    type Output = throwType;
    fn bitor(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<throwType> for u32 {
    type Output = throwType;
    fn bitor(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for throwType {
    type Output = throwType;
    fn bitxor(self, other: Self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for throwType {
    type Output = throwType;
    fn bitxor(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<throwType> for u32 {
    type Output = throwType;
    fn bitxor(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for throwType {
    type Output = throwType;
    fn not(self) -> throwType {
        throwType(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for throwType {
    type Output = throwType;
    fn shl(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for throwType {
    type Output = throwType;
    fn shl(self, other: i32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for throwType {
    type Output = throwType;
    fn shl(self, other: i8) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for throwType {
    type Output = throwType;
    fn shl(self, other: i16) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for throwType {
    type Output = throwType;
    fn shl(self, other: i64) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for throwType {
    type Output = throwType;
    fn shl(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for throwType {
    type Output = throwType;
    fn shl(self, other: u8) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for throwType {
    type Output = throwType;
    fn shl(self, other: u16) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for throwType {
    type Output = throwType;
    fn shl(self, other: u64) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for throwType {
    type Output = throwType;
    fn shl(self, other: usize) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for throwType {
    type Output = throwType;
    fn shr(self, other: throwType) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for throwType {
    type Output = throwType;
    fn shr(self, other: i32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for throwType {
    type Output = throwType;
    fn shr(self, other: i8) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for throwType {
    type Output = throwType;
    fn shr(self, other: i16) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for throwType {
    type Output = throwType;
    fn shr(self, other: i64) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for throwType {
    type Output = throwType;
    fn shr(self, other: u32) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for throwType {
    type Output = throwType;
    fn shr(self, other: u8) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for throwType {
    type Output = throwType;
    fn shr(self, other: u16) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for throwType {
    type Output = throwType;
    fn shr(self, other: u64) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for throwType {
    type Output = throwType;
    fn shr(self, other: usize) -> throwType {
        throwType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for throwType {}

impl Ord for throwType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static shiftError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static divideError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static overflowError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static floatError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static memoryError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rangeDoneError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rangePanicError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rangeExhaustedError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static rangeMissingPanicError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static panicnil: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::r#mod::godebugInc>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static runningPanicDefers: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static panicking: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static paniclk: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static didothers: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static deadlock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *shiftError.lock().unwrap() = None;
    *divideError.lock().unwrap() = None;
    *overflowError.lock().unwrap() = None;
    *floatError.lock().unwrap() = None;
    *memoryError.lock().unwrap() = None;
    *rangeDoneError.lock().unwrap() = None;
    *rangePanicError.lock().unwrap() = None;
    *rangeExhaustedError.lock().unwrap() = None;
    *rangeMissingPanicError.lock().unwrap() = None;
    *panicnil.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *runningPanicDefers.lock().unwrap() = Some(Default::default());
    *panicking.lock().unwrap() = Some(Default::default());
    *paniclk.lock().unwrap() = Some(Default::default());
    *didothers.lock().unwrap() = Some(false);
    *deadlock.lock().unwrap() = Some(Default::default());
    *shiftError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("negative shift amount".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *divideError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("integer divide by zero".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *overflowError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("integer overflow".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *floatError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("floating point error".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *memoryError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("invalid memory address or nil pointer dereference".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *rangeDoneError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after function for loop body returned false".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *rangePanicError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after loop body panic".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *rangeExhaustedError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after whole loop exit".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *rangeMissingPanicError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function recovered a loop body panic and did not resume panicking".to_string()))))) as Box<dyn StdError + Send + Sync>);
    *panicnil.lock().unwrap() = Some(Arc::new(Mutex::new(Some(godebugInc { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), ..Default::default() }))));
}


pub(crate) fn __go_zero_globals() {
    *shiftError.lock().unwrap() = None;
    *divideError.lock().unwrap() = None;
    *overflowError.lock().unwrap() = None;
    *floatError.lock().unwrap() = None;
    *memoryError.lock().unwrap() = None;
    *rangeDoneError.lock().unwrap() = None;
    *rangePanicError.lock().unwrap() = None;
    *rangeExhaustedError.lock().unwrap() = None;
    *rangeMissingPanicError.lock().unwrap() = None;
    *panicnil.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *runningPanicDefers.lock().unwrap() = Some(Default::default());
    *panicking.lock().unwrap() = Some(Default::default());
    *paniclk.lock().unwrap() = Some(Default::default());
    *didothers.lock().unwrap() = Some(false);
    *deadlock.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_43() {
    *shiftError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("negative shift amount".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_44() {
    *divideError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("integer divide by zero".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_45() {
    *overflowError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("integer overflow".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_46() {
    *floatError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("floating point error".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_47() {
    *memoryError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("invalid memory address or nil pointer dereference".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_48() {
    *rangeDoneError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after function for loop body returned false".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_49() {
    *rangePanicError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after loop body panic".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_50() {
    *rangeExhaustedError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function continued iteration after whole loop exit".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_51() {
    *rangeMissingPanicError.lock().unwrap() = Some(Box::new(errorString(Arc::new(Mutex::new(Some("range function recovered a loop body panic and did not resume panicking".to_string()))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_52() {
    *panicnil.lock().unwrap() = Some(Arc::new(Mutex::new(Some(godebugInc { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), ..Default::default() }))));
}


impl crate::runtime2::_panic {
    /// start initializes a panic to start unwinding the stack.
    ///
    /// If p.goexit is true, then start may return multiple times.
    pub fn start(&mut self, pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>) {
        let mut gp = getg();
                // Record the caller's PC and SP, so recovery can identify panics
                // that have been recovered. Also, so that if p is from Goexit, we
                // can restart its defer processing loop if a recovered panic tries
                // to jump past it.
        { let new_val = internal_runtime_sys::get_caller_p_c(); *self.start_p_c.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_s_p()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.start_s_p.lock().unwrap() = __moved_val; };
        if (*self.deferreturn.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = sp.lock().unwrap().as_ref().unwrap().clone(); *self.sp.lock().unwrap() = Some(new_val); };
        {
        let mut s: GoPtr<crate::runtime2::savedOpenDeferState> = GoPtr::raw({ let __ptr = (*gp.lock().unwrap().as_ref().unwrap()).param.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !s.is_nil() {
            *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = None;;
            { let new_val = { let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.retpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.retpc.lock().unwrap() = Some(new_val); };;
            { let new_val = GoPtr::local(Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.defer_bits_offset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()); self.defer_bits_ptr = new_val; };;
            { let new_val = add(Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.slots_offset.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.slots_ptr.lock().unwrap() = __moved_val; };;
        }
    }
                // recovery saved some state for us, so that we can resume
                // calling open-coded defers without unwinding the stack.
        return;
    }
                // recovery saved some state for us, so that we can resume
                // calling open-coded defers without unwinding the stack.
        { let new_val = (*gp.lock().unwrap().as_ref().unwrap())._panic.clone(); self.link = new_val; };
        { let new_val = Arc::new(Mutex::new({ let __ptr = noescape(Arc::new(Mutex::new(Some(self as *const _ as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<_panic>(unimplemented!("unsafe.Pointer conversion to _panic")) } })).clone(); (*gp.lock().unwrap().as_mut().unwrap())._panic = new_val; };
                // Initialize state machine, and find the first frame with a defer.
                //
                // Note: We could use startPC and startSP here, but callers will
                // never have defer statements themselves. By starting at their
                // caller instead, we avoid needing to unwind through an extra
                // frame. It also somewhat simplifies the terminating condition for
                // deferreturn.
        {
            let __tmp_0 = (*pc.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_1 = (*sp.lock().unwrap().as_ref().unwrap()).clone();
            *self.lr.lock().unwrap() = Some(__tmp_0);
            *self.fp.lock().unwrap() = Some(__tmp_1);
        };
        self.next_frame();
    }

    /// nextDefer returns the next deferred function to invoke, if any.
    ///
    /// Note: The "ok bool" result is necessary to correctly handle when
    /// the deferred function itself was nil (e.g., "defer (func())(nil)").
    pub fn next_defer(&mut self) -> (Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>, bool) {
        let mut gp = getg();

        if !(*self.deferreturn.clone().lock().unwrap().as_ref().unwrap()) {
        if { let __peer = (*gp.lock().unwrap().as_ref().unwrap())._panic.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq } {
        throw(Arc::new(Mutex::new(Some("bad panic stack".to_string()))));
    }
        if (*self.recovered.clone().lock().unwrap().as_ref().unwrap()) {
        mcall(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::runtime2::g>>>| { recovery(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::runtime2::g>>>) -> () + Send + Sync>))));
        throw(Arc::new(Mutex::new(Some("recovery failed".to_string()))));
    }
    }

                // does not return
                // The assembler adjusts p.argp in wrapper functions that shouldn't
                // be visible to recover(), so we need to restore it each iteration.
        { let new_val = add(Arc::new(Mutex::new(Some({ let __selector_holder = self.start_s_p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(internal_runtime_sys::MIN_FRAME_SIZE as usize)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.argp.lock().unwrap() = __moved_val; };

        loop {
        while { let __ptr_field = self.defer_bits_ptr.clone(); !__ptr_field.is_nil() } {
        let mut bits = Arc::new(Mutex::new(Some({ let __ptr_value = self.defer_bits_ptr.borrow(); __ptr_value.as_ref().unwrap().clone() })));

                // Check whether any open-coded defers are still pending.
                //
                // Note: We need to check this upfront (rather than after
                // clearing the top bit) because it's possible that Goexit
                // invokes a deferred call, and there were still more pending
                // open-coded defers in the frame; but then the deferred call
                // panic and invoked the remaining defers in the frame, before
                // recovering and restarting the Goexit loop.
        if { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let new_val = GoPtr::nil(); self.defer_bits_ptr = new_val; };
        break
    }

                // Find index of top bit set.
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = 7 as usize; let __tmp_y = (*Arc::new(Mutex::new(Some(internal_runtime_sys::leading_zeros8(Arc::new(Mutex::new(Some({ let __arg_holder = bits.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));

                // Clear bit and store it back.
        { let __rhs = { let __tmp_x = (1 as u8); let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let mut guard = bits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & ! __rhs); };
        { let new_val = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __ptr_target = self.defer_bits_ptr.clone(); __ptr_target.assign(Some(new_val)); };

        return (
            Arc::new(Mutex::new(Some(unimplemented!("unsafe.Pointer conversion to function value")))),
            true
        );
    }

                // Check whether any open-coded defers are still pending.
                //
                // Note: We need to check this upfront (rather than after
                // clearing the top bit) because it's possible that Goexit
                // invokes a deferred call, and there were still more pending
                // open-coded defers in the frame; but then the deferred call
                // panic and invoked the remaining defers in the frame, before
                // recovering and restarting the Goexit loop.
                // Find index of top bit set.
                // Clear bit and store it back.
        {
        let mut d = (*gp.lock().unwrap().as_ref().unwrap())._defer.clone();;
        if {
            let __go_cond_0 = { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*Arc::new(Mutex::new(Some((*self.sp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
                    __tmp_x == __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
            if (*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).rangefunc.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        deferconvert(d.clone());
        pop_defer(gp.clone());
        // TODO: unsupported goto recheck
    };
            let mut r#fn = (*d.lock().unwrap().as_ref().unwrap()).r#fn.clone();;
            { let new_val = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.retpc.lock().unwrap() = Some(new_val); };;
            pop_defer(gp.clone());;
            return (r#fn.clone(), true);;
        }
    }

                // TODO(mdempsky): Instead of having each deferproc call have
                // its own "deferreturn(); return" sequence, we should just make
                // them reuse the one we emit for open-coded defers.
                // Unlink and free.
        if !self.next_frame() {
        return (Arc::new(Mutex::new(None)), false);
    }
    }
        unreachable!()
    }

    /// nextFrame finds the next frame that contains deferred calls, if any.
    pub fn next_frame(&mut self) -> bool {
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        if { let __tmp_x = (*self.lr.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return false;
    }
        let mut gp = getg();
        let gp_closure_clone = gp.clone(); let mut ok_closure_clone = ok.clone(); let mut p_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut limit: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        {
        let mut d = (*gp_closure_clone.lock().unwrap().as_ref().unwrap())._defer.clone();;
        if { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *limit.lock().unwrap() = Some(new_val); };;
        }
    }
        let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __selector_holder = p_closure_clone.lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some((*p_closure_clone.fp.lock().unwrap().as_ref().unwrap()) as usize))),
            Arc::new(Mutex::new(Some(0 as usize))),
            GoPtr::local(gp_closure_clone.clone()),
            Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(0 as u8))))))),
        );
        loop {
        if !(*u.lock().unwrap().as_ref().unwrap()).valid() {
        { let new_val = 0 as usize; *p_closure_clone.lr.lock().unwrap() = Some(new_val); };
        return;
    }
        if { let __tmp_x = (*(*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*limit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        break
    }
        if p_closure_clone.init_open_coded_defers(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
        ) {
        break
    }
        (*u.lock().unwrap().as_mut().unwrap()).next();
    }
        { let new_val = { let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *p_closure_clone.lr.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p_closure_clone.sp.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *p_closure_clone.fp.lock().unwrap() = __moved_val; };
        { let new_val = true; *ok_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
                // ok == false
                // TODO(mdempsky): If we populate u.frame.fn.deferreturn for
                // every frame containing a defer (not just open-coded defers),
                // then we can simply loop until we find the next frame where
                // it's non-zero.
                // found a frame with linked defers
                // found a frame with open-coded defers
        return (*ok.lock().unwrap().as_ref().unwrap());
    }

    pub fn init_open_coded_defers(&mut self, r#fn: Arc<Mutex<Option<funcInfo>>>, varp: Arc<Mutex<Option<usize>>>) -> bool {
        let mut fd = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__OPEN_CODED_DEFER_INFO as u8))));
        if { let __nil_result = (*fd.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        if { let __tmp_x = (*(*(*r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).deferreturn.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("missing deferreturn".to_string()))));
    }
        let (mut deferBitsOffset, __tmp_1) = readvarint_unsafe(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *fd.lock().unwrap() = __moved_tmp_1;;
        let mut deferBitsPtr: GoPtr<u8> = GoPtr::raw({ let __ptr = add(
            Arc::new(Mutex::new(Some({ let __arg_holder = varp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some((deferBitsOffset as usize).wrapping_neg())))
        ).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if { let __tmp_x = { let __ptr_value = deferBitsPtr.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return false;
    }
                // has open-coded defers, but none pending
        let (mut slotsOffset, __tmp_1) = readvarint_unsafe(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *fd.lock().unwrap() = __moved_tmp_1;;
        { let new_val = {
            let __tmp_x = (*r#fn.lock().unwrap().as_ref().unwrap()).entry();
            let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*r#fn.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).deferreturn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        }; *self.retpc.lock().unwrap() = Some(new_val); };
        { let new_val = deferBitsPtr.clone(); self.defer_bits_ptr = new_val; };
        { let new_val = add(
            Arc::new(Mutex::new(Some({ let __arg_holder = varp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some((slotsOffset as usize).wrapping_neg())))
        ); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.slots_ptr.lock().unwrap() = __moved_val; };
        true
    }
}

/// Same as above, but calling from the runtime is allowed.
///
/// Using this function is necessary for any panic that may be
/// generated by runtime.sigpanic, since those are always called by the
/// runtime.
pub fn panic_check2(err: Arc<Mutex<Option<String>>>) {
        // panic allocates, so to avoid recursive malloc, turn panics
        // during malloc into throws.
    let mut gp = getg();
    if { let __nil_result = (*gp.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some({ let __arg_holder = err.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

///go:yeswritebarrierrec
pub fn panicdivide() {
    panic_check2(Arc::new(Mutex::new(Some("integer divide by zero".to_string()))));
    std::panic::panic_any({ let __err_holder = divideError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::error::TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
}

pub fn panicoverflow() {
    panic_check2(Arc::new(Mutex::new(Some("integer overflow".to_string()))));
    std::panic::panic_any({ let __err_holder = overflowError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::error::TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
}

pub fn panicfloat() {
    panic_check2(Arc::new(Mutex::new(Some("floating point error".to_string()))));
    std::panic::panic_any({ let __err_holder = floatError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::error::TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
}

pub fn panicmem() {
    panic_check2(Arc::new(Mutex::new(Some("invalid memory address or nil pointer dereference".to_string()))));
    std::panic::panic_any({ let __err_holder = memoryError.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::error::TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<crate::error::plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } });
}

pub fn panicmem_addr(addr: Arc<Mutex<Option<usize>>>) {
    panic_check2(Arc::new(Mutex::new(Some("invalid memory address or nil pointer dereference".to_string()))));
    std::panic::panic_any(Box::new(errorAddressString { msg: Arc::new(Mutex::new(Some("invalid memory address or nil pointer dereference".to_string()))), addr: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Box<dyn Any + Send + Sync>);
}

/// badDefer returns a fixed bad defer pointer for poisoning an atomic defer list head.
pub fn bad_defer() -> GoPtr<crate::runtime2::_defer> {
    GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(1 as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}

/// deferconvert converts the rangefunc defer list of d0 into an ordinary list
/// following d0.
/// See the doc comment for deferrangefunc for details.
pub fn deferconvert(d0: Arc<Mutex<Option<_defer>>>) {
    let mut head = (*d0.lock().unwrap().as_ref().unwrap()).head.clone();
    if RACEENABLED {
        racereadpc(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&head) as usize))),
            Arc::new(Mutex::new(Some(internal_runtime_sys::get_caller_p_c()))),
            Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(deferconvert.clone()) as Box<dyn Any + Send + Sync>)))))))
        );
    }
    let mut tail = (*d0.lock().unwrap().as_ref().unwrap()).link.clone();
    { let new_val = false; *(*d0.lock().unwrap().as_ref().unwrap()).rangefunc.lock().unwrap() = Some(new_val); };

    let mut d: Arc<Mutex<Option<_defer>>> = Arc::new(Mutex::new(None));
    loop {
        { let new_val = { let __recv = head.clone(); let __recv_ptr: *const internal_runtime_atomic::types::Pointer<crate::runtime2::_defer> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_runtime_atomic::types::Pointer<crate::runtime2::_defer> }; let __result = unsafe { &*__recv_ptr }.load(); __result }.clone(); d = new_val; };
        if { let __recv = head.clone(); let __recv_ptr: *const internal_runtime_atomic::types::Pointer<crate::runtime2::_defer> = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_runtime_atomic::types::Pointer<crate::runtime2::_defer> }; let __result = unsafe { &*__recv_ptr }.compare_and_swap(
            d.clone(),
            { let __go_ptr = bad_defer().clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } },
        ); __result } {
        break
    }
    }
    if { let __nil_result = (*d.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    let mut d1 = d.clone();
    loop {
        { let new_val = { let __selector_holder = (*d0.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*d1.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*d0.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*d1.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        if { let __nil_target = (*d1.lock().unwrap().as_ref().unwrap()).link.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = tail.clone(); (*d1.lock().unwrap().as_mut().unwrap()).link = new_val; };
        break
    }
        { let new_val = (*d1.lock().unwrap().as_ref().unwrap()).link.clone(); d1 = new_val; };
    }
    { let new_val = d.clone(); (*d0.lock().unwrap().as_mut().unwrap()).link = new_val; };
    ()
}

/// popDefer pops the head of gp's defer list and frees it.
pub fn pop_defer(gp: Arc<Mutex<Option<g>>>) {
    let mut d = (*gp.lock().unwrap().as_ref().unwrap())._defer.clone();
    *(*d.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = None;

        // We must not copy the stack between the updating gp._defer and setting
        // d.link to nil. Between these two steps, d is not on any defer list, so
        // stack copying won't adjust stack pointers in it (namely, d.link). Hence,
        // if we were to copy the stack, d could then contain a stale pointer.
    { let new_val = (*d.lock().unwrap().as_ref().unwrap()).link.clone(); (*gp.lock().unwrap().as_mut().unwrap())._defer = new_val; };
    *(*d.lock().unwrap().as_ref().unwrap()).link.lock().unwrap() = None;

        // After this point we can copy the stack.
    if !(*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).heap.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return;
    }

    let mut mp = acquirem();
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    if {
        let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
        let __tmp_y = (({ let __cap_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32);
        __tmp_x == __tmp_y
    } {
                // Transfer half of local cache to the central cache.
        let mut first: Arc<Mutex<Option<_defer>>> = Arc::new(Mutex::new(None));let mut last: Arc<Mutex<Option<_defer>>> = Arc::new(Mutex::new(None));
        while {
            let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32);
            let __tmp_y = ({ let __tmp_x = (({ let __cap_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = 2; __tmp_x / __tmp_y } as i32);
            __tmp_x > __tmp_y
        } {
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        let mut d = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = Default::default();
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); pp.with_mut(|__ptr_value| { __ptr_value.deferpool = new_val; }); };
        if { let __nil_result = (*first.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = d.clone(); first = new_val; };
    } else {
        { let new_val = d.clone(); (*last.lock().unwrap().as_mut().unwrap()).link = new_val; };
    }
        { let new_val = d.clone(); last = new_val; };
    }
        lock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).deferlock.clone()));
        { let new_val = (*sched.lock().unwrap().as_ref().unwrap()).deferpool.clone(); (*last.lock().unwrap().as_mut().unwrap()).link = new_val; };
        { let new_val = first.clone(); (*sched.lock().unwrap().as_mut().unwrap()).deferpool = new_val; };
        unlock(GoPtr::local((*sched.lock().unwrap().as_ref().unwrap()).deferlock.clone()));
    }

        // Transfer half of local cache to the central cache.
    { let new_val = _defer { heap: Arc::new(Mutex::new(Some(false))), rangefunc: Arc::new(Mutex::new(Some(false))), sp: Arc::new(Mutex::new(Some(0))), pc: Arc::new(Mutex::new(Some(0))), r#fn: Default::default(), link: Default::default(), head: Default::default() }; *d.lock().unwrap() = Some(new_val); };

    { let new_val = { let __append_target = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.deferpool.clone()); __ptr_value }.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(d.clone()); __append_target.clone() }; pp.with_mut(|__ptr_value| { __ptr_value.deferpool = new_val; }); };

    releasem(GoPtr::local(mp.clone()));
    {
        let __tmp_0 = None;
        let __tmp_1 = GoPtr::nil();
        *mp.lock().unwrap() = __tmp_0;
        pp = __tmp_1.clone();
    };
}

/// readvarintUnsafe reads the uint32 in varint format starting at fd, and returns the
/// uint32 and a pointer to the byte following the varint.
///
/// The implementation is the same with runtime.readvarint, except that this function
/// uses unsafe.Pointer for speed.
pub fn readvarint_unsafe(mut fd: Arc<Mutex<Option<usize>>>) -> (u32, Arc<Mutex<Option<usize>>>) {
    let mut r: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut shift: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut b = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = fd.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = add(
            Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<u8>())))
        ); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fd.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 128 as u8; __tmp_x < __tmp_y } {
        return (
            { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; __tmp_x + __tmp_y },
            { let __owned = fd.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }
        );
    }
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7F as u8; __tmp_x & __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31; __tmp_x & __tmp_y }); __tmp_x << __tmp_y }; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = 7; let mut guard = shift.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 28; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("Bad varint".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
}

/// throw triggers a fatal error that dumps a stack trace and exits.
///
/// throw should be used for runtime-internal fatal errors where Go itself,
/// rather than user code, may be at fault for the failure.
///
/// throw should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///   - github.com/cockroachdb/pebble
///   - github.com/dgraph-io/ristretto
///   - github.com/outcaste-io/ristretto
///   - github.com/pingcap/br
///   - gvisor.dev/gvisor
///   - github.com/sagernet/gvisor
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname throw
///go:nosplit
pub fn throw(s: Arc<Mutex<Option<String>>>) {
        // Everything throw does should be recursively nosplit so it
        // can be called even when it's unsafe to grow the stack.
    let s_closure_clone = s.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "fatal error: ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        printindented(Arc::new(Mutex::new(Some({ let __arg_holder = s_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // logically printpanicval(s), but avoids convTstring write barrier
    fatalthrow(Arc::new(Mutex::new(Some(throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))))))));
}

/// fatal triggers a fatal error that dumps a stack trace and exits.
///
/// fatal is equivalent to throw, but is used when user code is expected to be
/// at fault for the failure, such as racing map writes.
///
/// fatal does not include runtime frames, system goroutines, or frame metadata
/// (fp, sp, pc) in the stack trace unless GOTRACEBACK=system or higher.
///
///go:nosplit
pub fn fatal(s: Arc<Mutex<Option<String>>>) {
        // Everything fatal does should be recursively nosplit so it
        // can be called even when it's unsafe to grow the stack.
    printlock();
    let s_closure_clone = s.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        {
            let __go_print_arg_0 = format!("{}", "fatal error: ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        printindented(Arc::new(Mutex::new(Some({ let __arg_holder = s_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // logically printpanicval(s), but avoids convTstring write barrier
    fatalthrow(Arc::new(Mutex::new(Some(throwType(Arc::new(Mutex::new(Some(THROW_TYPE_USER as u32))))))));
    printunlock();
}

/// Unwind the stack after a deferred function calls recover
/// after a panic. Then arrange to continue running as though
/// the caller of the deferred function returned normally.
///
/// However, if unwinding the stack would skip over a Goexit call, we
/// return into the Goexit loop instead, so it can continue processing
/// defers instead.
pub fn recovery(gp: Arc<Mutex<Option<g>>>) {
    let mut p = (*gp.lock().unwrap().as_ref().unwrap())._panic.clone();
    let (mut pc, mut sp, mut fp) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).retpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_ref().unwrap()).fp.lock().unwrap().as_ref().unwrap()) as usize))));
    let (mut p0, mut saveOpenDeferState) = (p.clone(), Arc::new(Mutex::new(Some({ let __ptr_field = (*p.lock().unwrap().as_ref().unwrap()).defer_bits_ptr.clone(); !__ptr_field.is_nil() } && { let __tmp_x = { let __ptr_value = (*p.lock().unwrap().as_ref().unwrap()).defer_bits_ptr.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y }))));

        // Unwind the panic stack.
    while { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_ref().unwrap()).start_s_p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Don't allow jumping past a pending Goexit.
                // Instead, have its _panic.start() call return again.
                //
                // TODO(mdempsky): In this case, Goexit will resume walking the
                // stack where it left off, which means it will need to rewalk
                // frames that we've already processed.
                //
                // There's a similar issue with nested panics, when the inner
                // panic supersedes the outer panic. Again, we end up needing to
                // walk the same stack frames.
                //
                // These are probably pretty rare occurrences in practice, and
                // they don't seem any worse than the existing logic. But if we
                // move the unwinding state into _panic, we could detect when we
                // run into where the last panic started, and then just pick up
                // where it left off instead.
                //
                // With how subtle defer handling is, this might not actually be
                // worthwhile though.
        if (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).goexit.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
            let __tmp_0 = { let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).start_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_1 = Arc::new(Mutex::new(Some((*(*p.lock().unwrap().as_ref().unwrap()).start_s_p.lock().unwrap().as_ref().unwrap()) as usize)));
            *pc.lock().unwrap() = Some(__tmp_0);
            *sp.lock().unwrap() = __tmp_1.lock().unwrap().take();
        };
        { let new_val = false; *saveOpenDeferState.lock().unwrap() = Some(new_val); };
        break
    }

                // goexit is unwinding the stack anyway
        (*runningPanicDefers.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).link.clone(); p = new_val; };
    }
        // Don't allow jumping past a pending Goexit.
        // Instead, have its _panic.start() call return again.
        //
        // TODO(mdempsky): In this case, Goexit will resume walking the
        // stack where it left off, which means it will need to rewalk
        // frames that we've already processed.
        //
        // There's a similar issue with nested panics, when the inner
        // panic supersedes the outer panic. Again, we end up needing to
        // walk the same stack frames.
        //
        // These are probably pretty rare occurrences in practice, and
        // they don't seem any worse than the existing logic. But if we
        // move the unwinding state into _panic, we could detect when we
        // run into where the last panic started, and then just pick up
        // where it left off instead.
        //
        // With how subtle defer handling is, this might not actually be
        // worthwhile though.
        // goexit is unwinding the stack anyway
    { let new_val = p.clone(); (*gp.lock().unwrap().as_mut().unwrap())._panic = new_val; };

    if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = 0 as u32; *(*gp.lock().unwrap().as_ref().unwrap()).sig.lock().unwrap() = Some(new_val); };
    }

    if { let __nil_target = (*gp.lock().unwrap().as_ref().unwrap()).param.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        throw(Arc::new(Mutex::new(Some("unexpected gp.param".to_string()))));
    }
    if { let __v = (*saveOpenDeferState.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // If we're returning to deferreturn and there are more open-coded
                // defers for it to call, save enough state for it to be able to
                // pick up where p0 left off.
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(savedOpenDeferState { retpc: Arc::new(Mutex::new(Some({ let __selector_holder = (*p0.lock().unwrap().as_ref().unwrap()).retpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), defer_bits_offset: Arc::new(Mutex::new(Some({
            let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some((*p0.lock().unwrap().as_ref().unwrap()).defer_bits_ptr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*p0.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        }))), slots_offset: Arc::new(Mutex::new(Some({
            let __tmp_x = (*Arc::new(Mutex::new(Some((*(*p0.lock().unwrap().as_ref().unwrap()).slots_ptr.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some((*(*p0.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x - __tmp_y
        }))), ..Default::default() })))) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*gp.lock().unwrap().as_ref().unwrap()).param.lock().unwrap() = __moved_val; };
    }

        // If we're returning to deferreturn and there are more open-coded
        // defers for it to call, save enough state for it to be able to
        // pick up where p0 left off.
        // We need to save deferBitsPtr and slotsPtr too, but those are
        // stack pointers. To avoid issues around heap objects pointing
        // to the stack, save them as offsets from SP.
        // TODO(mdempsky): Currently, we rely on frames containing "defer"
        // to end with "CALL deferreturn; RET". This allows deferreturn to
        // finish running any pending defers in the frame.
        //
        // But we should be able to tell whether there are still pending
        // defers here. If there aren't, we can just jump directly to the
        // "RET" instruction. And if there are, we don't need an actual
        // "CALL deferreturn" instruction; we can simulate it with something
        // like:
        //
        //	if usesLR {
        //		lr = pc
        //	} else {
        //		sp -= sizeof(pc)
        //		*(*uintptr)(sp) = pc
        //	}
        //	pc = funcPC(deferreturn)
        //
        // So that we effectively tail call into deferreturn, such that it
        // then returns to the simple "RET" epilogue. That would save the
        // overhead of the "deferreturn" call when there aren't actually any
        // pending defers left, and shrink the TEXT size of compiled
        // binaries. (Admittedly, both of these are modest savings.)
        // Ensure we're recovering within the appropriate stack.
    if { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && ({ let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y }) {
        {
            let __go_print_arg_0 = format!("{}", "recover: ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*sp.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", " not in [".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", ", ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        throw(Arc::new(Mutex::new(Some("bad recovery".to_string()))));
    }

        // Make the deferproc for this d return again,
        // this time returning 1. The calling function will
        // jump to the standard return epilogue.
    { let new_val = sp.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
    { let new_val = pc.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as usize; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };

        // Restore the bp on platforms that support frame pointers.
        // N.B. It's fine to not set anything for platforms that don't
        // support frame pointers, since nothing consumes them.
    if { let __tmp_x = internal_goarch::IS_AMD64; let __tmp_y = 0; __tmp_x != __tmp_y } {
                        // on x86, fp actually points one word higher than the top of
                        // the frame since the return address is saved on the stack by
                        // the caller
            { let new_val = { let __tmp_x = { let __v = (*fp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((2 as usize) * (internal_goarch::PTR_SIZE as usize)) as usize; __tmp_x - __tmp_y }; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).bp.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = internal_goarch::IS_ARM64; let __tmp_y = 0; __tmp_x != __tmp_y } {
                        // on arm64, the architectural bp points one word higher
                        // than the sp. fp is totally useless to us here, because it
                        // only gets us to the caller's fp.
            { let new_val = { let __tmp_x = { let __v = (*sp.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x - __tmp_y }; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).bp.lock().unwrap() = Some(new_val); };
        }

        // on x86, fp actually points one word higher than the top of
        // the frame since the return address is saved on the stack by
        // the caller
        // on arm64, the architectural bp points one word higher
        // than the sp. fp is totally useless to us here, because it
        // only gets us to the caller's fp.
        // The value in ret is delivered IN A REGISTER, even if there is a
        // stack ABI.
    { let new_val = 1 as usize; *(*(*gp.lock().unwrap().as_ref().unwrap()).sched.lock().unwrap().as_ref().unwrap()).ret.lock().unwrap() = Some(new_val); };
    gogo((*gp.lock().unwrap().as_ref().unwrap()).sched.clone());
}

/// fatalthrow implements an unrecoverable runtime throw. It freezes the
/// system, prints stack traces starting from its caller, and terminates the
/// process.
///
///go:nosplit
pub fn fatalthrow(t: Arc<Mutex<Option<throwType>>>) {
    let mut pc = internal_runtime_sys::get_caller_p_c();
    let mut sp = internal_runtime_sys::get_caller_s_p();
    let mut gp = getg();

    if {
        let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = throwType(Arc::new(Mutex::new(Some(THROW_TYPE_NONE as u32))));
        __tmp_x == __tmp_y
    } {
        { let new_val = t.lock().unwrap().as_ref().unwrap().clone(); *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.lock().unwrap() = Some(new_val); };
    }

        // Switch to the system stack to avoid any stack growth, which may make
        // things worse if the runtime is in a bad state.
    let gp_closure_clone = gp.clone(); let pc_closure_clone = pc.clone(); let sp_closure_clone = sp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        if is_secure_mode() {
        exit(Arc::new(Mutex::new(Some(2 as i32))));
    }
        startpanic_m();
        if dopanic_m(gp_closure_clone.clone(), Arc::new(Mutex::new(Some(pc_closure_clone))), Arc::new(Mutex::new(Some(sp_closure_clone)))) {
        crash();
    }
        exit(Arc::new(Mutex::new(Some(2 as i32))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

        // crash uses a decent amount of nosplit stack and we're already
        // low on stack in throw, so crash on the system stack (unlike
        // fatalpanic).
    { let new_val = 0; *Arc::new(Mutex::new(None::<i32>)).lock().unwrap() = Some(new_val); };
}

/// startpanic_m prepares for an unrecoverable panic.
///
/// It returns true if panic messages should be printed, or false if
/// the runtime is in bad shape and should just print stacks.
///
/// It must not have write barriers even though the write barrier
/// explicitly ignores writes once dying > 0. Write barriers still
/// assume that g.m.p != nil, and this function may not have P
/// in some contexts (e.g. a panic in a signal handler for a signal
/// sent to an M with no P).
///
///go:nowritebarrierrec
pub fn startpanic_m() -> bool {
    let mut gp = getg();
    if { let __tmp_x = (*(*(*mheap_.lock().unwrap().as_ref().unwrap()).cachealloc.lock().unwrap().as_ref().unwrap()).size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: panic before malloc heap initialized\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }

        // Disallow malloc during an unrecoverable panic. A panic
        // could happen in a signal handler, or in a throw, or inside
        // malloc itself. We want to catch if an allocation ever does
        // happen (even if we're not in one of these situations).
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

        // If we're dying because of a bad lock count, set it to a
        // good lock count so we don't recursively panic below.
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let new_val = 1 as i32; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap() = Some(new_val); };
    }

    {
        let _switch_val = { let __v = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == 0 as i32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Setting dying >0 has the side-effect of disabling this G's writebuf.
            { let new_val = 1 as i32; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap() = Some(new_val); };
            (*panicking.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));
            lock(GoPtr::local(paniclk.clone()));
            if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).schedtrace.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } || { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).scheddetail.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        schedtrace(Arc::new(Mutex::new(Some(true))));
    }
            freezetheworld();
            return true;
        }
        if !_matched && (_switch_val == 1 as i32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Something failed while panicking.
                        // Just print a stack trace and exit.
            { let new_val = 2 as i32; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap() = Some(new_val); };
            {
            let __go_print_arg_0 = format!("{}", "panic during panic\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
            return false;
        }
        if !_matched && (_switch_val == 2 as i32) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // This is a genuine bug in the runtime, we couldn't even
                        // print the stack trace successfully.
            { let new_val = 3 as i32; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).dying.lock().unwrap() = Some(new_val); };
            {
            let __go_print_arg_0 = format!("{}", "stack trace unavailable\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
            exit(Arc::new(Mutex::new(Some(4 as i32))));
            _fallthrough = true;
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // Can't even print! Just exit.
            exit(Arc::new(Mutex::new(Some(5 as i32))));
            return false;
        }
        unreachable!()
    }
}

/// gp is the crashing g running on this M, but may be a user G, while getg() is
/// always g0.
pub fn dopanic_m(gp: Arc<Mutex<Option<g>>>, pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>) -> bool {
    if { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        let mut signame = signame(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = (*signame.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "[signal ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*signame.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "[signal ".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sig.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", " code=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode0.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", " addr=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigcode1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " pc=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*gp.lock().unwrap().as_ref().unwrap()).sigpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "]\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
    }

    let (mut level, mut all, mut docrash) = gotraceback();
    if { let __tmp_x = level; let __tmp_y = 0 as i32; __tmp_x > __tmp_y } {
        if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        { let new_val = true; all = new_val; };
    }
        if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        goroutineheader(GoPtr::local(gp.clone()));
        traceback(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), GoPtr::local(gp.clone()));
    } else if {
        let __go_cond_0 = { let __tmp_x = level; let __tmp_y = 2 as i32; __tmp_x >= __tmp_y };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = {
                let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))));
                __tmp_x >= __tmp_y
            };
            __go_cond_1
        }
    } {
        {
            let __go_print_arg_0 = format!("{}", "\nruntime stack:\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        traceback(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), GoPtr::local(gp.clone()));
    }
        if !(*didothers.lock().unwrap().as_ref().unwrap()) && all {
        { let new_val = true; *didothers.lock().unwrap() = Some(new_val); };
        tracebackothers(GoPtr::local(gp.clone()));
    }
    }
    unlock(GoPtr::local(paniclk.clone()));

    if { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
                // Some other m is panicking too.
                // Let it print what it needs to print.
                // Wait forever without chewing up cpu.
                // It will exit when it's done.
        lock(GoPtr::local(deadlock.clone()));
        lock(GoPtr::local(deadlock.clone()));
    }

        // Some other m is panicking too.
        // Let it print what it needs to print.
        // Wait forever without chewing up cpu.
        // It will exit when it's done.
    print_debug_log();

    docrash
}

/// canpanic returns false if a signal should throw instead of
/// panicking.
///
///go:nosplit
pub fn canpanic() -> bool {
    let mut gp = getg();
    let mut mp = acquirem();

        // Is it okay for gp to panic instead of crashing the program?
        // Yes, as long as it is running Go code, not runtime code,
        // and not stuck in a system call.
    if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        releasem(GoPtr::local(mp.clone()));
        return false;
    }

        // N.B. mp.locks != 1 instead of 0 to account for acquirem.
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = {
                    let __go_cond_3 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).locks.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x != __tmp_y };
                    if __go_cond_3 {
                        true
                    } else {
                        let __go_cond_4 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).mallocing.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y };
                        __go_cond_4
                    }
                };
                if __go_cond_2 {
                    true
                } else {
                    let __go_cond_5 = {
                        let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = throwType(Arc::new(Mutex::new(Some(THROW_TYPE_NONE as u32))));
                        __tmp_x != __tmp_y
                    };
                    __go_cond_5
                }
            };
            if __go_cond_1 {
                true
            } else {
                let __go_cond_6 = { let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).preemptoff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y };
                __go_cond_6
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_7 = { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).dying.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y };
            __go_cond_7
        }
    } {
        releasem(GoPtr::local(mp.clone()));
        return false;
    }
    let mut status = readgstatus(GoPtr::local(gp.clone()));
    if { let __tmp_x = { let __tmp_x = status; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).syscallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        releasem(GoPtr::local(mp.clone()));
        return false;
    }
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*mp.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        releasem(GoPtr::local(mp.clone()));
        return false;
    }
    releasem(GoPtr::local(mp.clone()));
    true
}

/// shouldPushSigpanic reports whether pc should be used as sigpanic's
/// return PC (pushing a frame for the call). Otherwise, it should be
/// left alone so that LR is used as sigpanic's return PC, effectively
/// replacing the top-most frame with sigpanic. This is used by
/// preparePanic.
pub fn should_push_sigpanic(gp: GoPtr<crate::runtime2::g>, pc: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>) -> bool {
    if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Probably a call to a nil func. The old LR is more
                // useful in the stack trace. Not pushing the frame
                // will make the trace look like a call to sigpanic
                // instead. (Otherwise the trace will end at sigpanic
                // and we won't get to see who faulted.)
        return false;
    }

        // Probably a call to a nil func. The old LR is more
        // useful in the stack trace. Not pushing the frame
        // will make the trace look like a call to sigpanic
        // instead. (Otherwise the trace will end at sigpanic
        // and we won't get to see who faulted.)
        // If we don't recognize the PC as code, but we do recognize
        // the link register as code, then this assumes the panic was
        // caused by a call to non-code. In this case, we want to
        // ignore this call to make unwinding show the context.
        //
        // If we running C code, we're not going to recognize pc as a
        // Go function, so just assume it's good. Otherwise, traceback
        // may try to read a stale LR that looks like a Go code
        // pointer and wander into the woods.
    if (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).incgo.lock().unwrap().as_ref().unwrap()) || { let __recv = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).valid(); __result } {
                // This wasn't a bad call, so use PC as sigpanic's
                // return PC.
        return true;
    }
        // This wasn't a bad call, so use PC as sigpanic's
        // return PC.
    if { let __recv = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = lr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).valid(); __result } {
                // This was a bad call, but the LR is good, so use the
                // LR as sigpanic's return PC.
        return false;
    }

        // This was a bad call, but the LR is good, so use the
        // LR as sigpanic's return PC.
        // Neither the PC or LR is good. Hopefully pushing a frame
        // will work.
    true
}

/// isAbortPC reports whether pc is the program counter at which
/// runtime.abort raises a signal.
///
/// It is nosplit because it's part of the isgoexception
/// implementation.
///
///go:nosplit
pub fn is_abort_p_c(pc: Arc<Mutex<Option<usize>>>) -> bool {
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return false;
    }
    return {
        let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_ABORT as u8))));
        __tmp_x == __tmp_y
    };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
