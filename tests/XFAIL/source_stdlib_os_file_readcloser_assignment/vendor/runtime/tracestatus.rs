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

pub(crate) const TRACE_GO_BAD: u8 = 0;
pub(crate) const TRACE_GO_RUNNABLE: u8 = 1;
pub(crate) const TRACE_GO_RUNNING: u8 = 2;
pub(crate) const TRACE_GO_SYSCALL: u8 = 3;
pub(crate) const TRACE_GO_WAITING: u8 = 4;


pub(crate) const TRACE_PROC_BAD: u8 = 0;
pub(crate) const TRACE_PROC_RUNNING: u8 = 1;
pub(crate) const TRACE_PROC_IDLE: u8 = 2;
pub(crate) const TRACE_PROC_SYSCALL: u8 = 3;
pub(crate) const TRACE_PROC_SYSCALL_ABANDONED: u8 = 4;


/// traceGoStatus is the status of a goroutine.
///
/// They correspond directly to the various goroutine
/// statuses.
#[derive(Debug, Clone, Default)]
pub struct traceGoStatus(pub Arc<Mutex<Option<u8>>>);

impl Display for traceGoStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceGoStatus {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceGoStatus {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceGoStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceGoStatus {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceGoStatus> for u8 {
    fn eq(&self, other: &traceGoStatus) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceGoStatus> for u8 {
    fn partial_cmp(&self, other: &traceGoStatus) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceGoStatus {
    type Output = traceGoStatus;
    fn add(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn add(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn add(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceGoStatus {
    type Output = traceGoStatus;
    fn sub(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn sub(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn sub(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceGoStatus {
    type Output = traceGoStatus;
    fn mul(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn mul(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn mul(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceGoStatus {
    type Output = traceGoStatus;
    fn div(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn div(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn div(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceGoStatus {
    type Output = traceGoStatus;
    fn rem(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn rem(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn rem(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceGoStatus {
    type Output = traceGoStatus;
    fn bitand(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn bitand(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn bitand(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceGoStatus {
    type Output = traceGoStatus;
    fn bitor(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn bitor(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn bitor(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceGoStatus {
    type Output = traceGoStatus;
    fn bitxor(self, other: Self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn bitxor(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceGoStatus> for u8 {
    type Output = traceGoStatus;
    fn bitxor(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceGoStatus {
    type Output = traceGoStatus;
    fn not(self) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: i32) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: i8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: i16) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: i64) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: u32) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: u16) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: u64) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceGoStatus {
    type Output = traceGoStatus;
    fn shl(self, other: usize) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: traceGoStatus) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: i32) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: i8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: i16) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: i64) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: u32) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: u8) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: u16) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: u64) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceGoStatus {
    type Output = traceGoStatus;
    fn shr(self, other: usize) -> traceGoStatus {
        traceGoStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceGoStatus {}

impl Ord for traceGoStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceProcStatus is the status of a P.
///
/// They mostly correspond to the various P statuses.
#[derive(Debug, Clone, Default)]
pub struct traceProcStatus(pub Arc<Mutex<Option<u8>>>);

impl Display for traceProcStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceProcStatus {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceProcStatus {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceProcStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceProcStatus {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceProcStatus> for u8 {
    fn eq(&self, other: &traceProcStatus) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceProcStatus> for u8 {
    fn partial_cmp(&self, other: &traceProcStatus) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceProcStatus {
    type Output = traceProcStatus;
    fn add(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn add(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn add(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceProcStatus {
    type Output = traceProcStatus;
    fn sub(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn sub(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn sub(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceProcStatus {
    type Output = traceProcStatus;
    fn mul(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn mul(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn mul(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceProcStatus {
    type Output = traceProcStatus;
    fn div(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn div(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn div(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceProcStatus {
    type Output = traceProcStatus;
    fn rem(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn rem(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn rem(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceProcStatus {
    type Output = traceProcStatus;
    fn bitand(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn bitand(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn bitand(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceProcStatus {
    type Output = traceProcStatus;
    fn bitor(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn bitor(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn bitor(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceProcStatus {
    type Output = traceProcStatus;
    fn bitxor(self, other: Self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn bitxor(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceProcStatus> for u8 {
    type Output = traceProcStatus;
    fn bitxor(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceProcStatus {
    type Output = traceProcStatus;
    fn not(self) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: i32) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: i8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: i16) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: i64) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: u32) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: u16) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: u64) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceProcStatus {
    type Output = traceProcStatus;
    fn shl(self, other: usize) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: traceProcStatus) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: i32) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: i8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: i16) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: i64) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: u32) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: u8) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: u16) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: u64) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceProcStatus {
    type Output = traceProcStatus;
    fn shr(self, other: usize) -> traceProcStatus {
        traceProcStatus(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceProcStatus {}

impl Ord for traceProcStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceSchedResourceState is shared state for scheduling resources (i.e. fields common to
/// both Gs and Ps).
#[derive(Clone)]
pub struct traceSchedResourceState {
    pub status_traced: Arc<Mutex<Option<[internal_runtime_atomic::types::Uint32; 3]>>>,
    pub seq: Arc<Mutex<Option<[u64; 2]>>>,
}

impl traceSchedResourceState {
    pub fn __go_value_clone(&self) -> Self {
        Self { status_traced: { let __guard = self.status_traced.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, seq: { let __guard = self.seq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceSchedResourceState {
    fn default() -> Self {
        Self { status_traced: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), seq: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))) }
    }
}

impl std::fmt::Display for traceSchedResourceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice(&self.status_traced), format_slice(&self.seq))
    }
}

impl GoJsonDecode for traceSchedResourceState {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::tracebuf::traceWriter {
    /// writeGoStatus emits a GoStatus event as well as any active ranges on the goroutine.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn write_go_status(&self, goid: Arc<Mutex<Option<u64>>>, mid: Arc<Mutex<Option<i64>>>, status: Arc<Mutex<Option<traceGoStatus>>>, markAssist: Arc<Mutex<Option<bool>>>, stackID: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
        let mut __self = self.clone();
                // The status should never be bad. Some invariant must have been violated.
        if { let __tmp_x = (*status.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_BAD as u8)))); __tmp_x == __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: goid=".to_string()), format!("{}", { let __v = (*goid.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("attempted to trace a bad status for a goroutine".to_string()))));
    }
                // Trace the status.
        if { let __tmp_x = { let __v = (*stackID.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        { let new_val = __self.event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_STATUS as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*goid.lock().unwrap().as_ref().unwrap()) as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*mid.lock().unwrap().as_ref().unwrap()) as u64 as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    } else {
        { let new_val = __self.event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_GO_STATUS_STACK as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*goid.lock().unwrap().as_ref().unwrap()) as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*mid.lock().unwrap().as_ref().unwrap()) as u64 as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*stackID.lock().unwrap().as_ref().unwrap()) as u64))))])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
                // Trace any special ranges that are in-progress.
        if { let __v = (*markAssist.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = __self.event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_MARK_ASSIST_ACTIVE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*goid.lock().unwrap().as_ref().unwrap()) as u64))))])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// writeProcStatusForP emits a ProcStatus event for the provided p based on its status.
    ///
    /// The caller must fully own pp and it must be prevented from transitioning (e.g. this can be
    /// called by a forEachP callback or from a STW).
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn write_proc_status_for_p(&self, pp: GoPtr<crate::runtime2::p>, inSTW: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
        let mut __self = self.clone();
        if !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __selector_holder = (*__self.trace_locker.lock().unwrap().as_ref().unwrap()).gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        let mut status: Arc<Mutex<Option<traceProcStatus>>> = Arc::new(Mutex::new(Some(traceProcStatus(Arc::new(Mutex::new(Some(0)))))));
        { let _switch_val = { let __v = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.status.clone()); __ptr_value }.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (__PIDLE as u32) || _switch_val == (__PGCSTOP as u32) {
            { let new_val = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_IDLE as u8)))); *status.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().status.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __PGCSTOP as u32; __tmp_x == __tmp_y } && { let __v = (*inSTW.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // N.B. a P that is running and currently has the world stopped will be
                // in _Pgcstop, but we model it as running in the tracer.
        { let new_val = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))); *status.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (__PRUNNING as u32) {
            { let new_val = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_RUNNING as u8)))); *status.lock().unwrap() = Some(new_val); };
                        // There's a short window wherein the goroutine may have entered _Gsyscall
                        // but it still owns the P (it's not in _Psyscall yet). The goroutine entering
                        // _Gsyscall is the tracer's signal that the P its bound to is also in a syscall,
                        // so we need to emit a status that matches. See #64318.
            if { let __left_addr = crate::runtime2::puintptr::ptr(&(*(*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).addr(); let __right_addr = pp.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __ptr_field = (*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() } && { let __tmp_x = { let __tmp_x = readgstatus((*(*__self.trace_locker.lock().unwrap().as_ref().unwrap()).mp.lock().unwrap().as_ref().unwrap()).curg.clone()); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GSYSCALL as u32; __tmp_x == __tmp_y } {
        { let new_val = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL as u8)))); *status.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (__PSYSCALL as u32) {
            { let new_val = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_SYSCALL as u8)))); *status.lock().unwrap() = Some(new_val); };
        } else {
            throw(Arc::new(Mutex::new(Some("attempt to trace invalid or unsupported P status".to_string()))));
        }
    }
                // N.B. a P that is running and currently has the world stopped will be
                // in _Pgcstop, but we model it as running in the tracer.
                // There's a short window wherein the goroutine may have entered _Gsyscall
                // but it still owns the P (it's not in _Psyscall yet). The goroutine entering
                // _Gsyscall is the tracer's signal that the P its bound to is also in a syscall,
                // so we need to emit a status that matches. See #64318.
        { let new_val = __self.write_proc_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), Arc::new(Mutex::new(Some({ let __arg_holder = status.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    /// writeProcStatus emits a ProcStatus event with all the provided information.
    ///
    /// The caller must have taken ownership of a P's status writing, and the P must be
    /// prevented from transitioning.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn write_proc_status(&self, pid: Arc<Mutex<Option<u64>>>, status: Arc<Mutex<Option<traceProcStatus>>>, inSweep: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<crate::tracebuf::traceWriter>>> {
        let mut __self = self.clone();
                // The status should never be bad. Some invariant must have been violated.
        if { let __tmp_x = (*status.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = traceProcStatus(Arc::new(Mutex::new(Some(TRACE_PROC_BAD as u8)))); __tmp_x == __tmp_y } {
        eprint!("{}{}{}", format!("{}", "runtime: pid=".to_string()), format!("{}", { let __v = (*pid.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("attempted to trace a bad status for a proc".to_string()))));
    }
                // Trace the status.
        { let new_val = __self.event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_PROC_STATUS as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*pid.lock().unwrap().as_ref().unwrap()) as u64)))), crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*{ let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
                // Trace any special ranges that are in-progress.
        if { let __v = (*inSweep.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = __self.event(Arc::new(Mutex::new(Some(crate::traceevent::traceEv(Arc::new(Mutex::new(Some(TRACE_EV_G_C_SWEEP_ACTIVE as u8))))))), Arc::new(Mutex::new(Some(vec![crate::traceevent::traceArg(Arc::new(Mutex::new(Some((*pid.lock().unwrap().as_ref().unwrap()) as u64))))])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        Arc::new(Mutex::new(Some(__self.clone())))
    }
}

impl traceSchedResourceState {
    /// acquireStatus acquires the right to emit a Status event for the scheduling resource.
    ///
    /// nosplit because it's part of writing an event for an M, which must not
    /// have any stack growth.
    ///
    ///go:nosplit
    pub fn acquire_status(&mut self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        if !{ let __seq = { let __seq_holder = self.status_traced.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as usize; __tmp_x % __tmp_y }) as usize].clone() }.compare_and_swap(Arc::new(Mutex::new(Some(0 as u32))), Arc::new(Mutex::new(Some(1 as u32)))) {
        return false;
    }
        self.ready_next_gen(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        true
    }

    /// readyNextGen readies r for the generation following gen.
    pub fn ready_next_gen(&mut self, gen: Arc<Mutex<Option<usize>>>) {
        let mut nextGen = trace_next_gen(Arc::new(Mutex::new(Some({ let __arg_holder = gen.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*self.seq.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = nextGen; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize] = 0 as u64;
        { let __seq = { let __seq_holder = self.status_traced.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = nextGen; let __tmp_y = 3 as usize; __tmp_x % __tmp_y }) as usize].clone() }.store(Arc::new(Mutex::new(Some(0 as u32))));
    }

    /// statusWasTraced returns true if the sched resource's status was already acquired for tracing.
    pub fn status_was_traced(&self, gen: Arc<Mutex<Option<usize>>>) -> bool {
        return { let __tmp_x = { let __seq = { let __seq_holder = self.status_traced.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as usize; __tmp_x % __tmp_y }) as usize].clone() }.load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }

    /// setStatusTraced indicates that the resource's status was already traced, for example
    /// when a goroutine is created.
    pub fn set_status_traced(&self, gen: Arc<Mutex<Option<usize>>>) {
        { let __seq = { let __seq_holder = self.status_traced.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3 as usize; __tmp_x % __tmp_y }) as usize].clone() }.store(Arc::new(Mutex::new(Some(1 as u32))));
    }

    /// nextSeq returns the next sequence number for the resource.
    pub fn next_seq(&mut self, gen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::traceevent::traceArg>>> {
        { let __idx = { let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y } as usize; let mut __seq_guard = self.seq.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
        Arc::new(Mutex::new(Some(crate::traceevent::traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.seq.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*gen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() } as u64)))))))
    }
}

/// goStatusToTraceGoStatus translates the internal status to tracGoStatus.
///
/// status must not be _Gdead or any status whose name has the suffix "_unused."
///
/// nosplit because it's part of writing an event for an M, which must not
/// have any stack growth.
///
///go:nosplit
pub fn go_status_to_trace_go_status(status: Arc<Mutex<Option<u32>>>, wr: Arc<Mutex<Option<waitReason>>>) -> Arc<Mutex<Option<traceGoStatus>>> {
        // N.B. Ignore the _Gscan bit. We don't model it in the tracer.
    let mut tgs: Arc<Mutex<Option<traceGoStatus>>> = Arc::new(Mutex::new(Some(traceGoStatus(Arc::new(Mutex::new(Some(0)))))));
    { let _switch_val = { let __tmp_x = { let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y };
    if _switch_val == (__GRUNNABLE as u32) {
            { let new_val = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNABLE as u8)))); *tgs.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GRUNNING as u32) || _switch_val == (__GCOPYSTACK as u32) {
            { let new_val = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8)))); *tgs.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GSYSCALL as u32) {
            { let new_val = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_SYSCALL as u8)))); *tgs.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (__GWAITING as u32) || _switch_val == (__GPREEMPTED as u32) {
                        // There are a number of cases where a G might end up in
                        // _Gwaiting but it's actually running in a non-preemptive
                        // state but needs to present itself as preempted to the
                        // garbage collector and traceAdvance (via suspendG). In
                        // these cases, we're not going to emit an event, and we
                        // want these goroutines to appear in the final trace as
                        // if they're running, not blocked.
            { let new_val = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_WAITING as u8)))); *tgs.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = { let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y } && crate::runtime2::waitReason::is_waiting_for_suspend_g(&(*wr.lock().unwrap().as_ref().unwrap())) {
        { let new_val = traceGoStatus(Arc::new(Mutex::new(Some(TRACE_GO_RUNNING as u8)))); *tgs.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (__GDEAD as u32) {
            throw(Arc::new(Mutex::new(Some("tried to trace dead goroutine".to_string()))));
        } else {
            throw(Arc::new(Mutex::new(Some("tried to trace goroutine with invalid or unsupported status".to_string()))));
        }
    }
        // There are a number of cases where a G might end up in
        // _Gwaiting but it's actually running in a non-preemptive
        // state but needs to present itself as preempted to the
        // garbage collector and traceAdvance (via suspendG). In
        // these cases, we're not going to emit an event, and we
        // want these goroutines to appear in the final trace as
        // if they're running, not blocked.
    return { let __owned = tgs.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

impl GoValueClone for traceSchedResourceState {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
