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

pub(crate) const RWMUTEX_MAX_READERS: i32 = 1 << 30;


/// A rwmutex is a reader/writer mutual exclusion lock.
/// The lock can be held by an arbitrary number of readers or a single writer.
/// This is a variant of sync.RWMutex, for the runtime package.
/// Like mutex, rwmutex blocks the calling M.
/// It does not interact with the goroutine scheduler.
#[derive(Clone)]
pub struct rwmutex {
    pub r_lock: Arc<Mutex<Option<mutex>>>,
    pub readers: Arc<Mutex<Option<muintptr>>>,
    pub reader_pass: Arc<Mutex<Option<u32>>>,
    pub w_lock: Arc<Mutex<Option<mutex>>>,
    pub writer: Arc<Mutex<Option<muintptr>>>,
    pub reader_count: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub reader_wait: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub read_rank: Arc<Mutex<Option<lockRank>>>,
}

impl rwmutex {
    pub fn __go_value_clone(&self) -> Self {
        Self { r_lock: { let __guard = self.r_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, readers: { let __guard = self.readers.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reader_pass: { let __guard = self.reader_pass.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, w_lock: { let __guard = self.w_lock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, writer: { let __guard = self.writer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reader_count: { let __guard = self.reader_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, reader_wait: { let __guard = self.reader_wait.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, read_rank: { let __guard = self.read_rank.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for rwmutex {
    fn default() -> Self {
        Self { r_lock: Arc::new(Mutex::new(Some(mutex::default()))), readers: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), reader_pass: Arc::new(Mutex::new(Some(0))), w_lock: Arc::new(Mutex::new(Some(mutex::default()))), writer: Arc::new(Mutex::new(Some(crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0))))))), reader_count: Arc::new(Mutex::new(Some(Default::default()))), reader_wait: Arc::new(Mutex::new(Some(Default::default()))), read_rank: Arc::new(Mutex::new(Some(crate::lockrank::lockRank(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for rwmutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.r_lock.lock().unwrap().as_ref().unwrap()), (*self.readers.lock().unwrap().as_ref().unwrap()), (*self.reader_pass.lock().unwrap().as_ref().unwrap()), (*self.w_lock.lock().unwrap().as_ref().unwrap()), (*self.writer.lock().unwrap().as_ref().unwrap()), (*self.reader_count.lock().unwrap().as_ref().unwrap()), (*self.reader_wait.lock().unwrap().as_ref().unwrap()), (*self.read_rank.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for rwmutex {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl rwmutex {
    /// Lock ranking an rwmutex has two aspects:
    ///
    /// Semantic ranking: this rwmutex represents some higher level lock that
    /// protects some resource (e.g., allocmLock protects creation of new Ms). The
    /// read and write locks of that resource need to be represented in the lock
    /// rank.
    ///
    /// Internal ranking: as an implementation detail, rwmutex uses two mutexes:
    /// rLock and wLock. These have lock order requirements: wLock must be locked
    /// before rLock. This also needs to be represented in the lock rank.
    ///
    /// Semantic ranking is represented by acquiring readRank during read lock and
    /// writeRank during write lock.
    ///
    /// wLock is held for the duration of a write lock, so it uses writeRank
    /// directly, both for semantic and internal ranking. rLock is only held
    /// temporarily inside the rlock/lock methods, so it uses readRankInternal to
    /// represent internal ranking. Semantic ranking is represented by a separate
    /// acquire of readRank for the duration of a read lock.
    ///
    /// The lock ranking must document this ordering:
    ///   - readRankInternal is a leaf lock.
    ///   - readRank is taken before readRankInternal.
    ///   - writeRank is taken before readRankInternal.
    ///   - readRank is placed in the lock order wherever a read lock of this rwmutex
    ///     belongs.
    ///   - writeRank is placed in the lock order wherever a write lock of this
    ///     rwmutex belongs.
    pub fn init(&mut self, readRank: Arc<Mutex<Option<lockRank>>>, readRankInternal: Arc<Mutex<Option<lockRank>>>, writeRank: Arc<Mutex<Option<lockRank>>>) {
        { let new_val = readRank.lock().unwrap().as_ref().unwrap().clone(); *self.read_rank.lock().unwrap() = Some(new_val); };
        lock_init(GoPtr::local(self.r_lock.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = readRankInternal.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        lock_init(GoPtr::local(self.w_lock.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = writeRank.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// rlock locks rw for reading.
    pub fn rlock(&mut self) {
                // The reader must not be allowed to lose its P or else other
                // things blocking on the lock may consume all of the Ps and
                // deadlock (issue #20903). Alternatively, we could drop the P
                // while sleeping.
        acquire_lock_rank_and_m(Arc::new(Mutex::new(Some({ let __selector_holder = self.read_rank.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        lock_with_rank_may_acquire(self.r_lock.clone(), get_lock_rank(GoPtr::local(self.r_lock.clone())));
        if { let __tmp_x = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
                // A writer is pending. Park on the reader queue.
        let mut rw_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        lock(GoPtr::local(rw_closure_clone.r_lock.clone()));
        if { let __tmp_x = (*rw_closure_clone.reader_pass.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x > __tmp_y } {
        { let __target = rw_closure_clone.reader_pass.clone(); let __rhs = 1 as u32; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        unlock(GoPtr::local(rw_closure_clone.r_lock.clone()));
    } else {
        let mut m = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*rw_closure_clone.readers.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*m.lock().unwrap().as_ref().unwrap()).schedlink.lock().unwrap() = Some(new_val); };
        (*rw_closure_clone.readers.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(m.clone()));
        unlock(GoPtr::local(rw_closure_clone.r_lock.clone()));
        notesleep((*m.lock().unwrap().as_ref().unwrap()).park.clone());
        noteclear((*m.lock().unwrap().as_ref().unwrap()).park.clone());
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    }
    }

    /// runlock undoes a single rlock call on rw.
    pub fn runlock(&self) {
        {
        let mut r = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));;
        if { let __tmp_x = r; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
            if { let __tmp_x = { let __tmp_x = r; let __tmp_y = 1 as i32; __tmp_x + __tmp_y }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } || { let __tmp_x = { let __tmp_x = r; let __tmp_y = 1 as i32; __tmp_x + __tmp_y }; let __tmp_y = -RWMUTEX_MAX_READERS as i32; __tmp_x == __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runlock of unlocked rwmutex".to_string()))));
    };
            if { let __tmp_x = (*self.reader_wait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        lock(GoPtr::local(self.r_lock.clone()));
        let mut w: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*self.writer.lock().unwrap().as_ref().unwrap()));
        if !w.is_nil() {
        notewakeup({ let __ptr_value = w.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());
    }
        unlock(GoPtr::local(self.r_lock.clone()));
    };
        }
    }
                // A writer is pending.
                // The last reader unblocks the writer.
        release_lock_rank_and_m(Arc::new(Mutex::new(Some({ let __selector_holder = self.read_rank.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

    /// lock locks rw for writing.
    pub fn lock(&self) {
                // Resolve competition with other writers and stick to our P.
        lock(GoPtr::local(self.w_lock.clone()));
        let mut m = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
                // Announce that there is a pending writer.
        let mut r = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-RWMUTEX_MAX_READERS as i32)))); let __tmp_y = RWMUTEX_MAX_READERS as i32; __tmp_x + __tmp_y })));
                // Wait for any active readers to complete.
        lock(GoPtr::local(self.r_lock.clone()));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = (*self.reader_wait.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0 as i32; __tmp_x != __tmp_y } {
                // Wait for reader to wake us up.
        let m_closure_clone = m.clone(); let mut rw_closure_clone = (*self).clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        (*rw_closure_clone.writer.lock().unwrap().as_mut().unwrap()).set(GoPtr::local(m_closure_clone.clone()));
        unlock(GoPtr::local(rw_closure_clone.r_lock.clone()));
        notesleep((*m_closure_clone.lock().unwrap().as_ref().unwrap()).park.clone());
        noteclear((*m_closure_clone.lock().unwrap().as_ref().unwrap()).park.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    } else {
        unlock(GoPtr::local(self.r_lock.clone()));
    }
    }

    /// unlock unlocks rw for writing.
    pub fn unlock(&mut self) {
                // Announce to readers that there is no active writer.
        let mut r = (*self.reader_count.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(RWMUTEX_MAX_READERS as i32))));
        if { let __tmp_x = r; let __tmp_y = RWMUTEX_MAX_READERS as i32; __tmp_x >= __tmp_y } {
        throw(Arc::new(Mutex::new(Some("unlock of unlocked rwmutex".to_string()))));
    }
                // Unblock blocked readers.
        lock(GoPtr::local(self.r_lock.clone()));
        while !crate::runtime2::muintptr::ptr(&(*self.readers.lock().unwrap().as_ref().unwrap())).is_nil() {
        let mut reader: GoPtr<crate::runtime2::m> = crate::runtime2::muintptr::ptr(&(*self.readers.lock().unwrap().as_ref().unwrap()));
        { let new_val = crate::runtime2::muintptr(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = reader.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.readers.lock().unwrap() = Some(new_val); };
        (*{ let __ptr_value = reader.with_mut(|__ptr_value| __ptr_value.schedlink.clone()); __ptr_value }.lock().unwrap().as_mut().unwrap()).set(GoPtr::nil());
        notewakeup({ let __ptr_value = reader.with_mut(|__ptr_value| __ptr_value.park.clone()); __ptr_value }.clone());
        { let __rhs = 1 as i32; r = r - __rhs; };
    }
                // If r > 0, there are pending readers that aren't on the
                // queue. Tell them to skip waiting.
        { let __target = self.reader_pass.clone(); let __rhs = (*Arc::new(Mutex::new(Some(r as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        unlock(GoPtr::local(self.r_lock.clone()));
                // Allow other writers to proceed.
        unlock(GoPtr::local(self.w_lock.clone()));
    }
}

impl GoValueClone for rwmutex {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
