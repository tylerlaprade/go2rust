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

pub(crate) const TRACE_EV_NONE: u8 = 0;
pub(crate) const TRACE_EV_EVENT_BATCH: u8 = 1;
pub(crate) const TRACE_EV_STACKS: u8 = 2;
pub(crate) const TRACE_EV_STACK: u8 = 3;
pub(crate) const TRACE_EV_STRINGS: u8 = 4;
pub(crate) const TRACE_EV_STRING: u8 = 5;
pub(crate) const TRACE_EV_C_P_U_SAMPLES: u8 = 6;
pub(crate) const TRACE_EV_C_P_U_SAMPLE: u8 = 7;
pub(crate) const TRACE_EV_FREQUENCY: u8 = 8;
pub(crate) const TRACE_EV_PROCS_CHANGE: u8 = 9;
pub(crate) const TRACE_EV_PROC_START: u8 = 10;
pub(crate) const TRACE_EV_PROC_STOP: u8 = 11;
pub(crate) const TRACE_EV_PROC_STEAL: u8 = 12;
pub(crate) const TRACE_EV_PROC_STATUS: u8 = 13;
pub(crate) const TRACE_EV_GO_CREATE: u8 = 14;
pub(crate) const TRACE_EV_GO_CREATE_SYSCALL: u8 = 15;
pub(crate) const TRACE_EV_GO_START: u8 = 16;
pub(crate) const TRACE_EV_GO_DESTROY: u8 = 17;
pub(crate) const TRACE_EV_GO_DESTROY_SYSCALL: u8 = 18;
pub(crate) const TRACE_EV_GO_STOP: u8 = 19;
pub(crate) const TRACE_EV_GO_BLOCK: u8 = 20;
pub(crate) const TRACE_EV_GO_UNBLOCK: u8 = 21;
pub(crate) const TRACE_EV_GO_SYSCALL_BEGIN: u8 = 22;
pub(crate) const TRACE_EV_GO_SYSCALL_END: u8 = 23;
pub(crate) const TRACE_EV_GO_SYSCALL_END_BLOCKED: u8 = 24;
pub(crate) const TRACE_EV_GO_STATUS: u8 = 25;
pub(crate) const TRACE_EV_S_T_W_BEGIN: u8 = 26;
pub(crate) const TRACE_EV_S_T_W_END: u8 = 27;
pub(crate) const TRACE_EV_G_C_ACTIVE: u8 = 28;
pub(crate) const TRACE_EV_G_C_BEGIN: u8 = 29;
pub(crate) const TRACE_EV_G_C_END: u8 = 30;
pub(crate) const TRACE_EV_G_C_SWEEP_ACTIVE: u8 = 31;
pub(crate) const TRACE_EV_G_C_SWEEP_BEGIN: u8 = 32;
pub(crate) const TRACE_EV_G_C_SWEEP_END: u8 = 33;
pub(crate) const TRACE_EV_G_C_MARK_ASSIST_ACTIVE: u8 = 34;
pub(crate) const TRACE_EV_G_C_MARK_ASSIST_BEGIN: u8 = 35;
pub(crate) const TRACE_EV_G_C_MARK_ASSIST_END: u8 = 36;
pub(crate) const TRACE_EV_HEAP_ALLOC: u8 = 37;
pub(crate) const TRACE_EV_HEAP_GOAL: u8 = 38;
pub(crate) const TRACE_EV_GO_LABEL: u8 = 39;
pub(crate) const TRACE_EV_USER_TASK_BEGIN: u8 = 40;
pub(crate) const TRACE_EV_USER_TASK_END: u8 = 41;
pub(crate) const TRACE_EV_USER_REGION_BEGIN: u8 = 42;
pub(crate) const TRACE_EV_USER_REGION_END: u8 = 43;
pub(crate) const TRACE_EV_USER_LOG: u8 = 44;
pub(crate) const TRACE_EV_GO_SWITCH: u8 = 45;
pub(crate) const TRACE_EV_GO_SWITCH_DESTROY: u8 = 46;
pub(crate) const TRACE_EV_GO_CREATE_BLOCKED: u8 = 47;
pub(crate) const TRACE_EV_GO_STATUS_STACK: u8 = 48;
pub(crate) const TRACE_EV_EXPERIMENTAL_BATCH: u8 = 49;


/// Event types in the trace, args are given in square brackets.
///
/// Naming scheme:
///   - Time range event pairs have suffixes "Begin" and "End".
///   - "Start", "Stop", "Create", "Destroy", "Block", "Unblock"
///     are suffixes reserved for scheduling resources.
///
/// NOTE: If you add an event type, make sure you also update all
/// tables in this file!
#[derive(Debug, Clone, Default)]
pub struct traceEv(pub Arc<Mutex<Option<u8>>>);

impl Display for traceEv {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceEv {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for traceEv {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceEv {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for traceEv {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceEv> for u8 {
    fn eq(&self, other: &traceEv) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceEv> for u8 {
    fn partial_cmp(&self, other: &traceEv) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceEv {
    type Output = traceEv;
    fn add(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for traceEv {
    type Output = traceEv;
    fn add(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceEv> for u8 {
    type Output = traceEv;
    fn add(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceEv {
    type Output = traceEv;
    fn sub(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for traceEv {
    type Output = traceEv;
    fn sub(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceEv> for u8 {
    type Output = traceEv;
    fn sub(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceEv {
    type Output = traceEv;
    fn mul(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for traceEv {
    type Output = traceEv;
    fn mul(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceEv> for u8 {
    type Output = traceEv;
    fn mul(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceEv {
    type Output = traceEv;
    fn div(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for traceEv {
    type Output = traceEv;
    fn div(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceEv> for u8 {
    type Output = traceEv;
    fn div(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceEv {
    type Output = traceEv;
    fn rem(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for traceEv {
    type Output = traceEv;
    fn rem(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceEv> for u8 {
    type Output = traceEv;
    fn rem(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceEv {
    type Output = traceEv;
    fn bitand(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for traceEv {
    type Output = traceEv;
    fn bitand(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceEv> for u8 {
    type Output = traceEv;
    fn bitand(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceEv {
    type Output = traceEv;
    fn bitor(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for traceEv {
    type Output = traceEv;
    fn bitor(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceEv> for u8 {
    type Output = traceEv;
    fn bitor(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceEv {
    type Output = traceEv;
    fn bitxor(self, other: Self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for traceEv {
    type Output = traceEv;
    fn bitxor(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceEv> for u8 {
    type Output = traceEv;
    fn bitxor(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceEv {
    type Output = traceEv;
    fn not(self) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceEv {
    type Output = traceEv;
    fn shl(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceEv {
    type Output = traceEv;
    fn shl(self, other: i32) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceEv {
    type Output = traceEv;
    fn shl(self, other: i8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceEv {
    type Output = traceEv;
    fn shl(self, other: i16) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceEv {
    type Output = traceEv;
    fn shl(self, other: i64) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceEv {
    type Output = traceEv;
    fn shl(self, other: u32) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceEv {
    type Output = traceEv;
    fn shl(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceEv {
    type Output = traceEv;
    fn shl(self, other: u16) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceEv {
    type Output = traceEv;
    fn shl(self, other: u64) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceEv {
    type Output = traceEv;
    fn shl(self, other: usize) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceEv {
    type Output = traceEv;
    fn shr(self, other: traceEv) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceEv {
    type Output = traceEv;
    fn shr(self, other: i32) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceEv {
    type Output = traceEv;
    fn shr(self, other: i8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceEv {
    type Output = traceEv;
    fn shr(self, other: i16) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceEv {
    type Output = traceEv;
    fn shr(self, other: i64) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceEv {
    type Output = traceEv;
    fn shr(self, other: u32) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceEv {
    type Output = traceEv;
    fn shr(self, other: u8) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceEv {
    type Output = traceEv;
    fn shr(self, other: u16) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceEv {
    type Output = traceEv;
    fn shr(self, other: u64) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceEv {
    type Output = traceEv;
    fn shr(self, other: usize) -> traceEv {
        traceEv(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceEv {}

impl Ord for traceEv {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceArg is a simple wrapper type to help ensure that arguments passed
/// to traces are well-formed.
#[derive(Debug, Clone, Default)]
pub struct traceArg(pub Arc<Mutex<Option<u64>>>);

impl Display for traceArg {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for traceArg {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for traceArg {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for traceArg {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for traceArg {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<traceArg> for u64 {
    fn eq(&self, other: &traceArg) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<traceArg> for u64 {
    fn partial_cmp(&self, other: &traceArg) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for traceArg {
    type Output = traceArg;
    fn add(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for traceArg {
    type Output = traceArg;
    fn add(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<traceArg> for u64 {
    type Output = traceArg;
    fn add(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for traceArg {
    type Output = traceArg;
    fn sub(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for traceArg {
    type Output = traceArg;
    fn sub(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<traceArg> for u64 {
    type Output = traceArg;
    fn sub(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for traceArg {
    type Output = traceArg;
    fn mul(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for traceArg {
    type Output = traceArg;
    fn mul(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<traceArg> for u64 {
    type Output = traceArg;
    fn mul(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for traceArg {
    type Output = traceArg;
    fn div(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for traceArg {
    type Output = traceArg;
    fn div(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<traceArg> for u64 {
    type Output = traceArg;
    fn div(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for traceArg {
    type Output = traceArg;
    fn rem(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for traceArg {
    type Output = traceArg;
    fn rem(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<traceArg> for u64 {
    type Output = traceArg;
    fn rem(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for traceArg {
    type Output = traceArg;
    fn bitand(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for traceArg {
    type Output = traceArg;
    fn bitand(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<traceArg> for u64 {
    type Output = traceArg;
    fn bitand(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for traceArg {
    type Output = traceArg;
    fn bitor(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for traceArg {
    type Output = traceArg;
    fn bitor(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<traceArg> for u64 {
    type Output = traceArg;
    fn bitor(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for traceArg {
    type Output = traceArg;
    fn bitxor(self, other: Self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for traceArg {
    type Output = traceArg;
    fn bitxor(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<traceArg> for u64 {
    type Output = traceArg;
    fn bitxor(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for traceArg {
    type Output = traceArg;
    fn not(self) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for traceArg {
    type Output = traceArg;
    fn shl(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for traceArg {
    type Output = traceArg;
    fn shl(self, other: i32) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for traceArg {
    type Output = traceArg;
    fn shl(self, other: i8) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for traceArg {
    type Output = traceArg;
    fn shl(self, other: i16) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for traceArg {
    type Output = traceArg;
    fn shl(self, other: i64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for traceArg {
    type Output = traceArg;
    fn shl(self, other: u32) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for traceArg {
    type Output = traceArg;
    fn shl(self, other: u8) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for traceArg {
    type Output = traceArg;
    fn shl(self, other: u16) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for traceArg {
    type Output = traceArg;
    fn shl(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for traceArg {
    type Output = traceArg;
    fn shl(self, other: usize) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for traceArg {
    type Output = traceArg;
    fn shr(self, other: traceArg) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for traceArg {
    type Output = traceArg;
    fn shr(self, other: i32) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for traceArg {
    type Output = traceArg;
    fn shr(self, other: i8) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for traceArg {
    type Output = traceArg;
    fn shr(self, other: i16) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for traceArg {
    type Output = traceArg;
    fn shr(self, other: i64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for traceArg {
    type Output = traceArg;
    fn shr(self, other: u32) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for traceArg {
    type Output = traceArg;
    fn shr(self, other: u8) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for traceArg {
    type Output = traceArg;
    fn shr(self, other: u16) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for traceArg {
    type Output = traceArg;
    fn shr(self, other: u64) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for traceArg {
    type Output = traceArg;
    fn shr(self, other: usize) -> traceArg {
        traceArg(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for traceArg {}

impl Ord for traceArg {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// traceEventWriter is the high-level API for writing trace events.
///
/// See the comment on traceWriter about style for more details as to why
/// this type and its methods are structured the way they are.
#[derive(Clone)]
pub struct traceEventWriter {
    pub tl: Arc<Mutex<Option<traceLocker>>>,
}

impl traceEventWriter {
    pub fn __go_value_clone(&self) -> Self {
        Self { tl: { let __guard = self.tl.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for traceEventWriter {
    fn default() -> Self {
        Self { tl: Arc::new(Mutex::new(Some(traceLocker::default()))) }
    }
}

impl std::fmt::Display for traceEventWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.tl.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for traceEventWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::traceruntime::traceLocker {
    /// eventWriter creates a new traceEventWriter. It is the main entrypoint for writing trace events.
    ///
    /// Before creating the event writer, this method will emit a status for the current goroutine
    /// or proc if it exists, and if it hasn't had its status emitted yet. goStatus and procStatus indicate
    /// what the status of goroutine or P should be immediately *before* the events that are about to
    /// be written using the eventWriter (if they exist). No status will be written if there's no active
    /// goroutine or P.
    ///
    /// Callers can elect to pass a constant value here if the status is clear (e.g. a goroutine must have
    /// been Runnable before a GoStart). Otherwise, callers can query the status of either the goroutine
    /// or P and pass the appropriate status.
    ///
    /// In this case, the default status should be traceGoBad or traceProcBad to help identify bugs sooner.
    pub fn event_writer(&self, goStatus: Arc<Mutex<Option<traceGoStatus>>>, procStatus: Arc<Mutex<Option<traceProcStatus>>>) -> Arc<Mutex<Option<traceEventWriter>>> {
        let mut __self = self.clone();
        {
        let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*__self.mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));;
        if !pp.is_nil() && !(*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let __recv = { let __recv = __self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_proc_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), Arc::new(Mutex::new(Some({ let __arg_holder = procStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).in_sweep.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
        {
        let mut gp: GoPtr<crate::runtime2::g> = (*__self.mp.lock().unwrap().as_ref().unwrap()).curg.clone();;
        if !gp.is_nil() && !(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).status_was_traced(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) && (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.trace.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).acquire_status(Arc::new(Mutex::new(Some({ let __selector_holder = __self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
            { let __recv = { let __recv = __self.writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).write_go_status(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), Arc::new(Mutex::new(Some({ let __selector_holder = (*__self.mp.lock().unwrap().as_ref().unwrap()).procid.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = goStatus.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.in_mark_assist.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as u64)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };;
        }
    }
                /* no stack */
        Arc::new(Mutex::new(Some(traceEventWriter { tl: Arc::new(Mutex::new(Some(__self.clone()))), ..Default::default() })))
    }

    /// stack takes a stack trace skipping the provided number of frames.
    /// It then returns a traceArg representing that stack which may be
    /// passed to write.
    pub fn stack(&self, skip: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<traceArg>>> {
        Arc::new(Mutex::new(Some(traceArg(Arc::new(Mutex::new(Some(trace_stack(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), GoPtr::nil(), Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) as u64)))))))
    }

    /// startPC takes a start PC for a goroutine and produces a unique
    /// stack ID for it.
    ///
    /// It then returns a traceArg representing that stack which may be
    /// passed to write.
    pub fn start_p_c(&self, pc: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<traceArg>>> {
                // +PCQuantum because makeTraceFrame expects return PCs and subtracts PCQuantum.
        Arc::new(Mutex::new(Some(traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).stack_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some(vec![LOGICAL_STACK_SENTINEL as usize, { let __tmp_x = start_p_c_for_trace(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }])))) as u64)))))))
    }

    /// string returns a traceArg representing s which may be passed to write.
    /// The string is assumed to be relatively short and popular, so it may be
    /// stored for a while in the string dictionary.
    pub fn string(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<traceArg>>> {
        Arc::new(Mutex::new(Some(traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)))))))
    }

    /// uniqueString returns a traceArg representing s which may be passed to write.
    /// The string is assumed to be unique or long, so it will be written out to
    /// the trace eagerly.
    pub fn unique_string(&self, s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<traceArg>>> {
        Arc::new(Mutex::new(Some(traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).string_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.emit(Arc::new(Mutex::new(Some({ let __selector_holder = self.gen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) as u64)))))))
    }

    /// rtype returns a traceArg representing typ which may be passed to write.
    pub fn rtype(&self, typ: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<traceArg>>> {
        Arc::new(Mutex::new(Some(traceArg(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*trace.lock().unwrap().as_ref().unwrap()).type_tab.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (*self.gen.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x % __tmp_y }) as usize].clone() }.put(typ.clone()) as u64)))))))
    }
}

impl traceEventWriter {
    /// event writes out a trace event.
    pub fn event(&self, ev: Arc<Mutex<Option<traceEv>>>, args: Arc<Mutex<Option<Vec<traceArg>>>>) {
        { let __recv = { let __recv = (*self.tl.lock().unwrap().as_ref().unwrap()).writer(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).event(Arc::new(Mutex::new(Some({ let __arg_holder = ev.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).end(); __result };
    }
}

impl GoValueClone for traceEventWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
