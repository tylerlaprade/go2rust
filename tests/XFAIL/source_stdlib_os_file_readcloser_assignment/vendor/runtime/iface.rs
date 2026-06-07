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

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const ITAB_INIT_SIZE: i32 = 512;


/// Note: change the formula in the mallocgc call in itabAdd if you change these fields.
#[derive(Clone)]
pub struct itabTableType {
    pub size: Arc<Mutex<Option<usize>>>,
    pub count: Arc<Mutex<Option<usize>>>,
    pub entries: Arc<Mutex<Option<[Arc<Mutex<Option<internal_abi::iface::ITab>>>; 512]>>>,
}

impl itabTableType {
    pub fn __go_value_clone(&self) -> Self {
        Self { size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, entries: { let __guard = self.entries.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for itabTableType {
    fn default() -> Self {
        Self { size: Arc::new(Mutex::new(Some(0))), count: Arc::new(Mutex::new(Some(0))), entries: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))) }
    }
}

impl std::fmt::Display for itabTableType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.size.lock().unwrap().as_ref().unwrap()), (*self.count.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.entries))
    }
}

impl GoJsonDecode for itabTableType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The specialized convTx routines need a type descriptor to use when calling mallocgc.
/// We don't need the type to be exact, just to have the correct size, alignment, and pointer-ness.
/// However, when debugging, it'd be nice to have some indication in mallocgc where the types came from,
/// so we use named types here.
/// We then construct interface values of these types,
/// and then extract the type word to use as needed.
#[derive(Debug, Clone, Default)]
pub struct uint16InterfacePtr(pub Arc<Mutex<Option<u16>>>);

impl Display for uint16InterfacePtr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for uint16InterfacePtr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u16> for uint16InterfacePtr {
    fn eq(&self, other: &u16) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for uint16InterfacePtr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u16> for uint16InterfacePtr {
    fn partial_cmp(&self, other: &u16) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<uint16InterfacePtr> for u16 {
    fn eq(&self, other: &uint16InterfacePtr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<uint16InterfacePtr> for u16 {
    fn partial_cmp(&self, other: &uint16InterfacePtr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn add(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn add(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn add(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn sub(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn sub(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn sub(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn mul(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn mul(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn mul(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn div(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn div(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn div(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn rem(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn rem(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn rem(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitand(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitand(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn bitand(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitor(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitor(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn bitor(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitxor(self, other: Self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn bitxor(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<uint16InterfacePtr> for u16 {
    type Output = uint16InterfacePtr;
    fn bitxor(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn not(self) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: i32) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: i8) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: i16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: i64) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: u32) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: u8) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: u64) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shl(self, other: usize) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: uint16InterfacePtr) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: i32) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: i8) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: i16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: i64) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: u32) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: u8) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: u16) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: u64) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for uint16InterfacePtr {
    type Output = uint16InterfacePtr;
    fn shr(self, other: usize) -> uint16InterfacePtr {
        uint16InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for uint16InterfacePtr {}

impl Ord for uint16InterfacePtr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// The specialized convTx routines need a type descriptor to use when calling mallocgc.
/// We don't need the type to be exact, just to have the correct size, alignment, and pointer-ness.
/// However, when debugging, it'd be nice to have some indication in mallocgc where the types came from,
/// so we use named types here.
/// We then construct interface values of these types,
/// and then extract the type word to use as needed.
#[derive(Debug, Clone, Default)]
pub struct uint32InterfacePtr(pub Arc<Mutex<Option<u32>>>);

impl Display for uint32InterfacePtr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for uint32InterfacePtr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for uint32InterfacePtr {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for uint32InterfacePtr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for uint32InterfacePtr {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<uint32InterfacePtr> for u32 {
    fn eq(&self, other: &uint32InterfacePtr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<uint32InterfacePtr> for u32 {
    fn partial_cmp(&self, other: &uint32InterfacePtr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn add(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn add(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn add(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn sub(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn sub(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn sub(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn mul(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn mul(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn mul(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn div(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn div(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn div(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn rem(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn rem(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn rem(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitand(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitand(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn bitand(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitor(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitor(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn bitor(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitxor(self, other: Self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn bitxor(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<uint32InterfacePtr> for u32 {
    type Output = uint32InterfacePtr;
    fn bitxor(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn not(self) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: i32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: i8) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: i16) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: i64) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: u8) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: u16) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: u64) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shl(self, other: usize) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: uint32InterfacePtr) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: i32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: i8) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: i16) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: i64) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: u32) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: u8) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: u16) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: u64) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for uint32InterfacePtr {
    type Output = uint32InterfacePtr;
    fn shr(self, other: usize) -> uint32InterfacePtr {
        uint32InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for uint32InterfacePtr {}

impl Ord for uint32InterfacePtr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// The specialized convTx routines need a type descriptor to use when calling mallocgc.
/// We don't need the type to be exact, just to have the correct size, alignment, and pointer-ness.
/// However, when debugging, it'd be nice to have some indication in mallocgc where the types came from,
/// so we use named types here.
/// We then construct interface values of these types,
/// and then extract the type word to use as needed.
#[derive(Debug, Clone, Default)]
pub struct uint64InterfacePtr(pub Arc<Mutex<Option<u64>>>);

impl Display for uint64InterfacePtr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for uint64InterfacePtr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for uint64InterfacePtr {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for uint64InterfacePtr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for uint64InterfacePtr {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<uint64InterfacePtr> for u64 {
    fn eq(&self, other: &uint64InterfacePtr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<uint64InterfacePtr> for u64 {
    fn partial_cmp(&self, other: &uint64InterfacePtr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn add(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn add(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn add(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn sub(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn sub(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn sub(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn mul(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn mul(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn mul(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn div(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn div(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn div(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn rem(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn rem(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn rem(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitand(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitand(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn bitand(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitor(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitor(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn bitor(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitxor(self, other: Self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn bitxor(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<uint64InterfacePtr> for u64 {
    type Output = uint64InterfacePtr;
    fn bitxor(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn not(self) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: i32) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: i8) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: i16) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: i64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: u32) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: u8) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: u16) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shl(self, other: usize) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: uint64InterfacePtr) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: i32) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: i8) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: i16) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: i64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: u32) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: u8) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: u16) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: u64) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for uint64InterfacePtr {
    type Output = uint64InterfacePtr;
    fn shr(self, other: usize) -> uint64InterfacePtr {
        uint64InterfacePtr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for uint64InterfacePtr {}

impl Ord for uint64InterfacePtr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// The specialized convTx routines need a type descriptor to use when calling mallocgc.
/// We don't need the type to be exact, just to have the correct size, alignment, and pointer-ness.
/// However, when debugging, it'd be nice to have some indication in mallocgc where the types came from,
/// so we use named types here.
/// We then construct interface values of these types,
/// and then extract the type word to use as needed.
#[derive(Debug, Clone, Default)]
pub struct stringInterfacePtr(pub Arc<Mutex<Option<String>>>);

impl Display for stringInterfacePtr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for stringInterfacePtr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


/// The specialized convTx routines need a type descriptor to use when calling mallocgc.
/// We don't need the type to be exact, just to have the correct size, alignment, and pointer-ness.
/// However, when debugging, it'd be nice to have some indication in mallocgc where the types came from,
/// so we use named types here.
/// We then construct interface values of these types,
/// and then extract the type word to use as needed.
#[derive(Debug, Clone, Default)]
pub struct sliceInterfacePtr(pub Arc<Mutex<Option<Vec<u8>>>>);

impl Display for sliceInterfacePtr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
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


pub(crate) static itabLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::runtime2::mutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static itabTable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<itabTableType>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static itabTableInit: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<itabTableType>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint16Eface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint32Eface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint64Eface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stringEface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sliceEface: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Any + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint16Type: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint32Type: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static uint64Type: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static stringType: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static sliceType: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<GoPtr<internal_abi::r#type::Type>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static emptyTypeAssertCache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_abi::switch::TypeAssertCache>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static emptyInterfaceSwitchCache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<internal_abi::switch::InterfaceSwitchCache>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static staticuint64s: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u64; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *itabLock.lock().unwrap() = Some(Default::default());
    *itabTable.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *itabTableInit.lock().unwrap() = Some(Default::default());
    *uint16Eface.lock().unwrap() = None;
    *uint32Eface.lock().unwrap() = None;
    *uint64Eface.lock().unwrap() = None;
    *stringEface.lock().unwrap() = None;
    *sliceEface.lock().unwrap() = None;
    *uint16Type.lock().unwrap() = Some(GoPtr::nil());
    *uint32Type.lock().unwrap() = Some(GoPtr::nil());
    *uint64Type.lock().unwrap() = Some(GoPtr::nil());
    *stringType.lock().unwrap() = Some(GoPtr::nil());
    *sliceType.lock().unwrap() = Some(GoPtr::nil());
    *emptyTypeAssertCache.lock().unwrap() = Some(Default::default());
    *emptyInterfaceSwitchCache.lock().unwrap() = Some(Default::default());
    *staticuint64s.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *itabTableInit.lock().unwrap() = Some(itabTableType { size: Arc::new(Mutex::new(Some(ITAB_INIT_SIZE as usize))), ..Default::default() });
    *itabTable.lock().unwrap() = Some(itabTableInit.clone());
    *uint16Eface.lock().unwrap() = Some(Box::new(uint16InterfacePtr(Arc::new(Mutex::new(Some(0 as u16))))) as Box<dyn Any + Send + Sync>);
    *uint32Eface.lock().unwrap() = Some(Box::new(uint32InterfacePtr(Arc::new(Mutex::new(Some(0 as u32))))) as Box<dyn Any + Send + Sync>);
    *uint64Eface.lock().unwrap() = Some(Box::new(uint64InterfacePtr(Arc::new(Mutex::new(Some(0 as u64))))) as Box<dyn Any + Send + Sync>);
    *stringEface.lock().unwrap() = Some(Box::new(stringInterfacePtr(Arc::new(Mutex::new(Some("".to_string()))))) as Box<dyn Any + Send + Sync>);
    *sliceEface.lock().unwrap() = Some(Box::new(sliceInterfacePtr(Arc::new(Mutex::new(None::<Vec<u8>>)))) as Box<dyn Any + Send + Sync>);
    *uint16Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint16Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
    *uint32Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint32Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
    *uint64Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint64Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
    *stringType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(stringEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
    *sliceType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(sliceEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
    *emptyTypeAssertCache.lock().unwrap() = Some(internal_abi::switch::TypeAssertCache { mask: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() });
    *emptyInterfaceSwitchCache.lock().unwrap() = Some(internal_abi::switch::InterfaceSwitchCache { mask: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *itabLock.lock().unwrap() = Some(Default::default());
    *itabTable.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *itabTableInit.lock().unwrap() = Some(Default::default());
    *uint16Eface.lock().unwrap() = None;
    *uint32Eface.lock().unwrap() = None;
    *uint64Eface.lock().unwrap() = None;
    *stringEface.lock().unwrap() = None;
    *sliceEface.lock().unwrap() = None;
    *uint16Type.lock().unwrap() = Some(GoPtr::nil());
    *uint32Type.lock().unwrap() = Some(GoPtr::nil());
    *uint64Type.lock().unwrap() = Some(GoPtr::nil());
    *stringType.lock().unwrap() = Some(GoPtr::nil());
    *sliceType.lock().unwrap() = Some(GoPtr::nil());
    *emptyTypeAssertCache.lock().unwrap() = Some(Default::default());
    *emptyInterfaceSwitchCache.lock().unwrap() = Some(Default::default());
    *staticuint64s.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_7() {
    *itabTableInit.lock().unwrap() = Some(itabTableType { size: Arc::new(Mutex::new(Some(ITAB_INIT_SIZE as usize))), ..Default::default() });
}


pub(crate) fn __go_init_order_8() {
    *itabTable.lock().unwrap() = Some(itabTableInit.clone());
}


pub(crate) fn __go_init_order_9() {
    *uint16Eface.lock().unwrap() = Some(Box::new(uint16InterfacePtr(Arc::new(Mutex::new(Some(0 as u16))))) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_order_10() {
    *uint32Eface.lock().unwrap() = Some(Box::new(uint32InterfacePtr(Arc::new(Mutex::new(Some(0 as u32))))) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_order_11() {
    *uint64Eface.lock().unwrap() = Some(Box::new(uint64InterfacePtr(Arc::new(Mutex::new(Some(0 as u64))))) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_order_12() {
    *stringEface.lock().unwrap() = Some(Box::new(stringInterfacePtr(Arc::new(Mutex::new(Some("".to_string()))))) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_order_13() {
    *sliceEface.lock().unwrap() = Some(Box::new(sliceInterfacePtr(Arc::new(Mutex::new(None::<Vec<u8>>)))) as Box<dyn Any + Send + Sync>);
}


pub(crate) fn __go_init_order_14() {
    *uint16Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint16Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_init_order_15() {
    *uint32Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint32Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_init_order_16() {
    *uint64Type.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(uint64Eface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_init_order_17() {
    *stringType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(stringEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_init_order_18() {
    *sliceType.lock().unwrap() = Some({ let __field = { let __ptr = eface_of(sliceEface.clone()); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap()._type.clone() }.clone(); __field });
}


pub(crate) fn __go_init_order_19() {
    *emptyTypeAssertCache.lock().unwrap() = Some(internal_abi::switch::TypeAssertCache { mask: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() });
}


pub(crate) fn __go_init_order_20() {
    *emptyInterfaceSwitchCache.lock().unwrap() = Some(internal_abi::switch::InterfaceSwitchCache { mask: Arc::new(Mutex::new(Some(0 as usize))), ..Default::default() });
}


impl itabTableType {
    /// find finds the given interface/type pair in t.
    /// Returns nil if the given interface/type pair isn't present.
    pub fn find(&self, inter: GoPtr<internal_abi::r#type::InterfaceType>, typ: GoPtr<internal_abi::r#type::Type>) -> GoPtr<internal_abi::iface::ITab> {
                // Implemented using quadratic probing.
                // Probe sequence is h(i) = h0 + i*(i+1)/2 mod 2^k.
                // We're guaranteed to hit all table entries using this probe sequence.
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let mut h = Arc::new(Mutex::new(Some({ let __tmp_x = itab_hash_func(inter.clone(), typ.clone()); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y })));
        let mut i = Arc::new(Mutex::new(Some(1 as usize)));
    loop {
        let mut p: GoPtr<Arc<Mutex<Option<internal_abi::iface::ITab>>>> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.entries.clone()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });

                // Use atomic read here so if we see m != nil, we also see
                // the initializations of the fields of m.
                // m := *p
        let mut m: GoPtr<internal_abi::iface::ITab> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(p.addr())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        if m.is_nil() {
        return GoPtr::nil();
    }
        if { let __left_addr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.inter.clone()); __ptr_value }.addr(); let __right_addr = inter.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __left_addr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.addr(); let __right_addr = typ.addr(); let __eq = __left_addr == __right_addr; __eq } {
        return m.clone();
    }
        { let __rhs = (*i.lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*mask.lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// add adds the given itab to itab table t.
    /// itabLock must be held.
    pub fn add(&mut self, m: GoPtr<internal_abi::iface::ITab>) {
                // See comment in find about the probe sequence.
                // Insert new itab in the first empty spot in the probe sequence.
        let mut mask = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.size.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x - __tmp_y })));
        let mut h = Arc::new(Mutex::new(Some({ let __tmp_x = itab_hash_func({ let __go_ptr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.inter.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }, { let __go_ptr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }); let __tmp_y = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y })));
        let mut i = Arc::new(Mutex::new(Some(1 as usize)));
    loop {
        let mut p: GoPtr<Arc<Mutex<Option<internal_abi::iface::ITab>>>> = GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.entries.clone()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        let mut m2: GoPtr<internal_abi::iface::ITab> = { let __ptr_slot = p.borrow(); GoPtr::local(__ptr_slot.as_ref().unwrap().clone()) };
        if { let __left_addr = m2.addr(); let __right_addr = m.addr(); let __eq = __left_addr == __right_addr; __eq } {
                // A given itab may be used in more than one module
                // and thanks to the way global symbol resolution works, the
                // pointed-to itab may already have been inserted into the
                // global 'hash'.
        return;
    }
                // A given itab may be used in more than one module
                // and thanks to the way global symbol resolution works, the
                // pointed-to itab may already have been inserted into the
                // global 'hash'.
        if m2.is_nil() {
                // Use atomic write here so if a reader sees m, it also
                // sees the correctly initialized fields of m.
                // NoWB is ok because m is not in heap memory.
                // *p = m
        internal_runtime_atomic::storep_no_w_b(Arc::new(Mutex::new(Some(p.addr()))), Arc::new(Mutex::new(Some(m.addr()))));
        { let __target = self.count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
                // Use atomic write here so if a reader sees m, it also
                // sees the correctly initialized fields of m.
                // NoWB is ok because m is not in heap memory.
                // *p = m
        { let __rhs = (*i.lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*mask.lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() & __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
}

pub fn itab_hash_func(inter: GoPtr<internal_abi::r#type::InterfaceType>, typ: GoPtr<internal_abi::r#type::Type>) -> usize {
        // compiler has provided some good hash codes for us.
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*(*{ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hash.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = typ.borrow(); __ptr_value.as_ref().unwrap().hash.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x ^ __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap())
}

/// getitab should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/bytedance/sonic
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname getitab
pub fn getitab(inter: GoPtr<internal_abi::r#type::InterfaceType>, typ: GoPtr<internal_abi::r#type::Type>, canfail: Arc<Mutex<Option<bool>>>) -> GoPtr<internal_abi::iface::ITab> {
    if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("internal error - misuse of itab".to_string()))));
    }

        // easy case
    if { let __tmp_x = { let __tmp_x = { let __selector_holder = { let __ptr_value = typ.with_mut(|__ptr_value| __ptr_value.t_flag.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_UNCOMMON as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
        if { let __v = (*canfail.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return GoPtr::nil();
    }
        let mut name = { let __recv = to_r_type(GoPtr::local({ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(crate::error::TypeAssertionError { _interface: Default::default(), concrete: typ.clone(), asserted: { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone().clone(), missing_method: (*name.lock().unwrap().as_ref().unwrap()).name(), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>);
    }

    let mut m: GoPtr<internal_abi::iface::ITab> = GoPtr::nil();

        // First, look in the existing table to see if we can find the itab we need.
        // This is by far the most common case, so do it without locks.
        // Use atomic to ensure we see any previous writes done by the thread
        // that updates the itabTable field (with atomic.Storep in itabAdd).
    let mut t: GoPtr<itabTableType> = GoPtr::raw({ let __ptr = internal_runtime_atomic::loadp(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(itabTable.clone())))) as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    'finish: {
        {
        m = { let __result = t.with_mut(|__recv_value| __recv_value.find(inter.clone(), typ.clone())); __result };;
        if !m.is_nil() {
            break 'finish;;
        }
    }

                // Not found.  Grab the lock and try again.
        lock(GoPtr::local(itabLock.clone()));
        {
        m = { let __recv_holder = (*itabTable.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).find(inter.clone(), typ.clone()); __result };;
        if !m.is_nil() {
            unlock(GoPtr::local(itabLock.clone()));;
            break 'finish;;
        }
    }

                // Entry doesn't exist yet. Make a new entry & add it.
        m = GoPtr::raw({ let __ptr = persistentalloc(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(std::mem::size_of::<internal_abi::iface::ITab>()))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(0 as usize))), (*memstats.lock().unwrap().as_ref().unwrap()).other_sys.clone()).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __go_ptr = inter.clone(); match __go_ptr { GoPtr::Nil => internal_abi::GoPtr::nil(), GoPtr::Local(__value) => internal_abi::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_abi::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_abi::GoPtr::slice_elem(internal_abi::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }; m.with_mut(|__ptr_value| { __ptr_value.inter = new_val; }); };
        { let new_val = { let __go_ptr = typ.clone(); match __go_ptr { GoPtr::Nil => internal_abi::GoPtr::nil(), GoPtr::Local(__value) => internal_abi::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => internal_abi::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => internal_abi::GoPtr::slice_elem(internal_abi::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }; m.with_mut(|__ptr_value| { __ptr_value.r#type = new_val; }); };

                // The hash is used in type switches. However, compiler statically generates itab's
                // for all interface/type pairs used in switches (which are added to itabTable
                // in itabsinit). The dynamically-generated itab's never participate in type switches,
                // and thus the hash is irrelevant.
                // Note: m.Hash is _not_ the hash used for the runtime itabTable hash table.
        { let new_val = 0 as u32; *{ let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.hash.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        itab_init(m.clone(), Arc::new(Mutex::new(Some(true))));
        itab_add(m.clone());
        unlock(GoPtr::local(itabLock.clone()));
    }
    if { let __tmp_x = { let __seq = { let __seq_holder = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.fun.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        return m.clone();
    }
    if { let __v = (*canfail.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return GoPtr::nil();
    }

        // this can only happen if the conversion
        // was already done once using the , ok form
        // and we have a cached negative result.
        // The cached result doesn't record which
        // interface function was missing, so initialize
        // the itab again to get the missing function name.
    std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(crate::error::TypeAssertionError { concrete: typ.clone(), asserted: { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone().clone(), missing_method: itab_init(m.clone(), Arc::new(Mutex::new(Some(false)))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>);
    unreachable!()
}

/// itabAdd adds the given itab to the itab hash table.
/// itabLock must be held.
pub fn itab_add(m: GoPtr<internal_abi::iface::ITab>) {
        // Bugs can lead to calling this while mallocing is set,
        // typically because this is called while panicking.
        // Crash reliably, rather than only when we need to grow
        // the hash table.
    if { let __tmp_x = (*(*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).mallocing.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("malloc deadlock".to_string()))));
    }

    let mut t = (*itabTable.lock().unwrap().as_ref().unwrap()).clone();
    if { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).count.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = 3 as usize; let __tmp_y = ({ let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 4 as usize; __tmp_x / __tmp_y }); __tmp_x * __tmp_y }; __tmp_x >= __tmp_y } {
                // Grow hash table.
                // t2 = new(itabTableType) + some additional entries
                // We lie and tell malloc we want pointer-free memory because
                // all the pointed-to values are not in the heap.
        let mut t2: GoPtr<itabTableType> = GoPtr::raw({ let __ptr = mallocgc(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = 2 as usize; let __tmp_y = { let __tmp_x = 2 as usize; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; __tmp_x + __tmp_y }); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y }))), GoPtr::nil(), Arc::new(Mutex::new(Some(true)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = { let __tmp_x = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as usize; __tmp_x * __tmp_y }; *{ let __ptr_value = t2.with_mut(|__ptr_value| __ptr_value.size.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Copy over entries.
                // Note: while copying, other threads may look for an itab and
                // fail to find it. That's ok, they will then try to get the itab lock
                // and as a consequence wait until this copying is complete.
        iterate_itabs(Arc::new(Mutex::new(Some({ let __recv = t2.clone(); Box::new(move |__arg0: Arc<Mutex<Option<internal_abi::iface::ITab>>>| { __recv.with_mut(|__recv_value| __recv_value.add(GoPtr::local(__arg0))) }) as Box<dyn FnMut(Arc<Mutex<Option<internal_abi::iface::ITab>>>) -> () + Send + Sync> }))));
        if { let __tmp_x = (*{ let __ptr_value = t2.borrow(); __ptr_value.as_ref().unwrap().count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).count.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("mismatched count during itab table copy".to_string()))));
    }
                // Publish new hash table. Use an atomic write: see comment in getitab.
        atomicstorep(Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(itabTable.clone())))) as usize))), Arc::new(Mutex::new(Some(t2.addr()))));
                // Adopt the new table as our own.
        { let new_val = (*itabTable.lock().unwrap().as_ref().unwrap()).clone(); t = new_val; };
    }
        // Grow hash table.
        // t2 = new(itabTableType) + some additional entries
        // We lie and tell malloc we want pointer-free memory because
        // all the pointed-to values are not in the heap.
        // Copy over entries.
        // Note: while copying, other threads may look for an itab and
        // fail to find it. That's ok, they will then try to get the itab lock
        // and as a consequence wait until this copying is complete.
        // Publish new hash table. Use an atomic write: see comment in getitab.
        // Adopt the new table as our own.
        // Note: the old table can be GC'ed here.
    { let __recv = t.clone(); let __recv_ptr: *mut itabTableType = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut itabTableType }; let __result = unsafe { &mut *__recv_ptr }.add(m.clone()); __result };
}

/// itabInit fills in the m.Fun array with all the code pointers for
/// the m.Inter/m.Type pair. If the type does not implement the interface,
/// it sets m.Fun[0] to 0 and returns the name of an interface function that is missing.
/// If !firstTime, itabInit will not write anything to m.Fun (see issue 65962).
/// It is ok to call this multiple times on the same m, even concurrently
/// (although it will only be called once with firstTime==true).
pub fn itab_init(m: GoPtr<internal_abi::iface::ITab>, firstTime: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
    let mut inter: GoPtr<internal_abi::r#type::InterfaceType> = { let __go_ptr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.inter.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
    let mut typ: GoPtr<internal_abi::r#type::Type> = { let __go_ptr = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
    let mut x = { let __result = typ.with_mut(|__recv_value| __recv_value.uncommon()); __result };

        // both inter and typ have method sorted by name,
        // and interface names are unique,
        // so can iterate over both in lock step;
        // the loop is O(ni+nt) not O(ni*nt).
    let mut ni = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
    let mut nt = Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mcount.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32)));
    let mut xmhdr = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&x) as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).moff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[internal_abi::r#type::Method; 65536]>(unimplemented!("unsafe.Pointer conversion to [internal_abi::r#type::Method; 65536]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = 0; let __high = ({ let __v = (*nt.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = ({ let __v = (*nt.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    let mut j = Arc::new(Mutex::new(Some(0)));
    let mut methods = Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = { let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.fun.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[usize; 65536]>(unimplemented!("unsafe.Pointer conversion to [usize; 65536]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = 0; let __high = ({ let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = ({ let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    let mut fun0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    let mut k = Arc::new(Mutex::new(Some(0)));
    'imethods: while { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*ni.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut i: Option<GoSliceElemPtr<internal_abi::r#type::Imethod>> = Some(GoSliceElemPtr::new({ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(), ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut itype: GoPtr<internal_abi::r#type::Type> = { let __recv = to_r_type(GoPtr::local({ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*i.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        let mut name = { let __recv = to_r_type(GoPtr::local({ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*i.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
        let mut iname = (*name.lock().unwrap().as_ref().unwrap()).name();
        let mut ipkg = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*ipkg.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ipkg.lock().unwrap() = __moved_val; };
    }
        while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*nt.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut t: Option<GoSliceElemPtr<internal_abi::r#type::Method>> = Some(GoSliceElemPtr::new(xmhdr.clone(), ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut rtyp = to_r_type(typ.clone());
        let mut tname = (*rtyp.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __left_addr = (*rtyp.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).mtyp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).addr(); let __right_addr = itype.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __tmp_x = (*(*tname.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*iname.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        let mut pkgPath = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = tname.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*pkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let __recv = (*rtyp.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).pkg_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pkgPath.lock().unwrap() = __moved_val; };
    }
        if (*tname.lock().unwrap().as_ref().unwrap()).is_exported() || { let __tmp_x = (*pkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ipkg.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        let mut ifn = (*rtyp.lock().unwrap().as_ref().unwrap()).text_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.as_ref().unwrap().borrow().as_ref().unwrap()).ifn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = ifn.lock().unwrap().as_ref().unwrap().clone(); *fun0.lock().unwrap() = Some(new_val); };
    } else if { let __v = (*firstTime.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*methods.lock().unwrap().as_mut().unwrap())[({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*ifn.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
                // we'll set m.Fun[0] at the end
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'imethods
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // we'll set m.Fun[0] at the end
                // didn't find method
                // Leaves m.Fun[0] set to 0.
        return { let __owned = iname.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // we'll set m.Fun[0] at the end
        // didn't find method
        // Leaves m.Fun[0] set to 0.
    if { let __v = (*firstTime.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        (*{ let __ptr_value = m.with_mut(|__ptr_value| __ptr_value.fun.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some((*fun0.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()).clone();
    }
    Arc::new(Mutex::new(Some("".to_string())))
}

pub fn assert_e2_i(inter: GoPtr<internal_abi::r#type::InterfaceType>, t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<internal_abi::iface::ITab> {
    if t.is_nil() {
                // explicit conversions require non-nil interface value.
        std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(crate::error::TypeAssertionError { _interface: Default::default(), concrete: GoPtr::nil(), asserted: { let __ptr_value = inter.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone().clone(), missing_method: Arc::new(Mutex::new(Some("".to_string()))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>);
    }
        // explicit conversions require non-nil interface value.
    getitab(inter.clone(), t.clone(), Arc::new(Mutex::new(Some(false))))
}

pub fn assert_e2_i2(inter: GoPtr<internal_abi::r#type::InterfaceType>, t: GoPtr<internal_abi::r#type::Type>) -> GoPtr<internal_abi::iface::ITab> {
    if t.is_nil() {
        return GoPtr::nil();
    }
    getitab(inter.clone(), t.clone(), Arc::new(Mutex::new(Some(true))))
}

pub fn iterate_itabs(r#fn: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<internal_abi::iface::ITab>>>) -> () + Send + Sync>>>>) {
        // Note: only runs during stop the world or with itabLock held,
        // so no other locks/atomics needed.
    let mut t = (*itabTable.lock().unwrap().as_ref().unwrap()).clone();
    let mut i = Arc::new(Mutex::new(Some(0 as usize)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).size.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut m = (*Arc::new(Mutex::new({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&(*t.lock().unwrap().as_ref().unwrap()).entries.clone()) as usize))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x * __tmp_y })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<Arc<Mutex<Option<internal_abi::iface::ITab>>>>(unimplemented!("unsafe.Pointer conversion to Arc<Mutex<Option<internal_abi::iface::ITab>>>")) } })).lock().unwrap().as_mut().unwrap()).clone();
        if { let __nil_result = (*m.lock().unwrap()).is_some(); __nil_result } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<internal_abi::iface::ITab>>>) -> () + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<internal_abi::iface::ITab>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(m.clone()) };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
}

/// The linker redirects a reference of a method that it determined
/// unreachable to a reference to this function, so it will throw if
/// ever called.
pub fn unreachable_method() {
    throw(Arc::new(Mutex::new(Some("unreachable method called. linker bug?".to_string()))));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for itabTableType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
