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

pub(crate) const POLL_NO_ERROR: i32 = 0;
pub(crate) const POLL_ERR_CLOSING: i32 = 1;
pub(crate) const POLL_ERR_TIMEOUT: i32 = 2;
pub(crate) const POLL_ERR_NOT_POLLABLE: i32 = 3;


pub(crate) const PD_NIL: usize = 0;
pub(crate) const PD_READY: usize = 1;
pub(crate) const PD_WAIT: usize = 2;


pub(crate) const POLL_BLOCK_SIZE: i32 = 4 * 1024;


pub(crate) const POLL_CLOSING: i32 = 1 << 0;
pub(crate) const POLL_EVENT_ERR: i32 = 1 << 1;
pub(crate) const POLL_EXPIRED_READ_DEADLINE: i32 = 1 << 2;
pub(crate) const POLL_EXPIRED_WRITE_DEADLINE: i32 = 1 << 3;
pub(crate) const POLL_F_D_SEQ: i32 = 1 << 4;


pub(crate) const POLL_F_D_SEQ_BITS: i32 = 20;
pub(crate) const POLL_F_D_SEQ_MASK: i32 = (1 << POLL_F_D_SEQ_BITS) - 1;


/// Network poller descriptor.
///
/// No heap pointers.
#[derive(Clone)]
pub struct pollDesc {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub link: Arc<Mutex<Option<pollDesc>>>,
    pub fd: Arc<Mutex<Option<usize>>>,
    pub fdseq: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub atomic_info: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub rg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub wg: Arc<Mutex<Option<internal_runtime_atomic::types::Uintptr>>>,
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub closing: Arc<Mutex<Option<bool>>>,
    pub rrun: Arc<Mutex<Option<bool>>>,
    pub wrun: Arc<Mutex<Option<bool>>>,
    pub user: Arc<Mutex<Option<u32>>>,
    pub rseq: Arc<Mutex<Option<usize>>>,
    pub rt: Arc<Mutex<Option<timer>>>,
    pub rd: Arc<Mutex<Option<i64>>>,
    pub wseq: Arc<Mutex<Option<usize>>>,
    pub wt: Arc<Mutex<Option<timer>>>,
    pub wd: Arc<Mutex<Option<i64>>>,
    pub self_: GoPtr<pollDesc>,
}

impl pollDesc {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, link: self.link.clone(), fd: { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fdseq: { let __guard = self.fdseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, atomic_info: { let __guard = self.atomic_info.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rg: { let __guard = self.rg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wg: { let __guard = self.wg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, closing: { let __guard = self.closing.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rrun: { let __guard = self.rrun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wrun: { let __guard = self.wrun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, user: { let __guard = self.user.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rseq: { let __guard = self.rseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rt: { let __guard = self.rt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rd: { let __guard = self.rd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wseq: { let __guard = self.wseq.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wt: { let __guard = self.wt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wd: { let __guard = self.wd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, self_: self.self_.clone() }
    }
}


impl Default for pollDesc {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), link: Arc::new(Mutex::new(None)), fd: Arc::new(Mutex::new(Some(0))), fdseq: Arc::new(Mutex::new(Some(Default::default()))), atomic_info: Arc::new(Mutex::new(Some(Default::default()))), rg: Arc::new(Mutex::new(Some(Default::default()))), wg: Arc::new(Mutex::new(Some(Default::default()))), lock: Arc::new(Mutex::new(Some(mutex::default()))), closing: Arc::new(Mutex::new(Some(false))), rrun: Arc::new(Mutex::new(Some(false))), wrun: Arc::new(Mutex::new(Some(false))), user: Arc::new(Mutex::new(Some(0))), rseq: Arc::new(Mutex::new(Some(0))), rt: Arc::new(Mutex::new(Some(timer::default()))), rd: Arc::new(Mutex::new(Some(0))), wseq: Arc::new(Mutex::new(Some(0))), wt: Arc::new(Mutex::new(Some(timer::default()))), wd: Arc::new(Mutex::new(Some(0))), self_: GoPtr::nil() }
    }
}

impl std::fmt::Display for pollDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { let __guard = self.link.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.fd.lock().unwrap().as_ref().unwrap()), (*self.fdseq.lock().unwrap().as_ref().unwrap()), (*self.atomic_info.lock().unwrap().as_ref().unwrap()), (*self.rg.lock().unwrap().as_ref().unwrap()), (*self.wg.lock().unwrap().as_ref().unwrap()), (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.closing.lock().unwrap().as_ref().unwrap()), (*self.rrun.lock().unwrap().as_ref().unwrap()), (*self.wrun.lock().unwrap().as_ref().unwrap()), (*self.user.lock().unwrap().as_ref().unwrap()), (*self.rseq.lock().unwrap().as_ref().unwrap()), (*self.rt.lock().unwrap().as_ref().unwrap()), (*self.rd.lock().unwrap().as_ref().unwrap()), (*self.wseq.lock().unwrap().as_ref().unwrap()), (*self.wt.lock().unwrap().as_ref().unwrap()), (*self.wd.lock().unwrap().as_ref().unwrap()), { if self.self_.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for pollDesc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// pollInfo is the bits needed by netpollcheckerr, stored atomically,
/// mostly duplicating state that is manipulated under lock in pollDesc.
/// The one exception is the pollEventErr bit, which is maintained only
/// in the pollInfo.
#[derive(Debug, Clone, Default)]
pub struct pollInfo(pub Arc<Mutex<Option<u32>>>);

impl Display for pollInfo {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for pollInfo {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for pollInfo {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for pollInfo {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for pollInfo {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<pollInfo> for u32 {
    fn eq(&self, other: &pollInfo) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<pollInfo> for u32 {
    fn partial_cmp(&self, other: &pollInfo) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for pollInfo {
    type Output = pollInfo;
    fn add(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for pollInfo {
    type Output = pollInfo;
    fn add(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<pollInfo> for u32 {
    type Output = pollInfo;
    fn add(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for pollInfo {
    type Output = pollInfo;
    fn sub(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for pollInfo {
    type Output = pollInfo;
    fn sub(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<pollInfo> for u32 {
    type Output = pollInfo;
    fn sub(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for pollInfo {
    type Output = pollInfo;
    fn mul(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for pollInfo {
    type Output = pollInfo;
    fn mul(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<pollInfo> for u32 {
    type Output = pollInfo;
    fn mul(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for pollInfo {
    type Output = pollInfo;
    fn div(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for pollInfo {
    type Output = pollInfo;
    fn div(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<pollInfo> for u32 {
    type Output = pollInfo;
    fn div(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for pollInfo {
    type Output = pollInfo;
    fn rem(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for pollInfo {
    type Output = pollInfo;
    fn rem(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<pollInfo> for u32 {
    type Output = pollInfo;
    fn rem(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for pollInfo {
    type Output = pollInfo;
    fn bitand(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for pollInfo {
    type Output = pollInfo;
    fn bitand(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitand(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for pollInfo {
    type Output = pollInfo;
    fn bitor(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for pollInfo {
    type Output = pollInfo;
    fn bitor(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitor(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for pollInfo {
    type Output = pollInfo;
    fn bitxor(self, other: Self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for pollInfo {
    type Output = pollInfo;
    fn bitxor(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<pollInfo> for u32 {
    type Output = pollInfo;
    fn bitxor(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for pollInfo {
    type Output = pollInfo;
    fn not(self) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: i64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: u64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for pollInfo {
    type Output = pollInfo;
    fn shl(self, other: usize) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: pollInfo) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: i64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u32) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u8) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u16) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: u64) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for pollInfo {
    type Output = pollInfo;
    fn shr(self, other: usize) -> pollInfo {
        pollInfo(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for pollInfo {}

impl Ord for pollInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct pollCache {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub first: GoPtr<pollDesc>,
}

impl pollCache {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first: self.first.clone() }
    }
}


impl Default for pollCache {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), first: GoPtr::nil() }
    }
}

impl std::fmt::Display for pollCache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), { if self.first.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } })
    }
}

impl GoJsonDecode for pollCache {
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


pub(crate) static netpollInitLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static netpollInited: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pollcache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<pollCache>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static netpollWaiters: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::Uint32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pdEface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static pdType: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<internal_abi::r#type::Type>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *netpollInitLock.lock().unwrap() = Some(Default::default());
    *netpollInited.lock().unwrap() = Some(Default::default());
    *pollcache.lock().unwrap() = Some(Default::default());
    *netpollWaiters.lock().unwrap() = Some(Default::default());
    *pdEface.lock().unwrap() = None;
    *pdType.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *pdEface.lock().unwrap() = Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<pollDesc>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<pollDesc>>>>("pointer", true, "struct", false); __boxed });
    *pdType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(pdEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_zero_globals() {
    *netpollInitLock.lock().unwrap() = Some(Default::default());
    *netpollInited.lock().unwrap() = Some(Default::default());
    *pollcache.lock().unwrap() = Some(Default::default());
    *netpollWaiters.lock().unwrap() = Some(Default::default());
    *pdEface.lock().unwrap() = None;
    *pdType.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_38() {
    *pdEface.lock().unwrap() = Some({ let __boxed = Box::new(Arc::new(Mutex::new(None::<pollDesc>))) as Box<dyn Any + Send + Sync>; go_register_any_type_with_elem::<Arc<Mutex<Option<pollDesc>>>>("pointer", true, "struct", false); __boxed });
}


pub(crate) fn __go_init_order_39() {
    *pdType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(pdEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


impl pollInfo {
    pub fn closing(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_CLOSING as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn event_err(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EVENT_ERR as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn expired_read_deadline(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EXPIRED_READ_DEADLINE as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    pub fn expired_write_deadline(&self) -> bool {
        return { let __tmp_x = pollInfo(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & POLL_EXPIRED_WRITE_DEADLINE as u32))))); let __tmp_y = pollInfo(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }
}

impl pollDesc {
    /// info returns the pollInfo corresponding to pd.
    pub fn info(&self) -> Arc<Mutex<Option<pollInfo>>> {
        Arc::new(Mutex::new(Some(pollInfo(Arc::new(Mutex::new(Some((*self.atomic_info.lock().unwrap().as_mut().unwrap()).load() as u32)))))))
    }

    /// publishInfo updates pd.atomicInfo (returned by pd.info)
    /// using the other values in pd.
    /// It must be called while holding pd.lock,
    /// and it must be called after changing anything
    /// that might affect the info bits.
    /// In practice this means after changing closing
    /// or changing rd or wd from < 0 to >= 0.
    pub fn publish_info(&self) {
        let mut info: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        if (*self.closing.clone().lock().unwrap().as_ref().unwrap()) {
        { let __rhs = POLL_CLOSING as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = (*self.rd.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __rhs = POLL_EXPIRED_READ_DEADLINE as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = (*self.wd.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let __rhs = POLL_EXPIRED_WRITE_DEADLINE as u32; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.fdseq.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = POLL_F_D_SEQ_MASK as usize; __tmp_x & __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = POLL_F_D_SEQ; __tmp_x << __tmp_y }; let mut guard = info.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
                // Set all of x except the pollEventErr bit.
        let mut x = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load();
        while !(*self.atomic_info.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(x))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x & __tmp_y }); let __tmp_y = { let __v = (*info.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })))) {
        { let new_val = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load(); x = new_val; };
    }
    }

    /// setEventErr sets the result of pd.info().eventErr() to b.
    /// We only change the error bit if seq == 0 or if seq matches pollFDSeq
    /// (issue #59545).
    pub fn set_event_err(&self, b: Arc<Mutex<Option<bool>>>, seq: Arc<Mutex<Option<usize>>>) {
        let mut mSeq = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = POLL_F_D_SEQ_MASK as usize; __tmp_x & __tmp_y }) as u32)));
        let mut x = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load();
        let mut xSeq = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_F_D_SEQ; __tmp_x >> __tmp_y }); let __tmp_y = POLL_F_D_SEQ_MASK as u32; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*xSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return;
    }
        while { let __tmp_x = ({ let __tmp_x = { let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }); let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } && !(*self.atomic_info.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(x))), Arc::new(Mutex::new(Some({ let __tmp_x = x; let __tmp_y = POLL_EVENT_ERR as u32; __tmp_x ^ __tmp_y })))) {
        { let new_val = (*self.atomic_info.lock().unwrap().as_mut().unwrap()).load(); x = new_val; };
        let mut xSeq = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = x; let __tmp_y = POLL_F_D_SEQ; __tmp_x >> __tmp_y }); let __tmp_y = POLL_F_D_SEQ_MASK as u32; __tmp_x & __tmp_y })));
        if { let __tmp_x = { let __v = (*seq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*xSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*mSeq.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        return;
    }
    }
    }

    /// makeArg converts pd to an interface{}.
    /// makeArg does not do any allocation. Normally, such
    /// a conversion requires an allocation because pointers to
    /// types which embed internal/runtime/sys.NotInHeap (which pollDesc is)
    /// must be stored in interfaces indirectly. See issue 42076.
    pub fn make_arg(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let mut i: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut x: GoPtr<crate::runtime2::eface> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&i.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = GoPtr::local((*pdType.lock().unwrap().as_ref().unwrap()).clone()); x.with_mut(|__ptr_value| { __ptr_value._type = new_val; }); };
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(self.self_.clone())))) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = x.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
        i.clone()
    }
}

impl pollCache {
    pub fn free(&mut self, pd: GoPtr<pollDesc>) {
                // pd can't be shared here, but lock anyhow because
                // that's what publishInfo documents.
        lock(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
                // Increment the fdseq field, so that any currently
                // running netpoll calls will not mark pd as ready.
        let mut fdseq = (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.fdseq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).load();
        { let new_val = { let __tmp_x = ({ let __tmp_x = fdseq; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }); let __tmp_y = (((1 as usize) << (TAGGED_POINTER_BITS as usize)) - (1 as usize)) as usize; __tmp_x & __tmp_y }; fdseq = new_val; };
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.fdseq.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(fdseq))));
        { let __recv_value = pd.borrow(); let __result = (*__recv_value.as_ref().unwrap()).publish_info(); __result };
        unlock(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()));
        lock(GoPtr::local(self.lock.clone()));
        { let new_val = self.first.clone(); pd.with_mut(|__ptr_value| { __ptr_value.link = new_val; }); };
        { let new_val = pd.clone(); self.first = new_val; };
        unlock(GoPtr::local(self.lock.clone()));
    }

    pub fn alloc(&mut self) -> GoPtr<pollDesc> {
        lock(GoPtr::local(self.lock.clone()));
        if { let __ptr_field = self.first.clone(); __ptr_field.is_nil() } {
        const pdSize: usize = std::mem::size_of::<pollDesc>();

        let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = POLL_BLOCK_SIZE as usize; let __tmp_y = pdSize as usize; __tmp_x / __tmp_y })));
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = 1 as usize; *n.lock().unwrap() = Some(new_val); };
    }
                // Must be in non-GC memory because can be referenced
                // only from epoll/kqueue internals.
        let mut mem = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pdSize as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone());
        let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut pd: GoPtr<pollDesc> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some({ let __arg_holder = mem.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pdSize as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        lock_init(GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.lock.clone()); __ptr_value }.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_DESC as i32))))))));
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.rt.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
        (*{ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.wt.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
        { let new_val = self.first.clone(); pd.with_mut(|__ptr_value| { __ptr_value.link = new_val; }); };
        { let new_val = pd.clone(); self.first = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Must be in non-GC memory because can be referenced
                // only from epoll/kqueue internals.
        let mut pd: GoPtr<pollDesc> = self.first.clone();
        { let new_val = GoPtr::local({ let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.link.clone()); __ptr_value }.clone()); self.first = new_val; };
        unlock(GoPtr::local(self.lock.clone()));
        pd.clone()
    }
}

pub fn netpoll_generic_init() {
    if { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        lock_init(GoPtr::local(netpollInitLock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_NETPOLL_INIT as i32))))))));
        lock_init(GoPtr::local((*pollcache.lock().unwrap().as_ref().unwrap()).lock.clone()), Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(LOCK_RANK_POLL_CACHE as i32))))))));
        lock(GoPtr::local(netpollInitLock.clone()));
        if { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        netpollinit();
        (*netpollInited.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
    }
        unlock(GoPtr::local(netpollInitLock.clone()));
    }
}

pub fn netpollinited() -> bool {
    return { let __tmp_x = (*netpollInited.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
}

/// netpollready is called by the platform-specific netpoll function.
/// It declares that the fd associated with pd is ready for I/O.
/// The toRun argument is used to build a list of goroutines to return
/// from netpoll. The mode argument is 'r', 'w', or 'r'+'w' to indicate
/// whether the fd is ready for reading or writing or both.
///
/// This returns a delta to apply to netpollWaiters.
///
/// This may run while the world is stopped, so write barriers are not allowed.
///
///go:nowritebarrier
pub fn netpollready(toRun: Arc<Mutex<Option<gList>>>, pd: GoPtr<pollDesc>, mode: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut delta = Arc::new(Mutex::new(Some(0 as i32)));
    let mut rg: GoPtr<crate::runtime2::g> = GoPtr::nil();let mut wg: GoPtr<crate::runtime2::g> = GoPtr::nil();
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('r' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ('r' as i32); let __tmp_y = ('w' as i32); __tmp_x + __tmp_y } as i32; __tmp_x == __tmp_y } {
        rg = netpollunblock(pd.clone(), Arc::new(Mutex::new(Some(('r' as i32) as i32))), Arc::new(Mutex::new(Some(true))), delta.clone());
    }
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('w' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ('r' as i32); let __tmp_y = ('w' as i32); __tmp_x + __tmp_y } as i32; __tmp_x == __tmp_y } {
        wg = netpollunblock(pd.clone(), Arc::new(Mutex::new(Some(('w' as i32) as i32))), Arc::new(Mutex::new(Some(true))), delta.clone());
    }
    if !rg.is_nil() {
        { let __recv = toRun.clone(); let __recv_ptr: *const crate::proc::gList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::proc::gList }; let __result = unsafe { &*__recv_ptr }.push(rg.clone()); __result };
    }
    if !wg.is_nil() {
        { let __recv = toRun.clone(); let __recv_ptr: *const crate::proc::gList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::proc::gList }; let __result = unsafe { &*__recv_ptr }.push(wg.clone()); __result };
    }
    return { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// netpollunblock moves either pd.rg (if mode == 'r') or
/// pd.wg (if mode == 'w') into the pdReady state.
/// This returns any goroutine blocked on pd.{rg,wg}.
/// It adds any adjustment to netpollWaiters to *delta;
/// this adjustment should be applied after the goroutine has
/// been marked ready.
pub fn netpollunblock(pd: GoPtr<pollDesc>, mode: Arc<Mutex<Option<i32>>>, ioready: Arc<Mutex<Option<bool>>>, delta: Arc<Mutex<Option<i32>>>) -> GoPtr<crate::runtime2::g> {
    let mut gpp = { let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.rg.clone()); __ptr_value }.clone();
    if { let __tmp_x = { let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('w' as i32); __tmp_x == __tmp_y } {
        { let new_val = { let __ptr_value = pd.with_mut(|__ptr_value| __ptr_value.wg.clone()); __ptr_value }.clone().clone(); gpp = new_val; };
    }

    loop {
        let mut old = { let __recv = gpp.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Uintptr = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Uintptr }; let __result = unsafe { &mut *__recv_ptr }.load(); __result };
        if { let __tmp_x = old; let __tmp_y = PD_READY as usize; __tmp_x == __tmp_y } {
        return GoPtr::nil();
    }
        if { let __tmp_x = old; let __tmp_y = PD_NIL as usize; __tmp_x == __tmp_y } && !{ let __v = (*ioready.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Only set pdReady for ioready. runtime_pollWait
                // will check for timeout/cancel before waiting.
        return GoPtr::nil();
    }
                // Only set pdReady for ioready. runtime_pollWait
                // will check for timeout/cancel before waiting.
        let mut new = Arc::new(Mutex::new(Some(PD_NIL)));
        if { let __v = (*ioready.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = PD_READY as usize; *new.lock().unwrap() = Some(new_val); };
    }
        if { let __recv = gpp.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::Uintptr = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::Uintptr }; let __result = unsafe { &mut *__recv_ptr }.compare_and_swap(Arc::new(Mutex::new(Some(old))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        if { let __tmp_x = old; let __tmp_y = PD_WAIT as usize; __tmp_x == __tmp_y } {
        { let new_val = PD_NIL as usize; old = new_val; };
    } else if { let __tmp_x = old; let __tmp_y = PD_NIL as usize; __tmp_x != __tmp_y } {
        { let __rhs = 1 as i32; let mut guard = delta.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        return GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(old))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
    }
}

/// netpollAnyWaiters reports whether any goroutines are waiting for I/O.
pub fn netpoll_any_waiters() -> bool {
    return { let __tmp_x = (*netpollWaiters.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y };
}

/// netpollAdjustWaiters adds delta to netpollWaiters.
pub fn netpoll_adjust_waiters(delta: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = { let __v = (*delta.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        (*netpollWaiters.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = delta.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for pollDesc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pollCache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
