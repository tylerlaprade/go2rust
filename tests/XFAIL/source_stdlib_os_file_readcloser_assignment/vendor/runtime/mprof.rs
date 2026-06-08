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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MEM_PROFILE: i32 = 1 + 0;
pub(crate) const BLOCK_PROFILE: i32 = 1 + 1;
pub(crate) const MUTEX_PROFILE: i32 = 1 + 2;
pub(crate) const BUCK_HASH_SIZE: i32 = 179999;
pub(crate) const MAX_SKIP: i32 = 6;
pub(crate) const MAX_PROF_STACK_DEPTH: i32 = 1024;


pub(crate) const M_PROF_CYCLE_WRAP: u32 = (3 as u32) * ((2 as u32) << (24 as u32)) as u32;


pub(crate) const GOROUTINE_PROFILE_ABSENT: u32 = 0;
pub(crate) const GOROUTINE_PROFILE_IN_PROGRESS: u32 = 1;
pub(crate) const GOROUTINE_PROFILE_SATISFIED: u32 = 2;


#[derive(Debug, Clone, Default)]
pub struct bucketType(pub Arc<Mutex<Option<i32>>>);

impl Display for bucketType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for bucketType {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for bucketType {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for bucketType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for bucketType {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<bucketType> for i32 {
    fn eq(&self, other: &bucketType) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<bucketType> for i32 {
    fn partial_cmp(&self, other: &bucketType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for bucketType {
    type Output = bucketType;
    fn add(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for bucketType {
    type Output = bucketType;
    fn add(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<bucketType> for i32 {
    type Output = bucketType;
    fn add(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for bucketType {
    type Output = bucketType;
    fn sub(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for bucketType {
    type Output = bucketType;
    fn sub(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<bucketType> for i32 {
    type Output = bucketType;
    fn sub(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for bucketType {
    type Output = bucketType;
    fn mul(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for bucketType {
    type Output = bucketType;
    fn mul(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<bucketType> for i32 {
    type Output = bucketType;
    fn mul(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for bucketType {
    type Output = bucketType;
    fn div(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for bucketType {
    type Output = bucketType;
    fn div(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<bucketType> for i32 {
    type Output = bucketType;
    fn div(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for bucketType {
    type Output = bucketType;
    fn neg(self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for bucketType {
    type Output = bucketType;
    fn rem(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for bucketType {
    type Output = bucketType;
    fn rem(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<bucketType> for i32 {
    type Output = bucketType;
    fn rem(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for bucketType {
    type Output = bucketType;
    fn bitand(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for bucketType {
    type Output = bucketType;
    fn bitand(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<bucketType> for i32 {
    type Output = bucketType;
    fn bitand(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for bucketType {
    type Output = bucketType;
    fn bitor(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for bucketType {
    type Output = bucketType;
    fn bitor(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<bucketType> for i32 {
    type Output = bucketType;
    fn bitor(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for bucketType {
    type Output = bucketType;
    fn bitxor(self, other: Self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for bucketType {
    type Output = bucketType;
    fn bitxor(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<bucketType> for i32 {
    type Output = bucketType;
    fn bitxor(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for bucketType {
    type Output = bucketType;
    fn not(self) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for bucketType {
    type Output = bucketType;
    fn shl(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for bucketType {
    type Output = bucketType;
    fn shl(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for bucketType {
    type Output = bucketType;
    fn shl(self, other: i8) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for bucketType {
    type Output = bucketType;
    fn shl(self, other: i16) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for bucketType {
    type Output = bucketType;
    fn shl(self, other: i64) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for bucketType {
    type Output = bucketType;
    fn shl(self, other: u32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for bucketType {
    type Output = bucketType;
    fn shl(self, other: u8) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for bucketType {
    type Output = bucketType;
    fn shl(self, other: u16) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for bucketType {
    type Output = bucketType;
    fn shl(self, other: u64) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for bucketType {
    type Output = bucketType;
    fn shl(self, other: usize) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for bucketType {
    type Output = bucketType;
    fn shr(self, other: bucketType) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for bucketType {
    type Output = bucketType;
    fn shr(self, other: i32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for bucketType {
    type Output = bucketType;
    fn shr(self, other: i8) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for bucketType {
    type Output = bucketType;
    fn shr(self, other: i16) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for bucketType {
    type Output = bucketType;
    fn shr(self, other: i64) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for bucketType {
    type Output = bucketType;
    fn shr(self, other: u32) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for bucketType {
    type Output = bucketType;
    fn shr(self, other: u8) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for bucketType {
    type Output = bucketType;
    fn shr(self, other: u16) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for bucketType {
    type Output = bucketType;
    fn shr(self, other: u64) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for bucketType {
    type Output = bucketType;
    fn shr(self, other: usize) -> bucketType {
        bucketType(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for bucketType {}

impl Ord for bucketType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A bucket holds per-call-stack profiling information.
/// The representation is a bit sleazy, inherited from C.
/// This struct defines the bucket header. It is followed in
/// memory by the stack words and then the actual record
/// data, either a memRecord or a blockRecord.
///
/// Per-call-stack profiling information.
/// Lookup by hashing call stack into a linked-list hash table.
///
/// None of the fields in this bucket header are modified after
/// creation, including its next and allnext links.
///
/// No heap pointers.
#[derive(Clone)]
pub struct bucket {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub next: Arc<Mutex<Option<bucket>>>,
    pub allnext: Arc<Mutex<Option<bucket>>>,
    pub typ: Arc<Mutex<Option<bucketType>>>,
    pub hash: Arc<Mutex<Option<usize>>>,
    pub size: Arc<Mutex<Option<usize>>>,
    pub nstk: Arc<Mutex<Option<usize>>>,
}

impl bucket {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next: self.next.clone(), allnext: self.allnext.clone(), typ: { let __guard = self.typ.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hash: { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nstk: { let __guard = self.nstk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for bucket {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), next: Arc::new(Mutex::new(None)), allnext: Arc::new(Mutex::new(None)), typ: Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(0))))))), hash: Arc::new(Mutex::new(Some(0))), size: Arc::new(Mutex::new(Some(0))), nstk: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for bucket {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.allnext.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.typ.lock().unwrap().as_ref().unwrap()), (*self.hash.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), (*self.nstk.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for bucket {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A memRecord is the bucket data for a bucket of type memProfile,
/// part of the memory profile.
#[derive(Debug, Clone)]
pub struct memRecord {
    pub active: Arc<Mutex<Option<memRecordCycle>>>,
    pub future: Arc<Mutex<Option<[memRecordCycle; 3]>>>,
}

impl memRecord {
    pub fn __go_value_clone(&self) -> Self {
        Self { active: { let __guard = self.active.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, future: { let __guard = self.future.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for memRecord {
    fn default() -> Self {
        Self { active: Arc::new(Mutex::new(Some(memRecordCycle::default()))), future: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }
    }
}

impl std::fmt::Display for memRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.active.lock().unwrap().as_ref().unwrap()), format_slice(&self.future))
    }
}

impl GoJsonDecode for memRecord {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// memRecordCycle
#[derive(Debug, Clone)]
pub struct memRecordCycle {
    pub allocs: Arc<Mutex<Option<usize>>>,
    pub frees: Arc<Mutex<Option<usize>>>,
    pub alloc_bytes: Arc<Mutex<Option<usize>>>,
    pub free_bytes: Arc<Mutex<Option<usize>>>,
}

impl memRecordCycle {
    pub fn __go_value_clone(&self) -> Self {
        Self { allocs: { let __guard = self.allocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, frees: { let __guard = self.frees.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc_bytes: { let __guard = self.alloc_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, free_bytes: { let __guard = self.free_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for memRecordCycle {
    fn default() -> Self {
        Self { allocs: Arc::new(Mutex::new(Some(0))), frees: Arc::new(Mutex::new(Some(0))), alloc_bytes: Arc::new(Mutex::new(Some(0))), free_bytes: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for memRecordCycle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.allocs.lock().unwrap().as_ref().unwrap()), (*self.frees.lock().unwrap().as_ref().unwrap()), (*self.alloc_bytes.lock().unwrap().as_ref().unwrap()), (*self.free_bytes.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for memRecordCycle {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A blockRecord is the bucket data for a bucket of type blockProfile,
/// which is used in blocking and mutex profiles.
#[derive(Debug, Clone)]
pub struct blockRecord {
    pub count: Arc<Mutex<Option<f64>>>,
    pub cycles: Arc<Mutex<Option<i64>>>,
}

impl blockRecord {
    pub fn __go_value_clone(&self) -> Self {
        Self { count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cycles: { let __guard = self.cycles.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for blockRecord {
    fn default() -> Self {
        Self { count: Arc::new(Mutex::new(Some(0.0))), cycles: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for blockRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.count.lock().unwrap().as_ref().unwrap()), (*self.cycles.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for blockRecord {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct buckhashArray(pub Arc<Mutex<Option<[internal_runtime_atomic::types::UnsafePointer; 179999]>>>);

impl Default for buckhashArray {
    fn default() -> Self {
        buckhashArray(Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))))
    }
}


/// mProfCycleHolder holds the global heap profile cycle number (wrapped at
/// mProfCycleWrap, stored starting at bit 1), and a flag (stored at bit 0) to
/// indicate whether future[cycle] in all buckets has been queued to flush into
/// the active profile.
#[derive(Clone)]
pub struct mProfCycleHolder {
    pub value: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl mProfCycleHolder {
    pub fn __go_value_clone(&self) -> Self {
        Self { value: { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mProfCycleHolder {
    fn default() -> Self {
        Self { value: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for mProfCycleHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.value.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mProfCycleHolder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// lockTimer assists with profiling contention on runtime-internal locks.
///
/// There are several steps between the time that an M experiences contention and
/// when that contention may be added to the profile. This comes from our
/// constraints: We need to keep the critical section of each lock small,
/// especially when those locks are contended. The reporting code cannot acquire
/// new locks until the M has released all other locks, which means no memory
/// allocations and encourages use of (temporary) M-local storage.
///
/// The M will have space for storing one call stack that caused contention, and
/// for the magnitude of that contention. It will also have space to store the
/// magnitude of additional contention the M caused, since it only has space to
/// remember one call stack and might encounter several contention events before
/// it releases all of its locks and is thus able to transfer the local buffer
/// into the profile.
///
/// The M will collect the call stack when it unlocks the contended lock. That
/// minimizes the impact on the critical section of the contended lock, and
/// matches the mutex profile's behavior for contention in sync.Mutex: measured
/// at the Unlock method.
///
/// The profile for contention on sync.Mutex blames the caller of Unlock for the
/// amount of contention experienced by the callers of Lock which had to wait.
/// When there are several critical sections, this allows identifying which of
/// them is responsible.
///
/// Matching that behavior for runtime-internal locks will require identifying
/// which Ms are blocked on the mutex. The semaphore-based implementation is
/// ready to allow that, but the futex-based implementation will require a bit
/// more work. Until then, we report contention on runtime-internal locks with a
/// call stack taken from the unlock call (like the rest of the user-space
/// "mutex" profile), but assign it a duration value based on how long the
/// previous lock call took (like the user-space "block" profile).
///
/// Thus, reporting the call stacks of runtime-internal lock contention is
/// guarded by GODEBUG for now. Set GODEBUG=runtimecontentionstacks=1 to enable.
///
/// TODO(rhysh): plumb through the delay duration, remove GODEBUG, update comment
///
/// The M will track this by storing a pointer to the lock; lock/unlock pairs for
/// runtime-internal locks are always on the same M.
///
/// Together, that demands several steps for recording contention. First, when
/// finally acquiring a contended lock, the M decides whether it should plan to
/// profile that event by storing a pointer to the lock in its "to be profiled
/// upon unlock" field. If that field is already set, it uses the relative
/// magnitudes to weight a random choice between itself and the other lock, with
/// the loser's time being added to the "additional contention" field. Otherwise
/// if the M's call stack buffer is occupied, it does the comparison against that
/// sample's magnitude.
///
/// Second, having unlocked a mutex the M checks to see if it should capture the
/// call stack into its local buffer. Finally, when the M unlocks its last mutex,
/// it transfers the local buffer into the profile. As part of that step, it also
/// transfers any "additional contention" time to the profile. Any lock
/// contention that it experiences while adding samples to the profile will be
/// recorded later as "additional contention" and not include a call stack, to
/// avoid an echo.
#[derive(Debug, Clone)]
pub struct lockTimer {
    pub lock: GoPtr<crate::runtime2::mutex>,
    pub time_rate: Arc<Mutex<Option<i64>>>,
    pub time_start: Arc<Mutex<Option<i64>>>,
    pub tick_start: Arc<Mutex<Option<i64>>>,
}

impl lockTimer {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: self.lock.clone(), time_rate: { let __guard = self.time_rate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, time_start: { let __guard = self.time_start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tick_start: { let __guard = self.tick_start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for lockTimer {
    fn default() -> Self {
        Self { lock: GoPtr::nil(), time_rate: Arc::new(Mutex::new(Some(0))), time_start: Arc::new(Mutex::new(Some(0))), tick_start: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for lockTimer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { if self.lock.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } }, (*self.time_rate.lock().unwrap().as_ref().unwrap()), (*self.time_start.lock().unwrap().as_ref().unwrap()), (*self.tick_start.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for lockTimer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct mLockProfile {
    pub wait_time: Arc<Mutex<Option<internal_runtime_atomic::types::Int64>>>,
    pub stack: Arc<Mutex<Option<Vec<usize>>>>,
    pub pending: Arc<Mutex<Option<usize>>>,
    pub cycles: Arc<Mutex<Option<i64>>>,
    pub cycles_lost: Arc<Mutex<Option<i64>>>,
    pub have_stack: Arc<Mutex<Option<bool>>>,
    pub disabled: Arc<Mutex<Option<bool>>>,
}

impl mLockProfile {
    pub fn __go_value_clone(&self) -> Self {
        Self { wait_time: { let __guard = self.wait_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stack: self.stack.clone(), pending: { let __guard = self.pending.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cycles: { let __guard = self.cycles.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cycles_lost: { let __guard = self.cycles_lost.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, have_stack: { let __guard = self.have_stack.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disabled: { let __guard = self.disabled.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mLockProfile {
    fn default() -> Self {
        Self { wait_time: Arc::new(Mutex::new(Some(Default::default()))), stack: Arc::new(Mutex::new(None)), pending: Arc::new(Mutex::new(Some(0))), cycles: Arc::new(Mutex::new(Some(0))), cycles_lost: Arc::new(Mutex::new(Some(0))), have_stack: Arc::new(Mutex::new(Some(false))), disabled: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for mLockProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {}}}", (*self.wait_time.lock().unwrap().as_ref().unwrap()), format_slice(&self.stack), (*self.pending.lock().unwrap().as_ref().unwrap()), (*self.cycles.lock().unwrap().as_ref().unwrap()), (*self.cycles_lost.lock().unwrap().as_ref().unwrap()), (*self.have_stack.lock().unwrap().as_ref().unwrap()), (*self.disabled.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mLockProfile {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// goroutineProfileState indicates the status of a goroutine's stack for the
/// current in-progress goroutine profile. Goroutines' stacks are initially
/// "Absent" from the profile, and end up "Satisfied" by the time the profile is
/// complete. While a goroutine's stack is being captured, its
/// goroutineProfileState will be "InProgress" and it will not be able to run
/// until the capture completes and the state moves to "Satisfied".
///
/// Some goroutines (the finalizer goroutine, which at various times can be
/// either a "system" or a "user" goroutine, and the goroutine that is
/// coordinating the profile, any goroutines created during the profile) move
/// directly to the "Satisfied" state.
#[derive(Debug, Clone, Default)]
pub struct goroutineProfileState(pub Arc<Mutex<Option<u32>>>);

impl Display for goroutineProfileState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for goroutineProfileState {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for goroutineProfileState {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for goroutineProfileState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for goroutineProfileState {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<goroutineProfileState> for u32 {
    fn eq(&self, other: &goroutineProfileState) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<goroutineProfileState> for u32 {
    fn partial_cmp(&self, other: &goroutineProfileState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for goroutineProfileState {
    type Output = goroutineProfileState;
    fn add(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn add(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn add(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for goroutineProfileState {
    type Output = goroutineProfileState;
    fn sub(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn sub(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn sub(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for goroutineProfileState {
    type Output = goroutineProfileState;
    fn mul(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn mul(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn mul(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for goroutineProfileState {
    type Output = goroutineProfileState;
    fn div(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn div(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn div(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for goroutineProfileState {
    type Output = goroutineProfileState;
    fn rem(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn rem(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn rem(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitand(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitand(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn bitand(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitor(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitor(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn bitor(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitxor(self, other: Self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn bitxor(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<goroutineProfileState> for u32 {
    type Output = goroutineProfileState;
    fn bitxor(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for goroutineProfileState {
    type Output = goroutineProfileState;
    fn not(self) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: i32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: i8) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: i16) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: i64) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: u8) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: u16) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: u64) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shl(self, other: usize) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: goroutineProfileState) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: i32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: i8) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: i16) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: i64) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: u32) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: u8) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: u16) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: u64) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for goroutineProfileState {
    type Output = goroutineProfileState;
    fn shr(self, other: usize) -> goroutineProfileState {
        goroutineProfileState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for goroutineProfileState {}

impl Ord for goroutineProfileState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone, Default)]
pub struct goroutineProfileStateHolder(pub Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>);

impl Display for goroutineProfileStateHolder {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


pub(crate) static profInsertLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static profBlockLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static profMemActiveLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static profMemFutureLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[crate::runtime2::mutex; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mbuckets: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static bbuckets: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static xbuckets: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static buckhash: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mProfCycle: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<mProfCycleHolder>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static blockprofilerate: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static mutexprofilerate: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static MemProfileRate: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static disableMemoryProfiling: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static goroutineProfile: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct21>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *profInsertLock.lock().unwrap() = Some(Default::default());
    *profBlockLock.lock().unwrap() = Some(Default::default());
    *profMemActiveLock.lock().unwrap() = Some(Default::default());
    *profMemFutureLock.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *mbuckets.lock().unwrap() = Some(Default::default());
    *bbuckets.lock().unwrap() = Some(Default::default());
    *xbuckets.lock().unwrap() = Some(Default::default());
    *buckhash.lock().unwrap() = Some(Default::default());
    *mProfCycle.lock().unwrap() = Some(Default::default());
    *blockprofilerate.lock().unwrap() = Some(0);
    *mutexprofilerate.lock().unwrap() = Some(0);
    *MemProfileRate.lock().unwrap() = Some(0);
    *disableMemoryProfiling.lock().unwrap() = Some(false);
    *goroutineProfile.lock().unwrap() = Some(Default::default());
    *MemProfileRate.lock().unwrap() = Some(524288);
    *goroutineProfile.lock().unwrap() = Some(AnonymousStruct21 { sema: Arc::new(Mutex::new(Some(1 as u32))), active: Default::default(), offset: Default::default(), records: Default::default(), labels: Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *profInsertLock.lock().unwrap() = Some(Default::default());
    *profBlockLock.lock().unwrap() = Some(Default::default());
    *profMemActiveLock.lock().unwrap() = Some(Default::default());
    *profMemFutureLock.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *mbuckets.lock().unwrap() = Some(Default::default());
    *bbuckets.lock().unwrap() = Some(Default::default());
    *xbuckets.lock().unwrap() = Some(Default::default());
    *buckhash.lock().unwrap() = Some(Default::default());
    *mProfCycle.lock().unwrap() = Some(Default::default());
    *blockprofilerate.lock().unwrap() = Some(0);
    *mutexprofilerate.lock().unwrap() = Some(0);
    *MemProfileRate.lock().unwrap() = Some(0);
    *disableMemoryProfiling.lock().unwrap() = Some(false);
    *goroutineProfile.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_33() {
    *MemProfileRate.lock().unwrap() = Some(524288);
}


pub(crate) fn __go_init_order_34() {
    *goroutineProfile.lock().unwrap() = Some(AnonymousStruct21 { sema: Arc::new(Mutex::new(Some(1 as u32))), active: Default::default(), offset: Default::default(), records: Default::default(), labels: Default::default() });
}


impl memRecordCycle {
    /// add accumulates b into a. It does not zero b.
    pub fn add(&mut self, b: GoPtr<memRecordCycle>) {
        { let __target = self.allocs.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().allocs.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.frees.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().frees.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.alloc_bytes.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().alloc_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = self.free_bytes.clone(); let __rhs = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().free_bytes.clone() }.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
}

impl mProfCycleHolder {
    /// read returns the current cycle count.
    pub fn read(&self) -> u32 {
    let mut cycle: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

        let mut v = (*self.value.lock().unwrap().as_mut().unwrap()).load();
        { let new_val = { let __tmp_x = v; let __tmp_y = 1; __tmp_x >> __tmp_y }; *cycle.lock().unwrap() = Some(new_val); };
        return { let __v = (*cycle.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// setFlushed sets the flushed flag. It returns the current cycle count and the
    /// previous value of the flushed flag.
    pub fn set_flushed(&self) -> (u32, bool) {
    let mut cycle: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut alreadyFlushed: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        loop {
        let mut prev = (*self.value.lock().unwrap().as_mut().unwrap()).load();
        { let new_val = { let __tmp_x = prev; let __tmp_y = 1; __tmp_x >> __tmp_y }; *cycle.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = ({ let __tmp_x = prev; let __tmp_y = 0x1 as u32; __tmp_x & __tmp_y }); let __tmp_y = 0 as u32; __tmp_x != __tmp_y }; *alreadyFlushed.lock().unwrap() = Some(new_val); };
        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = prev; let __tmp_y = 0x1 as u32; __tmp_x | __tmp_y })));
        if (*self.value.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(prev))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return ({ let __v = (*cycle.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*alreadyFlushed.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
    }
    }

    /// increment increases the cycle count by one, wrapping the value at
    /// mProfCycleWrap. It clears the flushed flag.
    pub fn increment(&self) {
                // We explicitly wrap mProfCycle rather than depending on
                // uint wraparound because the memRecord.future ring does not
                // itself wrap at a power of two.
        loop {
        let mut prev = (*self.value.lock().unwrap().as_mut().unwrap()).load();
        let mut cycle = Arc::new(Mutex::new(Some({ let __tmp_x = prev; let __tmp_y = 1; __tmp_x >> __tmp_y })));
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*cycle.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }); let __tmp_y = M_PROF_CYCLE_WRAP as u32; __tmp_x % __tmp_y }; *cycle.lock().unwrap() = Some(new_val); };
        let mut next = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*cycle.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x << __tmp_y })));
        if (*self.value.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(prev))), Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        break
    }
    }
    }
}

impl bucket {
    /// stk returns the slice in b holding the stack. The caller can assume that the
    /// backing array is immutable.
    pub fn stk(&self) -> Arc<Mutex<Option<Vec<usize>>>> {
        let mut stk: GoPtr<[usize; 1024]> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some(std::mem::size_of::<bucket>())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if { let __tmp_x = (*self.nstk.lock().unwrap().as_ref().unwrap()); let __tmp_y = MAX_PROF_STACK_DEPTH as usize; __tmp_x > __tmp_y } {
                // prove that slicing works; otherwise a failure requires a P
        throw(Arc::new(Mutex::new(Some("bad profile stack count".to_string()))));
    }
                // prove that slicing works; otherwise a failure requires a P
        Arc::new(Mutex::new(Some({ let __seq_ref = stk.borrow(); let mut __seq = __seq_ref.as_ref().unwrap().clone(); let __low = 0; let __high = (*self.nstk.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = (*self.nstk.clone().lock().unwrap().as_ref().unwrap()) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }

    /// mp returns the memRecord associated with the memProfile bucket b.
    pub fn mp(&self) -> GoPtr<memRecord> {
        if { let __tmp_x = { let __selector_holder = self.typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = bucketType(Arc::new(Mutex::new(Some(MEM_PROFILE as i32)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of bucket.mp".to_string()))));
    }
        let mut data = add(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<bucket>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = (*self.nstk.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
        return GoPtr::raw({ let __ptr = data.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }

    /// bp returns the blockRecord associated with the blockProfile bucket b.
    pub fn bp(&self) -> GoPtr<blockRecord> {
        if { let __tmp_x = { let __selector_holder = self.typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = bucketType(Arc::new(Mutex::new(Some(BLOCK_PROFILE as i32)))); __tmp_x != __tmp_y } && { let __tmp_x = { let __selector_holder = self.typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32)))); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad use of bucket.bp".to_string()))));
    }
        let mut data = add(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<bucket>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = (*self.nstk.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))));
        return GoPtr::raw({ let __ptr = data.clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    }
}

impl lockTimer {
    pub fn begin(&mut self) {
        let mut rate = Arc::new(Mutex::new(Some(internal_runtime_atomic::load64(mutexprofilerate.clone()) as i64)));
        { let new_val = G_TRACKING_PERIOD as i64; *self.time_rate.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.time_rate.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = rate.lock().unwrap().as_ref().unwrap().clone(); *self.time_rate.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.time_rate.lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = nanotime(); *self.time_start.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        { let new_val = cputicks(); *self.tick_start.lock().unwrap() = Some(new_val); };
    }
    }

    pub fn end(&self) {
        let mut gp = getg();
        if { let __tmp_x = (*self.time_start.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        let mut nowTime = nanotime();
        (*(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_lock_profile.lock().unwrap().as_ref().unwrap()).wait_time.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = nowTime; let __tmp_y = (*self.time_start.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = (*self.time_rate.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))));
    }
        if { let __tmp_x = (*self.tick_start.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        let mut nowTick = cputicks();
        (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_lock_profile.lock().unwrap().as_mut().unwrap()).record_lock(Arc::new(Mutex::new(Some({ let __tmp_x = nowTick; let __tmp_y = (*self.tick_start.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), self.lock.clone());
    }
    }
}

impl mLockProfile {
    pub fn record_lock(&mut self, mut cycles: Arc<Mutex<Option<i64>>>, l: GoPtr<crate::runtime2::mutex>) {
        if { let __tmp_x = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = 0 as i64; *cycles.lock().unwrap() = Some(new_val); };
    }
        if (*self.disabled.clone().lock().unwrap().as_ref().unwrap()) {
                // We're experiencing contention while attempting to report contention.
                // Make a note of its magnitude, but don't allow it to be the sole cause
                // of another contention report.
        { let __target = self.cycles_lost.clone(); let __rhs = (*cycles.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return;
    }
                // We're experiencing contention while attempting to report contention.
                // Make a note of its magnitude, but don't allow it to be the sole cause
                // of another contention report.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(l.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.pending.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // Optimization: we'd already planned to profile this same lock (though
                // possibly from a different unlock site).
        { let __target = self.cycles.clone(); let __rhs = (*cycles.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return;
    }
                // Optimization: we'd already planned to profile this same lock (though
                // possibly from a different unlock site).
        {
        let mut prev = Arc::new(Mutex::new(Some({ let __selector_holder = self.cycles.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = { let __v = (*prev.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
            if { let __tmp_x = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        return;
    };
            let mut prevScore = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand64() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*prev.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })));;
            let mut thisScore = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(cheaprand64() as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*cycles.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y })));;
            if { let __tmp_x = { let __v = (*prevScore.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*thisScore.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let __target = self.cycles_lost.clone(); let __rhs = (*cycles.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return;
    } else {
        { let __target = self.cycles_lost.clone(); let __rhs = (*prev.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    };
        }
    }
                // We can only store one call stack for runtime-internal lock contention
                // on this M, and we've already got one. Decide which should stay, and
                // add the other to the report for runtime._LostContendedRuntimeLock.
                // Saving the *mutex as a uintptr is safe because:
                //  - lockrank_on.go does this too, which gives it regular exercise
                //  - the lock would only move if it's stack allocated, which means it
                //      cannot experience multi-M contention
        { let new_val = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(l.addr()))).lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.pending.lock().unwrap() = __moved_val; };
        { let new_val = cycles.lock().unwrap().as_ref().unwrap().clone(); *self.cycles.lock().unwrap() = Some(new_val); };
    }

    /// From unlock2, we might not be holding a p in this code.
    ///
    ///go:nowritebarrierrec
    pub fn record_unlock(&mut self, l: GoPtr<crate::runtime2::mutex>) {
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(l.addr()))).lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.pending.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        self.capture_stack();
    }
        {
        let mut gp = getg();;
        if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i32; __tmp_x == __tmp_y } && (*(*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).m_lock_profile.lock().unwrap().as_ref().unwrap()).have_stack.lock().unwrap().as_ref().unwrap()) {
            self.store();;
        }
    }
    }

    pub fn capture_stack(&mut self) {
        if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // profstackdepth is set to 0 by the user, so mp.profStack is nil and we
                // can't record a stack trace.
        return;
    }
                // profstackdepth is set to 0 by the user, so mp.profStack is nil and we
                // can't record a stack trace.
        let mut skip = Arc::new(Mutex::new(Some(3)));
        if STATIC_LOCK_RANKING {
                // When static lock ranking is enabled, we'll always be on the system
                // stack at this point. There will be a runtime.unlockWithRank.func1
                // frame, and if the call to runtime.unlock took place on a user stack
                // then there'll also be a runtime.systemstack frame. To keep stack
                // traces somewhat consistent whether or not static lock ranking is
                // enabled, we'd like to skip those. But it's hard to tell how long
                // we've been on the system stack so accept an extra frame in that case,
                // with a leaf of "runtime.unlockWithRank runtime.unlock" instead of
                // "runtime.unlock".
        { let __rhs = 1; let mut guard = skip.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // When static lock ranking is enabled, we'll always be on the system
                // stack at this point. There will be a runtime.unlockWithRank.func1
                // frame, and if the call to runtime.unlock took place on a user stack
                // then there'll also be a runtime.systemstack frame. To keep stack
                // traces somewhat consistent whether or not static lock ranking is
                // enabled, we'd like to skip those. But it's hard to tell how long
                // we've been on the system stack so accept an extra frame in that case,
                // with a leaf of "runtime.unlockWithRank runtime.unlock" instead of
                // "runtime.unlock".
                // runtime.unlockWithRank.func1
        { let new_val = 0 as usize; *self.pending.lock().unwrap() = Some(new_val); };
        { let new_val = true; *self.have_stack.lock().unwrap() = Some(new_val); };
        (*self.stack.lock().unwrap().as_mut().unwrap())[(0) as usize] = LOGICAL_STACK_SENTINEL as usize;
        if { let __tmp_x = (*(*debug.lock().unwrap().as_ref().unwrap()).runtime_contention_stacks.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        (*self.stack.lock().unwrap().as_mut().unwrap())[(1) as usize] = { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__lost_contended_runtime_lock.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y };
        (*self.stack.lock().unwrap().as_mut().unwrap())[(2) as usize] = 0 as usize;
        return;
    }
        let mut nstk: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let mut gp = getg();
        let mut sp = internal_runtime_sys::get_caller_s_p();
        let mut pc = internal_runtime_sys::get_caller_p_c();
        let gp_closure_clone = gp.clone(); let mut nstk_closure_clone = nstk.clone(); let pc_closure_clone = pc.clone(); let mut prof_closure_clone = (*self).clone(); let skip_closure_clone = skip.clone(); let sp_closure_clone = sp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*u.lock().unwrap().as_mut().unwrap()).init_at(Arc::new(Mutex::new(Some(pc_closure_clone))), Arc::new(Mutex::new(Some(sp_closure_clone))), Arc::new(Mutex::new(Some(0 as usize))), GoPtr::local(gp_closure_clone.clone()), Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some((UNWIND_SILENT_ERRORS as u8 | UNWIND_JUMP_STACK as u8) as u8))))))));
        { let new_val = { let __tmp_x = 1; let __tmp_y = traceback_p_cs(u.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skip_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = prof_closure_clone.stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __tmp_x + __tmp_y }; *nstk_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
        if { let __tmp_x = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.stack.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        (*self.stack.lock().unwrap().as_mut().unwrap())[({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = 0 as usize;
    }
    }

    pub fn store(&mut self) {
                // Report any contention we experience within this function as "lost"; it's
                // important that the act of reporting a contention event not lead to a
                // reportable contention event. This also means we can use prof.stack
                // without copying, since it won't change during this function.
        let mut mp = acquirem();
        { let new_val = true; *self.disabled.lock().unwrap() = Some(new_val); };
        let mut nstk = Arc::new(Mutex::new(Some({ let __selector_holder = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        {
        let mut pc = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));;
        if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
            { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *nstk.lock().unwrap() = Some(new_val); };;
            break;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        let (mut cycles, mut lost) = (Arc::new(Mutex::new(Some({ let __selector_holder = self.cycles.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.cycles_lost.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let __tmp_0 = 0; let __tmp_1 = 0; *self.cycles.lock().unwrap() = Some(__tmp_0 as i64); *self.cycles_lost.lock().unwrap() = Some(__tmp_1 as i64); };
        { let new_val = false; *self.have_stack.lock().unwrap() = Some(new_val); };
        let mut rate = Arc::new(Mutex::new(Some(internal_runtime_atomic::load64(mutexprofilerate.clone()) as i64)));
        save_block_event_stack(Arc::new(Mutex::new(Some({ let __arg_holder = cycles.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = self.stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32))))))));
        if { let __tmp_x = { let __v = (*lost.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        let mut lostStk = Arc::new(Mutex::new(Some([LOGICAL_STACK_SENTINEL as usize, { let __tmp_x = internal_abi::func_p_c_a_b_i_internal(Arc::new(Mutex::new(Some(Box::new(__lost_contended_runtime_lock.clone()) as Box<dyn Any + Send + Sync>)))); let __tmp_y = internal_runtime_sys::P_C_QUANTUM as usize; __tmp_x + __tmp_y }])));
        save_block_event_stack(Arc::new(Mutex::new(Some({ let __arg_holder = lost.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = lostStk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32))))))));
    }
        { let new_val = false; *self.disabled.lock().unwrap() = Some(new_val); };
        releasem(GoPtr::local(mp.clone()));
    }
}

impl goroutineProfileStateHolder {
    pub fn load(&self) -> Arc<Mutex<Option<goroutineProfileState>>> {
        Arc::new(Mutex::new(Some(goroutineProfileState(Arc::new(Mutex::new(Some({ let __recv = Arc::new(Mutex::new(Some(internal_runtime_atomic::types::Uint32::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).load(); __result } as u32)))))))
    }

    pub fn store(&self, value: Arc<Mutex<Option<goroutineProfileState>>>) {
        { let __recv = Arc::new(Mutex::new(Some(internal_runtime_atomic::types::Uint32::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*{ let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32)))); __result };
    }

    pub fn compare_and_swap(&self, old: Arc<Mutex<Option<goroutineProfileState>>>, new: Arc<Mutex<Option<goroutineProfileState>>>) -> bool {
        { let __recv = Arc::new(Mutex::new(Some(internal_runtime_atomic::types::Uint32::default()))); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32)))); __result }
    }
}

/// newBucket allocates a bucket with the given type and number of stack entries.
pub fn new_bucket(typ: Arc<Mutex<Option<bucketType>>>, nstk: Arc<Mutex<Option<i32>>>) -> GoPtr<bucket> {
    let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<bucket>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some((*nstk.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(std::mem::size_of::<usize>()))).lock().unwrap().as_ref().unwrap()) as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y })));
    { let _switch_val = (*typ.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (bucketType(Arc::new(Mutex::new(Some(MEM_PROFILE as i32))))) {
            { let __rhs = (*Arc::new(Mutex::new(Some(std::mem::size_of::<memRecord>()))).lock().unwrap().as_ref().unwrap()) as usize; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        } else if _switch_val == (bucketType(Arc::new(Mutex::new(Some(BLOCK_PROFILE as i32))))) || _switch_val == (bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32))))) {
            { let __rhs = (*Arc::new(Mutex::new(Some(std::mem::size_of::<blockRecord>()))).lock().unwrap().as_ref().unwrap()) as usize; let mut guard = size.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        } else {
            throw(Arc::new(Mutex::new(Some("invalid profile bucket type".to_string()))));
        }
    }

    let mut b: GoPtr<bucket> = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).buckhash_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    { let new_val = typ.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.typ.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = Arc::new(Mutex::new(Some((*nstk.lock().unwrap().as_ref().unwrap()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *{ let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.nstk.clone()); __ptr_value }.lock().unwrap() = __moved_val; };
    b.clone()
}

/// Return the bucket for stk[0:nstk], allocating new bucket if needed.
pub fn stkbucket(typ: Arc<Mutex<Option<bucketType>>>, size: Arc<Mutex<Option<usize>>>, stk: Arc<Mutex<Option<Vec<usize>>>>, alloc: Arc<Mutex<Option<bool>>>) -> GoPtr<bucket> {
    let mut bh: GoPtr<buckhashArray> = GoPtr::raw({ let __ptr = (*buckhash.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if bh.is_nil() {
        lock(GoPtr::local(profInsertLock.clone()));
                // check again under the lock
        bh = GoPtr::raw({ let __ptr = (*buckhash.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if bh.is_nil() {
        bh = GoPtr::raw({ let __ptr = sys_alloc(Arc::new(Mutex::new(Some(std::mem::size_of::<buckhashArray>()))), (*memstats.lock().unwrap().as_ref().unwrap()).buckhash_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if bh.is_nil() {
        throw(Arc::new(Mutex::new(Some("runtime: cannot allocate memory".to_string()))));
    }
        (*buckhash.lock().unwrap().as_mut().unwrap()).store_no_w_b(Arc::new(Mutex::new(Some(bh.addr()))));
    }
        unlock(GoPtr::local(profInsertLock.clone()));
    }

        // check again under the lock
        // Hash stack.
    let mut h: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = stk.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for pc in __range_values.iter().copied() {
        { let __rhs = (pc as usize); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x << __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };
    } }

        // hash in size
    { let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x << __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x >> __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };

        // finalize
    { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x << __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    { let __rhs = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 11; __tmp_x >> __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() ^ __rhs); };

    let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = BUCK_HASH_SIZE as usize; __tmp_x % __tmp_y }) as i32)));

        // first check optimistically, without the lock
    let mut b: GoPtr<bucket> = GoPtr::raw({ let __ptr = { let __named_array = bh.borrow(); let __seq_holder = __named_array.as_ref().unwrap().0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    while !b.is_nil() {
        if { let __tmp_x = { let __selector_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.typ.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().hash.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().size.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && eqslice({ let __result = b.with_mut(|__recv_value| __recv_value.stk()); __result }, stk.clone()) {
        return b.clone();
    }
        b = GoPtr::local({ let __ptr_value = b.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value });
    }

    if !{ let __v = (*alloc.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return GoPtr::nil();
    }

    lock(GoPtr::local(profInsertLock.clone()));

        // check again under the insertion lock
    let mut b: GoPtr<bucket> = GoPtr::raw({ let __ptr = { let __named_array = bh.borrow(); let __seq_holder = __named_array.as_ref().unwrap().0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    while !b.is_nil() {
        if { let __tmp_x = { let __selector_holder = { let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.typ.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().hash.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __ptr_value = b.borrow(); __ptr_value.as_ref().unwrap().size.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && eqslice({ let __result = b.with_mut(|__recv_value| __recv_value.stk()); __result }, stk.clone()) {
        unlock(GoPtr::local(profInsertLock.clone()));
        return b.clone();
    }
        b = GoPtr::local({ let __ptr_value = b.borrow(); let __field_value = __ptr_value.as_ref().unwrap().next.clone(); __field_value });
    }

        // Create new bucket.
    let mut b: GoPtr<bucket> = new_bucket(Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    { let _src = { let __copy_src_holder = stk.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*{ let __result = b.with_mut(|__recv_value| __recv_value.stk()); __result }.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*{ let __result = b.with_mut(|__recv_value| __recv_value.stk()); __result }.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.hash.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    { let new_val = size.lock().unwrap().as_ref().unwrap().clone(); *{ let __ptr_value = b.with_mut(|__ptr_value| __ptr_value.size.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };

    let mut allnext: Arc<Mutex<Option<internal_runtime_atomic::types::UnsafePointer>>> = Arc::new(Mutex::new(None));
    if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = bucketType(Arc::new(Mutex::new(Some(MEM_PROFILE as i32)))); __tmp_x == __tmp_y } {
        { let new_val = mbuckets.clone().clone(); allnext = new_val; };
    } else if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32)))); __tmp_x == __tmp_y } {
        { let new_val = xbuckets.clone().clone(); allnext = new_val; };
    } else {
        { let new_val = bbuckets.clone().clone(); allnext = new_val; };
    }

    { let new_val = Arc::new(Mutex::new({ let __ptr = { let __named_array = bh.borrow(); let __seq_holder = __named_array.as_ref().unwrap().0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.load().clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<bucket>(unimplemented!("unsafe.Pointer conversion to bucket")) } })).clone(); b.with_mut(|__ptr_value| { __ptr_value.next = new_val; }); };
    { let new_val = Arc::new(Mutex::new({ let __ptr = { let __recv = allnext.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::UnsafePointer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::UnsafePointer }; let __result = unsafe { &mut *__recv_ptr }.load(); __result }.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<bucket>(unimplemented!("unsafe.Pointer conversion to bucket")) } })).clone(); b.with_mut(|__ptr_value| { __ptr_value.allnext = new_val; }); };

    { let __named_array = bh.borrow(); let __seq_holder = __named_array.as_ref().unwrap().0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.store_no_w_b(Arc::new(Mutex::new(Some(b.addr()))));
    { let __recv = allnext.clone(); let __recv_ptr: *mut internal_runtime_atomic::types::UnsafePointer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut internal_runtime_atomic::types::UnsafePointer }; let __result = unsafe { &mut *__recv_ptr }.store_no_w_b(Arc::new(Mutex::new(Some(b.addr())))); __result };

    unlock(GoPtr::local(profInsertLock.clone()));
    b.clone()
}

pub fn eqslice(x: Arc<Mutex<Option<Vec<usize>>>>, y: Arc<Mutex<Option<Vec<usize>>>>) -> bool {
    if { let __tmp_x = ((*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*y.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        return false;
    }
    { let __range_holder = x.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, xi) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = xi; let __tmp_y = { let __seq = { let __seq_holder = y.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; __tmp_x != __tmp_y } {
        return false;
    }
    } }
    true
}

/// mProf_NextCycle publishes the next heap profile cycle and creates a
/// fresh heap profile cycle. This operation is fast and can be done
/// during STW. The caller must call mProf_Flush before calling
/// mProf_NextCycle again.
///
/// This is called by mark termination during STW so allocations and
/// frees after the world is started again count towards a new heap
/// profiling cycle.
pub fn m_prof__next_cycle() {
    (*mProfCycle.lock().unwrap().as_ref().unwrap()).increment();
}

/// mProf_Flush flushes the events from the current heap profiling
/// cycle into the active profile. After this it is safe to start a new
/// heap profiling cycle with mProf_NextCycle.
///
/// This is called by GC after mark termination starts the world. In
/// contrast with mProf_NextCycle, this is somewhat expensive, but safe
/// to do concurrently.
pub fn m_prof__flush() {
    let (mut cycle, mut alreadyFlushed) = (*mProfCycle.lock().unwrap().as_ref().unwrap()).set_flushed();
    if alreadyFlushed {
        return;
    }

    let mut index = Arc::new(Mutex::new(Some({ let __tmp_x = cycle; let __tmp_y = (*Arc::new(Mutex::new(Some((*memRecord { active: Arc::new(Mutex::new(Some(Default::default()))), future: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }.future.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y })));
    lock(GoPtr::local(profMemActiveLock.clone()));
    lock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
    m_prof__flush_locked(Arc::new(Mutex::new(Some({ let __arg_holder = index.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    unlock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
    unlock(GoPtr::local(profMemActiveLock.clone()));
}

/// mProf_FlushLocked flushes the events from the heap profiling cycle at index
/// into the active profile. The caller must hold the lock for the active profile
/// (profMemActiveLock) and for the profiling cycle at index
/// (profMemFutureLock[index]).
pub fn m_prof__flush_locked(index: Arc<Mutex<Option<u32>>>) {
    assert_lock_held(GoPtr::local(profMemActiveLock.clone()));
    assert_lock_held(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
    let mut head: GoPtr<bucket> = GoPtr::raw({ let __ptr = (*mbuckets.lock().unwrap().as_mut().unwrap()).load().clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let mut b: GoPtr<bucket> = head.clone();
    while !b.is_nil() {
        let mut mp: GoPtr<memRecord> = { let __result = b.with_mut(|__recv_value| __recv_value.mp()); __result };

                // Flush cycle C into the published profile and clear
                // it for reuse.
        let mut mpc: Option<GoArrayElemPtr<memRecordCycle, 3>> = Some(GoArrayElemPtr::new({ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.future.clone()); __ptr_value }.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        (*{ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.active.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).add(GoPtr::array_elem_opt(mpc.clone()));
        { let new_val = memRecordCycle { allocs: Arc::new(Mutex::new(Some(0))), frees: Arc::new(Mutex::new(Some(0))), alloc_bytes: Arc::new(Mutex::new(Some(0))), free_bytes: Arc::new(Mutex::new(Some(0))) }; *mpc.as_ref().unwrap().borrow_mut() = Some(new_val); };
        b = GoPtr::local({ let __ptr_value = b.borrow(); let __field_value = __ptr_value.as_ref().unwrap().allnext.clone(); __field_value });
    }
}

/// Called by malloc to record a profiled block.
pub fn m_prof__malloc(mp: Arc<Mutex<Option<m>>>, p: Arc<Mutex<Option<usize>>>, size: Arc<Mutex<Option<usize>>>) {
    if { let __nil_target = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // mp.profStack is nil if we happen to sample an allocation during the
                // initialization of mp. This case is rare, so we just ignore such
                // allocations. Change MemProfileRate to 1 if you need to reproduce such
                // cases for testing purposes.
        return;
    }

        // mp.profStack is nil if we happen to sample an allocation during the
        // initialization of mp. This case is rare, so we just ignore such
        // allocations. Change MemProfileRate to 1 if you need to reproduce such
        // cases for testing purposes.
        // Only use the part of mp.profStack we need and ignore the extra space
        // reserved for delayed inline expansion with frame pointer unwinding.
    let mut nstk = callers_1(Arc::new(Mutex::new(Some(5))), Arc::new(Mutex::new(Some({ let __seq_holder = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ((*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    let mut index = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*mProfCycle.lock().unwrap().as_ref().unwrap()).read(); let __tmp_y = 2 as u32; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*memRecord { active: Arc::new(Mutex::new(Some(Default::default()))), future: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }.future.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y })));

    let mut b: GoPtr<bucket> = stkbucket(Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(MEM_PROFILE as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (nstk) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(true))));
    let mut mr: GoPtr<memRecord> = { let __result = b.with_mut(|__recv_value| __recv_value.mp()); __result };
    let mut mpc: Option<GoArrayElemPtr<memRecordCycle, 3>> = Some(GoArrayElemPtr::new({ let __ptr_value = mr.with_mut(|__ptr_value| __ptr_value.future.clone()); __ptr_value }.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));

    lock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
    { let __target = (*mpc.as_ref().unwrap().borrow().as_ref().unwrap()).allocs.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __target = (*mpc.as_ref().unwrap().borrow().as_ref().unwrap()).alloc_bytes.clone(); let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    unlock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));

        // Setprofilebucket locks a bunch of other mutexes, so we call it outside of
        // the profiler locks. This reduces potential contention and chances of
        // deadlocks. Since the object must be alive during the call to
        // mProf_Malloc, it's fine to do this non-atomically.
    let b_closure_clone = b.clone(); let p_closure_clone = p.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        setprofilebucket(Arc::new(Mutex::new(Some({ let __arg_holder = p_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), b_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// Called when freeing a profiled block.
pub fn m_prof__free(b: GoPtr<bucket>, size: Arc<Mutex<Option<usize>>>) {
    let mut index = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = (*mProfCycle.lock().unwrap().as_ref().unwrap()).read(); let __tmp_y = 1 as u32; __tmp_x + __tmp_y }); let __tmp_y = (*Arc::new(Mutex::new(Some((*memRecord { active: Arc::new(Mutex::new(Some(Default::default()))), future: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))) }.future.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x % __tmp_y })));

    let mut mp: GoPtr<memRecord> = { let __result = b.with_mut(|__recv_value| __recv_value.mp()); __result };
    let mut mpc: Option<GoArrayElemPtr<memRecordCycle, 3>> = Some(GoArrayElemPtr::new({ let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.future.clone()); __ptr_value }.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));

    lock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
    { let __target = (*mpc.as_ref().unwrap().borrow().as_ref().unwrap()).frees.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    { let __target = (*mpc.as_ref().unwrap().borrow().as_ref().unwrap()).free_bytes.clone(); let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    unlock(GoPtr::array_elem(GoArrayElemPtr::new(profMemFutureLock.clone(), ({ let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)));
}

pub fn blockevent(mut cycles: Arc<Mutex<Option<i64>>>, skip: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        { let new_val = 1 as i64; *cycles.lock().unwrap() = Some(new_val); };
    }

    let mut rate = Arc::new(Mutex::new(Some(internal_runtime_atomic::load64(blockprofilerate.clone()) as i64)));
    if blocksampled(Arc::new(Mutex::new(Some({ let __arg_holder = cycles.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        saveblockevent(Arc::new(Mutex::new(Some({ let __arg_holder = cycles.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(BLOCK_PROFILE as i32))))))));
    }
}

/// blocksampled returns true for all events where cycles >= rate. Shorter
/// events have a cycles/rate random chance of returning true.
pub fn blocksampled(cycles: Arc<Mutex<Option<i64>>>, rate: Arc<Mutex<Option<i64>>>) -> bool {
    if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } || ({ let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = cheaprand64(); let __tmp_y = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }; let __tmp_y = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y }) {
        return false;
    }
    true
}

/// saveblockevent records a profile event of the type specified by which.
/// cycles is the quantity associated with this event and rate is the sampling rate,
/// used to adjust the cycles value in the manner determined by the profile type.
/// skip is the number of frames to omit from the traceback associated with the event.
/// The traceback will be recorded from the stack of the goroutine associated with the current m.
/// skip should be positive if this event is recorded from the current stack
/// (e.g. when this is not called from a system stack)
pub fn saveblockevent(cycles: Arc<Mutex<Option<i64>>>, rate: Arc<Mutex<Option<i64>>>, mut skip: Arc<Mutex<Option<i32>>>, which: Arc<Mutex<Option<bucketType>>>) {
    if { let __tmp_x = (*{ let __field = (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // profstackdepth is set to 0 by the user, so mp.profStack is nil and we
                // can't record a stack trace.
        return;
    }
        // profstackdepth is set to 0 by the user, so mp.profStack is nil and we
        // can't record a stack trace.
    if { let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x > __tmp_y } {
        eprint!("{}{}", format!("{}", "requested skip=".to_string()), format!("{}", { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        throw(Arc::new(Mutex::new(Some("invalid skip value".to_string()))));
    }
    let mut gp = getg();
    let mut mp = acquirem();

    let mut nstk: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if tracefpunwindoff() || (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).has_cgo_on_stack() {
        if { let __ptr_field = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone(); __ptr_field.is_nil() } || { let __left_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        { let new_val = callers_1(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); __field }); *nstk.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = gcallers((*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); __field }); *nstk.lock().unwrap() = Some(new_val); };
    }
    } else {
        if { let __ptr_field = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone(); __ptr_field.is_nil() } || { let __left_addr = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq } {
        if { let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
                // We skip one fewer frame than the provided value for frame
                // pointer unwinding because the skip value includes the current
                // frame, whereas the saved frame pointer will give us the
                // caller's return address first (so, not including
                // saveblockevent)
        { let __rhs = 1; let mut guard = skip.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // We skip one fewer frame than the provided value for frame
                // pointer unwinding because the skip value includes the current
                // frame, whereas the saved frame pointer will give us the
                // caller's return address first (so, not including
                // saveblockevent)
        { let new_val = fp_traceback_partial_expand(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(getfp()))), { let __field = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); __field }); *nstk.lock().unwrap() = Some(new_val); };
    } else {
        (*(*mp.lock().unwrap().as_ref().unwrap()).prof_stack.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*(*{ let __ptr_value = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
        { let new_val = { let __tmp_x = 1; let __tmp_y = fp_traceback_partial_expand(Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).bp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __seq_holder = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); __tmp_x + __tmp_y }; *nstk.lock().unwrap() = Some(new_val); };
    }
    }

        // We skip one fewer frame than the provided value for frame
        // pointer unwinding because the skip value includes the current
        // frame, whereas the saved frame pointer will give us the
        // caller's return address first (so, not including
        // saveblockevent)
    save_block_event_stack(Arc::new(Mutex::new(Some({ let __arg_holder = cycles.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __seq_holder = (*mp.lock().unwrap().as_ref().unwrap()).prof_stack.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = which.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    releasem(GoPtr::local(mp.clone()));
}

/// fpTracebackPartialExpand records a call stack obtained starting from fp.
/// This function will skip the given number of frames, properly accounting for
/// inlining, and save remaining frames as "physical" return addresses. The
/// consumer should later use CallersFrames or similar to expand inline frames.
pub fn fp_traceback_partial_expand(skip: Arc<Mutex<Option<i32>>>, mut fp: Arc<Mutex<Option<usize>>>, pcBuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut lastFuncID = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8)))))));
    let mut n_closure_clone = n.clone(); let pcBuf_closure_clone = pcBuf.clone(); let mut skip_closure_clone = skip.clone(); let mut skipOrAdd = Arc::new(Mutex::new(Some(Box::new(move |retPC: Arc<Mutex<Option<usize>>>| -> bool {
        if { let __tmp_x = { let __v = (*skip_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = skip_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    } else if { let __tmp_x = ({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        (*pcBuf_closure_clone.lock().unwrap().as_mut().unwrap())[({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*retPC.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let mut guard = n_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __tmp_x = ({ let __v = (*n_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf_closure_clone.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>)));
    while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __nil_result = (*fp.lock().unwrap()).is_some(); __nil_result } {
                // return addr sits one word above the frame pointer
        let mut pc = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*fp.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x + __tmp_y }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));

        if { let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut callPC = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let mut fi = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = callPC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = fi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = callPC.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        while (*uf.lock().unwrap().as_ref().unwrap()).valid() {
        let mut sf = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_WRAPPER as u8)))); __tmp_x == __tmp_y } && elide_wrapper_calling(Arc::new(Mutex::new(Some({ let __arg_holder = lastFuncID.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
    } else {
        let mut more = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> = { let mut __f_guard = skipOrAdd.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y })))) };;
        if !more {
            return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
                // ignore wrappers
        { let new_val = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*sf.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *lastFuncID.lock().unwrap() = Some(new_val); };
        { let new_val = (*u.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };
    }
    } else {
                // We've skipped the desired number of frames, so no need
                // to perform further inline expansion now.
        (*pcBuf.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // ignore wrappers
                // We've skipped the desired number of frames, so no need
                // to perform further inline expansion now.
                // follow the frame pointer to the next one
        { let new_val = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = fp.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fp.lock().unwrap() = __moved_val; };
    }
        // return addr sits one word above the frame pointer
        // ignore wrappers
        // We've skipped the desired number of frames, so no need
        // to perform further inline expansion now.
        // follow the frame pointer to the next one
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn save_block_event_stack(cycles: Arc<Mutex<Option<i64>>>, rate: Arc<Mutex<Option<i64>>>, stk: Arc<Mutex<Option<Vec<usize>>>>, which: Arc<Mutex<Option<bucketType>>>) {
    let mut b: GoPtr<bucket> = stkbucket(Arc::new(Mutex::new(Some({ let __arg_holder = which.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), stk.clone(), Arc::new(Mutex::new(Some(true))));
    let mut bp: GoPtr<blockRecord> = { let __result = b.with_mut(|__recv_value| __recv_value.bp()); __result };

    lock(GoPtr::local(profBlockLock.clone()));

        // We want to up-scale the count and cycles according to the
        // probability that the event was sampled. For block profile events,
        // the sample probability is 1 if cycles >= rate, and cycles / rate
        // otherwise. For mutex profile events, the sample probability is 1 / rate.
        // We scale the events by 1 / (probability the event was sampled).
    if { let __tmp_x = (*which.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = bucketType(Arc::new(Mutex::new(Some(BLOCK_PROFILE as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Remove sampling bias, see discussion on http://golang.org/cl/299991.
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.count.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some((*rate.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*cycles.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.cycles.clone()); __ptr_value }.clone(); let __rhs = (*rate.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else if { let __tmp_x = (*which.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32)))); __tmp_x == __tmp_y } {
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.count.clone()); __ptr_value }.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*rate.lock().unwrap().as_ref().unwrap()) as f64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.cycles.clone()); __ptr_value }.clone(); let __rhs = { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } else {
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.count.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1.0); }
        { let __target = { let __ptr_value = bp.with_mut(|__ptr_value| __ptr_value.cycles.clone()); __ptr_value }.clone(); let __rhs = (*cycles.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        // Remove sampling bias, see discussion on http://golang.org/cl/299991.
    unlock(GoPtr::local(profBlockLock.clone()));
}

///go:linkname mutexevent sync.event
pub fn mutexevent(mut cycles: Arc<Mutex<Option<i64>>>, skip: Arc<Mutex<Option<i32>>>) {
    if { let __tmp_x = { let __v = (*cycles.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        { let new_val = 0 as i64; *cycles.lock().unwrap() = Some(new_val); };
    }
    let mut rate = Arc::new(Mutex::new(Some(internal_runtime_atomic::load64(mutexprofilerate.clone()) as i64)));
    if { let __tmp_x = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } && { let __tmp_x = { let __tmp_x = cheaprand64(); let __tmp_y = { let __v = (*rate.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x % __tmp_y }; let __tmp_y = 0 as i64; __tmp_x == __tmp_y } {
        saveblockevent(Arc::new(Mutex::new(Some({ let __arg_holder = cycles.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rate.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(bucketType(Arc::new(Mutex::new(Some(MUTEX_PROFILE as i32))))))));
    }
}

/// tryRecordGoroutineProfileWB asserts that write barriers are allowed and calls
/// tryRecordGoroutineProfile.
///
///go:yeswritebarrierrec
pub fn try_record_goroutine_profile_w_b(gp1: Arc<Mutex<Option<g>>>) {
    if crate::runtime2::puintptr::ptr(&(*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap())).is_nil() {
        throw(Arc::new(Mutex::new(Some("no P available, write barriers are forbidden".to_string()))));
    }
    try_record_goroutine_profile(GoPtr::local(gp1.clone()), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(move || { osyield() }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// tryRecordGoroutineProfile ensures that gp1 has the appropriate representation
/// in the current goroutine profile: either that it should not be profiled, or
/// that a snapshot of its call stack and labels are now in the profile.
pub fn try_record_goroutine_profile(gp1: GoPtr<crate::runtime2::g>, pcbuf: Arc<Mutex<Option<Vec<usize>>>>, r#yield: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) {
    if { let __tmp_x = readgstatus(gp1.clone()); let __tmp_y = __GDEAD as u32; __tmp_x == __tmp_y } {
                // Dead goroutines should not appear in the profile. Goroutines that
                // start while profile collection is active will get goroutineProfiled
                // set to goroutineProfileSatisfied before transitioning out of _Gdead,
                // so here we check _Gdead first.
        return;
    }

        // Dead goroutines should not appear in the profile. Goroutines that
        // start while profile collection is active will get goroutineProfiled
        // set to goroutineProfileSatisfied before transitioning out of _Gdead,
        // so here we check _Gdead first.
    loop {
        let mut prev = (*{ let __ptr_value = gp1.with_mut(|__ptr_value| __ptr_value.goroutine_profiled.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).load();
        if { let __tmp_x = (*prev.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = goroutineProfileState(Arc::new(Mutex::new(Some(GOROUTINE_PROFILE_SATISFIED as u32)))); __tmp_x == __tmp_y } {
                // This goroutine is already in the profile (or is new since the
                // start of collection, so shouldn't appear in the profile).
        break
    }
                // This goroutine is already in the profile (or is new since the
                // start of collection, so shouldn't appear in the profile).
        if { let __tmp_x = (*prev.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = goroutineProfileState(Arc::new(Mutex::new(Some(GOROUTINE_PROFILE_IN_PROGRESS as u32)))); __tmp_x == __tmp_y } {
                // Something else is adding gp1 to the goroutine profile right now.
                // Give that a moment to finish.
        { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        continue
    }

                // Something else is adding gp1 to the goroutine profile right now.
                // Give that a moment to finish.
                // While we have gp1.goroutineProfiled set to
                // goroutineProfileInProgress, gp1 may appear _Grunnable but will not
                // actually be able to run. Disable preemption for ourselves, to make
                // sure we finish profiling gp1 right away instead of leaving it stuck
                // in this limbo.
        let mut mp = acquirem();
        if (*{ let __ptr_value = gp1.with_mut(|__ptr_value| __ptr_value.goroutine_profiled.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(goroutineProfileState(Arc::new(Mutex::new(Some(GOROUTINE_PROFILE_ABSENT as u32))))))), Arc::new(Mutex::new(Some(goroutineProfileState(Arc::new(Mutex::new(Some(GOROUTINE_PROFILE_IN_PROGRESS as u32)))))))) {
        do_record_goroutine_profile(gp1.clone(), pcbuf.clone());
        (*{ let __ptr_value = gp1.with_mut(|__ptr_value| __ptr_value.goroutine_profiled.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some(goroutineProfileState(Arc::new(Mutex::new(Some(GOROUTINE_PROFILE_SATISFIED as u32))))))));
    }
        releasem(GoPtr::local(mp.clone()));
    }
}

/// doRecordGoroutineProfile writes gp1's call stack and labels to an in-progress
/// goroutine profile. Preemption is disabled.
///
/// This may be called via tryRecordGoroutineProfile in two ways: by the
/// goroutine that is coordinating the goroutine profile (running on its own
/// stack), or from the scheduler in preparation to execute gp1 (running on the
/// system stack).
pub fn do_record_goroutine_profile(gp1: GoPtr<crate::runtime2::g>, pcbuf: Arc<Mutex<Option<Vec<usize>>>>) {
    if is_system_goroutine(gp1.clone(), Arc::new(Mutex::new(Some(false)))) {
                // System goroutines should not appear in the profile.
                // Check this here and not in tryRecordGoroutineProfile because isSystemGoroutine
                // may change on a goroutine while it is executing, so while the scheduler might
                // see a system goroutine, goroutineProfileWithLabelsConcurrent might not, and
                // this inconsistency could cause invariants to be violated, such as trying to
                // record the stack of a running goroutine below. In short, we still want system
                // goroutines to participate in the same state machine on gp1.goroutineProfiled as
                // everything else, we just don't record the stack in the profile.
        return;
    }
        // System goroutines should not appear in the profile.
        // Check this here and not in tryRecordGoroutineProfile because isSystemGoroutine
        // may change on a goroutine while it is executing, so while the scheduler might
        // see a system goroutine, goroutineProfileWithLabelsConcurrent might not, and
        // this inconsistency could cause invariants to be violated, such as trying to
        // record the stack of a running goroutine below. In short, we still want system
        // goroutines to participate in the same state machine on gp1.goroutineProfiled as
        // everything else, we just don't record the stack in the profile.
    if { let __tmp_x = readgstatus(gp1.clone()); let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y } {
        eprint!("{}{}{}", format!("{}", "doRecordGoroutineProfile gp1=".to_string()), format!("{}", (*{ let __ptr_value = gp1.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap())), format!("{}", "\n".to_string()));
        throw(Arc::new(Mutex::new(Some("cannot read stack of running goroutine".to_string()))));
    }

    let mut offset = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*(*goroutineProfile.lock().unwrap().as_ref().unwrap()).offset.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i64)))) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));

    if { let __tmp_x = ({ let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*goroutineProfile.lock().unwrap().as_ref().unwrap()).records.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
                // Should be impossible, but better to return a truncated profile than
                // to crash the entire process at this point. Instead, deal with it in
                // goroutineProfileWithLabelsConcurrent where we have more context.
        return;
    }

        // Should be impossible, but better to return a truncated profile than
        // to crash the entire process at this point. Instead, deal with it in
        // goroutineProfileWithLabelsConcurrent where we have more context.
        // saveg calls gentraceback, which may call cgo traceback functions. When
        // called from the scheduler, this is on the system stack already so
        // traceback.go:cgoContextPCs will avoid calling back into the scheduler.
        //
        // When called from the goroutine coordinating the profile, we still have
        // set gp1.goroutineProfiled to goroutineProfileInProgress and so are still
        // preventing it from being truly _Grunnable. So we'll use the system stack
        // to avoid schedule delays.
    let gp1_closure_clone = gp1.clone(); let offset_closure_clone = offset.clone(); let pcbuf_closure_clone = pcbuf.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        saveg(Arc::new(Mutex::new(Some(!(0 as usize) as usize))), Arc::new(Mutex::new(Some(!(0 as usize) as usize))), gp1_closure_clone.clone(), GoPtr::slice_elem(GoSliceElemPtr::new((*goroutineProfile.lock().unwrap().as_ref().unwrap()).records.clone(), ({ let __v = (*offset_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize)), pcbuf_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));

    if { let __nil_target = (*goroutineProfile.lock().unwrap().as_ref().unwrap()).labels.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        (*(*goroutineProfile.lock().unwrap().as_ref().unwrap()).labels.lock().unwrap().as_mut().unwrap())[({ let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*{ let __ptr_value = gp1.borrow(); __ptr_value.as_ref().unwrap().labels.clone() }.lock().unwrap().as_ref().unwrap()).clone();
    }
}

pub fn saveg(pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>, r: GoPtr<internal_profilerecord::r#mod::StackRecord>, mut pcbuf: Arc<Mutex<Option<Vec<usize>>>>) {
        // To reduce memory usage, we want to allocate a r.Stack that is just big
        // enough to hold gp's stack trace. Naively we might achieve this by
        // recording our stack trace into mp.profStack, and then allocating a
        // r.Stack of the right size. However, mp.profStack is also used for
        // allocation profiling, so it could get overwritten if the slice allocation
        // gets profiled. So instead we record the stack trace into a temporary
        // pcbuf which is usually given to us by our caller. When it's not, we have
        // to allocate one here. This will only happen for goroutines that were in a
        // syscall when the goroutine profile started or for goroutines that manage
        // to execute before we finish iterating over all the goroutines.
    if { let __nil_result = (*pcbuf.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = make_prof_stack(); pcbuf = new_val; };
    }

    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*u.lock().unwrap().as_mut().unwrap()).init_at(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as usize))), gp.clone(), Arc::new(Mutex::new(Some(crate::traceback::unwindFlags(Arc::new(Mutex::new(Some(UNWIND_SILENT_ERRORS as u8))))))));
    let mut n = traceback_p_cs(u.clone(), Arc::new(Mutex::new(Some(0))), pcbuf.clone());
    { let new_val = Arc::new(Mutex::new(Some(vec![0; (n) as usize]))); r.with_mut(|__ptr_value| { __ptr_value.stack = new_val; }); };
    { let _src = { let __copy_src_holder = pcbuf.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*{ let __ptr_value = r.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*{ let __ptr_value = r.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
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


pub(crate) type goroutineProfile = AnonymousStruct21;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for bucket {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for memRecord {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for memRecordCycle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for blockRecord {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mProfCycleHolder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for lockTimer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for mLockProfile {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
