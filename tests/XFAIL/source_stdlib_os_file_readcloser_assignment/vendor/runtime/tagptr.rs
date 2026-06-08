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

pub(crate) const MIN_TAG_BITS: i32 = 10;


/// taggedPointer is a pointer with a numeric tag.
/// The size of the numeric tag is GOARCH-dependent,
/// currently at least 10 bits.
/// This should only be used with pointers allocated outside the Go heap.
#[derive(Debug, Clone, Default)]
pub struct taggedPointer(pub Arc<Mutex<Option<u64>>>);

impl Display for taggedPointer {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for taggedPointer {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for taggedPointer {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for taggedPointer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for taggedPointer {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<taggedPointer> for u64 {
    fn eq(&self, other: &taggedPointer) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<taggedPointer> for u64 {
    fn partial_cmp(&self, other: &taggedPointer) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for taggedPointer {
    type Output = taggedPointer;
    fn add(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for taggedPointer {
    type Output = taggedPointer;
    fn add(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn add(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for taggedPointer {
    type Output = taggedPointer;
    fn sub(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for taggedPointer {
    type Output = taggedPointer;
    fn sub(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn sub(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for taggedPointer {
    type Output = taggedPointer;
    fn mul(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for taggedPointer {
    type Output = taggedPointer;
    fn mul(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn mul(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for taggedPointer {
    type Output = taggedPointer;
    fn div(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for taggedPointer {
    type Output = taggedPointer;
    fn div(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn div(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for taggedPointer {
    type Output = taggedPointer;
    fn rem(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for taggedPointer {
    type Output = taggedPointer;
    fn rem(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn rem(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for taggedPointer {
    type Output = taggedPointer;
    fn bitand(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for taggedPointer {
    type Output = taggedPointer;
    fn bitand(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn bitand(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for taggedPointer {
    type Output = taggedPointer;
    fn bitor(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for taggedPointer {
    type Output = taggedPointer;
    fn bitor(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn bitor(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for taggedPointer {
    type Output = taggedPointer;
    fn bitxor(self, other: Self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for taggedPointer {
    type Output = taggedPointer;
    fn bitxor(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<taggedPointer> for u64 {
    type Output = taggedPointer;
    fn bitxor(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for taggedPointer {
    type Output = taggedPointer;
    fn not(self) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: i32) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: i8) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: i16) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: i64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: u32) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: u8) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: u16) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for taggedPointer {
    type Output = taggedPointer;
    fn shl(self, other: usize) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: taggedPointer) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: i32) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: i8) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: i16) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: i64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: u32) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: u8) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: u16) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: u64) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for taggedPointer {
    type Output = taggedPointer;
    fn shr(self, other: usize) -> taggedPointer {
        taggedPointer(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for taggedPointer {}

impl Ord for taggedPointer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}
