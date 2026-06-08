use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_libc2::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
use crate::rlimit::*;
use crate::rlimit_darwin::*;
use crate::route_bsd::*;
use crate::route_darwin::*;
use crate::sockcmsg_unix::*;
use crate::sockcmsg_unix_other::*;
use crate::r#mod::*;
use crate::syscall_bsd::*;
use crate::syscall_darwin::*;
use crate::syscall_darwin_arm64::*;
use crate::time_nofake::*;
use crate::timestruct::*;
use crate::zerrors_darwin_arm64::*;
use crate::zsyscall_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;
use crate::ztypes_darwin_arm64::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DARWIN64_BIT: bool = (go_const_str_eq(runtime::G_O_O_S, "darwin") || go_const_str_eq(runtime::G_O_O_S, "ios")) && SIZEOF_PTR == 8;
pub(crate) const NETBSD32_BIT: bool = go_const_str_eq(runtime::G_O_O_S, "netbsd") && SIZEOF_PTR == 4;


#[derive(Clone)]
pub struct mmapper {
    pub mutex: sync::mutex::Mutex,
    pub active: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<u8>, Arc<Mutex<Option<Vec<u8>>>>>>>>,
    pub mmap: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>,
    pub munmap: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>,
}

impl mmapper {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), active: self.active.clone(), mmap: self.mmap.clone(), munmap: self.munmap.clone() }
    }
}


impl Default for mmapper {
    fn default() -> Self {
        Self { mutex: Default::default(), active: Arc::new(Mutex::new(None)), mmap: Arc::new(Mutex::new(None)), munmap: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for mmapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", "<map>", "<func>", "<func>")
    }
}

impl GoJsonDecode for mmapper {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An Errno is an unsigned number describing an error condition.
/// It implements the error interface. The zero Errno is by convention
/// a non-error, so code to convert from Errno to error should use:
///
///	err = nil
///	if errno != 0 {
///		err = errno
///	}
///
/// Errno values can be tested against error values using [errors.Is].
/// For example:
///
///	_, _, err := syscall.Syscall(...)
///	if errors.Is(err, fs.ErrNotExist) ...
#[derive(Debug, Clone, Default)]
pub struct Errno(pub Arc<Mutex<Option<usize>>>);

impl Display for Errno {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Errno {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for Errno {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Errno {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for Errno {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Errno> for usize {
    fn eq(&self, other: &Errno) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Errno> for usize {
    fn partial_cmp(&self, other: &Errno) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Errno {
    type Output = Errno;
    fn add(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for Errno {
    type Output = Errno;
    fn add(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Errno> for usize {
    type Output = Errno;
    fn add(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Errno {
    type Output = Errno;
    fn sub(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for Errno {
    type Output = Errno;
    fn sub(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Errno> for usize {
    type Output = Errno;
    fn sub(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Errno {
    type Output = Errno;
    fn mul(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for Errno {
    type Output = Errno;
    fn mul(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Errno> for usize {
    type Output = Errno;
    fn mul(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Errno {
    type Output = Errno;
    fn div(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for Errno {
    type Output = Errno;
    fn div(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Errno> for usize {
    type Output = Errno;
    fn div(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Errno {
    type Output = Errno;
    fn rem(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for Errno {
    type Output = Errno;
    fn rem(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Errno> for usize {
    type Output = Errno;
    fn rem(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Errno {
    type Output = Errno;
    fn bitand(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for Errno {
    type Output = Errno;
    fn bitand(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Errno> for usize {
    type Output = Errno;
    fn bitand(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Errno {
    type Output = Errno;
    fn bitor(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for Errno {
    type Output = Errno;
    fn bitor(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Errno> for usize {
    type Output = Errno;
    fn bitor(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Errno {
    type Output = Errno;
    fn bitxor(self, other: Self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for Errno {
    type Output = Errno;
    fn bitxor(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Errno> for usize {
    type Output = Errno;
    fn bitxor(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Errno {
    type Output = Errno;
    fn not(self) -> Errno {
        Errno(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Errno {
    type Output = Errno;
    fn shl(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Errno {
    type Output = Errno;
    fn shl(self, other: i32) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Errno {
    type Output = Errno;
    fn shl(self, other: i8) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Errno {
    type Output = Errno;
    fn shl(self, other: i16) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Errno {
    type Output = Errno;
    fn shl(self, other: i64) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Errno {
    type Output = Errno;
    fn shl(self, other: u32) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Errno {
    type Output = Errno;
    fn shl(self, other: u8) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Errno {
    type Output = Errno;
    fn shl(self, other: u16) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Errno {
    type Output = Errno;
    fn shl(self, other: u64) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Errno {
    type Output = Errno;
    fn shl(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Errno {
    type Output = Errno;
    fn shr(self, other: Errno) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Errno {
    type Output = Errno;
    fn shr(self, other: i32) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Errno {
    type Output = Errno;
    fn shr(self, other: i8) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Errno {
    type Output = Errno;
    fn shr(self, other: i16) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Errno {
    type Output = Errno;
    fn shr(self, other: i64) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Errno {
    type Output = Errno;
    fn shr(self, other: u32) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Errno {
    type Output = Errno;
    fn shr(self, other: u8) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Errno {
    type Output = Errno;
    fn shr(self, other: u16) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Errno {
    type Output = Errno;
    fn shr(self, other: u64) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Errno {
    type Output = Errno;
    fn shr(self, other: usize) -> Errno {
        Errno(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Errno {}

impl Ord for Errno {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A Signal is a number describing a process signal.
/// It implements the [os.Signal] interface.
#[derive(Debug, Clone, Default)]
pub struct Signal(pub Arc<Mutex<Option<i32>>>);

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Signal {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for Signal {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for Signal {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Signal> for i32 {
    fn eq(&self, other: &Signal) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Signal> for i32 {
    fn partial_cmp(&self, other: &Signal) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Signal {
    type Output = Signal;
    fn add(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for Signal {
    type Output = Signal;
    fn add(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Signal> for i32 {
    type Output = Signal;
    fn add(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Signal {
    type Output = Signal;
    fn sub(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for Signal {
    type Output = Signal;
    fn sub(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Signal> for i32 {
    type Output = Signal;
    fn sub(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Signal {
    type Output = Signal;
    fn mul(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for Signal {
    type Output = Signal;
    fn mul(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Signal> for i32 {
    type Output = Signal;
    fn mul(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Signal {
    type Output = Signal;
    fn div(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for Signal {
    type Output = Signal;
    fn div(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Signal> for i32 {
    type Output = Signal;
    fn div(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for Signal {
    type Output = Signal;
    fn neg(self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Signal {
    type Output = Signal;
    fn rem(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for Signal {
    type Output = Signal;
    fn rem(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Signal> for i32 {
    type Output = Signal;
    fn rem(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Signal {
    type Output = Signal;
    fn bitand(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for Signal {
    type Output = Signal;
    fn bitand(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Signal> for i32 {
    type Output = Signal;
    fn bitand(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Signal {
    type Output = Signal;
    fn bitor(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for Signal {
    type Output = Signal;
    fn bitor(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Signal> for i32 {
    type Output = Signal;
    fn bitor(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Signal {
    type Output = Signal;
    fn bitxor(self, other: Self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for Signal {
    type Output = Signal;
    fn bitxor(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Signal> for i32 {
    type Output = Signal;
    fn bitxor(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Signal {
    type Output = Signal;
    fn not(self) -> Signal {
        Signal(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Signal {
    type Output = Signal;
    fn shl(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Signal {
    type Output = Signal;
    fn shl(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Signal {
    type Output = Signal;
    fn shl(self, other: i8) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Signal {
    type Output = Signal;
    fn shl(self, other: i16) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Signal {
    type Output = Signal;
    fn shl(self, other: i64) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Signal {
    type Output = Signal;
    fn shl(self, other: u32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Signal {
    type Output = Signal;
    fn shl(self, other: u8) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Signal {
    type Output = Signal;
    fn shl(self, other: u16) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Signal {
    type Output = Signal;
    fn shl(self, other: u64) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Signal {
    type Output = Signal;
    fn shl(self, other: usize) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Signal {
    type Output = Signal;
    fn shr(self, other: Signal) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Signal {
    type Output = Signal;
    fn shr(self, other: i32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Signal {
    type Output = Signal;
    fn shr(self, other: i8) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Signal {
    type Output = Signal;
    fn shr(self, other: i16) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Signal {
    type Output = Signal;
    fn shr(self, other: i64) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Signal {
    type Output = Signal;
    fn shr(self, other: u32) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Signal {
    type Output = Signal;
    fn shr(self, other: u8) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Signal {
    type Output = Signal;
    fn shr(self, other: u16) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Signal {
    type Output = Signal;
    fn shr(self, other: u64) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Signal {
    type Output = Signal;
    fn shr(self, other: usize) -> Signal {
        Signal(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Signal {}

impl Ord for Signal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub trait Sockaddr: std::fmt::Display + Any {
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool;
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Sockaddr + Send + Sync> {
    fn clone(&self) -> Self {
        Sockaddr::__go_clone_box_sockaddr(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct SockaddrInet4 {
    pub port: Arc<Mutex<Option<i32>>>,
    pub addr: Arc<Mutex<Option<[u8; 4]>>>,
    pub raw: Arc<Mutex<Option<RawSockaddrInet4>>>,
}

impl SockaddrInet4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { port: { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, raw: { let __guard = self.raw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SockaddrInet4 {
    fn default() -> Self {
        Self { port: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(RawSockaddrInet4::default()))) }
    }
}

impl std::fmt::Display for SockaddrInet4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.port.lock().unwrap().as_ref().unwrap()), format_slice(&self.addr), (*self.raw.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SockaddrInet4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Port") {
            out.port = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addr") {
            out.addr = <Arc<Mutex<Option<[u8; 4]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct SockaddrInet6 {
    pub port: Arc<Mutex<Option<i32>>>,
    pub zone_id: Arc<Mutex<Option<u32>>>,
    pub addr: Arc<Mutex<Option<[u8; 16]>>>,
    pub raw: Arc<Mutex<Option<RawSockaddrInet6>>>,
}

impl SockaddrInet6 {
    pub fn __go_value_clone(&self) -> Self {
        Self { port: { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, zone_id: { let __guard = self.zone_id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, raw: { let __guard = self.raw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SockaddrInet6 {
    fn default() -> Self {
        Self { port: Arc::new(Mutex::new(Some(0))), zone_id: Arc::new(Mutex::new(Some(0))), addr: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), raw: Arc::new(Mutex::new(Some(RawSockaddrInet6::default()))) }
    }
}

impl std::fmt::Display for SockaddrInet6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.port.lock().unwrap().as_ref().unwrap()), (*self.zone_id.lock().unwrap().as_ref().unwrap()), format_slice(&self.addr), (*self.raw.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SockaddrInet6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Port") {
            out.port = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("ZoneId") {
            out.zone_id = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Addr") {
            out.addr = <Arc<Mutex<Option<[u8; 16]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct SockaddrUnix {
    pub name: Arc<Mutex<Option<String>>>,
    pub raw: Arc<Mutex<Option<RawSockaddrUnix>>>,
}

impl SockaddrUnix {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, raw: { let __guard = self.raw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SockaddrUnix {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), raw: Arc::new(Mutex::new(Some(RawSockaddrUnix::default()))) }
    }
}

impl std::fmt::Display for SockaddrUnix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.raw.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SockaddrUnix {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static Stdin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Stdout: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Stderr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errEAGAIN: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errEINVAL: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errENOENT: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static SocketDisableIPv6: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static ioSync: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Stdin.lock().unwrap() = Some(0);
    *Stdout.lock().unwrap() = Some(0);
    *Stderr.lock().unwrap() = Some(0);
    *errEAGAIN.lock().unwrap() = None;
    *errEINVAL.lock().unwrap() = None;
    *errENOENT.lock().unwrap() = None;
    *SocketDisableIPv6.lock().unwrap() = Some(false);
    *ioSync.lock().unwrap() = Some(0);
    *Stdin.lock().unwrap() = Some(0);
    *Stdout.lock().unwrap() = Some(1);
    *Stderr.lock().unwrap() = Some(2);
    *errEAGAIN.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_A_G_A_I_N as usize))))) as Box<dyn StdError + Send + Sync>);
    *errEINVAL.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>);
    *errENOENT.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_N_O_E_N_T as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_zero_globals() {
    *Stdin.lock().unwrap() = Some(0);
    *Stdout.lock().unwrap() = Some(0);
    *Stderr.lock().unwrap() = Some(0);
    *errEAGAIN.lock().unwrap() = None;
    *errEINVAL.lock().unwrap() = None;
    *errENOENT.lock().unwrap() = None;
    *SocketDisableIPv6.lock().unwrap() = Some(false);
    *ioSync.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_3() {
    *Stdin.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_4() {
    *Stdout.lock().unwrap() = Some(1);
}


pub(crate) fn __go_init_order_5() {
    *Stderr.lock().unwrap() = Some(2);
}


pub(crate) fn __go_init_order_6() {
    *errEAGAIN.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_A_G_A_I_N as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_7() {
    *errEINVAL.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>);
}


pub(crate) fn __go_init_order_8() {
    *errENOENT.lock().unwrap() = Some(Box::new(Errno(Arc::new(Mutex::new(Some(E_N_O_E_N_T as usize))))) as Box<dyn StdError + Send + Sync>);
}


impl mmapper {
    pub fn mmap(&mut self, fd: Arc<Mutex<Option<i32>>>, offset: Arc<Mutex<Option<i64>>>, length: Arc<Mutex<Option<i32>>>, prot: Arc<Mutex<Option<i32>>>, flags: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut data: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __tmp_x = { let __v = (*length.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        {
        *data.lock().unwrap() = None;;
        { let new_val = Box::new(Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>; *err.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (data.clone(), err.clone());
    }
    }
                        // Map the requested memory.
            let (mut addr, mut errno) = { let __f_holder = self.mmap.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = prot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
            if { let __nil_result = (*errno.lock().unwrap()).is_some(); __nil_result } {
        {
        *data.lock().unwrap() = None;;
        { let __rhs_holder = errno.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (data.clone(), err.clone());
    }
    }
                        // Use unsafe to turn addr into a []byte.
            let mut b = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result };
                        // Register mapping in m and return it.
            let mut p: Option<GoSliceElemPtr<u8>> = Some(GoSliceElemPtr::new(b.clone(), ({ let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize));
            self.mutex.lock();
            let mut m_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        m_defer_captured.mutex.unlock();
    }));
            { let __map_key = GoLocalPtrKey::from_slice_elem(p.clone()); let __map_value = b.clone(); (*self.active.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            {
        { let new_val = b.lock().unwrap().as_ref().unwrap().clone(); *data.lock().unwrap() = Some(new_val); };;
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (data.clone(), err.clone());
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
                (data.clone(), err.clone())
            }
        }
    }

    pub fn munmap(&self, data: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*data.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        {
        { let new_val = Box::new(Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>; *err.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
                        // Find the base of the mapping.
            let mut p: Option<GoSliceElemPtr<u8>> = Some(GoSliceElemPtr::new(data.clone(), ({ let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize));
            self.mutex.lock();
            let mut m_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        m_defer_captured.mutex.unlock();
    }));
            let mut b = { let __map = { let __map_holder = self.active.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::from_slice_elem(p.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
            if { let __nil_result = (*b.lock().unwrap()).is_none(); __nil_result } || { let __left = GoSliceElemPtr::new(b.clone(), (0) as usize); let __right = GoSliceElemPtr::new(data.clone(), (0) as usize); let __eq = Arc::ptr_eq(&__left.slice, &__right.slice) && __left.index == __right.index; !__eq } {
        {
        { let new_val = Box::new(Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>; *err.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
                        // Unmap the memory and update m.
            {
        let mut errno = { let __f_holder = self.munmap.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize)))) };;
        if { let __nil_result = (*errno.lock().unwrap()).is_some(); __nil_result } {
            {
        { let __rhs_holder = errno.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    };
        }
    }
            { let __map_handle = self.active.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::from_slice_elem(p.clone())); };
            {
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
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
                err.clone()
            }
        }
    }

    pub fn lock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.lock()
    }

    pub fn try_lock(&mut self) -> bool {
        let embedded_ref = &mut self.mutex;
        embedded_ref.try_lock()
    }

    pub fn unlock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.unlock()
    }
}

impl Errno {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = 0; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x <= __tmp_y } && { let __tmp_x = ((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 107; __tmp_x < __tmp_y } {
        let mut s = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = errors.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    }
        return Arc::new(Mutex::new(Some(format!("{}{}", "errno ".to_string(), (*internal_itoa::itoa(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32)))).lock().unwrap().as_ref().unwrap())))));
    }

    pub fn is(&self, target: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> bool {
        { let _switch_val = target.clone();
    if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __field = internal_oserror::ErrPermission.clone(); __field }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_A_C_C_E_S as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_P_E_R_M as usize)))); __tmp_x == __tmp_y };
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __field = internal_oserror::ErrExist.clone(); __field }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_E_X_I_S_T as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_N_O_T_E_M_P_T_Y as usize)))); __tmp_x == __tmp_y };
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __field = internal_oserror::ErrNotExist.clone(); __field }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_N_O_E_N_T as usize)))); __tmp_x == __tmp_y };
        } else if { let __left_holder = _switch_val.clone(); let __left_guard = __left_holder.lock().unwrap(); let __right_holder = { let __field = errors::ErrUnsupported.clone(); __field }; let __right_guard = __right_holder.lock().unwrap(); match (__left_guard.as_ref(), __right_guard.as_ref()) { (Some(__left), Some(__right)) => std::ptr::addr_eq(&**__left, &**__right), (None, None) => true, _ => false } } {
            return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_N_O_S_Y_S as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_N_O_T_S_U_P as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_O_P_N_O_T_S_U_P_P as usize)))); __tmp_x == __tmp_y };
        }
    }
        false
    }

    pub fn temporary(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_I_N_T_R as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_M_F_I_L_E as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_N_F_I_L_E as usize)))); __tmp_x == __tmp_y } || Errno::timeout(self);
    }

    pub fn timeout(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_A_G_A_I_N as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_W_O_U_L_D_B_L_O_C_K as usize)))); __tmp_x == __tmp_y } || { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Errno(Arc::new(Mutex::new(Some(E_T_I_M_E_D_O_U_T as usize)))); __tmp_x == __tmp_y };
    }
}

impl StdError for Errno {}


impl Signal {
    pub fn signal(&self) {
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = Signal(Arc::new(Mutex::new(Some(0 as i32)))); let __tmp_y = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = ((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = 32; __tmp_x < __tmp_y } {
        let mut str = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = signals.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        if { let __tmp_x = (*str.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        return { let __owned = str.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    }
        return Arc::new(Mutex::new(Some(format!("{}{}", "signal ".to_string(), (*internal_itoa::itoa(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32)))).lock().unwrap().as_ref().unwrap())))));
    }
}

impl SockaddrInet4 {
}

impl Sockaddr for SockaddrInet4 {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        SockaddrInet4::sockaddr(self)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrInet4>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SockaddrInet4Ptr(pub Arc<Mutex<Option<SockaddrInet4>>>);

impl std::fmt::Display for SockaddrInet4Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sockaddr for SockaddrInet4Ptr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        SockaddrInet4::sockaddr(__recv)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrInet4Ptr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SockaddrInet6 {
}

impl Sockaddr for SockaddrInet6 {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        SockaddrInet6::sockaddr(self)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrInet6>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SockaddrInet6Ptr(pub Arc<Mutex<Option<SockaddrInet6>>>);

impl std::fmt::Display for SockaddrInet6Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sockaddr for SockaddrInet6Ptr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        SockaddrInet6::sockaddr(__recv)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrInet6Ptr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl SockaddrUnix {
}

impl Sockaddr for SockaddrUnix {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        SockaddrUnix::sockaddr(self)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrUnix>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SockaddrUnixPtr(pub Arc<Mutex<Option<SockaddrUnix>>>);

impl std::fmt::Display for SockaddrUnixPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sockaddr for SockaddrUnixPtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        SockaddrUnix::sockaddr(__recv)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrUnixPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// clen returns the index of the first NULL byte in n or len(n) if n contains no NULL byte.
pub fn clen(n: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
    {
        let mut i = internal_bytealg::index_byte(n.clone(), Arc::new(Mutex::new(Some(0 as u8))));;
        if { let __tmp_x = i; let __tmp_y = -1; __tmp_x != __tmp_y } {
            return i;;
        }
    }
    (*n.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
}

/// errnoErr returns common boxed Errno values, to prevent
/// allocations at runtime.
pub fn errno_err(e: Arc<Mutex<Option<Errno>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    { let _switch_val = (*e.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (Errno(Arc::new(Mutex::new(Some(0 as usize))))) {
            return Arc::new(Mutex::new(None));
        } else if _switch_val == (Errno(Arc::new(Mutex::new(Some(E_A_G_A_I_N as usize))))) {
            return errEAGAIN.clone();
        } else if _switch_val == (Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) {
            return errEINVAL.clone();
        } else if _switch_val == (Errno(Arc::new(Mutex::new(Some(E_N_O_E_N_T as usize))))) {
            return errENOENT.clone();
        }
    }
    return Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>)));
}

pub fn read(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    { let (__tmp_0, __tmp_1) = read_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if internal_race::ENABLED {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_race::write_range(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        internal_race::acquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&ioSync.clone()) as usize))));
    }
    }
    if internal_msan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_msan::write(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if internal_asan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_asan::write(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn write(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if internal_race::ENABLED {
        internal_race::release_merge(Arc::new(Mutex::new(Some(Arc::as_ptr(&ioSync.clone()) as usize))));
    }
    if FAKETIME && ({ let __tmp_x = { let __v = (*fd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*fd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y }) {
        { let new_val = faketime_write(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *n.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __tmp_0 = 0; let __tmp_1 = errno_err(Arc::new(Mutex::new(Some(Errno(Arc::new(Mutex::new(Some(-((*n.lock().unwrap().as_ref().unwrap())) as usize)))))))); *n.lock().unwrap() = Some(__tmp_0); *err.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
    } else {
        { let (__tmp_0, __tmp_1) = write_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
    if internal_race::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_race::read_range(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if internal_msan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_msan::read(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if internal_asan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_asan::read(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn pread(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, offset: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    { let (__tmp_0, __tmp_1) = pread_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if internal_race::ENABLED {
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_race::write_range(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        internal_race::acquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&ioSync.clone()) as usize))));
    }
    }
    if internal_msan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_msan::write(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if internal_asan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_asan::write(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn pwrite(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, offset: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if internal_race::ENABLED {
        internal_race::release_merge(Arc::new(Mutex::new(Some(Arc::as_ptr(&ioSync.clone()) as usize))));
    }
    { let (__tmp_0, __tmp_1) = pwrite_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if internal_race::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_race::read_range(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if internal_msan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_msan::read(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    if internal_asan::ENABLED && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        internal_asan::read(Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as usize))));
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn getsockopt_int(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut value: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut vallen = Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(4 as u32)))))));
    { let __rhs_holder = getsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&n.clone()) as usize))), vallen.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    return ((*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn recvfrom(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut from: Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut rsa: Arc<Mutex<Option<RawSockaddrAny>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut len: Arc<Mutex<Option<_Socklen>>> = Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(SIZEOF_SOCKADDR_ANY as u32)))))));
    {
        { let (__tmp_0, __tmp_1) = recvfrom_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rsa.clone(), len.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return ((*n.lock().unwrap().as_ref().unwrap()), from.clone(), err.clone());;
        }
    }
    if { let __tmp_x = (*(*(*rsa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).family.lock().unwrap().as_ref().unwrap()); let __tmp_y = A_F__U_N_S_P_E_C as u8; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = any_to_sockaddr(GoPtr::local(rsa.clone())); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *from.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), from.clone(), err.clone());
}

pub fn recvmsg(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut oobn: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut recvflags: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut from: Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut rsa: Arc<Mutex<Option<RawSockaddrAny>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3) = recvmsg_raw(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), oob.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rsa.clone()); *n.lock().unwrap() = Some(__tmp_0); *oobn.lock().unwrap() = Some(__tmp_1); *recvflags.lock().unwrap() = Some(__tmp_2); let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3; };

        // source address is only specified if the socket is unconnected
    if { let __tmp_x = (*(*(*rsa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap().as_ref().unwrap()).family.lock().unwrap().as_ref().unwrap()); let __tmp_y = A_F__U_N_S_P_E_C as u8; __tmp_x != __tmp_y } {
        { let (__tmp_0, __tmp_1) = any_to_sockaddr(GoPtr::local(rsa.clone())); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *from.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), (*oobn.lock().unwrap().as_ref().unwrap()), (*recvflags.lock().unwrap().as_ref().unwrap()), from.clone(), err.clone());
}

pub fn sendmsg_n(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, to: Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut ptr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut salen: Arc<Mutex<Option<_Socklen>>> = Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0)))))));
    if { let __nil_result = (*to.lock().unwrap()).is_some(); __nil_result } {
        { let (__tmp_0, __tmp_1, __tmp_2) = (*to.lock().unwrap().as_mut().unwrap()).sockaddr(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *ptr.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *salen.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0, err.clone());
    }
    }
    return sendmsg_n_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), oob.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = salen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn sendto(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, to: Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut ptr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));let mut salen: Arc<Mutex<Option<_Socklen>>> = Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0)))))));
    if { let __nil_result = (*to.lock().unwrap()).is_some(); __nil_result } {
        { let (__tmp_0, __tmp_1, __tmp_2) = (*to.lock().unwrap().as_mut().unwrap()).sockaddr(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *ptr.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *salen.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    }
    return sendto_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ptr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = salen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn setsockopt_byte(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, value: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&value.clone()) as usize))), Arc::new(Mutex::new(Some(1 as usize))))
}

pub fn setsockopt_int(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, value: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut n = Arc::new(Mutex::new(Some((*value.lock().unwrap().as_ref().unwrap()) as i32)));
    return setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&n.clone()) as usize))), Arc::new(Mutex::new(Some(4 as usize))));
}

pub fn setsockopt_inet4_addr(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, value: Arc<Mutex<Option<[u8; 4]>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = value.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))), Arc::new(Mutex::new(Some(4 as usize))))
}

pub fn setsockopt_i_p_mreq(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, mreq: Arc<Mutex<Option<IPMreq>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&mreq) as usize))), Arc::new(Mutex::new(Some(SIZEOF_I_P_MREQ as usize))))
}

pub fn setsockopt_i_pv6_mreq(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, mreq: Arc<Mutex<Option<IPv6Mreq>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&mreq) as usize))), Arc::new(Mutex::new(Some(SIZEOF_I_PV6_MREQ as usize))))
}

pub fn setsockopt_linger(fd: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, opt: Arc<Mutex<Option<i32>>>, l: Arc<Mutex<Option<Linger>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    setsockopt(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = level.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = opt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Arc::as_ptr(&l) as usize))), Arc::new(Mutex::new(Some(SIZEOF_LINGER as usize))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mmapper {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SockaddrInet4 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SockaddrInet6 {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SockaddrUnix {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
