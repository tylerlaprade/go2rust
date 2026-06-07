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

/// Per-thread (in Go, per-P) cache for small objects.
/// This includes a small object cache and local allocation stats.
/// No locking needed because it is per-thread (per-P).
///
/// mcaches are allocated from non-GC'd memory, so any heap pointers
/// must be specially handled.
#[derive(Clone)]
pub struct mcache {
    pub __blank_0_0: Arc<Mutex<Option<internal_runtime_sys::nih::NotInHeap>>>,
    pub next_sample: Arc<Mutex<Option<i64>>>,
    pub mem_prof_rate: Arc<Mutex<Option<i32>>>,
    pub scan_alloc: Arc<Mutex<Option<usize>>>,
    pub tiny: Arc<Mutex<Option<usize>>>,
    pub tinyoffset: Arc<Mutex<Option<usize>>>,
    pub tiny_allocs: Arc<Mutex<Option<usize>>>,
    pub alloc: Arc<Mutex<Option<[GoPtr<crate::mheap::mspan>; 136]>>>,
    pub stackcache: Arc<Mutex<Option<[stackfreelist; 4]>>>,
    pub flush_gen: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
}

impl mcache {
    pub fn __go_value_clone(&self) -> Self {
        Self { __blank_0_0: { let __guard = self.__blank_0_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, next_sample: { let __guard = self.next_sample.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mem_prof_rate: { let __guard = self.mem_prof_rate.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scan_alloc: { let __guard = self.scan_alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tiny: { let __guard = self.tiny.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tinyoffset: { let __guard = self.tinyoffset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tiny_allocs: { let __guard = self.tiny_allocs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alloc: { let __guard = self.alloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stackcache: { let __guard = self.stackcache.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, flush_gen: { let __guard = self.flush_gen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for mcache {
    fn default() -> Self {
        Self { __blank_0_0: Arc::new(Mutex::new(Some(Default::default()))), next_sample: Arc::new(Mutex::new(Some(0))), mem_prof_rate: Arc::new(Mutex::new(Some(0))), scan_alloc: Arc::new(Mutex::new(Some(0))), tiny: Arc::new(Mutex::new(Some(0))), tinyoffset: Arc::new(Mutex::new(Some(0))), tiny_allocs: Arc::new(Mutex::new(Some(0))), alloc: Arc::new(Mutex::new(Some(std::array::from_fn(|_| GoPtr::nil())))), stackcache: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Default::default())))), flush_gen: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for mcache {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {}}}", (*self.__blank_0_0.lock().unwrap().as_ref().unwrap()), (*self.next_sample.lock().unwrap().as_ref().unwrap()), (*self.mem_prof_rate.lock().unwrap().as_ref().unwrap()), (*self.scan_alloc.lock().unwrap().as_ref().unwrap()), (*self.tiny.lock().unwrap().as_ref().unwrap()), (*self.tinyoffset.lock().unwrap().as_ref().unwrap()), (*self.tiny_allocs.lock().unwrap().as_ref().unwrap()), { let __guard = self.alloc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("[{}]", __v.iter().map(|__p| if __p.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() }).collect::<Vec<_>>().join(" ")), None => "[]".to_string() } }, format_slice(&self.stackcache), (*self.flush_gen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for mcache {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A gclink is a node in a linked list of blocks, like mlink,
/// but it is opaque to the garbage collector.
/// The GC does not trace the pointers during collection,
/// and the compiler does not emit write barriers for assignments
/// of gclinkptr values. Code should store references to gclinks
/// as gclinkptr, not as *gclink.
#[derive(Debug, Clone)]
pub struct gclink {
    pub next: Arc<Mutex<Option<gclinkptr>>>,
}

impl gclink {
    pub fn __go_value_clone(&self) -> Self {
        Self { next: { let __guard = self.next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for gclink {
    fn default() -> Self {
        Self { next: Arc::new(Mutex::new(Some(gclinkptr(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for gclink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.next.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for gclink {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A gclinkptr is a pointer to a gclink, but it is opaque
/// to the garbage collector.
#[derive(Debug, Clone, Default)]
pub struct gclinkptr(pub Arc<Mutex<Option<usize>>>);

impl Display for gclinkptr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for gclinkptr {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for gclinkptr {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for gclinkptr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for gclinkptr {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<gclinkptr> for usize {
    fn eq(&self, other: &gclinkptr) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<gclinkptr> for usize {
    fn partial_cmp(&self, other: &gclinkptr) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for gclinkptr {
    type Output = gclinkptr;
    fn add(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for gclinkptr {
    type Output = gclinkptr;
    fn add(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<gclinkptr> for usize {
    type Output = gclinkptr;
    fn add(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for gclinkptr {
    type Output = gclinkptr;
    fn sub(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for gclinkptr {
    type Output = gclinkptr;
    fn sub(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<gclinkptr> for usize {
    type Output = gclinkptr;
    fn sub(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for gclinkptr {
    type Output = gclinkptr;
    fn mul(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for gclinkptr {
    type Output = gclinkptr;
    fn mul(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<gclinkptr> for usize {
    type Output = gclinkptr;
    fn mul(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for gclinkptr {
    type Output = gclinkptr;
    fn div(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for gclinkptr {
    type Output = gclinkptr;
    fn div(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<gclinkptr> for usize {
    type Output = gclinkptr;
    fn div(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for gclinkptr {
    type Output = gclinkptr;
    fn rem(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for gclinkptr {
    type Output = gclinkptr;
    fn rem(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<gclinkptr> for usize {
    type Output = gclinkptr;
    fn rem(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for gclinkptr {
    type Output = gclinkptr;
    fn bitand(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for gclinkptr {
    type Output = gclinkptr;
    fn bitand(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<gclinkptr> for usize {
    type Output = gclinkptr;
    fn bitand(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for gclinkptr {
    type Output = gclinkptr;
    fn bitor(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for gclinkptr {
    type Output = gclinkptr;
    fn bitor(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<gclinkptr> for usize {
    type Output = gclinkptr;
    fn bitor(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for gclinkptr {
    type Output = gclinkptr;
    fn bitxor(self, other: Self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for gclinkptr {
    type Output = gclinkptr;
    fn bitxor(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<gclinkptr> for usize {
    type Output = gclinkptr;
    fn bitxor(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for gclinkptr {
    type Output = gclinkptr;
    fn not(self) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: i32) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: i8) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: i16) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: i64) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: u32) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: u8) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: u16) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: u64) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for gclinkptr {
    type Output = gclinkptr;
    fn shl(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: gclinkptr) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: i32) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: i8) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: i16) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: i64) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: u32) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: u8) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: u16) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: u64) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for gclinkptr {
    type Output = gclinkptr;
    fn shr(self, other: usize) -> gclinkptr {
        gclinkptr(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for gclinkptr {}

impl Ord for gclinkptr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Debug, Clone)]
pub struct stackfreelist {
    pub list: Arc<Mutex<Option<gclinkptr>>>,
    pub size: Arc<Mutex<Option<usize>>>,
}

impl stackfreelist {
    pub fn __go_value_clone(&self) -> Self {
        Self { list: { let __guard = self.list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for stackfreelist {
    fn default() -> Self {
        Self { list: Arc::new(Mutex::new(Some(gclinkptr(Arc::new(Mutex::new(Some(0))))))), size: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for stackfreelist {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.list.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for stackfreelist {
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


pub(crate) type debugPtrmask = AnonymousStruct5;


pub(crate) type globalAlloc = AnonymousStruct4;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) static emptymspan: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::mheap::mspan>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *emptymspan.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *emptymspan.lock().unwrap() = Some(Default::default());
}


impl gclinkptr {
    /// ptr returns the *gclink form of p.
    /// The result should be used for accessing fields, not stored
    /// in other data structures.
    pub fn ptr(&self) -> GoPtr<gclink> {
        GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }
}

impl mcache {
    /// refill acquires a new span of span class spc for c. This span will
    /// have at least one free object. The current span in c must be full.
    ///
    /// Must run in a non-preemptible context since otherwise the owner of
    /// c could change.
    pub fn refill(&mut self, spc: Arc<Mutex<Option<spanClass>>>) {
                // Return the current cached span to the central lists.
        let mut s: GoPtr<crate::mheap::mspan> = self.alloc.lock().unwrap().as_ref().unwrap()[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone();
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("refill of span with free space remaining".to_string()))));
    }
        if { let __left_addr = s.addr(); let __right_addr = { let __ptr = GoPtr::local(emptymspan.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq } {
                // Mark this span as no longer cached.
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as u32; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("bad sweepgen in refill".to_string()))));
    }
        (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).uncache_span(s.clone());
                // Count up how many slots were used and record it.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        let mut slotsUsed = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count_before_cache.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        { let __elem_ptr_0 = Some(GoArrayElemPtr::new((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).small_alloc_count.clone(), (crate::mheap::spanClass::sizeclass(&(*spc.lock().unwrap().as_ref().unwrap()))) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::xadd64(__arg0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = slotsUsed.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
                // Flush tinyAllocs.
        if { let __tmp_x = (*spc.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::mheap::spanClass(Arc::new(Mutex::new(Some(TINY_SPAN_CLASS as u8)))); __tmp_x == __tmp_y } {
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).tiny_alloc_count.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.tiny_allocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        { let new_val = 0 as usize; *self.tiny_allocs.lock().unwrap() = Some(new_val); };
    }
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Count the allocs in inconsistent, internal stats.
        let mut bytesAllocated = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*slotsUsed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_alloc.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = bytesAllocated.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
                // Clear the second allocCount just to be safe.
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count_before_cache.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
    }
                // Mark this span as no longer cached.
                // Count up how many slots were used and record it.
                // Flush tinyAllocs.
                // Count the allocs in inconsistent, internal stats.
                // Clear the second allocCount just to be safe.
                // Get a new cached span from the central lists.
        s = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).cache_span();
        if s.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("span has no free space".to_string()))));
    }
                // Indicate that this span is cached and prevent asynchronous
                // sweeping in the next sweep phase.
        { let new_val = { let __tmp_x = (*{ let __field = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 3 as u32; __tmp_x + __tmp_y }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.sweepgen.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Store the current alloc count for accounting later.
        { let new_val = { let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count_before_cache.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Update heapLive and flush scanAlloc.
                //
                // We have not yet allocated anything new into the span, but we
                // assume that all of its slots will get used, so this makes
                // heapLive an overestimate.
                //
                // When the span gets uncached, we'll fix up this overestimate
                // if necessary (see releaseAll).
                //
                // We pick an overestimate here because an underestimate leads
                // the pacer to believe that it's in better shape than it is,
                // which appears to lead to more memory used. See #53738 for
                // more details.
        let mut usedBytes = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().elemsize.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y })));
        (*gcController.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*usedBytes.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.scan_alloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        { let new_val = 0 as usize; *self.scan_alloc.lock().unwrap() = Some(new_val); };
        (*self.alloc.lock().unwrap().as_mut().unwrap())[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize] = s.clone();
    }

    /// allocLarge allocates a span for a large object.
    pub fn alloc_large(&self, size: Arc<Mutex<Option<usize>>>, noscan: Arc<Mutex<Option<bool>>>) -> GoPtr<crate::mheap::mspan> {
        if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_SIZE as usize; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
        let mut npages = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_SHIFT; __tmp_x >> __tmp_y })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_MASK as usize; __tmp_x & __tmp_y }; let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let mut guard = npages.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // Deduct credit for this span allocation and sweep if
                // necessary. mHeap_Alloc will also sweep npages, so this only
                // pays the debt down to npage pages.
        deduct_sweep_credit(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = __PAGE_SIZE as usize; __tmp_x * __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut spc = make_span_class(Arc::new(Mutex::new(Some(0 as u8))), Arc::new(Mutex::new(Some({ let __arg_holder = noscan.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut s: GoPtr<crate::mheap::mspan> = (*mheap_.lock().unwrap().as_mut().unwrap()).alloc(Arc::new(Mutex::new(Some({ let __arg_holder = npages.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = spc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if s.is_nil() {
        throw(Arc::new(Mutex::new(Some("out of memory".to_string()))));
    }
                // Count the alloc in consistent, external stats.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_alloc.clone(), Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64))));
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).large_alloc_count.clone(), Arc::new(Mutex::new(Some(1 as i64))));
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Count the alloc in inconsistent, internal stats.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_alloc.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*npages.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64))));
                // Update heapLive.
        (*gcController.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().npages.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = PAGE_SIZE as usize; __tmp_x * __tmp_y }) as i64))), Arc::new(Mutex::new(Some(0 as i64))));
                // Put the large span in the mcentral swept list so that it's
                // visible to the background sweeper.
        { let __recv = (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*spc.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).full_swept(Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).push(s.clone()); __result };
                // Adjust s.limit down to the object-containing part of the span.
                //
                // This is just to create a slightly tighter bound on the limit.
                // It's totally OK if the garbage collector, in particular
                // conservative scanning, can temporarily observes an inflated
                // limit. It will simply mark the whole object or just skip it
                // since we're in the mark phase anyway.
        { let new_val = { let __tmp_x = { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.limit.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
        { let __recv_value = s.borrow(); let __result = (*__recv_value.as_ref().unwrap()).init_heap_bits(); __result };
        s.clone()
    }

    pub fn release_all(&mut self) {
                // Take this opportunity to flush scanAlloc.
        let mut scanAlloc = Arc::new(Mutex::new(Some({ let __selector_holder = self.scan_alloc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64)));
        { let new_val = 0 as usize; *self.scan_alloc.lock().unwrap() = Some(new_val); };
        let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut dHeapLive = Arc::new(Mutex::new(Some(0 as i64)));
        for i in 0..(({ let __range_holder = self.alloc.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut s: GoPtr<crate::mheap::mspan> = self.alloc.lock().unwrap().as_ref().unwrap()[(i) as usize].clone();
        if { let __left_addr = s.addr(); let __right_addr = { let __ptr = GoPtr::local(emptymspan.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq } {
        let mut slotsUsed = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count_before_cache.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y })));
        { let new_val = 0 as u16; *{ let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.alloc_count_before_cache.clone()); __ptr_value }.lock().unwrap() = Some(new_val); };
                // Adjust smallAllocCount for whatever was allocated.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        { let __elem_ptr_0 = Some(GoArrayElemPtr::new((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).small_alloc_count.clone(), (crate::mheap::spanClass::sizeclass(&(crate::mheap::spanClass(Arc::new(Mutex::new(Some(i as u8))))))) as usize)); let __arg0 = Arc::new(Mutex::new(__elem_ptr_0.as_ref().and_then(|__ptr| (*__ptr.borrow()).clone()))); let __result = internal_runtime_atomic::xadd64(__arg0.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = slotsUsed.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); if let Some(__ptr) = __elem_ptr_0.as_ref() { let mut __elem_guard_0 = __ptr.borrow_mut(); *__elem_guard_0 = (*__arg0.lock().unwrap()).clone(); }; __result };
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Adjust the actual allocs in inconsistent, internal stats.
                // We assumed earlier that the full span gets allocated.
        (*(*gcController.lock().unwrap().as_ref().unwrap()).total_alloc.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*slotsUsed.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }))));
        if { let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().sweepgen.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __tmp_x = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as u32; __tmp_x + __tmp_y }; __tmp_x != __tmp_y } {
                // refill conservatively counted unallocated slots in gcController.heapLive.
                // Undo this.
                //
                // If this span was cached before sweep, then gcController.heapLive was totally
                // recomputed since caching this span, so we don't do this for stale spans.
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().nelems.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = s.borrow(); __ptr_value.as_ref().unwrap().alloc_count.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = s.with_mut(|__ptr_value| __ptr_value.elemsize.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = dHeapLive.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // refill conservatively counted unallocated slots in gcController.heapLive.
                // Undo this.
                //
                // If this span was cached before sweep, then gcController.heapLive was totally
                // recomputed since caching this span, so we don't do this for stale spans.
                // Release the span to the mcentral.
        (*{ let __seq = { let __seq_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).central.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.mcentral.lock().unwrap().as_ref().unwrap()).uncache_span(s.clone());
        (*self.alloc.lock().unwrap().as_mut().unwrap())[(i) as usize] = GoPtr::local(emptymspan.clone());
    }
    }
                // Adjust smallAllocCount for whatever was allocated.
                // Adjust the actual allocs in inconsistent, internal stats.
                // We assumed earlier that the full span gets allocated.
                // refill conservatively counted unallocated slots in gcController.heapLive.
                // Undo this.
                //
                // If this span was cached before sweep, then gcController.heapLive was totally
                // recomputed since caching this span, so we don't do this for stale spans.
                // Release the span to the mcentral.
                // Clear tinyalloc pool.
        { let new_val = 0 as usize; *self.tiny.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *self.tinyoffset.lock().unwrap() = Some(new_val); };
                // Flush tinyAllocs.
        let mut stats: Option<GoArrayElemPtr<heapStatsDelta, 3>> = (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).acquire();
        internal_runtime_atomic::xadd64((*stats.as_ref().unwrap().borrow().as_ref().unwrap()).tiny_alloc_count.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.tiny_allocs.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))));
        { let new_val = 0 as usize; *self.tiny_allocs.lock().unwrap() = Some(new_val); };
        (*(*memstats.lock().unwrap().as_ref().unwrap()).heap_stats.lock().unwrap().as_mut().unwrap()).release();
                // Update heapLive and heapScan.
        (*gcController.lock().unwrap().as_ref().unwrap()).update(Arc::new(Mutex::new(Some({ let __arg_holder = dHeapLive.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = scanAlloc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// prepareForSweep flushes c if the system has entered a new sweep phase
    /// since c was populated. This must happen between the sweep phase
    /// starting and the first allocation from c.
    pub fn prepare_for_sweep(&mut self) {
                // Alternatively, instead of making sure we do this on every P
                // between starting the world and allocating on that P, we
                // could leave allocate-black on, allow allocation to continue
                // as usual, use a ragged barrier at the beginning of sweep to
                // ensure all cached spans are swept, and then disable
                // allocate-black. However, with this approach it's difficult
                // to avoid spilling mark bits into the *next* GC cycle.
        let mut sg = Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut flushGen = (*self.flush_gen.lock().unwrap().as_mut().unwrap()).load();
        if { let __tmp_x = flushGen; let __tmp_y = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return;
    } else if { let __tmp_x = flushGen; let __tmp_y = { let __tmp_x = { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2 as u32; __tmp_x - __tmp_y }; __tmp_x != __tmp_y } {
        eprintln!("{} {} {} {}", format!("{}", "bad flushGen".to_string()), format!("{}", flushGen), format!("{}", "in prepareForSweep; sweepgen".to_string()), format!("{}", { let __v = (*sg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        throw(Arc::new(Mutex::new(Some("bad flushGen".to_string()))));
    }
        self.release_all();
        stackcache_clear(Arc::new(Mutex::new(Some(self.clone()))));
        (*self.flush_gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
}

pub fn allocmcache() -> Arc<Mutex<Option<mcache>>> {
    let mut c: Arc<Mutex<Option<mcache>>> = Arc::new(Mutex::new(None));
    let mut c_closure_clone = c.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        { let new_val = Arc::new(Mutex::new({ let __ptr = (*(*mheap_.lock().unwrap().as_ref().unwrap()).cachealloc.lock().unwrap().as_mut().unwrap()).alloc().clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<mcache>(unimplemented!("unsafe.Pointer conversion to mcache")) } })).clone(); c_closure_clone = new_val; };
        (*(*c_closure_clone.lock().unwrap().as_ref().unwrap()).flush_gen.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __selector_holder = (*mheap_.lock().unwrap().as_ref().unwrap()).sweepgen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    for i in 0..(({ let __range_holder = (*c.lock().unwrap().as_ref().unwrap()).alloc.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        (*(*c.lock().unwrap().as_ref().unwrap()).alloc.lock().unwrap().as_mut().unwrap())[(i) as usize] = GoPtr::local(emptymspan.clone());
    }
    { let new_val = next_sample(); *(*c.lock().unwrap().as_ref().unwrap()).next_sample.lock().unwrap() = Some(new_val); };
    return c.clone();
}

/// freemcache releases resources associated with this
/// mcache and puts the object onto a free list.
///
/// In some cases there is no way to simply release
/// resources, such as statistics, so donate them to
/// a different mcache (the recipient).
pub fn freemcache(c: Arc<Mutex<Option<mcache>>>) {
    let c_closure_clone = c.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        { let __recv = c_closure_clone.clone(); let __recv_ptr: *mut mcache = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut mcache }; let __result = unsafe { &mut *__recv_ptr }.release_all(); __result };
        stackcache_clear(c_closure_clone.clone());
        lock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
        (*(*mheap_.lock().unwrap().as_ref().unwrap()).cachealloc.lock().unwrap().as_mut().unwrap()).free(Arc::new(Mutex::new(Some(Arc::as_ptr(&c) as usize))));
        unlock(GoPtr::local((*mheap_.lock().unwrap().as_ref().unwrap()).lock.clone()));
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
}

/// getMCache is a convenience function which tries to obtain an mcache.
///
/// Returns nil if we're not bootstrapping or we don't have a P. The caller's
/// P must not change, so we must be in a non-preemptible state.
pub fn get_m_cache(mp: Arc<Mutex<Option<m>>>) -> Arc<Mutex<Option<mcache>>> {
        // Grab the mcache, since that's where stats live.
    let mut pp: GoPtr<crate::runtime2::p> = crate::runtime2::puintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).p.lock().unwrap().as_ref().unwrap()));
    let mut c: Arc<Mutex<Option<mcache>>> = Arc::new(Mutex::new(None));
    if pp.is_nil() {
                // We will be called without a P while bootstrapping,
                // in which case we use mcache0, which is set in mallocinit.
                // mcache0 is cleared when bootstrapping is complete,
                // by procresize.
        { let new_val = (*mcache0.lock().unwrap().as_ref().unwrap()).clone(); c = new_val; };
    } else {
        { let new_val = { let __ptr_value = pp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().mcache.clone(); __field_value }; c = new_val; };
    }
        // We will be called without a P while bootstrapping,
        // in which case we use mcache0, which is set in mallocinit.
        // mcache0 is cleared when bootstrapping is complete,
        // by procresize.
    return c.clone();
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for mcache {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for gclink {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for stackfreelist {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
