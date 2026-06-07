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

pub(crate) const PROF_READER_SLEEPING: u64 = ((1 as u64) << (32 as u64));
pub(crate) const PROF_WRITE_EXTRA: u64 = ((1 as u64) << (33 as u64));


pub(crate) const PROF_BUF_BLOCKING: i32 = 0;
pub(crate) const PROF_BUF_NON_BLOCKING: i32 = 1;


/// A profBuf is a lock-free buffer for profiling events,
/// safe for concurrent use by one reader and one writer.
/// The writer may be a signal handler running without a user g.
/// The reader is assumed to be a user g.
///
/// Each logged event corresponds to a fixed size header, a list of
/// uintptrs (typically a stack), and exactly one unsafe.Pointer tag.
/// The header and uintptrs are stored in the circular buffer data and the
/// tag is stored in a circular buffer tags, running in parallel.
/// In the circular buffer data, each event takes 2+hdrsize+len(stk)
/// words: the value 2+hdrsize+len(stk), then the time of the event, then
/// hdrsize words giving the fixed-size header, and then len(stk) words
/// for the stack.
///
/// The current effective offsets into the tags and data circular buffers
/// for reading and writing are stored in the high 30 and low 32 bits of r and w.
/// The bottom bits of the high 32 are additional flag bits in w, unused in r.
/// "Effective" offsets means the total number of reads or writes, mod 2^length.
/// The offset in the buffer is the effective offset mod the length of the buffer.
/// To make wraparound mod 2^length match wraparound mod length of the buffer,
/// the length of the buffer must be a power of two.
///
/// If the reader catches up to the writer, a flag passed to read controls
/// whether the read blocks until more data is available. A read returns a
/// pointer to the buffer data itself; the caller is assumed to be done with
/// that data at the next read. The read offset rNext tracks the next offset to
/// be returned by read. By definition, r ≤ rNext ≤ w (before wraparound),
/// and rNext is only used by the reader, so it can be accessed without atomics.
///
/// If the writer gets ahead of the reader, so that the buffer fills,
/// future writes are discarded and replaced in the output stream by an
/// overflow entry, which has size 2+hdrsize+1, time set to the time of
/// the first discarded write, a header of all zeroed words, and a "stack"
/// containing one word, the number of discarded writes.
///
/// Between the time the buffer fills and the buffer becomes empty enough
/// to hold more data, the overflow entry is stored as a pending overflow
/// entry in the fields overflow and overflowTime. The pending overflow
/// entry can be turned into a real record by either the writer or the
/// reader. If the writer is called to write a new record and finds that
/// the output buffer has room for both the pending overflow entry and the
/// new record, the writer emits the pending overflow entry and the new
/// record into the buffer. If the reader is called to read data and finds
/// that the output buffer is empty but that there is a pending overflow
/// entry, the reader will return a synthesized record for the pending
/// overflow entry.
///
/// Only the writer can create or add to a pending overflow entry, but
/// either the reader or the writer can clear the pending overflow entry.
/// A pending overflow entry is indicated by the low 32 bits of 'overflow'
/// holding the number of discarded writes, and overflowTime holding the
/// time of the first discarded write. The high 32 bits of 'overflow'
/// increment each time the low 32 bits transition from zero to non-zero
/// or vice versa. This sequence number avoids ABA problems in the use of
/// compare-and-swap to coordinate between reader and writer.
/// The overflowTime is only written when the low 32 bits of overflow are
/// zero, that is, only when there is no pending overflow entry, in
/// preparation for creating a new one. The reader can therefore fetch and
/// clear the entry atomically using
///
///	for {
///		overflow = load(&b.overflow)
///		if uint32(overflow) == 0 {
///			// no pending entry
///			break
///		}
///		time = load(&b.overflowTime)
///		if cas(&b.overflow, overflow, ((overflow>>32)+1)<<32) {
///			// pending entry cleared
///			break
///		}
///	}
///	if uint32(overflow) > 0 {
///		emit entry for uint32(overflow), time
///	}
#[derive(Clone)]
pub struct profBuf {
    pub r: Arc<Mutex<Option<profAtomic>>>,
    pub w: Arc<Mutex<Option<profAtomic>>>,
    pub overflow: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub overflow_time: Arc<Mutex<Option<internal_runtime_atomic::types::Uint64>>>,
    pub eof: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub hdrsize: Arc<Mutex<Option<usize>>>,
    pub data: Arc<Mutex<Option<Vec<u64>>>>,
    pub tags: Arc<Mutex<Option<Vec<usize>>>>,
    pub r_next: Arc<Mutex<Option<profIndex>>>,
    pub overflow_buf: Arc<Mutex<Option<Vec<u64>>>>,
    pub wait: Arc<Mutex<Option<note>>>,
}

impl profBuf {
    pub fn __go_value_clone(&self) -> Self {
        Self { r: { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, w: { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, overflow: { let __guard = self.overflow.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, overflow_time: { let __guard = self.overflow_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, eof: { let __guard = self.eof.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hdrsize: { let __guard = self.hdrsize.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, data: self.data.clone(), tags: self.tags.clone(), r_next: { let __guard = self.r_next.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, overflow_buf: self.overflow_buf.clone(), wait: { let __guard = self.wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for profBuf {
    fn default() -> Self {
        Self { r: Arc::new(Mutex::new(Some(profAtomic(Arc::new(Mutex::new(Some(0))))))), w: Arc::new(Mutex::new(Some(profAtomic(Arc::new(Mutex::new(Some(0))))))), overflow: Arc::new(Mutex::new(Some(Default::default()))), overflow_time: Arc::new(Mutex::new(Some(Default::default()))), eof: Arc::new(Mutex::new(Some(Default::default()))), hdrsize: Arc::new(Mutex::new(Some(0))), data: Arc::new(Mutex::new(None)), tags: Arc::new(Mutex::new(None)), r_next: Arc::new(Mutex::new(Some(profIndex(Arc::new(Mutex::new(Some(0))))))), overflow_buf: Arc::new(Mutex::new(None)), wait: Arc::new(Mutex::new(Some(note::default()))) }
    }
}

impl std::fmt::Display for profBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", (*self.r.lock().unwrap().as_ref().unwrap()), (*self.w.lock().unwrap().as_ref().unwrap()), (*self.overflow.lock().unwrap().as_ref().unwrap()), (*self.overflow_time.lock().unwrap().as_ref().unwrap()), (*self.eof.lock().unwrap().as_ref().unwrap()), (*self.hdrsize.lock().unwrap().as_ref().unwrap()), format_slice(&self.data), format_slice(&self.tags), (*self.r_next.lock().unwrap().as_ref().unwrap()), format_slice(&self.overflow_buf), (*self.wait.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for profBuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A profAtomic is the atomically-accessed word holding a profIndex.
#[derive(Debug, Clone, Default)]
pub struct profAtomic(pub Arc<Mutex<Option<u64>>>);

impl Display for profAtomic {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for profAtomic {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for profAtomic {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for profAtomic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for profAtomic {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<profAtomic> for u64 {
    fn eq(&self, other: &profAtomic) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<profAtomic> for u64 {
    fn partial_cmp(&self, other: &profAtomic) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for profAtomic {
    type Output = profAtomic;
    fn add(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for profAtomic {
    type Output = profAtomic;
    fn add(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<profAtomic> for u64 {
    type Output = profAtomic;
    fn add(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for profAtomic {
    type Output = profAtomic;
    fn sub(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for profAtomic {
    type Output = profAtomic;
    fn sub(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<profAtomic> for u64 {
    type Output = profAtomic;
    fn sub(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for profAtomic {
    type Output = profAtomic;
    fn mul(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for profAtomic {
    type Output = profAtomic;
    fn mul(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<profAtomic> for u64 {
    type Output = profAtomic;
    fn mul(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for profAtomic {
    type Output = profAtomic;
    fn div(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for profAtomic {
    type Output = profAtomic;
    fn div(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<profAtomic> for u64 {
    type Output = profAtomic;
    fn div(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for profAtomic {
    type Output = profAtomic;
    fn rem(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for profAtomic {
    type Output = profAtomic;
    fn rem(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<profAtomic> for u64 {
    type Output = profAtomic;
    fn rem(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for profAtomic {
    type Output = profAtomic;
    fn bitand(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for profAtomic {
    type Output = profAtomic;
    fn bitand(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<profAtomic> for u64 {
    type Output = profAtomic;
    fn bitand(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for profAtomic {
    type Output = profAtomic;
    fn bitor(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for profAtomic {
    type Output = profAtomic;
    fn bitor(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<profAtomic> for u64 {
    type Output = profAtomic;
    fn bitor(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for profAtomic {
    type Output = profAtomic;
    fn bitxor(self, other: Self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for profAtomic {
    type Output = profAtomic;
    fn bitxor(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<profAtomic> for u64 {
    type Output = profAtomic;
    fn bitxor(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for profAtomic {
    type Output = profAtomic;
    fn not(self) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: i32) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: i8) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: i16) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: i64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: u32) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: u8) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: u16) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for profAtomic {
    type Output = profAtomic;
    fn shl(self, other: usize) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: profAtomic) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: i32) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: i8) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: i16) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: i64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: u32) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: u8) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: u16) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: u64) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for profAtomic {
    type Output = profAtomic;
    fn shr(self, other: usize) -> profAtomic {
        profAtomic(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for profAtomic {}

impl Ord for profAtomic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A profIndex is the packet tag and data counts and flags bits, described above.
#[derive(Debug, Clone, Default)]
pub struct profIndex(pub Arc<Mutex<Option<u64>>>);

impl Display for profIndex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for profIndex {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for profIndex {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for profIndex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for profIndex {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<profIndex> for u64 {
    fn eq(&self, other: &profIndex) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<profIndex> for u64 {
    fn partial_cmp(&self, other: &profIndex) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for profIndex {
    type Output = profIndex;
    fn add(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for profIndex {
    type Output = profIndex;
    fn add(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<profIndex> for u64 {
    type Output = profIndex;
    fn add(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for profIndex {
    type Output = profIndex;
    fn sub(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for profIndex {
    type Output = profIndex;
    fn sub(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<profIndex> for u64 {
    type Output = profIndex;
    fn sub(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for profIndex {
    type Output = profIndex;
    fn mul(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for profIndex {
    type Output = profIndex;
    fn mul(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<profIndex> for u64 {
    type Output = profIndex;
    fn mul(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for profIndex {
    type Output = profIndex;
    fn div(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for profIndex {
    type Output = profIndex;
    fn div(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<profIndex> for u64 {
    type Output = profIndex;
    fn div(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for profIndex {
    type Output = profIndex;
    fn rem(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for profIndex {
    type Output = profIndex;
    fn rem(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<profIndex> for u64 {
    type Output = profIndex;
    fn rem(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for profIndex {
    type Output = profIndex;
    fn bitand(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for profIndex {
    type Output = profIndex;
    fn bitand(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<profIndex> for u64 {
    type Output = profIndex;
    fn bitand(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for profIndex {
    type Output = profIndex;
    fn bitor(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for profIndex {
    type Output = profIndex;
    fn bitor(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<profIndex> for u64 {
    type Output = profIndex;
    fn bitor(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for profIndex {
    type Output = profIndex;
    fn bitxor(self, other: Self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for profIndex {
    type Output = profIndex;
    fn bitxor(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<profIndex> for u64 {
    type Output = profIndex;
    fn bitxor(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for profIndex {
    type Output = profIndex;
    fn not(self) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for profIndex {
    type Output = profIndex;
    fn shl(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for profIndex {
    type Output = profIndex;
    fn shl(self, other: i32) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for profIndex {
    type Output = profIndex;
    fn shl(self, other: i8) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for profIndex {
    type Output = profIndex;
    fn shl(self, other: i16) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for profIndex {
    type Output = profIndex;
    fn shl(self, other: i64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for profIndex {
    type Output = profIndex;
    fn shl(self, other: u32) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for profIndex {
    type Output = profIndex;
    fn shl(self, other: u8) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for profIndex {
    type Output = profIndex;
    fn shl(self, other: u16) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for profIndex {
    type Output = profIndex;
    fn shl(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for profIndex {
    type Output = profIndex;
    fn shl(self, other: usize) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for profIndex {
    type Output = profIndex;
    fn shr(self, other: profIndex) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for profIndex {
    type Output = profIndex;
    fn shr(self, other: i32) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for profIndex {
    type Output = profIndex;
    fn shr(self, other: i8) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for profIndex {
    type Output = profIndex;
    fn shr(self, other: i16) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for profIndex {
    type Output = profIndex;
    fn shr(self, other: i64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for profIndex {
    type Output = profIndex;
    fn shr(self, other: u32) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for profIndex {
    type Output = profIndex;
    fn shr(self, other: u8) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for profIndex {
    type Output = profIndex;
    fn shr(self, other: u16) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for profIndex {
    type Output = profIndex;
    fn shr(self, other: u64) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for profIndex {
    type Output = profIndex;
    fn shr(self, other: usize) -> profIndex {
        profIndex(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for profIndex {}

impl Ord for profIndex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// profBufReadMode specifies whether to block when no data is available to read.
#[derive(Debug, Clone, Default)]
pub struct profBufReadMode(pub Arc<Mutex<Option<i32>>>);

impl Display for profBufReadMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for profBufReadMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for profBufReadMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for profBufReadMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for profBufReadMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<profBufReadMode> for i32 {
    fn eq(&self, other: &profBufReadMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<profBufReadMode> for i32 {
    fn partial_cmp(&self, other: &profBufReadMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for profBufReadMode {
    type Output = profBufReadMode;
    fn add(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn add(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn add(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for profBufReadMode {
    type Output = profBufReadMode;
    fn sub(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn sub(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn sub(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for profBufReadMode {
    type Output = profBufReadMode;
    fn mul(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn mul(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn mul(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for profBufReadMode {
    type Output = profBufReadMode;
    fn div(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn div(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn div(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for profBufReadMode {
    type Output = profBufReadMode;
    fn neg(self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for profBufReadMode {
    type Output = profBufReadMode;
    fn rem(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn rem(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn rem(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for profBufReadMode {
    type Output = profBufReadMode;
    fn bitand(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn bitand(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn bitand(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for profBufReadMode {
    type Output = profBufReadMode;
    fn bitor(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn bitor(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn bitor(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for profBufReadMode {
    type Output = profBufReadMode;
    fn bitxor(self, other: Self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn bitxor(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<profBufReadMode> for i32 {
    type Output = profBufReadMode;
    fn bitxor(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for profBufReadMode {
    type Output = profBufReadMode;
    fn not(self) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: i8) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: i16) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: i64) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: u32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: u8) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: u16) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: u64) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for profBufReadMode {
    type Output = profBufReadMode;
    fn shl(self, other: usize) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: profBufReadMode) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: i32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: i8) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: i16) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: i64) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: u32) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: u8) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: u16) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: u64) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for profBufReadMode {
    type Output = profBufReadMode;
    fn shr(self, other: usize) -> profBufReadMode {
        profBufReadMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for profBufReadMode {}

impl Ord for profBufReadMode {
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
pub struct AnonymousStruct22 {
    pub lock: Arc<Mutex<Option<mutex>>>,
    pub newm: Arc<Mutex<Option<muintptr>>>,
    pub waiting: Arc<Mutex<Option<bool>>>,
    pub wake: Arc<Mutex<Option<note>>>,
    pub have_template_thread: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct22 {
    pub fn __go_value_clone(&self) -> Self {
        Self { lock: { let __guard = self.lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, newm: { let __guard = self.newm.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, waiting: { let __guard = self.waiting.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wake: { let __guard = self.wake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, have_template_thread: { let __guard = self.have_template_thread.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct22 {
    fn default() -> Self {
        Self { lock: Arc::new(Mutex::new(Some(mutex::default()))), newm: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), waiting: Arc::new(Mutex::new(Some(false))), wake: Arc::new(Mutex::new(Some(note::default()))), have_template_thread: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct22 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.lock.lock().unwrap().as_ref().unwrap()), (*self.newm.lock().unwrap().as_ref().unwrap()), (*self.waiting.lock().unwrap().as_ref().unwrap()), (*self.wake.lock().unwrap().as_ref().unwrap()), (*self.have_template_thread.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct22 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct23 {
    pub signal_lock: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub hz: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct23 {
    pub fn __go_value_clone(&self) -> Self {
        Self { signal_lock: { let __guard = self.signal_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, hz: { let __guard = self.hz.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct23 {
    fn default() -> Self {
        Self { signal_lock: Arc::new(Mutex::new(Some(Default::default()))), hz: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct23 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.signal_lock.lock().unwrap().as_ref().unwrap()), (*self.hz.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct23 {
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


pub(crate) type newmHandoff = AnonymousStruct22;


pub(crate) type prof = AnonymousStruct23;


pub(crate) type scavenge = AnonymousStruct14;


pub(crate) type userArenaState = AnonymousStruct1;


pub(crate) type writeBarrier = AnonymousStruct10;


pub(crate) static overflowTag: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[usize; 1]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *overflowTag.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_zero_globals() {
    *overflowTag.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


impl profAtomic {
    pub fn load(&self) -> Arc<Mutex<Option<profIndex>>> {
        Arc::new(Mutex::new(Some(profIndex(Arc::new(Mutex::new(Some(internal_runtime_atomic::load64(Arc::new(Mutex::new(Some(u64::default())))) as u64)))))))
    }

    pub fn store(&self, new: Arc<Mutex<Option<profIndex>>>) {
        internal_runtime_atomic::store64(Arc::new(Mutex::new(Some(u64::default()))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))));
    }

    pub fn cas(&self, old: Arc<Mutex<Option<profIndex>>>, new: Arc<Mutex<Option<profIndex>>>) -> bool {
        internal_runtime_atomic::cas64(Arc::new(Mutex::new(Some(u64::default()))), Arc::new(Mutex::new(Some((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))), Arc::new(Mutex::new(Some((*{ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u64))))
    }
}

impl profIndex {
    pub fn data_count(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn tag_count(&self) -> u32 {
        (*Arc::new(Mutex::new(Some((((*self.0.lock().unwrap().as_ref().unwrap()) >> 34i32)) as u32))).lock().unwrap().as_ref().unwrap())
    }

    /// addCountsAndClearFlags returns the packed form of "x + (data, tag) - all flags".
    pub fn add_counts_and_clear_flags(&self, data: Arc<Mutex<Option<i32>>>, tag: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<profIndex>>> {
        Arc::new(Mutex::new(Some(profIndex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = ({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 34; __tmp_x >> __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*tag.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x << __tmp_y }; let __tmp_y = 2; __tmp_x >> __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }); let __tmp_y = 34; __tmp_x << __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*data.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x | __tmp_y } as u64)))))))
    }
}

impl profBuf {
    /// hasOverflow reports whether b has any overflow records pending.
    pub fn has_overflow(&self) -> bool {
        return { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.overflow.lock().unwrap().as_mut().unwrap()).load() as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x > __tmp_y };
    }

    /// takeOverflow consumes the pending overflow records, returning the overflow count
    /// and the time of the first overflow.
    /// When called by the reader, it is racing against incrementOverflow.
    pub fn take_overflow(&self) -> (u32, u64) {
    let mut count: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut time: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));

        let mut overflow = (*self.overflow.lock().unwrap().as_mut().unwrap()).load();
        { let new_val = (*self.overflow_time.lock().unwrap().as_mut().unwrap()).load(); *time.lock().unwrap() = Some(new_val); };
        loop {
        { let new_val = Arc::new(Mutex::new(Some(overflow as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *count.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
        { let new_val = 0 as u64; *time.lock().unwrap() = Some(new_val); };
        break
    }

                // Increment generation, clear overflow count in low bits.
        if (*self.overflow.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(overflow))), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = overflow; let __tmp_y = 32; __tmp_x >> __tmp_y }); let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); let __tmp_y = 32; __tmp_x << __tmp_y })))) {
        break
    }
        { let new_val = (*self.overflow.lock().unwrap().as_mut().unwrap()).load(); overflow = new_val; };
        { let new_val = (*self.overflow_time.lock().unwrap().as_mut().unwrap()).load(); *time.lock().unwrap() = Some(new_val); };
    }
                // Increment generation, clear overflow count in low bits.
        ((*Arc::new(Mutex::new(Some(overflow as u32))).lock().unwrap().as_ref().unwrap()), { let __v = (*time.lock().unwrap().as_ref().unwrap()).clone(); __v })
    }

    /// incrementOverflow records a single overflow at time now.
    /// It is racing against a possible takeOverflow in the reader.
    pub fn increment_overflow(&self, now: Arc<Mutex<Option<i64>>>) {
        loop {
        let mut overflow = (*self.overflow.lock().unwrap().as_mut().unwrap()).load();

                // Once we see b.overflow reach 0, it's stable: no one else is changing it underfoot.
                // We need to set overflowTime if we're incrementing b.overflow from 0.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(overflow as u32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Store overflowTime first so it's always available when overflow != 0.
        (*self.overflow_time.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some((*now.lock().unwrap().as_ref().unwrap()) as u64))));
        (*self.overflow.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = overflow; let __tmp_y = 32; __tmp_x >> __tmp_y }); let __tmp_y = 1 as u64; __tmp_x + __tmp_y }); let __tmp_y = 32; __tmp_x << __tmp_y }); let __tmp_y = 1 as u64; __tmp_x + __tmp_y }))));
        break
    }

                // Store overflowTime first so it's always available when overflow != 0.
                // Otherwise we're racing to increment against reader
                // who wants to set b.overflow to 0.
                // Out of paranoia, leave 2³²-1 a sticky overflow value,
                // to avoid wrapping around. Extremely unlikely.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(overflow as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = -1 as i32; __tmp_x == __tmp_y } {
        break
    }
        if (*self.overflow.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(overflow))), Arc::new(Mutex::new(Some({ let __tmp_x = overflow; let __tmp_y = 1 as u64; __tmp_x + __tmp_y })))) {
        break
    }
    }
    }

    /// canWriteRecord reports whether the buffer has room
    /// for a single contiguous record with a stack of length nstk.
    pub fn can_write_record(&self, nstk: Arc<Mutex<Option<i32>>>) -> bool {
        let mut br = (*self.r.lock().unwrap().as_ref().unwrap()).load();
        let mut bw = (*self.w.lock().unwrap().as_ref().unwrap()).load();
                // room for tag?
        if { let __tmp_x = ({ let __tmp_x = (count_sub(Arc::new(Mutex::new(Some(profIndex::tag_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::tag_count(&(*bw.lock().unwrap().as_ref().unwrap())))))) as i32); let __tmp_y = (({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x < __tmp_y } {
        return false;
    }
                // room for data?
        let mut nd = Arc::new(Mutex::new(Some({ let __tmp_x = (count_sub(Arc::new(Mutex::new(Some(profIndex::data_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())))))) as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y })));
        let mut want = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*nstk.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as i32)));
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        { let __rhs = { let __tmp_x = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }; let mut guard = nd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        return { let __tmp_x = { let __v = (*nd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y };
    }

    /// canWriteTwoRecords reports whether the buffer has room
    /// for two records with stack lengths nstk1, nstk2, in that order.
    /// Each record must be contiguous on its own, but the two
    /// records need not be contiguous (one can be at the end of the buffer
    /// and the other can wrap around and start at the beginning of the buffer).
    pub fn can_write_two_records(&self, nstk1: Arc<Mutex<Option<i32>>>, nstk2: Arc<Mutex<Option<i32>>>) -> bool {
        let mut br = (*self.r.lock().unwrap().as_ref().unwrap()).load();
        let mut bw = (*self.w.lock().unwrap().as_ref().unwrap()).load();
                // room for tag?
        if { let __tmp_x = ({ let __tmp_x = (count_sub(Arc::new(Mutex::new(Some(profIndex::tag_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::tag_count(&(*bw.lock().unwrap().as_ref().unwrap())))))) as i32); let __tmp_y = (({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return false;
    }
                // room for data?
        let mut nd = Arc::new(Mutex::new(Some({ let __tmp_x = (count_sub(Arc::new(Mutex::new(Some(profIndex::data_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())))))) as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y })));
                // first record
        let mut want = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 2; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*nstk1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })));
        let mut i = Arc::new(Mutex::new(Some(({ let __tmp_x = profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as i32)));
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        { let __rhs = { let __tmp_x = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }; let mut guard = nd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        { let __rhs = (*want.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = (*want.lock().unwrap().as_ref().unwrap()); let mut guard = nd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
                // second record
        { let new_val = { let __tmp_x = { let __tmp_x = 2; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*nstk2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *want.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        { let __rhs = { let __tmp_x = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }; let mut guard = nd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    }
                // Can't fit in trailing fragment of slice.
                // Skip over that and start over at beginning of slice.
        return { let __tmp_x = { let __v = (*nd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*want.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y };
    }

    /// write writes an entry to the profiling buffer b.
    /// The entry begins with a fixed hdr, which must have
    /// length b.hdrsize, followed by a variable-sized stack
    /// and a single tag pointer *tagPtr (or nil if tagPtr is nil).
    /// No write barriers allowed because this might be called from a signal handler.
    pub fn write(&mut self, tagPtr: Arc<Mutex<Option<usize>>>, now: Arc<Mutex<Option<i64>>>, hdr: Arc<Mutex<Option<Vec<u64>>>>, stk: Arc<Mutex<Option<Vec<usize>>>>) {
        if false {
        return;
    }
        if { let __tmp_x = ((*hdr.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()) as i32); __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("misuse of profBuf.write".to_string()))));
    }
        {
        let mut hasOverflow = self.has_overflow();;
        if hasOverflow && self.can_write_two_records(Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))) {
            let (mut count, mut time) = self.take_overflow();;
            if { let __tmp_x = count; let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        let mut stk: Arc<Mutex<Option<[usize; 1]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        (*stk.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some(count as usize))).lock().unwrap().as_ref().unwrap()).clone();
        self.write(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(time as i64))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __seq_holder = stk.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
    };
        } else if hasOverflow || !self.can_write_record(Arc::new(Mutex::new(Some((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)))) {
        self.increment_overflow(Arc::new(Mutex::new(Some({ let __arg_holder = now.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.wakeup_extra();
        return;
    }
    }
                // Room for both an overflow record and the one being written.
                // Write the overflow record if the reader hasn't gotten to it yet.
                // Only racing against reader, not other writers.
                // Pending overflow without room to write overflow and new records
                // or no overflow but also no room for new record.
                // There's room: write the record.
        let mut br = (*self.r.lock().unwrap().as_ref().unwrap()).load();
        let mut bw = (*self.w.lock().unwrap().as_ref().unwrap()).load();
                // Profiling tag
                //
                // The tag is a pointer, but we can't run a write barrier here.
                // We have interrupted the OS-level execution of gp, but the
                // runtime still sees gp as executing. In effect, we are running
                // in place of the real gp. Since gp is the only goroutine that
                // can overwrite gp.labels, the value of gp.labels is stable during
                // this signal handler: it will still be reachable from gp when
                // we finish executing. If a GC is in progress right now, it must
                // keep gp.labels alive, because gp.labels is reachable from gp.
                // If gp were to overwrite gp.labels, the deletion barrier would
                // still shade that pointer, which would preserve it for the
                // in-progress GC, so all is well. Any future GC will see the
                // value we copied when scanning b.tags (heap-allocated).
                // We arrange that the store here is always overwriting a nil,
                // so there is no need for a deletion barrier on b.tags[wt].
        let mut wt = Arc::new(Mutex::new(Some(({ let __tmp_x = profIndex::tag_count(&(*bw.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as i32)));
        if { let __nil_result = (*tagPtr.lock().unwrap()).is_some(); __nil_result } {
        { unimplemented!("unsafe.Pointer dereference assignment"); };
    }
                // Main record.
                // It has to fit in a contiguous section of the slice, so if it doesn't fit at the end,
                // leave a rewind marker (0) and start over at the beginning of the slice.
        let mut wd = Arc::new(Mutex::new(Some(({ let __tmp_x = profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as i32)));
        let mut nd = Arc::new(Mutex::new(Some({ let __tmp_x = (count_sub(Arc::new(Mutex::new(Some(profIndex::data_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap())))))) as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x + __tmp_y })));
        let mut skip = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __tmp_x = { let __v = (*wd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y } as i32); let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x > __tmp_y } {
        (*self.data.lock().unwrap().as_mut().unwrap())[({ let __v = (*wd.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = 0 as u64;
        { let new_val = { let __tmp_x = (({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*wd.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }; *skip.lock().unwrap() = Some(new_val); };
        { let __rhs = (*skip.lock().unwrap().as_ref().unwrap()); let mut guard = nd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        { let new_val = 0; *wd.lock().unwrap() = Some(new_val); };
    }
        let mut data = Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*wd.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        (*data.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()).clone();
        (*data.lock().unwrap().as_mut().unwrap())[(1) as usize] = (*Arc::new(Mutex::new(Some((*now.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()).clone();
                // header, zero-padded
        let mut i = { let _dst_start = (2) as usize; let _dst_len = (({ let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize) - _dst_start; let _src = { let __copy_src_holder = hdr.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*data.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let __clear_start = ({ let __tmp_x = 2; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __clear_end = ({ let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize; let __clear_holder = data.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
        { let __range_holder = stk.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, pc) in __range_values.iter().copied().enumerate() {
        (*data.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(i as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize] = (*Arc::new(Mutex::new(Some(pc as u64))).lock().unwrap().as_ref().unwrap()).clone();
    } }
        loop {
                // Commit write.
                // Racing with reader setting flag bits in b.w, to avoid lost wakeups.
        let mut old = (*self.w.lock().unwrap().as_ref().unwrap()).load();
        let mut new = profIndex::add_counts_and_clear_flags(&(*old.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*stk.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = ((*Arc::new(Mutex::new(Some({ let __selector_holder = self.hdrsize.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()) as i32); __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(1))));
        if !(*self.w.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        continue
    }

                // If there was a reader, wake it up.
        if { let __tmp_x = profIndex(Arc::new(Mutex::new(Some(((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & PROF_READER_SLEEPING as u64))))); let __tmp_y = profIndex(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        notewakeup(self.wait.clone());
    }
        break
    }
    }

    /// close signals that there will be no more writes on the buffer.
    /// Once all the data has been read from the buffer, reads will return eof=true.
    pub fn close(&self) {
        if { let __tmp_x = (*self.eof.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: profBuf already closed".to_string()))));
    }
        (*self.eof.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
        self.wakeup_extra();
    }

    /// wakeupExtra must be called after setting one of the "extra"
    /// atomic fields b.overflow or b.eof.
    /// It records the change in b.w and wakes up the reader if needed.
    pub fn wakeup_extra(&self) {
        loop {
        let mut old = (*self.w.lock().unwrap().as_ref().unwrap()).load();
        let mut new = Arc::new(Mutex::new(Some(profIndex(Arc::new(Mutex::new(Some(((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) | PROF_WRITE_EXTRA as u64))))))));
        if !(*self.w.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        continue
    }
        if { let __tmp_x = profIndex(Arc::new(Mutex::new(Some(((*{ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & PROF_READER_SLEEPING as u64))))); let __tmp_y = profIndex(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        notewakeup(self.wait.clone());
    }
        break
    }
    }

    pub fn read(&mut self, mode: Arc<Mutex<Option<profBufReadMode>>>) -> (Arc<Mutex<Option<Vec<u64>>>>, Arc<Mutex<Option<Vec<usize>>>>, bool) {
    let mut data: Arc<Mutex<Option<Vec<u64>>>> = Arc::new(Mutex::new(None));
    let mut tags: Arc<Mutex<Option<Vec<usize>>>> = Arc::new(Mutex::new(None));
    let mut eof: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        if false {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), true);
    }

        let mut br = Arc::new(Mutex::new(Some({ let __selector_holder = self.r_next.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

                // Commit previous read, returning that part of the ring to the writer.
                // First clear tags that have now been read, both to avoid holding
                // up the memory they point at for longer than necessary
                // and so that b.write can assume it is always overwriting
                // nil tag entries (see comment in b.write).
        let mut rPrev = (*self.r.lock().unwrap().as_ref().unwrap()).load();
        if { let __tmp_x = (*rPrev.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*br.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        let mut ntag = count_sub(Arc::new(Mutex::new(Some(profIndex::tag_count(&(*br.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::tag_count(&(*rPrev.lock().unwrap().as_ref().unwrap()))))));
        let mut ti = Arc::new(Mutex::new(Some(({ let __tmp_x = profIndex::tag_count(&(*rPrev.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as i32)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ntag; __tmp_x < __tmp_y } {
        (*self.tags.lock().unwrap().as_mut().unwrap())[({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = 0;
        {
        { let mut guard = ti.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        if { let __tmp_x = ({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x == __tmp_y } {
            { let new_val = 0; *ti.lock().unwrap() = Some(new_val); };;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        (*self.r.lock().unwrap().as_ref().unwrap()).store(Arc::new(Mutex::new(Some({ let __arg_holder = br.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        'read: loop {
            let mut bw = (*self.w.lock().unwrap().as_ref().unwrap()).load();
            let mut numData = count_sub(Arc::new(Mutex::new(Some(profIndex::data_count(&(*bw.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::data_count(&(*br.lock().unwrap().as_ref().unwrap()))))));
            if { let __tmp_x = numData; let __tmp_y = 0; __tmp_x == __tmp_y } {
        if self.has_overflow() {
                // No data to read, but there is overflow to report.
                // Racing with writer flushing b.overflow into a real record.
        let (mut count, mut time) = self.take_overflow();
        if { let __tmp_x = count; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // Lost the race, go around again.
        continue 'read;
    }
                // Lost the race, go around again.
                // Won the race, report overflow.
        let mut dst = self.overflow_buf.clone();
        (*dst.lock().unwrap().as_mut().unwrap())[(0) as usize] = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()).clone();
        (*dst.lock().unwrap().as_mut().unwrap())[(1) as usize] = time;
        { let __clear_start = (2) as usize; let __clear_end = ({ let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize; let __clear_holder = dst.clone(); let mut __clear_guard = __clear_holder.lock().unwrap(); if let Some(__clear_seq) = __clear_guard.as_mut() { assert!(__clear_start <= __clear_end && __clear_end <= __clear_seq.len()); for __clear_i in __clear_start..__clear_end { __clear_seq[__clear_i] = 0; } } };
        (*dst.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize] = (*Arc::new(Mutex::new(Some(count as u64))).lock().unwrap().as_ref().unwrap()).clone();
        return (Arc::new(Mutex::new(Some({ let __seq_holder = dst.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __tmp_x = 2 as usize; let __tmp_y = (*self.hdrsize.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = 1 as usize; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __seq_holder = overflowTag.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = (1) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), false);
    }
                // No data to read, but there is overflow to report.
                // Racing with writer flushing b.overflow into a real record.
                // Lost the race, go around again.
                // Won the race, report overflow.
        if { let __tmp_x = (*self.eof.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
                // No data, no overflow, EOF set: done.
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), true);
    }
                // No data, no overflow, EOF set: done.
        if { let __tmp_x = profIndex(Arc::new(Mutex::new(Some(((*{ let __v = (*bw.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & PROF_WRITE_EXTRA as u64))))); let __tmp_y = profIndex(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
                // Writer claims to have published extra information (overflow or eof).
                // Attempt to clear notification and then check again.
                // If we fail to clear the notification it means b.w changed,
                // so we still need to check again.
        (*self.w.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = bw.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*bw.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = profIndex(Arc::new(Mutex::new(Some(PROF_WRITE_EXTRA as u64)))); __tmp_x & ! __tmp_y }))));
        continue 'read;
    }
                // Writer claims to have published extra information (overflow or eof).
                // Attempt to clear notification and then check again.
                // If we fail to clear the notification it means b.w changed,
                // so we still need to check again.
                // Nothing to read right now.
                // Return or sleep according to mode.
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = profBufReadMode(Arc::new(Mutex::new(Some(PROF_BUF_NON_BLOCKING as i32)))); __tmp_x == __tmp_y } {
                // Necessary on Darwin, notetsleepg below does not work in signal handler, root cause of #61768.
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), false);
    }
                // Necessary on Darwin, notetsleepg below does not work in signal handler, root cause of #61768.
        if !(*self.w.lock().unwrap().as_ref().unwrap()).cas(Arc::new(Mutex::new(Some({ let __arg_holder = bw.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(profIndex(Arc::new(Mutex::new(Some(((*{ let __v = (*bw.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) | PROF_READER_SLEEPING as u64))))))))) {
        continue 'read;
    }
                // Committed to sleeping.
        notetsleepg(self.wait.clone(), Arc::new(Mutex::new(Some(-1 as i64))));
        noteclear(self.wait.clone());
        continue 'read;
    }
                        // No data to read, but there is overflow to report.
                        // Racing with writer flushing b.overflow into a real record.
                        // Lost the race, go around again.
                        // Won the race, report overflow.
                        // No data, no overflow, EOF set: done.
                        // Writer claims to have published extra information (overflow or eof).
                        // Attempt to clear notification and then check again.
                        // If we fail to clear the notification it means b.w changed,
                        // so we still need to check again.
                        // Nothing to read right now.
                        // Return or sleep according to mode.
                        // Necessary on Darwin, notetsleepg below does not work in signal handler, root cause of #61768.
                        // Committed to sleeping.
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = profIndex::data_count(&(*br.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.data.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
            if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = (numData as i32); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (numData) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
    } else {
        { let __rhs = (*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; numData = numData - __rhs; };
    }
                        // available in case of wraparound
            let mut skip = Arc::new(Mutex::new(Some(0)));
            if { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
                // Wraparound record. Go back to the beginning of the ring.
        { let new_val = (*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; *skip.lock().unwrap() = Some(new_val); };
        { let new_val = self.data.clone(); data = new_val; };
        if { let __tmp_x = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = (numData as i32); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (numData) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); data = new_val; };
    }
    }

                        // Wraparound record. Go back to the beginning of the ring.
            let mut ntag = count_sub(Arc::new(Mutex::new(Some(profIndex::tag_count(&(*bw.lock().unwrap().as_ref().unwrap()))))), Arc::new(Mutex::new(Some(profIndex::tag_count(&(*br.lock().unwrap().as_ref().unwrap()))))));
            if { let __tmp_x = ntag; let __tmp_y = 0; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: malformed profBuf buffer - tag and data out of sync".to_string()))));
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.tags.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = profIndex::tag_count(&(*br.lock().unwrap().as_ref().unwrap())); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.tags.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x % __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); tags = new_val; };
            if { let __tmp_x = ((*tags.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = (ntag as i32); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = tags.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (ntag) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); tags = new_val; };
    }

                        // Count out whole data records until either data or tags is done.
                        // They are always in sync in the buffer, but due to an end-of-slice
                        // wraparound we might need to stop early and return the rest
                        // in the next call.
            let mut di = Arc::new(Mutex::new(Some(0)));
            let mut ti = Arc::new(Mutex::new(Some(0)));
            while { let __tmp_x = ({ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } && { let __tmp_x = ({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*tags.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some((*di.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some((*data.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: malformed profBuf buffer - invalid size".to_string()))));
    }
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap()); let mut guard = di.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = ti.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                        // Remember how much we returned, to commit read on next call.
            { let new_val = profIndex::add_counts_and_clear_flags(&(*br.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some({ let __arg_holder = ti.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.r_next.lock().unwrap() = __moved_val; };

            if RACEENABLED {
                // Match racereleasemerge in runtime_setProfLabel,
                // so that the setting of the labels in runtime_setProfLabel
                // is treated as happening before any use of the labels
                // by our caller. The synchronization on labelSync itself is a fiction
                // for the race detector. The actual synchronization is handled
                // by the fact that the signal handler only reads from the current
                // goroutine and uses atomics to write the updated queue indices,
                // and then the read-out from the signal handler buffer uses
                // atomics to read those queue indices.
        raceacquire(Arc::new(Mutex::new(Some(Arc::as_ptr(&labelSync.clone()) as usize))));
    }

                        // Match racereleasemerge in runtime_setProfLabel,
                        // so that the setting of the labels in runtime_setProfLabel
                        // is treated as happening before any use of the labels
                        // by our caller. The synchronization on labelSync itself is a fiction
                        // for the race detector. The actual synchronization is handled
                        // by the fact that the signal handler only reads from the current
                        // goroutine and uses atomics to write the updated queue indices,
                        // and then the read-out from the signal handler buffer uses
                        // atomics to read those queue indices.
            return (Arc::new(Mutex::new(Some({ let __seq_holder = data.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*di.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __seq_holder = tags.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*ti.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), false);
        };
        unreachable!()
    }
}

/// countSub subtracts two counts obtained from profIndex.dataCount or profIndex.tagCount,
/// assuming that they are no more than 2^29 apart (guaranteed since they are never more than
/// len(data) or len(tags) apart, respectively).
/// tagCount wraps at 2^30, while dataCount wraps at 2^32.
/// This function works for both.
pub fn count_sub(x: Arc<Mutex<Option<u32>>>, y: Arc<Mutex<Option<u32>>>) -> i32 {
        // x-y is 32-bit signed or 30-bit signed; sign-extend to 32 bits and convert to int.
    (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 2; __tmp_x << __tmp_y }; let __tmp_y = 2; __tmp_x >> __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for profBuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
