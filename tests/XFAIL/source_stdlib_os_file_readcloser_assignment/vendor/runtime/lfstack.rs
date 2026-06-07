use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// lfstack is the head of a lock-free stack.
///
/// The zero value of lfstack is an empty list.
///
/// This stack is intrusive. Nodes must embed lfnode as the first field.
///
/// The stack does not keep GC-visible pointers to nodes, so the caller
/// must ensure the nodes are allocated outside the Go heap.
#[derive(Debug, Clone, Default)]
pub struct lfstack(pub Arc<Mutex<Option<u64>>>);

impl Display for lfstack {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for lfstack {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for lfstack {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for lfstack {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for lfstack {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<lfstack> for u64 {
    fn eq(&self, other: &lfstack) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<lfstack> for u64 {
    fn partial_cmp(&self, other: &lfstack) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for lfstack {
    type Output = lfstack;
    fn add(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for lfstack {
    type Output = lfstack;
    fn add(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<lfstack> for u64 {
    type Output = lfstack;
    fn add(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for lfstack {
    type Output = lfstack;
    fn sub(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for lfstack {
    type Output = lfstack;
    fn sub(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<lfstack> for u64 {
    type Output = lfstack;
    fn sub(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for lfstack {
    type Output = lfstack;
    fn mul(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for lfstack {
    type Output = lfstack;
    fn mul(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<lfstack> for u64 {
    type Output = lfstack;
    fn mul(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for lfstack {
    type Output = lfstack;
    fn div(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for lfstack {
    type Output = lfstack;
    fn div(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<lfstack> for u64 {
    type Output = lfstack;
    fn div(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for lfstack {
    type Output = lfstack;
    fn rem(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for lfstack {
    type Output = lfstack;
    fn rem(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<lfstack> for u64 {
    type Output = lfstack;
    fn rem(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for lfstack {
    type Output = lfstack;
    fn bitand(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for lfstack {
    type Output = lfstack;
    fn bitand(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<lfstack> for u64 {
    type Output = lfstack;
    fn bitand(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for lfstack {
    type Output = lfstack;
    fn bitor(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for lfstack {
    type Output = lfstack;
    fn bitor(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<lfstack> for u64 {
    type Output = lfstack;
    fn bitor(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for lfstack {
    type Output = lfstack;
    fn bitxor(self, other: Self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for lfstack {
    type Output = lfstack;
    fn bitxor(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<lfstack> for u64 {
    type Output = lfstack;
    fn bitxor(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for lfstack {
    type Output = lfstack;
    fn not(self) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for lfstack {
    type Output = lfstack;
    fn shl(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for lfstack {
    type Output = lfstack;
    fn shl(self, other: i32) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for lfstack {
    type Output = lfstack;
    fn shl(self, other: i8) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for lfstack {
    type Output = lfstack;
    fn shl(self, other: i16) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for lfstack {
    type Output = lfstack;
    fn shl(self, other: i64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for lfstack {
    type Output = lfstack;
    fn shl(self, other: u32) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for lfstack {
    type Output = lfstack;
    fn shl(self, other: u8) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for lfstack {
    type Output = lfstack;
    fn shl(self, other: u16) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for lfstack {
    type Output = lfstack;
    fn shl(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for lfstack {
    type Output = lfstack;
    fn shl(self, other: usize) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for lfstack {
    type Output = lfstack;
    fn shr(self, other: lfstack) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for lfstack {
    type Output = lfstack;
    fn shr(self, other: i32) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for lfstack {
    type Output = lfstack;
    fn shr(self, other: i8) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for lfstack {
    type Output = lfstack;
    fn shr(self, other: i16) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for lfstack {
    type Output = lfstack;
    fn shr(self, other: i64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for lfstack {
    type Output = lfstack;
    fn shr(self, other: u32) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for lfstack {
    type Output = lfstack;
    fn shr(self, other: u8) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for lfstack {
    type Output = lfstack;
    fn shr(self, other: u16) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for lfstack {
    type Output = lfstack;
    fn shr(self, other: u64) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for lfstack {
    type Output = lfstack;
    fn shr(self, other: usize) -> lfstack {
        lfstack(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for lfstack {}

impl Ord for lfstack {
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


impl lfstack {
    pub fn push(&self, node: Arc<Mutex<Option<lfnode>>>) {
        { let __target = (*node.lock().unwrap().as_ref().unwrap()).pushcnt.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut new = lfstack_pack(node.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*node.lock().unwrap().as_ref().unwrap()).pushcnt.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        {
        let mut node1: GoPtr<crate::runtime2::lfnode> = lfstack_unpack(Arc::new(Mutex::new(Some(new))));;
        if { let __left_addr = node1.addr(); let __right_addr = { let __ptr = GoPtr::local(node.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq } {
            eprint!("{}{}{}{}{}{}{}{}{}", format!("{}", "runtime: lfstack.push invalid packing: node=".to_string()), format!("{}", format!("&{}", (*node.lock().unwrap().as_ref().unwrap()))), format!("{}", " cnt=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*node.lock().unwrap().as_ref().unwrap()).pushcnt.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))))), format!("{}", " packed=".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(new as u64))))), format!("{}", " -> node=".to_string()), format!("{}", format!("0x{:x}", node1.addr())), format!("{}", "\n".to_string()));;
            throw(Arc::new(Mutex::new(Some("lfstack.push".to_string()))));;
        }
    }
        loop {
        let mut old = internal_runtime_atomic::load64(Arc::new(Mutex::new(Some(u64::default()))));
        { let new_val = old; *(*node.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
        if internal_runtime_atomic::cas64(Arc::new(Mutex::new(Some(u64::default()))), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some(new)))) {
        break
    }
    }
    }

    pub fn pop(&self) -> Arc<Mutex<Option<usize>>> {
        loop {
        let mut old = internal_runtime_atomic::load64(Arc::new(Mutex::new(Some(u64::default()))));
        if { let __tmp_x = old; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        let mut node: GoPtr<crate::runtime2::lfnode> = lfstack_unpack(Arc::new(Mutex::new(Some(old))));
        let mut next = internal_runtime_atomic::load64({ let __ptr_value = node.with_mut(|__ptr_value| __ptr_value.next.clone()); __ptr_value }.clone());
        if internal_runtime_atomic::cas64(Arc::new(Mutex::new(Some(u64::default()))), Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some(next)))) {
        return Arc::new(Mutex::new(Some(node.addr())));
    }
    }
    }

    pub fn empty(&self) -> bool {
        return { let __tmp_x = internal_runtime_atomic::load64(Arc::new(Mutex::new(Some(u64::default())))); let __tmp_y = 0 as u64; __tmp_x == __tmp_y };
    }
}

/// lfnodeValidate panics if node is not a valid address for use with
/// lfstack.push. This only needs to be called when node is allocated.
pub fn lfnode_validate(node: Arc<Mutex<Option<lfnode>>>) {
    {
        let (mut base, _, _) = find_object(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&node) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));;
        if { let __tmp_x = base; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
            throw(Arc::new(Mutex::new(Some("lfstack node allocated from the heap".to_string()))));;
        }
    }
    if { let __left_addr = lfstack_unpack(Arc::new(Mutex::new(Some(lfstack_pack(node.clone(), Arc::new(Mutex::new(Some(!(0 as usize) as usize)))))))).addr(); let __right_addr = { let __ptr = GoPtr::local(node.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq } {
        printlock();
        eprintln!("{} {}", format!("{}", "runtime: bad lfnode address".to_string()), format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&node) as usize))).lock().unwrap().as_ref().unwrap()) as usize as u64))))));
        throw(Arc::new(Mutex::new(Some("bad lfnode address".to_string()))));
    }
}

pub fn lfstack_pack(node: Arc<Mutex<Option<lfnode>>>, cnt: Arc<Mutex<Option<usize>>>) -> u64 {
    (*Arc::new(Mutex::new(Some((*(*tagged_pointer_pack(Arc::new(Mutex::new(Some(Arc::as_ptr(&node) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = cnt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap())
}

pub fn lfstack_unpack(val: Arc<Mutex<Option<u64>>>) -> GoPtr<crate::runtime2::lfnode> {
    GoPtr::raw({ let __ptr = crate::tagptr::taggedPointer::pointer(&(crate::tagptr::taggedPointer(Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()) as u64)))))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
}