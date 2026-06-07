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

pub(crate) const __C_T_L__H_W: i32 = 6;
pub(crate) const __H_W__N_C_P_U: i32 = 3;
pub(crate) const __H_W__P_A_G_E_S_I_Z_E: i32 = 7;


pub(crate) const __N_S_I_G: i32 = 32;
pub(crate) const __S_I__U_S_E_R: i32 = 0;
pub(crate) const __S_I_G__B_L_O_C_K: i32 = 1;
pub(crate) const __S_I_G__U_N_B_L_O_C_K: i32 = 2;
pub(crate) const __S_I_G__S_E_T_M_A_S_K: i32 = 3;
pub(crate) const __S_S__D_I_S_A_B_L_E: i32 = 4;


pub(crate) const SIG_PER_THREAD_SYSCALL: i64 = 1 << 31;


#[derive(Debug, Clone)]
pub struct mOS {
    pub initialized: Arc<Mutex<Option<bool>>>,
    pub mutex: Arc<Mutex<Option<pthreadmutex>>>,
    pub cond: Arc<Mutex<Option<pthreadcond>>>,
    pub count: Arc<Mutex<Option<i32>>>,
}

impl mOS {
    pub fn __go_value_clone(&self) -> Self {
        Self { initialized: { let __guard = self.initialized.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mutex: { let __guard = self.mutex.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cond: { let __guard = self.cond.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mOS {
    fn default() -> Self {
        Self { initialized: Arc::new(Mutex::new(Some(false))), mutex: Arc::new(Mutex::new(Some(pthreadmutex::default()))), cond: Arc::new(Mutex::new(Some(pthreadcond::default()))), count: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for mOS {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.initialized.lock().unwrap().as_ref().unwrap()), (*self.mutex.lock().unwrap().as_ref().unwrap()), (*self.cond.lock().unwrap().as_ref().unwrap()), (*self.count.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mOS {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct sigset(pub Arc<Mutex<Option<u32>>>);

impl Display for sigset {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for sigset {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for sigset {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for sigset {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for sigset {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<sigset> for u32 {
    fn eq(&self, other: &sigset) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<sigset> for u32 {
    fn partial_cmp(&self, other: &sigset) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for sigset {
    type Output = sigset;
    fn add(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for sigset {
    type Output = sigset;
    fn add(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<sigset> for u32 {
    type Output = sigset;
    fn add(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for sigset {
    type Output = sigset;
    fn sub(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for sigset {
    type Output = sigset;
    fn sub(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<sigset> for u32 {
    type Output = sigset;
    fn sub(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for sigset {
    type Output = sigset;
    fn mul(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for sigset {
    type Output = sigset;
    fn mul(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<sigset> for u32 {
    type Output = sigset;
    fn mul(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for sigset {
    type Output = sigset;
    fn div(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for sigset {
    type Output = sigset;
    fn div(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<sigset> for u32 {
    type Output = sigset;
    fn div(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for sigset {
    type Output = sigset;
    fn rem(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for sigset {
    type Output = sigset;
    fn rem(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<sigset> for u32 {
    type Output = sigset;
    fn rem(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for sigset {
    type Output = sigset;
    fn bitand(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for sigset {
    type Output = sigset;
    fn bitand(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<sigset> for u32 {
    type Output = sigset;
    fn bitand(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for sigset {
    type Output = sigset;
    fn bitor(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for sigset {
    type Output = sigset;
    fn bitor(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<sigset> for u32 {
    type Output = sigset;
    fn bitor(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for sigset {
    type Output = sigset;
    fn bitxor(self, other: Self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for sigset {
    type Output = sigset;
    fn bitxor(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<sigset> for u32 {
    type Output = sigset;
    fn bitxor(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for sigset {
    type Output = sigset;
    fn not(self) -> sigset {
        sigset(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for sigset {
    type Output = sigset;
    fn shl(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for sigset {
    type Output = sigset;
    fn shl(self, other: i32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for sigset {
    type Output = sigset;
    fn shl(self, other: i8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for sigset {
    type Output = sigset;
    fn shl(self, other: i16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for sigset {
    type Output = sigset;
    fn shl(self, other: i64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for sigset {
    type Output = sigset;
    fn shl(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for sigset {
    type Output = sigset;
    fn shl(self, other: u8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for sigset {
    type Output = sigset;
    fn shl(self, other: u16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for sigset {
    type Output = sigset;
    fn shl(self, other: u64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for sigset {
    type Output = sigset;
    fn shl(self, other: usize) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for sigset {
    type Output = sigset;
    fn shr(self, other: sigset) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for sigset {
    type Output = sigset;
    fn shr(self, other: i32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for sigset {
    type Output = sigset;
    fn shr(self, other: i8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for sigset {
    type Output = sigset;
    fn shr(self, other: i16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for sigset {
    type Output = sigset;
    fn shr(self, other: i64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for sigset {
    type Output = sigset;
    fn shr(self, other: u32) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for sigset {
    type Output = sigset;
    fn shr(self, other: u8) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for sigset {
    type Output = sigset;
    fn shr(self, other: u16) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for sigset {
    type Output = sigset;
    fn shr(self, other: u64) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for sigset {
    type Output = sigset;
    fn shr(self, other: usize) -> sigset {
        sigset(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for sigset {}

impl Ord for sigset {
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


pub(crate) static sigNoteRead: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigNoteWrite: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sigset_all: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sigset>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static executablePath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<String>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *sigNoteRead.lock().unwrap() = Some(0);
    *sigNoteWrite.lock().unwrap() = Some(0);
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(0)))));
    *executablePath.lock().unwrap() = Some(String::new());
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(!0 as u32)))));
}


pub(crate) fn __go_zero_globals() {
    *sigNoteRead.lock().unwrap() = Some(0);
    *sigNoteWrite.lock().unwrap() = Some(0);
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(0)))));
    *executablePath.lock().unwrap() = Some(String::new());
}


pub(crate) fn __go_init_order_42() {
    *sigset_all.lock().unwrap() = Some(sigset(Arc::new(Mutex::new(Some(!0 as u32)))));
}


///go:nosplit
pub fn semacreate(mp: Arc<Mutex<Option<m>>>) {
    if (*(*(*mp.lock().unwrap().as_ref().unwrap()).m_o_s.lock().unwrap().as_ref().unwrap()).initialized.lock().unwrap().as_ref().unwrap()) {
        return;
    }
    { let new_val = true; *(*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).initialized.lock().unwrap() = Some(new_val); };
    {
        let mut err = pthread_mutex_init((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone(), Arc::new(Mutex::new(None)));;
        if { let __tmp_x = err; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
            throw(Arc::new(Mutex::new(Some("pthread_mutex_init".to_string()))));;
        }
    }
    {
        let mut err = pthread_cond_init((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), Arc::new(Mutex::new(None)));;
        if { let __tmp_x = err; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
            throw(Arc::new(Mutex::new(Some("pthread_cond_init".to_string()))));;
        }
    }
}

///go:nosplit
pub fn semasleep(ns: Arc<Mutex<Option<i64>>>) -> i32 {
    let mut start: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        { let new_val = nanotime(); *start.lock().unwrap() = Some(new_val); };
    }
    let mut g = getg();
    let mut mp = (*g.lock().unwrap().as_ref().unwrap()).m.clone();
    if { let __left = g.clone(); let __right = (*mp.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // sema sleep/wakeup are implemented with pthreads, which are not async-signal-safe on Darwin.
        throw(Arc::new(Mutex::new(Some("semasleep on Darwin signal stack".to_string()))));
    }
        // sema sleep/wakeup are implemented with pthreads, which are not async-signal-safe on Darwin.
    pthread_mutex_lock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
    loop {
        if { let __tmp_x = (*(*(*mp.lock().unwrap().as_ref().unwrap()).m_o_s.lock().unwrap().as_ref().unwrap()).count.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __target = (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return 0;
    }
        if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x >= __tmp_y } {
        let mut spent = Arc::new(Mutex::new(Some({ let __tmp_x = nanotime(); let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*spent.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return -(1);
    }
        let mut t: Arc<Mutex<Option<timespec>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*t.lock().unwrap().as_mut().unwrap()).set_nsec(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*spent.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }))));
        let mut err = pthread_cond_timedwait_relative_np((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone(), t.clone());
        if { let __tmp_x = err; let __tmp_y = __E_T_I_M_E_D_O_U_T as i32; __tmp_x == __tmp_y } {
        pthread_mutex_unlock((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
        return -(1);
    }
    } else {
        pthread_cond_wait((*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).cond.clone(), (*(*mp.lock().unwrap().as_mut().unwrap()).m_o_s.lock().unwrap().as_mut().unwrap()).mutex.clone());
    }
    }
}

///go:nosplit
pub fn semawakeup(mp: GoPtr<crate::runtime2::m>) {
    {
        let mut g = getg();;
        if { let __left = g.clone(); let __right = (*(*g.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).gsignal.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
            throw(Arc::new(Mutex::new(Some("semawakeup on Darwin signal stack".to_string()))));;
        }
    }
    pthread_mutex_lock({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().mutex.clone(); __field }); __ptr_value }.clone());
    { let __target = { let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().count.clone(); __field }); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    if { let __tmp_x = (*{ let __ptr_value = mp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().m_o_s.lock().unwrap().as_ref().unwrap().count.clone(); __field_value }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        pthread_cond_signal({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().cond.clone(); __field }); __ptr_value }.clone());
    }
    pthread_mutex_unlock({ let __ptr_value = mp.with_mut(|__ptr_value| { let __field = __ptr_value.m_o_s.lock().unwrap().as_ref().unwrap().mutex.clone(); __field }); __ptr_value }.clone());
}

/// sigNoteWakeup wakes up a thread sleeping on a note created by sigNoteSetup.
pub fn sig_note_wakeup(__arg0: Arc<Mutex<Option<note>>>) {
    let mut b: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    write(Arc::new(Mutex::new(Some((*sigNoteWrite.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(Arc::as_ptr(&b.clone()) as usize))), Arc::new(Mutex::new(Some(1 as i32))));
}

/// May run with m.p==nil, so write barriers are not allowed.
///
///go:nowritebarrierrec
pub fn newosproc(mp: GoPtr<crate::runtime2::m>) {
    let mut stk = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if false {
        eprint!("{}{}{}{}{}{}{}{}{}{}{}", format!("{}", "newosproc stk=".to_string()), format!("{}", { let __v = (*stk.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", " m=".to_string()), format!("{}", format!("0x{:x}", mp.addr())), format!("{}", " g=".to_string()), format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap()))), format!("{}", " id=".to_string()), format!("{}", (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().id.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", " ostk=".to_string()), format!("{}", format!("0x{:x}", &mp as *const _ as usize)), format!("{}", "\n".to_string()));
    }

        // Initialize an attribute object.
    let mut attr: Arc<Mutex<Option<pthreadattr>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = pthread_attr_init(attr.clone()); *err.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*err.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }

        // Find out OS stack size for our own stack guard.
    let mut stacksize: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = pthread_attr_getstacksize(attr.clone(), stacksize.clone()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }
    { let new_val = stacksize.lock().unwrap().as_ref().unwrap().clone(); *(*(*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.g0.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap() = Some(new_val); };

        // Tell the pthread library we won't join with this thread.
    if { let __tmp_x = pthread_attr_setdetachstate(attr.clone(), Arc::new(Mutex::new(Some(2)))); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }

        // Finally, create the thread. It starts at mstart_stub, which does some low-level
        // setup and then calls mstart.
    let mut oset: Arc<Mutex<Option<sigset>>> = Arc::new(Mutex::new(Some(sigset(Arc::new(Mutex::new(Some(0)))))));
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), sigset_all.clone(), oset.clone());
    let attr_closure_clone = attr.clone(); let mp_closure_clone = mp.clone(); { let new_val = retry_on_e_a_g_a_i_n(Arc::new(Mutex::new(Some(Box::new(move || -> i32 {
        return pthread_create(attr_closure_clone.clone(), Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(mstart_stub.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(mp.addr()))));
    }) as Box<dyn FnMut() -> i32 + Send + Sync>)))); *err.lock().unwrap() = Some(new_val); };
    sigprocmask(Arc::new(Mutex::new(Some(__S_I_G__S_E_T_M_A_S_K as u32))), oset.clone(), Arc::new(Mutex::new(None)));
    if { let __tmp_x = { let __v = (*err.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        write_err_str(Arc::new(Mutex::new(Some(FAILTHREADCREATE.to_string()))));
        exit(Arc::new(Mutex::new(Some(1 as i32))));
    }
}

/// glue code to call mstart from pthread_create.
pub fn mstart_stub() {
    unimplemented!("Go function declaration has no body");
}


/// Called to initialize a new m (including the bootstrap m).
/// Called on the parent thread (main thread in case of bootstrap), can allocate memory.
pub fn mpreinit(mp: Arc<Mutex<Option<m>>>) {
    { let new_val = malg(Arc::new(Mutex::new(Some({ let __tmp_x = 32; let __tmp_y = 1024; __tmp_x * __tmp_y } as i32)))).clone(); (*mp.lock().unwrap().as_mut().unwrap()).gsignal = new_val; };
    { let new_val = mp.clone(); (*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_mut().unwrap()).m = new_val; };
    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y } {
                // mlock the signal stack to work around a kernel bug where it may
                // SIGILL when the signal stack is not faulted in while a signal
                // arrives. See issue 42774.
        mlock(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*(*mp.lock().unwrap().as_ref().unwrap()).gsignal.lock().unwrap().as_ref().unwrap()).stack.lock().unwrap().as_ref().unwrap()).hi.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*physPageSize.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = physPageSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
}

/// Called to initialize a new m (including the bootstrap m).
/// Called on the new thread, cannot allocate memory.
pub fn minit() {
        // iOS does not support alternate signal stack.
        // The signal handler handles it directly.
    if !({ let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y }) {
        minit_signal_stack();
    }
    minit_signal_mask();
    { let new_val = Arc::new(Mutex::new(Some((*(*pthread_self().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.lock().unwrap() = __moved_val; };
}

/// Called from dropm to undo the effect of an minit.
///
///go:nosplit
pub fn unminit() {
        // iOS does not support alternate signal stack.
        // See minit.
    if !({ let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm64".to_string(); __tmp_x == __tmp_y }) {
        unminit_signals();
    }
    { let new_val = 0 as u64; *(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).procid.lock().unwrap() = Some(new_val); };
}

///go:nosplit
pub fn osyield_no_g() {
    usleep_no_g(Arc::new(Mutex::new(Some(1 as u32))));
}

///go:nosplit
pub fn osyield() {
    usleep(Arc::new(Mutex::new(Some(1 as u32))));
}

///go:nosplit
///go:nowritebarrierrec
pub fn setsig(i: Arc<Mutex<Option<u32>>>, mut r#fn: Arc<Mutex<Option<usize>>>) {
    let mut sa: Arc<Mutex<Option<usigactiont>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = { let __tmp_x = { let __tmp_x = __S_A__S_I_G_I_N_F_O; let __tmp_y = __S_A__O_N_S_T_A_C_K; __tmp_x | __tmp_y }; let __tmp_y = __S_A__R_E_S_T_A_R_T; __tmp_x | __tmp_y } as i32; *(*sa.lock().unwrap().as_ref().unwrap()).sa_flags.lock().unwrap() = Some(new_val); };
    { let new_val = !(0 as u32) as u32; *(*sa.lock().unwrap().as_ref().unwrap()).sa_mask.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = { let __v = (*r#fn.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(sighandler.clone()) as Box<dyn Any + Send + Sync>)))); __tmp_x == __tmp_y } {
        if (*iscgo.lock().unwrap().as_ref().unwrap()) {
        { let new_val = internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(cgo_sigtramp.clone()) as Box<dyn Any + Send + Sync>)))); *r#fn.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(sigtramp.clone()) as Box<dyn Any + Send + Sync>)))); *r#fn.lock().unwrap() = Some(new_val); };
    }
    }
    { unimplemented!("unsafe.Pointer dereference assignment"); };
    sigaction(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), sa.clone(), Arc::new(Mutex::new(None)));
}

/// sigtramp is the callback from libc when a signal is received.
/// It is called with the C calling convention.
pub fn sigtramp() {
    unimplemented!("Go function declaration has no body");
}


pub fn cgo_sigtramp() {
    unimplemented!("Go function declaration has no body");
}


/// setSignalstackSP sets the ss_sp field of a stackt.
///
///go:nosplit
pub fn set_signalstack_s_p(s: Arc<Mutex<Option<stackt>>>, sp: Arc<Mutex<Option<usize>>>) {
    { unimplemented!("unsafe.Pointer dereference assignment"); };
}

///go:nosplit
///go:nowritebarrierrec
pub fn sigaddset(mask: Arc<Mutex<Option<sigset>>>, i: Arc<Mutex<Option<i32>>>) {
    { let __rhs = (*({ let __tmp_x = sigset(Arc::new(Mutex::new(Some(1 as u32)))); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
}

pub fn sigdelset(mask: Arc<Mutex<Option<sigset>>>, i: Arc<Mutex<Option<i32>>>) {
    { let __rhs = (*({ let __tmp_x = sigset(Arc::new(Mutex::new(Some(1 as u32)))); let __tmp_y = ({ let __tmp_x = (*Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x - __tmp_y }); __tmp_x << __tmp_y }).0.lock().unwrap().as_ref().unwrap()).clone(); let mut guard = mask.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
}

pub fn set_thread_c_p_u_profiler(hz: Arc<Mutex<Option<i32>>>) {
    set_thread_c_p_u_profiler_hz(Arc::new(Mutex::new(Some({ let __arg_holder = hz.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

///go:nosplit
pub fn valid_s_i_g_p_r_o_f(mp: Arc<Mutex<Option<m>>>, c: Arc<Mutex<Option<sigctxt>>>) -> bool {
    true
}

pub fn signal_m(mp: GoPtr<crate::runtime2::m>, sig_local: Arc<Mutex<Option<i32>>>) {
    pthread_kill(Arc::new(Mutex::new(Some(crate::defs_darwin_arm64::pthread(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.procid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))))))), Arc::new(Mutex::new(Some((*sig_local.lock().unwrap().as_ref().unwrap()) as u32))));
}

///go:nosplit
pub fn run_per_thread_syscall() {
    throw(Arc::new(Mutex::new(Some("runPerThreadSyscall only valid on linux".to_string()))));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mOS {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
