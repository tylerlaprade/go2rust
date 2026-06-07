use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::alg::*;
use crate::arena::*;
use crate::asan0::*;
use crate::atomic_pointer::*;
use crate::badlinkname::*;
use crate::cgo::*;
use crate::cgocall::*;
use crate::cgocallback::*;
use crate::cgocheck::*;
use crate::chan::*;
use crate::checkptr::*;
use crate::compiler::*;
use crate::complex::*;
use crate::coro::*;
use crate::covercounter::*;
use crate::covermeta::*;
use crate::cpuflags::*;
use crate::cpuflags_arm64::*;
use crate::cpuprof::*;
use crate::create_file_unix::*;
use crate::debug::*;
use crate::debugcall::*;
use crate::debuglog::*;
use crate::debuglog_off::*;
use crate::defs_darwin_arm64::*;
use crate::env_posix::*;
use crate::r#extern::*;
use crate::fastlog2::*;
use crate::fastlog2table::*;
use crate::fds_unix::*;
use crate::float::*;
use crate::hash64::*;
use crate::heapdump::*;
use crate::histogram::*;
use crate::iface::*;
use crate::lfstack::*;
use crate::linkname::*;
use crate::linkname_swiss::*;
use crate::linkname_unix::*;
use crate::lock_sema::*;
use crate::lock_spinbit::*;
use crate::lockrank::*;
use crate::lockrank_off::*;
use crate::malloc::*;
use crate::map_fast32_swiss::*;
use crate::map_fast64_swiss::*;
use crate::map_faststr_swiss::*;
use crate::map_swiss::*;
use crate::mbarrier::*;
use crate::mbitmap::*;
use crate::mcache::*;
use crate::mcentral::*;
use crate::mcheckmark::*;
use crate::mcleanup::*;
use crate::mem::*;
use crate::mem_darwin::*;
use crate::mem_nonsbrk::*;
use crate::metrics::*;
use crate::mfinal::*;
use crate::mfixalloc::*;
use crate::mgc::*;
use crate::mgclimit::*;
use crate::mgcmark::*;
use crate::mgcpacer::*;
use crate::mgcscavenge::*;
use crate::mgcstack::*;
use crate::mgcsweep::*;
use crate::mgcwork::*;
use crate::mheap::*;
use crate::minmax::*;
use crate::mpagealloc::*;
use crate::mpagealloc_64bit::*;
use crate::mpagecache::*;
use crate::mpallocbits::*;
use crate::mprof::*;
use crate::mranges::*;
use crate::msan0::*;
use crate::msize::*;
use crate::mspanset::*;
use crate::mstats::*;
use crate::mwbbuf::*;
use crate::nbpipe_pipe::*;
use crate::netpoll::*;
use crate::netpoll_kqueue::*;
use crate::netpoll_kqueue_event::*;
use crate::nonwindows_stub::*;
use crate::note_other::*;
use crate::os_darwin::*;
use crate::os_darwin_arm64::*;
use crate::os_nonopenbsd::*;
use crate::os_unix::*;
use crate::os_unix_nonlinux::*;
use crate::panic::*;
use crate::pinner::*;
use crate::plugin::*;
use crate::preempt::*;
use crate::preempt_nonwindows::*;
use crate::print::*;
use crate::proc::*;
use crate::profbuf::*;
use crate::proflabel::*;
use crate::race0::*;
use crate::rand::*;
use crate::rdebug::*;
use crate::retry::*;
use crate::r#mod::*;
use crate::runtime1::*;
use crate::runtime2::*;
use crate::runtime_boring::*;
use crate::rwmutex::*;
use crate::security_issetugid::*;
use crate::security_unix::*;
use crate::select::*;
use crate::sema::*;
use crate::signal_arm64::*;
use crate::signal_darwin::*;
use crate::signal_darwin_arm64::*;
use crate::signal_unix::*;
use crate::sigqueue::*;
use crate::sizeclasses::*;
use crate::slice::*;
use crate::softfloat64::*;
use crate::stack::*;
use crate::stkframe::*;
use crate::string::*;
use crate::stubs::*;
use crate::stubs_arm64::*;
use crate::stubs_nonlinux::*;
use crate::stubs_nonwasm::*;
use crate::symtab::*;
use crate::symtabinl::*;
use crate::synctest::*;
use crate::sys_arm64::*;
use crate::sys_darwin::*;
use crate::sys_darwin_arm64::*;
use crate::sys_libc::*;
use crate::sys_nonppc64x::*;
use crate::tagptr::*;
use crate::tagptr_64bit::*;
use crate::test_stubs::*;
use crate::time::*;
use crate::time_nofake::*;
use crate::timestub::*;
use crate::tls_stub::*;
use crate::trace::*;
use crate::traceallocfree::*;
use crate::traceback::*;
use crate::tracebuf::*;
use crate::tracecpu::*;
use crate::traceevent::*;
use crate::traceexp::*;
use crate::tracemap::*;
use crate::traceregion::*;
use crate::traceruntime::*;
use crate::tracestack::*;
use crate::tracestatus::*;
use crate::tracestring::*;
use crate::tracetime::*;
use crate::tracetype::*;
use crate::r#type::*;
use crate::typekind::*;
use crate::r#unsafe::*;
use crate::utf8::*;
use crate::vdso_in_none::*;
use crate::vgetrandom_unsupported::*;
use crate::write_err::*;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const BOUNDS_INDEX: u8 = 0;
pub(crate) const BOUNDS_SLICE_ALEN: u8 = 1;
pub(crate) const BOUNDS_SLICE_ACAP: u8 = 2;
pub(crate) const BOUNDS_SLICE_B: u8 = 3;
pub(crate) const BOUNDS_SLICE3_ALEN: u8 = 4;
pub(crate) const BOUNDS_SLICE3_ACAP: u8 = 5;
pub(crate) const BOUNDS_SLICE3_B: u8 = 6;
pub(crate) const BOUNDS_SLICE3_C: u8 = 7;
pub(crate) const BOUNDS_CONVERT: u8 = 8;


/// A TypeAssertionError explains a failed type assertion.
#[derive(Clone)]
pub struct TypeAssertionError {
    pub _interface: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub concrete: GoPtr<internal_abi::r#type::Type>,
    pub asserted: Arc<Mutex<Option<internal_abi::r#type::Type>>>,
    pub missing_method: Arc<Mutex<Option<String>>>,
}

impl TypeAssertionError {
    pub fn __go_value_clone(&self) -> Self {
        Self { _interface: self._interface.clone(), concrete: self.concrete.clone(), asserted: self.asserted.clone(), missing_method: { let __guard = self.missing_method.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for TypeAssertionError {
    fn default() -> Self {
        Self { _interface: Arc::new(Mutex::new(None)), concrete: GoPtr::nil(), asserted: Arc::new(Mutex::new(None)), missing_method: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for TypeAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for TypeAssertionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for TypeAssertionError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An errorString represents a runtime error described by a single string.
#[derive(Debug, Clone, Default)]
pub struct errorString(pub Arc<Mutex<Option<String>>>);

impl Display for errorString {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for errorString {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone)]
pub struct errorAddressString {
    pub msg: Arc<Mutex<Option<String>>>,
    pub addr: Arc<Mutex<Option<usize>>>,
}

impl errorAddressString {
    pub fn __go_value_clone(&self) -> Self {
        Self { msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, addr: { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for errorAddressString {
    fn default() -> Self {
        Self { msg: Arc::new(Mutex::new(Some(String::new()))), addr: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for errorAddressString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for errorAddressString {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// plainError represents a runtime error described a string without
/// the prefix "runtime error: " after invoking errorString.Error().
/// See Issue #14965.
#[derive(Debug, Clone, Default)]
pub struct plainError(pub Arc<Mutex<Option<String>>>);

impl Display for plainError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for plainError {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Debug, Clone, Default)]
pub struct boundsErrorCode(pub Arc<Mutex<Option<u8>>>);

impl Display for boundsErrorCode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for boundsErrorCode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for boundsErrorCode {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for boundsErrorCode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for boundsErrorCode {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<boundsErrorCode> for u8 {
    fn eq(&self, other: &boundsErrorCode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<boundsErrorCode> for u8 {
    fn partial_cmp(&self, other: &boundsErrorCode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for boundsErrorCode {
    type Output = boundsErrorCode;
    fn add(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn add(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn add(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for boundsErrorCode {
    type Output = boundsErrorCode;
    fn sub(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn sub(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn sub(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for boundsErrorCode {
    type Output = boundsErrorCode;
    fn mul(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn mul(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn mul(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for boundsErrorCode {
    type Output = boundsErrorCode;
    fn div(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn div(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn div(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for boundsErrorCode {
    type Output = boundsErrorCode;
    fn rem(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn rem(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn rem(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitand(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitand(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitand(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitor(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitor(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitor(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitxor(self, other: Self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn bitxor(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<boundsErrorCode> for u8 {
    type Output = boundsErrorCode;
    fn bitxor(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for boundsErrorCode {
    type Output = boundsErrorCode;
    fn not(self) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: i64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: u64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shl(self, other: usize) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: boundsErrorCode) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: i64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u32) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u8) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u16) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: u64) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for boundsErrorCode {
    type Output = boundsErrorCode;
    fn shr(self, other: usize) -> boundsErrorCode {
        boundsErrorCode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for boundsErrorCode {}

impl Ord for boundsErrorCode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub reuse: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
    pub fault: Arc<Mutex<Option<Vec<liveUserArenaChunk>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reuse: self.reuse.clone(), fault: self.fault.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), reuse: Arc::new(Mutex::new(None)), fault: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.reuse), format_slice(&self.fault))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub debug_log_reader: Arc<Mutex<Option<debugLogReader>>>,
    pub first: Arc<Mutex<Option<bool>>>,
    pub lost: Arc<Mutex<Option<u64>>>,
    pub next_tick: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { debug_log_reader: { let __guard = self.debug_log_reader.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first: { let __guard = self.first.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lost: { let __guard = self.lost.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next_tick: { let __guard = self.next_tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct2 {
    pub fn header(&mut self) -> (u64, u64, u64, i32) {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.header()
    }

    pub fn peek(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.peek()
    }

    pub fn print_val(&mut self) -> bool {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.print_val()
    }

    pub fn read_uint16_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u16 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint16_l_e_at(pos)
    }

    pub fn read_uint64_l_e_at(&self, pos: Arc<Mutex<Option<u64>>>) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_uint64_l_e_at(pos)
    }

    pub fn skip(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.skip()
    }

    pub fn uvarint(&mut self) -> u64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.uvarint()
    }

    pub fn varint(&mut self) -> i64 {
        // Forward to embedded type's method
        let embedded = self.debug_log_reader.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.varint()
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { debug_log_reader: Arc::new(Mutex::new(Some(debugLogReader::default()))), first: Arc::new(Mutex::new(Some(false))), lost: Arc::new(Mutex::new(Some(0))), next_tick: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.debug_log_reader.lock().unwrap().as_ref().unwrap()), (*self.first.lock().unwrap().as_ref().unwrap()), (*self.lost.lock().unwrap().as_ref().unwrap()), (*self.next_tick.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub tick: Arc<Mutex<Option<u64>>>,
    pub i: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { tick: { let __guard = self.tick.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, i: { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { tick: Arc::new(Mutex::new(Some(0))), i: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.tick.lock().unwrap().as_ref().unwrap()), (*self.i.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) static boundsErrorFmts: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 9]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static boundsNegErrorFmts: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 8]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *boundsErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *boundsNegErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *boundsErrorFmts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["index out of range [%x] with length %y".to_string(), "slice bounds out of range [:%x] with length %y".to_string(), "slice bounds out of range [:%x] with capacity %y".to_string(), "slice bounds out of range [%x:%y]".to_string(), "slice bounds out of range [::%x] with length %y".to_string(), "slice bounds out of range [::%x] with capacity %y".to_string(), "slice bounds out of range [:%x:%y]".to_string(), "slice bounds out of range [%x:%y:]".to_string(), "cannot convert slice with length %y to array or pointer to array with length %x".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
    *boundsNegErrorFmts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["index out of range [%x]".to_string(), "slice bounds out of range [:%x]".to_string(), "slice bounds out of range [:%x]".to_string(), "slice bounds out of range [%x:]".to_string(), "slice bounds out of range [::%x]".to_string(), "slice bounds out of range [::%x]".to_string(), "slice bounds out of range [:%x:]".to_string(), "slice bounds out of range [%x::]".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *boundsErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *boundsNegErrorFmts.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_2() {
    *boundsErrorFmts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["index out of range [%x] with length %y".to_string(), "slice bounds out of range [:%x] with length %y".to_string(), "slice bounds out of range [:%x] with capacity %y".to_string(), "slice bounds out of range [%x:%y]".to_string(), "slice bounds out of range [::%x] with length %y".to_string(), "slice bounds out of range [::%x] with capacity %y".to_string(), "slice bounds out of range [:%x:%y]".to_string(), "slice bounds out of range [%x:%y:]".to_string(), "cannot convert slice with length %y to array or pointer to array with length %x".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_3() {
    *boundsNegErrorFmts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["index out of range [%x]".to_string(), "slice bounds out of range [:%x]".to_string(), "slice bounds out of range [:%x]".to_string(), "slice bounds out of range [%x:]".to_string(), "slice bounds out of range [::%x]".to_string(), "slice bounds out of range [::%x]".to_string(), "slice bounds out of range [:%x:]".to_string(), "slice bounds out of range [%x::]".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl TypeAssertionError {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        let mut inter = Arc::new(Mutex::new(Some("interface".to_string())));
        if { let __nil_target = self._interface.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __recv = to_r_type(GoPtr::local(self._interface.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *inter.lock().unwrap() = __moved_val; };
    }
        let mut r#as = { let __recv = to_r_type(GoPtr::local(self.asserted.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result };
        if { let __ptr_field = self.concrete.clone(); __ptr_field.is_nil() } {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "interface conversion: ".to_string())); __s.push_str(&format!("{}", { let __v = (*inter.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", " is nil, not ".to_string())); __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s })));
    }
        let mut cs = { let __recv = to_r_type(self.concrete.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result };
        if { let __tmp_x = (*self.missing_method.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        let mut msg = Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "interface conversion: ".to_string())); __s.push_str(&format!("{}", { let __v = (*inter.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", " is ".to_string())); __s.push_str(&format!("{}", { let __v = (*cs.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", ", not ".to_string())); __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s })));
        if { let __tmp_x = (*cs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // provide slightly clearer error message
        if { let __tmp_x = (*{ let __recv = to_r_type(self.concrete.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkgpath(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = to_r_type(GoPtr::local(self.asserted.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pkgpath(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (types from different packages)".to_string()); };
    } else {
        { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&" (types from different scopes)".to_string()); };
    }
    }
                // provide slightly clearer error message
        return { let __owned = msg.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
                // provide slightly clearer error message
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "interface conversion: ".to_string())); __s.push_str(&format!("{}", { let __v = (*cs.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", " is not ".to_string())); __s.push_str(&format!("{}", { let __v = (*r#as.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", ": missing method ".to_string())); __s.push_str(&format!("{}", (*self.missing_method.clone().lock().unwrap().as_ref().unwrap()))); __s })));
    }
}

impl StdError for TypeAssertionError {}


impl errorString {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "runtime error: ".to_string(), (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).clone()))).lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for errorString {}


impl errorAddressString {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "runtime error: ".to_string(), (*self.msg.clone().lock().unwrap().as_ref().unwrap())))));
    }

    /// Addr returns the memory address where a fault occurred.
    /// The address provided is best-effort.
    /// The veracity of the result may depend on the platform.
    /// Errors providing this method will only be returned as
    /// a result of using [runtime/debug.SetPanicOnFault].
    pub fn addr(&self) -> usize {
        return (*self.addr.lock().unwrap().as_ref().unwrap());
    }
}

impl StdError for errorAddressString {}


impl plainError {
    pub fn runtime_error(&self) {
    }

    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).clone())))
    }
}

impl StdError for plainError {}


/// printindented prints s, replacing "\n" with "\n\t".
pub fn printindented(mut s: Arc<Mutex<Option<String>>>) {
    loop {
        let mut i = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('\n' as i32) as u8))));
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        { let __rhs = 1; i = i + __rhs; };
        eprint!("{}", format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap())));
        eprint!("{}", format!("{}", "\t".to_string()));
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    eprint!("{}", format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for TypeAssertionError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for errorAddressString {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
