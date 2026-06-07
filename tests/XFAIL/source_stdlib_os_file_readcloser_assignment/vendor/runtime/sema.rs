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
use crate::error::*;
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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SEM_TAB_SIZE: i32 = 251;


pub(crate) const SEMA_BLOCK_PROFILE: i32 = 1 << 0;
pub(crate) const SEMA_MUTEX_PROFILE: i32 = 1 << 1;


/// A semaRoot holds a balanced tree of sudog with distinct addresses (s.elem).
/// Each of those sudog may in turn point (through s.waitlink) to a list
/// of other sudogs waiting on the same address.
/// The operations on the inner lists of sudogs with the same address
/// are all O(1). The scanning of the top-level semaRoot list is O(log n),
/// where n is the number of distinct addresses with goroutines blocked
/// on them that hash to the given semaRoot.
/// See golang.org/issue/17953 for a program that worked badly
/// before we introduced the second level of list, and
/// BenchmarkSemTable/OneAddrCollision/* for a benchmark that exercises this.
#[derive(Clone)]
pub struct semaRoot {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub treap: Arc<Mutex<Option<sudog>>>,
    pub nwait: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl semaRoot {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, treap: self.treap.clone(), nwait: { let __guard = self.nwait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for semaRoot {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), treap: Arc::new(Mutex::new(None)), nwait: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for semaRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.treap.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.nwait.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for semaRoot {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct semTable(pub Arc<Mutex<Option<[AnonymousStruct30; 251]>>>);

impl Default for semTable {
    fn default() -> Self {
        semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))))
    }
}


#[derive(Debug, Clone, Default)]
pub struct semaProfileFlags(pub Arc<Mutex<Option<i32>>>);

impl Display for semaProfileFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for semaProfileFlags {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for semaProfileFlags {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for semaProfileFlags {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for semaProfileFlags {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<semaProfileFlags> for i32 {
    fn eq(&self, other: &semaProfileFlags) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<semaProfileFlags> for i32 {
    fn partial_cmp(&self, other: &semaProfileFlags) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for semaProfileFlags {
    type Output = semaProfileFlags;
    fn add(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn add(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn add(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for semaProfileFlags {
    type Output = semaProfileFlags;
    fn sub(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn sub(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn sub(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for semaProfileFlags {
    type Output = semaProfileFlags;
    fn mul(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn mul(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn mul(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for semaProfileFlags {
    type Output = semaProfileFlags;
    fn div(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn div(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn div(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for semaProfileFlags {
    type Output = semaProfileFlags;
    fn neg(self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for semaProfileFlags {
    type Output = semaProfileFlags;
    fn rem(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn rem(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn rem(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitand(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitand(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitand(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitor(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitor(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitor(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitxor(self, other: Self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn bitxor(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<semaProfileFlags> for i32 {
    type Output = semaProfileFlags;
    fn bitxor(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for semaProfileFlags {
    type Output = semaProfileFlags;
    fn not(self) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: i64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: u64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shl(self, other: usize) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: semaProfileFlags) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: i64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u32) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u8) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u16) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: u64) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for semaProfileFlags {
    type Output = semaProfileFlags;
    fn shr(self, other: usize) -> semaProfileFlags {
        semaProfileFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for semaProfileFlags {}

impl Ord for semaProfileFlags {
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


#[derive(Debug, Clone)]
pub struct AnonymousStruct10 {
    pub enabled: Arc<Mutex<Option<bool>>>,
    pub pad: Arc<Mutex<Option<[u8; 3]>>>,
    pub alignme: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct10 {
    pub fn __go_value_clone(&self) -> Self {
        Self { enabled: { let __guard = self.enabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alignme: { let __guard = self.alignme.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct10 {
    fn default() -> Self {
        Self { enabled: Arc::new(Mutex::new(Some(false))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), alignme: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct10 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.enabled.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad), (*self.alignme.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct10 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct11 {
    pub spin_after_ragged_barrier: Arc<Mutex<Option<internal_runtime_atomic::types::Bool>>>,
    pub restarted_due_to27993: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct11 {
    pub fn __go_value_clone(&self) -> Self {
        Self { spin_after_ragged_barrier: { let __guard = self.spin_after_ragged_barrier.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, restarted_due_to27993: { let __guard = self.restarted_due_to27993.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct11 {
    fn default() -> Self {
        Self { spin_after_ragged_barrier: Arc::new(Mutex::new(Some(Default::default()))), restarted_due_to27993: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct11 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.spin_after_ragged_barrier.lock().unwrap().as_ref().unwrap()), (*self.restarted_due_to27993.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct11 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct12 {
}
impl AnonymousStruct12 {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}


impl std::fmt::Display for AnonymousStruct12 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for AnonymousStruct12 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct13 {
    pub fill: Arc<Mutex<Option<u64>>>,
    pub capacity: Arc<Mutex<Option<u64>>>,
}
impl AnonymousStruct13 {
    pub fn __go_value_clone(&self) -> Self {
        Self { fill: { let __guard = self.fill.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, capacity: { let __guard = self.capacity.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct13 {
    fn default() -> Self {
        Self { fill: Arc::new(Mutex::new(Some(0))), capacity: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct13 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.fill.lock().unwrap().as_ref().unwrap()), (*self.capacity.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct13 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct14 {
    pub gc_percent_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub memory_limit_goal: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub assist_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub background_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
}
impl AnonymousStruct14 {
    pub fn __go_value_clone(&self) -> Self {
        Self { gc_percent_goal: { let __guard = self.gc_percent_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, memory_limit_goal: { let __guard = self.memory_limit_goal.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, assist_time: { let __guard = self.assist_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, background_time: { let __guard = self.background_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct14 {
    fn default() -> Self {
        Self { gc_percent_goal: Arc::new(Mutex::new(Some(Default::default()))), memory_limit_goal: Arc::new(Mutex::new(Some(Default::default()))), assist_time: Arc::new(Mutex::new(Some(Default::default()))), background_time: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct14 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.gc_percent_goal.lock().unwrap().as_ref().unwrap()), (*self.memory_limit_goal.lock().unwrap().as_ref().unwrap()), (*self.assist_time.lock().unwrap().as_ref().unwrap()), (*self.background_time.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct14 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct15 {
    pub index: Arc<Mutex<Option<scavengeIndex>>>,
    pub released_bg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub released_eager: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
}
impl AnonymousStruct15 {
    pub fn __go_value_clone(&self) -> Self {
        Self { index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_bg: { let __guard = self.released_bg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, released_eager: { let __guard = self.released_eager.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct15 {
    fn default() -> Self {
        Self { index: Arc::new(Mutex::new(Some(scavengeIndex::default()))), released_bg: Arc::new(Mutex::new(Some(Default::default()))), released_eager: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct15 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.index.lock().unwrap().as_ref().unwrap()), (*self.released_bg.lock().unwrap().as_ref().unwrap()), (*self.released_eager.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct15 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct16 {
    pub base: Arc<Mutex<Option<usize>>>,
    pub end: Arc<Mutex<Option<usize>>>,
}
impl AnonymousStruct16 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct16 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct16 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct17 {
    pub mcentral: Arc<Mutex<Option<mcentral>>>,
    pub pad: Arc<Mutex<Option<[u8; 88]>>>,
}
impl AnonymousStruct17 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mcentral: { let __guard = self.mcentral.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct17 {
    fn default() -> Self {
        Self { mcentral: Arc::new(Mutex::new(Some(mcentral::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct17 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mcentral.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct17 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct18 {
    pub arena_hints: GoPtr<crate::mheap::arenaHint>,
    pub quarantine_list: Arc<Mutex<Option<mSpanList>>>,
    pub ready_list: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct18 {
    pub fn __go_value_clone(&self) -> Self {
        Self { arena_hints: self.arena_hints.clone(), quarantine_list: { let __guard = self.quarantine_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ready_list: { let __guard = self.ready_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct18 {
    fn default() -> Self {
        Self { arena_hints: GoPtr::nil(), quarantine_list: Arc::new(Mutex::new(Some(mSpanList::default()))), ready_list: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct18 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { if self.arena_hints.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.quarantine_list.lock().unwrap().as_ref().unwrap()), (*self.ready_list.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct18 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct19 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: GoPtr<crate::mheap::gcBitsArena>,
    pub next: Arc<Mutex<Option<gcBitsArena>>>,
    pub current: Arc<Mutex<Option<gcBitsArena>>>,
    pub previous: Arc<Mutex<Option<gcBitsArena>>>,
}
impl AnonymousStruct19 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: self.free.clone(), next: self.next.clone(), current: self.current.clone(), previous: self.previous.clone() }
    }
}


impl Default for AnonymousStruct19 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: GoPtr::nil(), next: Arc::new(Mutex::new(None)), current: Arc::new(Mutex::new(None)), previous: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct19 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { if self.free.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.current.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.previous.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct19 {
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
pub struct AnonymousStruct20 {
    pub base: Arc<Mutex<Option<offAddr>>>,
    pub bound: Arc<Mutex<Option<offAddr>>>,
}
impl AnonymousStruct20 {
    pub fn __go_value_clone(&self) -> Self {
        Self { base: { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, bound: { let __guard = self.bound.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct20 {
    fn default() -> Self {
        Self { base: Arc::new(Mutex::new(Some(offAddr::default()))), bound: Arc::new(Mutex::new(Some(offAddr::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct20 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.base.lock().unwrap().as_ref().unwrap()), (*self.bound.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct20 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct21 {
    pub sema: Arc<Mutex<Option<u32>>>,
    pub active: Arc<Mutex<Option<bool>>>,
    pub offset: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub records: Arc<Mutex<Option<Vec<internal_profilerecord::r#mod::StackRecord>>>>,
    pub labels: Arc<Mutex<Option<Vec<usize>>>>,
}
impl AnonymousStruct21 {
    pub fn __go_value_clone(&self) -> Self {
        Self { sema: { let __guard = self.sema.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, active: { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, records: self.records.clone(), labels: self.labels.clone() }
    }
}


impl Default for AnonymousStruct21 {
    fn default() -> Self {
        Self { sema: Arc::new(Mutex::new(Some(0))), active: Arc::new(Mutex::new(Some(false))), offset: Arc::new(Mutex::new(Some(Default::default()))), records: Arc::new(Mutex::new(None)), labels: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct21 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.sema.lock().unwrap().as_ref().unwrap()), (*self.active.lock().unwrap().as_ref().unwrap()), (*self.offset.lock().unwrap().as_ref().unwrap()), format_slice(&self.records), format_slice(&self.labels))
    }
}

impl GoJsonDecode for AnonymousStruct21 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct22 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub newm: Arc<Mutex<Option<muintptr>>>,
    pub waiting: Arc<Mutex<Option<bool>>>,
    pub wake: Arc<Mutex<Option<note>>>,
    pub have_template_thread: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct22 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, newm: { let __guard = self.newm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiting: { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wake: { let __guard = self.wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, have_template_thread: { let __guard = self.have_template_thread.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct22 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), newm: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), waiting: Arc::new(Mutex::new(Some(false))), wake: Arc::new(Mutex::new(Some(note::default()))), have_template_thread: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.newm.lock().unwrap().as_ref().unwrap()), (*self.waiting.lock().unwrap().as_ref().unwrap()), (*self.wake.lock().unwrap().as_ref().unwrap()), (*self.have_template_thread.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct22 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct23 {
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub hz: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct23 {
    pub fn __go_value_clone(&self) -> Self {
        Self { signal_lock: { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hz: { let __guard = self.hz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct23 {
    fn default() -> Self {
        Self { signal_lock: Arc::new(Mutex::new(Some(Default::default()))), hz: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()), (*self.hz.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct23 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct24 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub seed: Arc<Mutex<Option<[u8; 32]>>>,
    pub state: Arc<Mutex<Option<internal_chacha8rand::chacha8::State>>>,
    pub init: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct24 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seed: { let __guard = self.seed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, init: { let __guard = self.init.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct24 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), seed: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), init: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct24 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), format_slice(&self.seed), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct24 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct25 {
    pub cgocheck: Arc<Mutex<Option<i32>>>,
    pub clobberfree: Arc<Mutex<Option<i32>>>,
    pub disablethp: Arc<Mutex<Option<i32>>>,
    pub dontfreezetheworld: Arc<Mutex<Option<i32>>>,
    pub efence: Arc<Mutex<Option<i32>>>,
    pub gccheckmark: Arc<Mutex<Option<i32>>>,
    pub gcpacertrace: Arc<Mutex<Option<i32>>>,
    pub gcshrinkstackoff: Arc<Mutex<Option<i32>>>,
    pub gcstoptheworld: Arc<Mutex<Option<i32>>>,
    pub gctrace: Arc<Mutex<Option<i32>>>,
    pub invalidptr: Arc<Mutex<Option<i32>>>,
    pub madvdontneed: Arc<Mutex<Option<i32>>>,
    pub runtime_contention_stacks: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub scavtrace: Arc<Mutex<Option<i32>>>,
    pub scheddetail: Arc<Mutex<Option<i32>>>,
    pub schedtrace: Arc<Mutex<Option<i32>>>,
    pub tracebackancestors: Arc<Mutex<Option<i32>>>,
    pub asyncpreemptoff: Arc<Mutex<Option<i32>>>,
    pub harddecommit: Arc<Mutex<Option<i32>>>,
    pub adaptivestackstart: Arc<Mutex<Option<i32>>>,
    pub tracefpunwindoff: Arc<Mutex<Option<i32>>>,
    pub traceadvanceperiod: Arc<Mutex<Option<i32>>>,
    pub trace_check_stack_ownership: Arc<Mutex<Option<i32>>>,
    pub profstackdepth: Arc<Mutex<Option<i32>>>,
    pub dataindependenttiming: Arc<Mutex<Option<i32>>>,
    pub malloc: Arc<Mutex<Option<bool>>>,
    pub inittrace: Arc<Mutex<Option<i32>>>,
    pub sbrk: Arc<Mutex<Option<i32>>>,
    pub traceallocfree: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub panicnil: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub asynctimerchan: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct25 {
    pub fn __go_value_clone(&self) -> Self {
        Self { cgocheck: { let __guard = self.cgocheck.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, clobberfree: { let __guard = self.clobberfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disablethp: { let __guard = self.disablethp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dontfreezetheworld: { let __guard = self.dontfreezetheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, efence: { let __guard = self.efence.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gccheckmark: { let __guard = self.gccheckmark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcpacertrace: { let __guard = self.gcpacertrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcshrinkstackoff: { let __guard = self.gcshrinkstackoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcstoptheworld: { let __guard = self.gcstoptheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gctrace: { let __guard = self.gctrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, invalidptr: { let __guard = self.invalidptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, madvdontneed: { let __guard = self.madvdontneed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runtime_contention_stacks: { let __guard = self.runtime_contention_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavtrace: { let __guard = self.scavtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scheddetail: { let __guard = self.scheddetail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedtrace: { let __guard = self.schedtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracebackancestors: { let __guard = self.tracebackancestors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asyncpreemptoff: { let __guard = self.asyncpreemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, harddecommit: { let __guard = self.harddecommit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, adaptivestackstart: { let __guard = self.adaptivestackstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracefpunwindoff: { let __guard = self.tracefpunwindoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceadvanceperiod: { let __guard = self.traceadvanceperiod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, trace_check_stack_ownership: { let __guard = self.trace_check_stack_ownership.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profstackdepth: { let __guard = self.profstackdepth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dataindependenttiming: { let __guard = self.dataindependenttiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, malloc: { let __guard = self.malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inittrace: { let __guard = self.inittrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sbrk: { let __guard = self.sbrk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceallocfree: { let __guard = self.traceallocfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, panicnil: { let __guard = self.panicnil.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asynctimerchan: { let __guard = self.asynctimerchan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct25 {
    fn default() -> Self {
        Self { cgocheck: Arc::new(Mutex::new(Some(0))), clobberfree: Arc::new(Mutex::new(Some(0))), disablethp: Arc::new(Mutex::new(Some(0))), dontfreezetheworld: Arc::new(Mutex::new(Some(0))), efence: Arc::new(Mutex::new(Some(0))), gccheckmark: Arc::new(Mutex::new(Some(0))), gcpacertrace: Arc::new(Mutex::new(Some(0))), gcshrinkstackoff: Arc::new(Mutex::new(Some(0))), gcstoptheworld: Arc::new(Mutex::new(Some(0))), gctrace: Arc::new(Mutex::new(Some(0))), invalidptr: Arc::new(Mutex::new(Some(0))), madvdontneed: Arc::new(Mutex::new(Some(0))), runtime_contention_stacks: Arc::new(Mutex::new(Some(Default::default()))), scavtrace: Arc::new(Mutex::new(Some(0))), scheddetail: Arc::new(Mutex::new(Some(0))), schedtrace: Arc::new(Mutex::new(Some(0))), tracebackancestors: Arc::new(Mutex::new(Some(0))), asyncpreemptoff: Arc::new(Mutex::new(Some(0))), harddecommit: Arc::new(Mutex::new(Some(0))), adaptivestackstart: Arc::new(Mutex::new(Some(0))), tracefpunwindoff: Arc::new(Mutex::new(Some(0))), traceadvanceperiod: Arc::new(Mutex::new(Some(0))), trace_check_stack_ownership: Arc::new(Mutex::new(Some(0))), profstackdepth: Arc::new(Mutex::new(Some(0))), dataindependenttiming: Arc::new(Mutex::new(Some(0))), malloc: Arc::new(Mutex::new(Some(false))), inittrace: Arc::new(Mutex::new(Some(0))), sbrk: Arc::new(Mutex::new(Some(0))), traceallocfree: Arc::new(Mutex::new(Some(Default::default()))), panicnil: Arc::new(Mutex::new(Some(Default::default()))), asynctimerchan: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.cgocheck.lock().unwrap().as_ref().unwrap()), (*self.clobberfree.lock().unwrap().as_ref().unwrap()), (*self.disablethp.lock().unwrap().as_ref().unwrap()), (*self.dontfreezetheworld.lock().unwrap().as_ref().unwrap()), (*self.efence.lock().unwrap().as_ref().unwrap()), (*self.gccheckmark.lock().unwrap().as_ref().unwrap()), (*self.gcpacertrace.lock().unwrap().as_ref().unwrap()), (*self.gcshrinkstackoff.lock().unwrap().as_ref().unwrap()), (*self.gcstoptheworld.lock().unwrap().as_ref().unwrap()), (*self.gctrace.lock().unwrap().as_ref().unwrap()), (*self.invalidptr.lock().unwrap().as_ref().unwrap()), (*self.madvdontneed.lock().unwrap().as_ref().unwrap()), (*self.runtime_contention_stacks.lock().unwrap().as_ref().unwrap()), (*self.scavtrace.lock().unwrap().as_ref().unwrap()), (*self.scheddetail.lock().unwrap().as_ref().unwrap()), (*self.schedtrace.lock().unwrap().as_ref().unwrap()), (*self.tracebackancestors.lock().unwrap().as_ref().unwrap()), (*self.asyncpreemptoff.lock().unwrap().as_ref().unwrap()), (*self.harddecommit.lock().unwrap().as_ref().unwrap()), (*self.adaptivestackstart.lock().unwrap().as_ref().unwrap()), (*self.tracefpunwindoff.lock().unwrap().as_ref().unwrap()), (*self.traceadvanceperiod.lock().unwrap().as_ref().unwrap()), (*self.trace_check_stack_ownership.lock().unwrap().as_ref().unwrap()), (*self.profstackdepth.lock().unwrap().as_ref().unwrap()), (*self.dataindependenttiming.lock().unwrap().as_ref().unwrap()), (*self.malloc.lock().unwrap().as_ref().unwrap()), (*self.inittrace.lock().unwrap().as_ref().unwrap()), (*self.sbrk.lock().unwrap().as_ref().unwrap()), (*self.traceallocfree.lock().unwrap().as_ref().unwrap()), (*self.panicnil.lock().unwrap().as_ref().unwrap()), (*self.asynctimerchan.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct25 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct26 {
    pub g_list: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct26 {
    pub fn __go_value_clone(&self) -> Self {
        Self { g_list: { let __guard = self.g_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct26 {
    pub fn empty(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.empty()
    }

    pub fn pop(&mut self) -> GoPtr<crate::runtime2::g> {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.pop()
    }

    pub fn push(&self, gp: GoPtr<crate::runtime2::g>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.push(gp)
    }

    pub fn push_all(&mut self, q: Arc<Mutex<Option<gQueue>>>) {
        // Forward to embedded type's method
        let embedded = self.g_list.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.push_all(q)
    }
}


impl Default for AnonymousStruct26 {
    fn default() -> Self {
        Self { g_list: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct26 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.g_list.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct26 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct27 {
    pub len: Arc<Mutex<Option<i32>>>,
    pub buf: Arc<Mutex<Option<[GoPtr<crate::mheap::mspan>; 128]>>>,
}
impl AnonymousStruct27 {
    pub fn __go_value_clone(&self) -> Self {
        Self { len: { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct27 {
    fn default() -> Self {
        Self { len: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil())))) }
    }
}

impl std::fmt::Display for AnonymousStruct27 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.len.lock().unwrap().as_ref().unwrap()), { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct27 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct28 {
    pub user: Arc<Mutex<Option<bool>>>,
    pub runnable: Arc<Mutex<Option<gQueue>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct28 {
    pub fn __go_value_clone(&self) -> Self {
        Self { user: { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runnable: { let __guard = self.runnable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct28 {
    fn default() -> Self {
        Self { user: Arc::new(Mutex::new(Some(false))), runnable: Arc::new(Mutex::new(Some(gQueue::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct28 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.user.lock().unwrap().as_ref().unwrap()), (*self.runnable.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct28 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct29 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub stack: Arc<Mutex<Option<gList>>>,
    pub no_stack: Arc<Mutex<Option<gList>>>,
    pub n: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct29 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, no_stack: { let __guard = self.no_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct29 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), stack: Arc::new(Mutex::new(Some(gList::default()))), no_stack: Arc::new(Mutex::new(Some(gList::default()))), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct29 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.stack.lock().unwrap().as_ref().unwrap()), (*self.no_stack.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct29 {
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


#[derive(Clone)]
pub struct AnonymousStruct30 {
    pub root: Arc<Mutex<Option<semaRoot>>>,
    pub pad: Arc<Mutex<Option<[u8; 104]>>>,
}
impl AnonymousStruct30 {
    pub fn __go_value_clone(&self) -> Self {
        Self { root: { let __guard = self.root.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pad: { let __guard = self.pad.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct30 {
    fn default() -> Self {
        Self { root: Arc::new(Mutex::new(Some(semaRoot::default()))), pad: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for AnonymousStruct30 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.root.lock().unwrap().as_ref().unwrap()), format_slice(&self.pad))
    }
}

impl GoJsonDecode for AnonymousStruct30 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct4 {
    pub mutex: Arc<Mutex<Option<mutex>>>,
    pub persistent_alloc: Arc<Mutex<Option<persistentAlloc>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, persistent_alloc: { let __guard = self.persistent_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        Self { mutex: Arc::new(Mutex::new(Some(mutex::default()))), persistent_alloc: Arc::new(Mutex::new(Some(persistentAlloc::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.mutex.lock().unwrap().as_ref().unwrap()), (*self.persistent_alloc.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct4 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub data: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone() }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), data: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { let __guard = self.data.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct5 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct6 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub free: Arc<Mutex<Option<mSpanList>>>,
    pub busy: Arc<Mutex<Option<mSpanList>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free: { let __guard = self.free.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, busy: { let __guard = self.busy.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), free: Arc::new(Mutex::new(Some(mSpanList::default()))), busy: Arc::new(Mutex::new(Some(mSpanList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.free.lock().unwrap().as_ref().unwrap()), (*self.busy.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct6 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct7 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct8 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub list: Arc<Mutex<Option<gList>>>,
}
impl AnonymousStruct8 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, list: { let __guard = self.list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct8 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), list: Arc::new(Mutex::new(Some(gList::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.list.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct8 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct9 {
    pub block: Arc<Mutex<Option<bool>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub q: Arc<Mutex<Option<gQueue>>>,
}
impl AnonymousStruct9 {
    pub fn __go_value_clone(&self) -> Self {
        Self { block: { let __guard = self.block.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, q: { let __guard = self.q.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct9 {
    fn default() -> Self {
        Self { block: Arc::new(Mutex::new(Some(false))), lock: Arc::new(Mutex::new(Some(mutex::default()))), q: Arc::new(Mutex::new(Some(gQueue::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct9 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.block.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.q.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct9 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type debug = AnonymousStruct25;


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type globalRand = AnonymousStruct24;


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) type newmHandoff = AnonymousStruct22;


pub(crate) type prof = AnonymousStruct23;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static semtable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<semTable>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *semtable.lock().unwrap() = Some(semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))));
}


pub(crate) fn __go_zero_globals() {
    *semtable.lock().unwrap() = Some(semTable(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default()))))));
}


impl semTable {
    pub fn root_for(&self, addr: GoPtr<u32>) -> Arc<Mutex<Option<semaRoot>>> {
        { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 3; __tmp_x >> __tmp_y }); let __tmp_y = SEM_TAB_SIZE as usize; __tmp_x % __tmp_y }) as usize].clone() }.root.clone()
    }
}

impl semaRoot {
    /// queue adds s to the blocked goroutines in semaRoot.
    pub fn queue(&mut self, addr: GoPtr<u32>, s: Arc<Mutex<Option<sudog>>>, lifo: Arc<Mutex<Option<bool>>>) {
        { let new_val = getg().clone(); (*s.lock().unwrap().as_mut().unwrap()).g = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(addr.addr()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = __moved_val; };
        *(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = 0 as u16; *(*s.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };
        let mut last: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
        let mut pt = Arc::new(Mutex::new(Some(self.treap.clone())));
        let mut t = (*pt.lock().unwrap().as_mut().unwrap()).clone();
    while { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Already have addr in list.
        if { let __v = (*lifo.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Substitute s in t's place in treap.
        { let new_val = s.clone(); let __dst = pt.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).ticket.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).parent.clone(); (*s.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).prev.clone(); (*s.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).next.clone(); (*s.lock().unwrap().as_mut().unwrap()).next = new_val; };
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = s.clone(); (*(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = s.clone(); (*(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
                // Add t first in s's wait list.
        { let new_val = t.clone(); (*s.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
        { let new_val = (*t.lock().unwrap().as_ref().unwrap()).waittail.clone(); (*s.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).waittail.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = t.clone(); (*s.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
    }
        { let new_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*s.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x + __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        *(*t.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;
    } else {
                // Add s to end of t's wait list.
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).waittail.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = s.clone(); (*t.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
    } else {
        { let new_val = s.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_mut().unwrap()).waitlink = new_val; };
    }
        { let new_val = s.clone(); (*t.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
        *(*s.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;
        if { let __tmp_x = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x + __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
        return;
    }
                // Already have addr in list.
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
        { let new_val = t.clone(); last = new_val; };
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*t.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).prev.clone()))).clone(); pt = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some((*t.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); pt = new_val; };
    }
        { let new_val = (*pt.lock().unwrap().as_mut().unwrap()).clone(); t = new_val; };
    }
                // Already have addr in list.
                // Substitute s in t's place in treap.
                // preserve head acquiretime as oldest time
                // Add t first in s's wait list.
                // Add s to end of t's wait list.
                // Add s as new leaf in tree of unique addrs.
                // The balanced tree is a treap using ticket as the random heap priority.
                // That is, it is a binary tree ordered according to the elem addresses,
                // but then among the space of possible binary trees respecting those
                // addresses, it is kept balanced on average by maintaining a heap ordering
                // on the ticket: s.ticket <= both s.prev.ticket and s.next.ticket.
                // https://en.wikipedia.org/wiki/Treap
                // https://faculty.washington.edu/aragon/pubs/rst89.pdf
                //
                // s.ticket compared with zero in couple of places, therefore set lowest bit.
                // It will not affect treap's quality noticeably.
        { let new_val = { let __tmp_x = cheaprand(); let __tmp_y = 1 as u32; __tmp_x | __tmp_y }; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        { let new_val = last.clone(); (*s.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = s.clone(); let __dst = pt.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
                // Rotate up into tree according to ticket (priority).
        while { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        if { let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.clone(); let __right = s.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        self.rotate_right({ let __field = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); __field });
    } else {
        if { let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).next.clone(); let __right = s.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        std::panic::panic_any(Box::new("semaRoot queue".to_string()) as Box<dyn Any + Send + Sync>);
    }
        self.rotate_left({ let __field = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); __field });
    }
    }
    }

    /// dequeue searches for and finds the first goroutine
    /// in semaRoot blocked on addr.
    /// If the sudog was being profiled, dequeue returns the time
    /// at which it was woken up as now. Otherwise now is 0.
    /// If there are additional entries in the wait list, dequeue
    /// returns tailtime set to the last entry's acquiretime.
    /// Otherwise tailtime is found.acquiretime.
    pub fn dequeue(&mut self, addr: GoPtr<u32>) -> (Arc<Mutex<Option<crate::runtime2::sudog>>>, i64, i64) {
    let mut found: Arc<Mutex<Option<sudog>>> = Arc::new(Mutex::new(None));
    let mut now: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut tailtime: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

        let mut ps = Arc::new(Mutex::new(Some(self.treap.clone())));
        let mut s = (*ps.lock().unwrap().as_mut().unwrap()).clone();
        'found: {
            while { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        break 'found;
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(addr.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).prev.clone()))).clone(); ps = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).next.clone()))).clone(); ps = new_val; };
    }
        { let new_val = (*ps.lock().unwrap().as_mut().unwrap()).clone(); s = new_val; };
    }
            return (Arc::new(Mutex::new(None)), 0, 0);

        }
        { let new_val = Arc::new(Mutex::new(Some(0 as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *now.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *now.lock().unwrap() = Some(new_val); };
    }
        {
        let mut t = (*s.lock().unwrap().as_ref().unwrap()).waitlink.clone();;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = t.clone(); let __dst = ps.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };;
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*t.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };;
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); (*t.lock().unwrap().as_mut().unwrap()).parent = new_val; };;
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); (*t.lock().unwrap().as_mut().unwrap()).prev = new_val; };;
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = t.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    };
            { let new_val = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); (*t.lock().unwrap().as_mut().unwrap()).next = new_val; };;
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = t.clone(); (*(*t.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    };
            if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).waitlink.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).waittail.clone(); (*t.lock().unwrap().as_mut().unwrap()).waittail = new_val; };
    } else {
        *(*t.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;
    };
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*t.lock().unwrap().as_ref().unwrap()).waiters.lock().unwrap() = Some(new_val); };;
            if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u16; __tmp_x > __tmp_y } {
        { let __target = (*t.lock().unwrap().as_ref().unwrap()).waiters.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    };
            { let new_val = now.lock().unwrap().as_ref().unwrap().clone(); *(*t.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };;
            { let new_val = { let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *tailtime.lock().unwrap() = Some(new_val); };;
            { let new_val = now.lock().unwrap().as_ref().unwrap().clone(); *(*(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };;
            *(*s.lock().unwrap().as_ref().unwrap()).waitlink.lock().unwrap() = None;;
            *(*s.lock().unwrap().as_ref().unwrap()).waittail.lock().unwrap() = None;;
        } else {
            while { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).prev.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        self.rotate_right(s.clone());
    } else {
        self.rotate_left(s.clone());
    }
    };
            if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = (*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.clone(); let __right = s.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        *(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
    } else {
        *(*(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
    }
    } else {
        *self.treap.lock().unwrap() = None;
    };
            { let new_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *tailtime.lock().unwrap() = Some(new_val); };;
        }
    }
                // Substitute t, also waiting on addr, for s in root tree of unique addrs.
                // Set head and tail acquire time to 'now',
                // because the caller will take care of charging
                // the delays before now for all entries in the list.
                // Rotate s down to be leaf of tree for removal, respecting priorities.
                // Remove s, now a leaf.
        *(*s.lock().unwrap().as_ref().unwrap()).parent.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).elem.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = None;
        *(*s.lock().unwrap().as_ref().unwrap()).prev.lock().unwrap() = None;
        { let new_val = 0 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
        return (s.clone(), { let __v = (*now.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*tailtime.lock().unwrap().as_ref().unwrap()).clone(); __v });
        unreachable!()
    }

    /// rotateLeft rotates the tree rooted at node x.
    /// turning (x a (y b c)) into (y (x a b) c).
    pub fn rotate_left(&mut self, x: Arc<Mutex<Option<sudog>>>) {
                // p -> (x a (y b c))
        let mut p = (*x.lock().unwrap().as_ref().unwrap()).parent.clone();
        let mut y = (*x.lock().unwrap().as_ref().unwrap()).next.clone();
        let mut b = (*y.lock().unwrap().as_ref().unwrap()).prev.clone();
        { let new_val = x.clone(); (*y.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        { let new_val = y.clone(); (*x.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = b.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        if { let __nil_result = (*b.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = x.clone(); (*b.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        { let new_val = p.clone(); (*y.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = y.clone(); self.treap = new_val; };
    } else if { let __left = (*p.lock().unwrap().as_ref().unwrap()).prev.clone(); let __right = x.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = y.clone(); (*p.lock().unwrap().as_mut().unwrap()).prev = new_val; };
    } else {
        if { let __left = (*p.lock().unwrap().as_ref().unwrap()).next.clone(); let __right = x.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("semaRoot rotateLeft".to_string()))));
    }
        { let new_val = y.clone(); (*p.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }
    }

    /// rotateRight rotates the tree rooted at node y.
    /// turning (y (x a b) c) into (x a (y b c)).
    pub fn rotate_right(&mut self, y: Arc<Mutex<Option<sudog>>>) {
                // p -> (y (x a b) c)
        let mut p = (*y.lock().unwrap().as_ref().unwrap()).parent.clone();
        let mut x = (*y.lock().unwrap().as_ref().unwrap()).prev.clone();
        let mut b = (*x.lock().unwrap().as_ref().unwrap()).next.clone();
        { let new_val = y.clone(); (*x.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = x.clone(); (*y.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        { let new_val = b.clone(); (*y.lock().unwrap().as_mut().unwrap()).prev = new_val; };
        if { let __nil_result = (*b.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = y.clone(); (*b.lock().unwrap().as_mut().unwrap()).parent = new_val; };
    }
        { let new_val = p.clone(); (*x.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = x.clone(); self.treap = new_val; };
    } else if { let __left = (*p.lock().unwrap().as_ref().unwrap()).prev.clone(); let __right = y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = x.clone(); (*p.lock().unwrap().as_mut().unwrap()).prev = new_val; };
    } else {
        if { let __left = (*p.lock().unwrap().as_ref().unwrap()).next.clone(); let __right = y.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("semaRoot rotateRight".to_string()))));
    }
        { let new_val = x.clone(); (*p.lock().unwrap().as_mut().unwrap()).next = new_val; };
    }
    }
}

pub fn ready_with_time(s: Arc<Mutex<Option<sudog>>>, traceskip: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = cputicks(); *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }
    goready(GoPtr::local((*s.lock().unwrap().as_ref().unwrap()).g.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = traceskip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

/// Called from runtime.
pub fn semacquire(addr: GoPtr<u32>) {
    semacquire1(addr.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32))))))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_SEMACQUIRE as u8))))))));
}

pub fn semacquire1(addr: GoPtr<u32>, lifo: Arc<Mutex<Option<bool>>>, profile: Arc<Mutex<Option<semaProfileFlags>>>, skipframes: Arc<Mutex<Option<i32>>>, reason: Arc<Mutex<Option<waitReason>>>) {
    let mut gp = getg();
    if { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        throw(Arc::new(Mutex::new(Some("semacquire not on the G stack".to_string()))));
    }

        // Easy case.
    if cansemacquire(addr.clone()) {
        return;
    }

        // Harder case:
        //	increment waiter count
        //	try cansemacquire one more time, return if succeeded
        //	enqueue itself as a waiter
        //	sleep
        //	(waiter descriptor is dequeued by signaler)
    let mut s = acquire_sudog();
    let mut root = (*semtable.lock().unwrap().as_ref().unwrap()).root_for(addr.clone());
    let mut t0 = Arc::new(Mutex::new(Some(0 as i64)));
    { let new_val = 0 as i64; *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as i64; *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
    { let new_val = 0 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = semaProfileFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*profile.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & SEMA_BLOCK_PROFILE as i32))))); let __tmp_y = semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*blockprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
        { let new_val = -1 as i64; *(*s.lock().unwrap().as_ref().unwrap()).releasetime.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = semaProfileFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*profile.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & SEMA_MUTEX_PROFILE as i32))))); let __tmp_y = semaProfileFlags(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = (*mutexprofilerate.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x > __tmp_y } {
        if { let __tmp_x = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = cputicks(); *t0.lock().unwrap() = Some(new_val); };
    }
        { let new_val = t0.lock().unwrap().as_ref().unwrap().clone(); *(*s.lock().unwrap().as_ref().unwrap()).acquiretime.lock().unwrap() = Some(new_val); };
    }
    loop {
        lock_with_rank(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32))))))));

                // Add ourselves to nwait to disable "easy case" in semrelease.
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));

                // Check cansemacquire to avoid missed wakeup.
        if cansemacquire(addr.clone()) {
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
        break
    }

                // Any semrelease after the cansemacquire knows we're waiting
                // (we set nwait above), so go to sleep.
        { let __recv = root.clone(); let __recv_ptr: *mut semaRoot = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut semaRoot }; let __result = unsafe { &mut *__recv_ptr }.queue(addr.clone(), s.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = lifo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        goparkunlock((*root.lock().unwrap().as_ref().unwrap()).lock.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = reason.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::traceruntime::traceBlockReason(Arc::new(Mutex::new(Some(TRACE_BLOCK_SYNC as u8))))))), Arc::new(Mutex::new(Some({ let __tmp_x = 4; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } || cansemacquire(addr.clone()) {
        break
    }
    }
        // Add ourselves to nwait to disable "easy case" in semrelease.
        // Check cansemacquire to avoid missed wakeup.
        // Any semrelease after the cansemacquire knows we're waiting
        // (we set nwait above), so go to sleep.
    if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        blockevent(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).releasetime.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*t0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
    }
    release_sudog(s.clone());
}

pub fn semrelease(addr: GoPtr<u32>) {
    semrelease1(addr.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(0))));
}

pub fn semrelease1(addr: GoPtr<u32>, handoff: Arc<Mutex<Option<bool>>>, skipframes: Arc<Mutex<Option<i32>>>) {
    let mut root = (*semtable.lock().unwrap().as_ref().unwrap()).root_for(addr.clone());
    internal_runtime_atomic::xadd({ let __go_ptr = addr.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some(1 as i32))));

        // Easy case: no waiters?
        // This check must happen after the xadd, to avoid a missed wakeup
        // (see loop in semacquire).
    if { let __tmp_x = (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return;
    }

        // Harder case: search for a waiter and wake it.
    lock_with_rank(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_ROOT as i32))))))));
    if { let __tmp_x = (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // The count is already consumed by another goroutine,
                // so no need to wake up another goroutine.
        unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
        return;
    }
        // The count is already consumed by another goroutine,
        // so no need to wake up another goroutine.
    let (mut s, mut t0, mut tailtime) = { let __recv = root.clone(); let __recv_ptr: *mut semaRoot = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut semaRoot }; let __result = unsafe { &mut *__recv_ptr }.dequeue(addr.clone()); __result };
    if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        (*(*root.lock().unwrap().as_ref().unwrap()).nwait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    }
    unlock(GoPtr::local((*root.lock().unwrap().as_ref().unwrap()).lock.clone()));
    if { let __nil_result = (*s.lock().unwrap()).is_some(); __nil_result } {
        let mut acquiretime = Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).acquiretime.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*acquiretime.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
                // Charge contention that this (delayed) unlock caused.
                // If there are N more goroutines waiting beyond the
                // one that's waking up, charge their delay as well, so that
                // contention holding up many goroutines shows up as
                // more costly than contention holding up a single goroutine.
                // It would take O(N) time to calculate how long each goroutine
                // has been waiting, so instead we charge avg(head-wait, tail-wait)*N.
                // head-wait is the longest wait and tail-wait is the shortest.
                // (When we do a lifo insertion, we preserve this property by
                // copying the old head's acquiretime into the inserted new head.
                // In that case the overall average may be slightly high, but that's fine:
                // the average of the ends is only an approximation to the actual
                // average anyway.)
                // The root.dequeue above changed the head and tail acquiretime
                // to the current time, so the next unlock will not re-count this contention.
        let mut dt0 = Arc::new(Mutex::new(Some({ let __tmp_x = t0; let __tmp_y = { let __v = (*acquiretime.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        let mut dt = { let __owned = dt0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        let mut dtail = Arc::new(Mutex::new(Some({ let __tmp_x = t0; let __tmp_y = tailtime; __tmp_x - __tmp_y })));
        { let __rhs = { let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __v = (*dtail.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*dt0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }); let __tmp_y = 2 as i64; __tmp_x / __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).waiters.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = dt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        mutexevent(Arc::new(Mutex::new(Some({ let __arg_holder = dt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = 3; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
    }
                // Charge contention that this (delayed) unlock caused.
                // If there are N more goroutines waiting beyond the
                // one that's waking up, charge their delay as well, so that
                // contention holding up many goroutines shows up as
                // more costly than contention holding up a single goroutine.
                // It would take O(N) time to calculate how long each goroutine
                // has been waiting, so instead we charge avg(head-wait, tail-wait)*N.
                // head-wait is the longest wait and tail-wait is the shortest.
                // (When we do a lifo insertion, we preserve this property by
                // copying the old head's acquiretime into the inserted new head.
                // In that case the overall average may be slightly high, but that's fine:
                // the average of the ends is only an approximation to the actual
                // average anyway.)
                // The root.dequeue above changed the head and tail acquiretime
                // to the current time, so the next unlock will not re-count this contention.
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("corrupted semaphore ticket".to_string()))));
    }
        if { let __v = (*handoff.lock().unwrap().as_ref().unwrap()).clone(); __v } && cansemacquire(addr.clone()) {
        { let new_val = 1 as u32; *(*s.lock().unwrap().as_ref().unwrap()).ticket.lock().unwrap() = Some(new_val); };
    }
        ready_with_time(s.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = 5; let __tmp_y = { let __v = (*skipframes.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))));
        if { let __tmp_x = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).ticket.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x == __tmp_y } && { let __tmp_x = (*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // Direct G handoff
                // readyWithTime has added the waiter G as runnext in the
                // current P; we now call the scheduler so that we start running
                // the waiter G immediately.
                // Note that waiter inherits our time slice: this is desirable
                // to avoid having a highly contended semaphore hog the P
                // indefinitely. goyield is like Gosched, but it emits a
                // "preempted" trace event instead and, more importantly, puts
                // the current G on the local runq instead of the global one.
                // We only do this in the starving regime (handoff=true), as in
                // the non-starving case it is possible for a different waiter
                // to acquire the semaphore while we are yielding/scheduling,
                // and this would be wasteful. We wait instead to enter starving
                // regime, and then we start to do direct handoffs of ticket and
                // P.
                // See issue 33747 for discussion.
        goyield();
    }
    }
}

pub fn cansemacquire(addr: GoPtr<u32>) -> bool {
    loop {
        let mut v = internal_runtime_atomic::load({ let __go_ptr = addr.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } });
        if { let __tmp_x = v; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        return false;
    }
        if internal_runtime_atomic::cas({ let __go_ptr = addr.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some({ let __tmp_x = v; let __tmp_y = 1 as u32; __tmp_x - __tmp_y })))) {
        return true;
    }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for semaRoot {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
