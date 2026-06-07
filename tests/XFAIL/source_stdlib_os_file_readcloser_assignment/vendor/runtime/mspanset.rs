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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SPAN_SET_BLOCK_ENTRIES: i32 = 512;
pub(crate) const SPAN_SET_INIT_SPINE_CAP: i32 = 256;


/// A spanSet is a set of *mspans.
///
/// spanSet is safe for concurrent push and pop operations.
#[derive(Clone)]
pub struct spanSet {
    pub spine_lock: Arc<Mutex<Option<mutex>>>,
    pub spine: Arc<Mutex<Option<atomicSpanSetSpinePointer>>>,
    pub spine_len: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub spine_cap: Arc<Mutex<Option<usize>>>,
    pub index: Arc<Mutex<Option<atomicHeadTailIndex>>>,
}

impl spanSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { spine_lock: { let __guard = self.spine_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spine: { let __guard = self.spine.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spine_len: { let __guard = self.spine_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spine_cap: { let __guard = self.spine_cap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, index: { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for spanSet {
    fn default() -> Self {
        Self { spine_lock: Arc::new(Mutex::new(Some(mutex::default()))), spine: Arc::new(Mutex::new(Some(atomicSpanSetSpinePointer::default()))), spine_len: Arc::new(Mutex::new(Some(Default::default()))), spine_cap: Arc::new(Mutex::new(Some(0))), index: Arc::new(Mutex::new(Some(atomicHeadTailIndex::default()))) }
    }
}

impl std::fmt::Display for spanSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.spine_lock.lock().unwrap().as_ref().unwrap()), (*self.spine.lock().unwrap().as_ref().unwrap()), (*self.spine_len.lock().unwrap().as_ref().unwrap()), (*self.spine_cap.lock().unwrap().as_ref().unwrap()), (*self.index.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for spanSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct spanSetBlock {
    pub lfnode: Arc<Mutex<Option<lfnode>>>,
    pub popped: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub spans: Arc<Mutex<Option<[atomicMSpanPointer; 512]>>>,
}

impl spanSetBlock {
    pub fn __go_value_clone(&self) -> Self {
        Self { lfnode: { let __guard = self.lfnode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, popped: { let __guard = self.popped.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, spans: { let __guard = self.spans.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for spanSetBlock {
    fn default() -> Self {
        Self { lfnode: Arc::new(Mutex::new(Some(lfnode::default()))), popped: Arc::new(Mutex::new(Some(Default::default()))), spans: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for spanSetBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.lfnode.lock().unwrap().as_ref().unwrap()), (*self.popped.lock().unwrap().as_ref().unwrap()), format_slice(&self.spans))
    }
}

impl GoJsonDecode for spanSetBlock {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicSpanSetSpinePointer is an atomically-accessed spanSetSpinePointer.
///
/// It has the same semantics as atomic.UnsafePointer.
#[derive(Clone)]
pub struct atomicSpanSetSpinePointer {
    pub a: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
}

impl atomicSpanSetSpinePointer {
    pub fn __go_value_clone(&self) -> Self {
        Self { a: { let __guard = self.a.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for atomicSpanSetSpinePointer {
    fn default() -> Self {
        Self { a: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for atomicSpanSetSpinePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.a.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for atomicSpanSetSpinePointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// spanSetSpinePointer represents a pointer to a contiguous block of atomic.Pointer[spanSetBlock].
#[derive(Debug, Clone)]
pub struct spanSetSpinePointer {
    pub p: Arc<Mutex<Option<usize>>>,
}

impl spanSetSpinePointer {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for spanSetSpinePointer {
    fn default() -> Self {
        Self { p: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for spanSetSpinePointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.p.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for spanSetSpinePointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// spanSetBlockAlloc represents a concurrent pool of spanSetBlocks.
#[derive(Debug, Clone)]
pub struct spanSetBlockAlloc {
    pub stack: Arc<Mutex<Option<lfstack>>>,
}

impl spanSetBlockAlloc {
    pub fn __go_value_clone(&self) -> Self {
        Self { stack: { let __guard = self.stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for spanSetBlockAlloc {
    fn default() -> Self {
        Self { stack: Arc::new(Mutex::new(Some(crate::lfstack::lfstack(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for spanSetBlockAlloc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.stack.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for spanSetBlockAlloc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// headTailIndex represents a combined 32-bit head and 32-bit tail
/// of a queue into a single 64-bit value.
#[derive(Debug, Clone, Default)]
pub struct headTailIndex(pub Arc<Mutex<Option<u64>>>);

impl Display for headTailIndex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for headTailIndex {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for headTailIndex {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for headTailIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for headTailIndex {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<headTailIndex> for u64 {
    fn eq(&self, other: &headTailIndex) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<headTailIndex> for u64 {
    fn partial_cmp(&self, other: &headTailIndex) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for headTailIndex {
    type Output = headTailIndex;
    fn add(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for headTailIndex {
    type Output = headTailIndex;
    fn add(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn add(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for headTailIndex {
    type Output = headTailIndex;
    fn sub(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for headTailIndex {
    type Output = headTailIndex;
    fn sub(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn sub(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for headTailIndex {
    type Output = headTailIndex;
    fn mul(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for headTailIndex {
    type Output = headTailIndex;
    fn mul(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn mul(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for headTailIndex {
    type Output = headTailIndex;
    fn div(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for headTailIndex {
    type Output = headTailIndex;
    fn div(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn div(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for headTailIndex {
    type Output = headTailIndex;
    fn rem(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for headTailIndex {
    type Output = headTailIndex;
    fn rem(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn rem(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for headTailIndex {
    type Output = headTailIndex;
    fn bitand(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitand(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitand(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for headTailIndex {
    type Output = headTailIndex;
    fn bitor(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitor(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitor(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for headTailIndex {
    type Output = headTailIndex;
    fn bitxor(self, other: Self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for headTailIndex {
    type Output = headTailIndex;
    fn bitxor(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<headTailIndex> for u64 {
    type Output = headTailIndex;
    fn bitxor(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for headTailIndex {
    type Output = headTailIndex;
    fn not(self) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: i64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for headTailIndex {
    type Output = headTailIndex;
    fn shl(self, other: usize) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: headTailIndex) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: i64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u32) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u8) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u16) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: u64) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for headTailIndex {
    type Output = headTailIndex;
    fn shr(self, other: usize) -> headTailIndex {
        headTailIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for headTailIndex {}

impl Ord for headTailIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// atomicHeadTailIndex is an atomically-accessed headTailIndex.
#[derive(Clone)]
pub struct atomicHeadTailIndex {
    pub u: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
}

impl atomicHeadTailIndex {
    pub fn __go_value_clone(&self) -> Self {
        Self { u: { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for atomicHeadTailIndex {
    fn default() -> Self {
        Self { u: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for atomicHeadTailIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.u.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for atomicHeadTailIndex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// atomicMSpanPointer is an atomic.Pointer[mspan]. Can't use generics because it's NotInHeap.
#[derive(Clone)]
pub struct atomicMSpanPointer {
    pub p: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>,
}

impl atomicMSpanPointer {
    pub fn __go_value_clone(&self) -> Self {
        Self { p: { let __guard = self.p.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for atomicMSpanPointer {
    fn default() -> Self {
        Self { p: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for atomicMSpanPointer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.p.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for atomicMSpanPointer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
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


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type gcBitsArenas = AnonymousStruct19;


pub(crate) type gcDebugMarkDone = AnonymousStruct11;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static spanSetBlockPool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<spanSetBlockAlloc>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *spanSetBlockPool.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *spanSetBlockPool.lock().unwrap() = Some(Default::default());
}


impl spanSet {
    /// push adds span s to buffer b. push is safe to call concurrently
    /// with other push and pop operations.
    pub fn push(&mut self, s: GoPtr<crate::mheap::mspan>) {
                // Obtain our slot.
        let mut cursor = Arc::new(Mutex::new(Some(({ let __tmp_x = headTailIndex::tail(&(*(*self.index.lock().unwrap().as_ref().unwrap()).inc_tail().lock().unwrap().as_ref().unwrap())); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }) as usize)));
        let (mut top, mut bottom) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*cursor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*cursor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x % __tmp_y }))));

                // Do we need to add a block?
        let mut spineLen = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load();
        let mut block: GoPtr<spanSetBlock> = GoPtr::nil();
        'retry: loop {
            if { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = spineLen; __tmp_x < __tmp_y } {
        block = GoPtr::local({ let __recv = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }; let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result });
    } else {
                // Add a new block to the spine, potentially growing
                // the spine.
        lock(GoPtr::local(self.spine_lock.clone()));
                // spineLen cannot change until we release the lock,
                // but may have changed while we were waiting.
        { let new_val = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load(); spineLen = new_val; };
        if { let __tmp_x = { let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = spineLen; __tmp_x < __tmp_y } {
        unlock(GoPtr::local(self.spine_lock.clone()));
        continue 'retry;
    }
        let mut spine = (*self.spine.lock().unwrap().as_ref().unwrap()).load();
        if { let __tmp_x = spineLen; let __tmp_y = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // Grow the spine.
        let mut newCap = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x * __tmp_y })));
        if { let __tmp_x = { let __v = (*newCap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = SPAN_SET_INIT_SPINE_CAP as usize; *newCap.lock().unwrap() = Some(new_val); };
    }
        let mut newSpine = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*newCap.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __selector_holder = internal_cpu::CacheLineSize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone());
        if { let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // Blocks are allocated off-heap, so
                // no write barriers.
        memmove(Arc::new(Mutex::new(Some({ let __arg_holder = newSpine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*spine.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*self.spine_cap.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))));
    }
                // Blocks are allocated off-heap, so
                // no write barriers.
        { let new_val = spanSetSpinePointer { p: Arc::new(Mutex::new(Some({ let __arg_holder = newSpine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }; *spine.lock().unwrap() = Some(new_val); };
                // Spine is allocated off-heap, so no write barrier.
        (*self.spine.lock().unwrap().as_ref().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some({ let __arg_holder = spine.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = newCap.lock().unwrap().as_ref().unwrap().clone(); *self.spine_cap.lock().unwrap() = Some(new_val); };
    }
                // Grow the spine.
                // Blocks are allocated off-heap, so
                // no write barriers.
                // Spine is allocated off-heap, so no write barrier.
                // We can't immediately free the old spine
                // since a concurrent push with a lower index
                // could still be reading from it. We let it
                // leak because even a 1TB heap would waste
                // less than 2MB of memory on old spines. If
                // this is a problem, we could free old spines
                // during STW.
                // Allocate a new block from the pool.
        block = (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).alloc();
                // Add it to the spine.
                // Blocks are allocated off-heap, so no write barrier.
        { let __recv = (*spine.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b({ let __go_ptr = block.clone(); match __go_ptr { GoPtr::Nil => internal_runtime_atomic::GoPtr::nil(), GoPtr::Local(__value) => internal_runtime_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_runtime_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_runtime_atomic::GoPtr::slice_elem(internal_runtime_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }); __result };
        (*self.spine_len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = spineLen; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }))));
        unlock(GoPtr::local(self.spine_lock.clone()));
    }

                        // Add a new block to the spine, potentially growing
                        // the spine.
                        // spineLen cannot change until we release the lock,
                        // but may have changed while we were waiting.
                        // Grow the spine.
                        // Blocks are allocated off-heap, so
                        // no write barriers.
                        // Spine is allocated off-heap, so no write barrier.
                        // We can't immediately free the old spine
                        // since a concurrent push with a lower index
                        // could still be reading from it. We let it
                        // leak because even a 1TB heap would waste
                        // less than 2MB of memory on old spines. If
                        // this is a problem, we could free old spines
                        // during STW.
                        // Allocate a new block from the pool.
                        // Add it to the spine.
                        // Blocks are allocated off-heap, so no write barrier.
                        // We have a block. Insert the span atomically, since there may be
                        // concurrent readers via the block API.
            { let __seq = { let __seq_holder = { let __ptr_value = block.with_mut(|__ptr_value| __ptr_value.spans.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store_no_w_b(s.clone());
            break 'retry;
        };
    }

    /// pop removes and returns a span from buffer b, or nil if b is empty.
    /// pop is safe to call concurrently with other pop and push operations.
    pub fn pop(&self) -> GoPtr<crate::mheap::mspan> {
        let mut head: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut tail: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        'claim_loop: loop {
        let mut headtail = (*self.index.lock().unwrap().as_ref().unwrap()).load();
        { let (__tmp_0, __tmp_1) = headTailIndex::split(&(*headtail.lock().unwrap().as_ref().unwrap())); *head.lock().unwrap() = Some(__tmp_0); *tail.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tail.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // The buf is empty, as far as we can tell.
        return GoPtr::nil();
    }

                // The buf is empty, as far as we can tell.
                // Check if the head position we want to claim is actually
                // backed by a block.
        let mut spineLen = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = spineLen; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*head.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as usize; __tmp_x / __tmp_y }; __tmp_x <= __tmp_y } {
                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
        return GoPtr::nil();
    }

                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
                // Try to claim the current head by CASing in an updated head.
                // This may fail transiently due to a push which modifies the
                // tail, so keep trying while the head isn't changing.
        let mut want = { let __owned = head.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        while { let __tmp_x = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        if (*self.index.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = headtail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), make_head_tail_index(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = tail.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) {
        break 'claim_loop
    }
        { let new_val = (*self.index.lock().unwrap().as_ref().unwrap()).load(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *headtail.lock().unwrap() = __moved_val; };
        { let (__tmp_0, __tmp_1) = headTailIndex::split(&(*headtail.lock().unwrap().as_ref().unwrap())); *head.lock().unwrap() = Some(__tmp_0); *tail.lock().unwrap() = Some(__tmp_1); };
    }
    }
                // The buf is empty, as far as we can tell.
                // Check if the head position we want to claim is actually
                // backed by a block.
                // We're racing with a spine growth and the allocation of
                // a new block (and maybe a new spine!), and trying to grab
                // the span at the index which is currently being pushed.
                // Instead of spinning, let's just notify the caller that
                // there's nothing currently here. Spinning on this is
                // almost definitely not worth it.
                // Try to claim the current head by CASing in an updated head.
                // This may fail transiently due to a push which modifies the
                // tail, so keep trying while the head isn't changing.
                // We failed to claim the spot we were after and the head changed,
                // meaning a popper got ahead of us. Try again from the top because
                // the buf may not be empty.
        let (mut top, mut bottom) = (Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x / __tmp_y }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*head.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x % __tmp_y }))));
                // We may be reading a stale spine pointer, but because the length
                // grows monotonically and we've already verified it, we'll definitely
                // be reading from a valid block.
        let mut blockp: GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
                // Given that the spine length is correct, we know we will never
                // see a nil block here, since the length is always updated after
                // the block is set.
        let mut block = { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result };
        let mut s: GoPtr<crate::mheap::mspan> = { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load();
        while s.is_nil() {
                // We raced with the span actually being set, but given that we
                // know a block for this span exists, the race window here is
                // extremely small. Try again.
        s = { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load();
    }
                // We raced with the span actually being set, but given that we
                // know a block for this span exists, the race window here is
                // extremely small. Try again.
                // Clear the pointer. This isn't strictly necessary, but defensively
                // avoids accidentally re-using blocks which could lead to memory
                // corruption. This way, we'll get a nil pointer access instead.
        { let __seq = { let __seq_holder = (*block.lock().unwrap().as_ref().unwrap()).spans.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*bottom.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store_no_w_b(GoPtr::nil());
                // Increase the popped count. If we are the last possible popper
                // in the block (note that bottom need not equal spanSetBlockEntries-1
                // due to races) then it's our responsibility to free the block.
                //
                // If we increment popped to spanSetBlockEntries, we can be sure that
                // we're the last popper for this block, and it's thus safe to free it.
                // Every other popper must have crossed this barrier (and thus finished
                // popping its corresponding mspan) by the time we get here. Because
                // we're the last popper, we also don't have to worry about concurrent
                // pushers (there can't be any). Note that we may not be the popper
                // which claimed the last slot in the block, we're just the last one
                // to finish popping.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x == __tmp_y } {
                // Clear the block's pointer.
        { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b(internal_runtime_atomic::GoPtr::nil()); __result };
                // Return the block to the block pool.
        (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).free(block.clone());
    }
                // Clear the block's pointer.
                // Return the block to the block pool.
        s.clone()
    }

    /// reset resets a spanSet which is empty. It will also clean up
    /// any left over blocks.
    ///
    /// Throws if the buf is not empty.
    ///
    /// reset may not be called concurrently with any other operations
    /// on the span set.
    pub fn reset(&self) {
        let (mut head, mut tail) = headTailIndex::split(&(*(*self.index.lock().unwrap().as_ref().unwrap()).load().lock().unwrap().as_ref().unwrap()));
        if { let __tmp_x = head; let __tmp_y = tail; __tmp_x < __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "head = ".to_string()), format!("{}", head), format!("{}", ", tail = ".to_string()), format!("{}", tail), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("attempt to clear non-empty span set".to_string()))));
    }
        let mut top = Arc::new(Mutex::new(Some({ let __tmp_x = head; let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x / __tmp_y })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.spine_len.lock().unwrap().as_mut().unwrap()).load(); __tmp_x < __tmp_y } {
                // If the head catches up to the tail and the set is empty,
                // we may not clean up the block containing the head and tail
                // since it may be pushed into again. In order to avoid leaking
                // memory since we're going to reset the head and tail, clean
                // up such a block now, if it exists.
        let mut blockp: GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> = { let __recv = (*self.spine.lock().unwrap().as_ref().unwrap()).load(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some((*top.lock().unwrap().as_ref().unwrap()) as usize)))); __result };
        let mut block = { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).load(); __result };
        if { let __nil_result = (*block.lock().unwrap()).is_some(); __nil_result } {
                // Check the popped value.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
        throw(Arc::new(Mutex::new(Some("span set block with unpopped elements found in reset".to_string()))));
    }
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
        if { let __tmp_x = (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = SPAN_SET_BLOCK_ENTRIES as u32; __tmp_x == __tmp_y } {
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
        throw(Arc::new(Mutex::new(Some("fully empty unfreed span set block found in reset".to_string()))));
    }
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
                // Clear the pointer to the block.
        { let __recv_value = blockp.borrow(); let __result = (*__recv_value.as_ref().unwrap()).store_no_w_b(internal_runtime_atomic::GoPtr::nil()); __result };
                // Return the block to the block pool.
        (*spanSetBlockPool.lock().unwrap().as_ref().unwrap()).free(block.clone());
    }
    }
                // If the head catches up to the tail and the set is empty,
                // we may not clean up the block containing the head and tail
                // since it may be pushed into again. In order to avoid leaking
                // memory since we're going to reset the head and tail, clean
                // up such a block now, if it exists.
                // Check the popped value.
                // popped should never be zero because that means we have
                // pushed at least one value but not yet popped if this
                // block pointer is not nil.
                // popped should also never be equal to spanSetBlockEntries
                // because the last popper should have made the block pointer
                // in this slot nil.
                // Clear the pointer to the block.
                // Return the block to the block pool.
        (*self.index.lock().unwrap().as_ref().unwrap()).reset();
        (*self.spine_len.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as usize))));
    }
}

impl atomicSpanSetSpinePointer {
    /// Loads the spanSetSpinePointer and returns it.
    ///
    /// It has the same semantics as atomic.UnsafePointer.
    pub fn load(&self) -> Arc<Mutex<Option<spanSetSpinePointer>>> {
        Arc::new(Mutex::new(Some(spanSetSpinePointer { p: (*self.a.lock().unwrap().as_mut().unwrap()).load(), ..Default::default() })))
    }

    /// Stores the spanSetSpinePointer.
    ///
    /// It has the same semantics as [atomic.UnsafePointer].
    pub fn store_no_w_b(&self, p: Arc<Mutex<Option<spanSetSpinePointer>>>) {
        (*self.a.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some({ let __selector_holder = (*p.lock().unwrap().as_ref().unwrap()).p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
}

impl spanSetSpinePointer {
    /// lookup returns &s[idx].
    pub fn lookup(&self, idx: Arc<Mutex<Option<usize>>>) -> GoPtr<internal_runtime_atomic::types::Pointer<spanSetBlock>> {
        GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __selector_holder = self.p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __tmp_x = internal_goarch::PTR_SIZE as usize; let __tmp_y = { let __v = (*idx.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }
}

impl spanSetBlockAlloc {
    /// alloc tries to grab a spanSetBlock out of the pool, and if it fails
    /// persistentallocs a new one and returns it.
    pub fn alloc(&self) -> GoPtr<spanSetBlock> {
        {
        let mut s: GoPtr<spanSetBlock> = GoPtr::raw({ let __ptr = (*self.stack.lock().unwrap().as_ref().unwrap()).pop().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });;
        if !s.is_nil() {
            return s.clone();;
        }
    }
        GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some(std::mem::size_of::<spanSetBlock>()))), Arc::new(Mutex::new(Some({ let __selector_holder = internal_cpu::CacheLineSize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), (*memstats.lock().unwrap().as_ref().unwrap()).gc_misc_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// free returns a spanSetBlock back to the pool.
    pub fn free(&self, block: Arc<Mutex<Option<spanSetBlock>>>) {
        (*(*block.lock().unwrap().as_ref().unwrap()).popped.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
        (*self.stack.lock().unwrap().as_ref().unwrap()).push((*block.lock().unwrap().as_ref().unwrap()).lfnode.clone());
    }
}

impl headTailIndex {
    /// head returns the head of a headTailIndex value.
    pub fn head(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) >> 32i32)) as u32))).lock().unwrap().as_ref().unwrap())
    }

    /// tail returns the tail of a headTailIndex value.
    pub fn tail(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap())
    }

    /// split splits the headTailIndex value into its parts.
    pub fn split(&self) -> (u32, u32) {
    let mut head: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut tail: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        (headTailIndex::head(self), headTailIndex::tail(self))
    }
}

impl atomicHeadTailIndex {
    /// load atomically reads a headTailIndex value.
    pub fn load(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).load() as u64)))))))
    }

    /// cas atomically compares-and-swaps a headTailIndex value.
    pub fn cas(&self, old: Arc<Mutex<Option<headTailIndex>>>, new: Arc<Mutex<Option<headTailIndex>>>) -> bool {
        (*self.u.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))
    }

    /// incHead atomically increments the head of a headTailIndex.
    pub fn inc_head(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(((1 as i64) << (32 as i64)) as i64)))) as u64)))))))
    }

    /// decHead atomically decrements the head of a headTailIndex.
    pub fn dec_head(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-(((1 as i64) << (32 as i64))) as i64)))) as u64)))))))
    }

    /// incTail atomically increments the tail of a headTailIndex.
    pub fn inc_tail(&self) -> Arc<Mutex<Option<headTailIndex>>> {
        let mut ht = Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some((*self.u.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64)))) as u64)))))));
                // Check for overflow.
        if { let __tmp_x = headTailIndex::tail(&(*ht.lock().unwrap().as_ref().unwrap())); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        eprint!("{}{}{}{}{}", format!("{}", "runtime: head = ".to_string()), format!("{}", headTailIndex::head(&(*ht.lock().unwrap().as_ref().unwrap()))), format!("{}", ", tail = ".to_string()), format!("{}", headTailIndex::tail(&(*ht.lock().unwrap().as_ref().unwrap()))), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("headTailIndex overflow".to_string()))));
    }
        return { let __owned = ht.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// reset clears the headTailIndex to (0, 0).
    pub fn reset(&self) {
        (*self.u.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u64))));
    }
}

impl atomicMSpanPointer {
    /// Load returns the *mspan.
    pub fn load(&self) -> GoPtr<crate::mheap::mspan> {
        GoPtr::raw({ let __ptr = (*self.p.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// Store stores an *mspan.
    pub fn store_no_w_b(&self, s: GoPtr<crate::mheap::mspan>) {
        (*self.p.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some(s.addr()))));
    }
}

impl spanSetBlock {
}

/// makeHeadTailIndex creates a headTailIndex value from a separate
/// head and tail.
pub fn make_head_tail_index(head: Arc<Mutex<Option<u32>>>, tail: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<headTailIndex>>> {
    Arc::new(Mutex::new(Some(headTailIndex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*head.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 32; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*tail.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for spanSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetBlock {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicSpanSetSpinePointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetSpinePointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for spanSetBlockAlloc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicHeadTailIndex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for atomicMSpanPointer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
