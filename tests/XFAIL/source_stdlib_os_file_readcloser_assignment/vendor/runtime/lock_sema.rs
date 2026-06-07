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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const LOCKED: usize = 1;


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


/// One-time notifications.
pub fn noteclear(n: Arc<Mutex<Option<note>>>) {
    { let new_val = 0 as usize; *(*n.lock().unwrap().as_ref().unwrap()).key.lock().unwrap() = Some(new_val); };
}

pub fn notewakeup(n: Arc<Mutex<Option<note>>>) {
    let mut v: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    loop {
        { let new_val = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone())); *v.lock().unwrap() = Some(new_val); };
        if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(LOCKED as usize)))) {
        break
    }
    }

        // Successfully set waitm to locked.
        // What was it before?
    if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        } else if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = LOCKED as usize; __tmp_x == __tmp_y } {
                        // Two notewakeups! Not allowed.
            throw(Arc::new(Mutex::new(Some("notewakeup - double wakeup".to_string()))));
        } else {
                        // Must be the waiting m. Wake it up.
            semawakeup(GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) }));
        }
}

pub fn notesleep(n: Arc<Mutex<Option<note>>>) {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("notesleep not on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    if !internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
                // Must be locked (got wakeup).
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = LOCKED as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("notesleep - waitm out of sync".to_string()))));
    }
        return;
    }

        // Must be locked (got wakeup).
        // Queued. Sleep.
    { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
    if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr == 0 } {
        semasleep(Arc::new(Mutex::new(Some(-1 as i64))));
    } else {
                // Sleep for an arbitrary-but-moderate interval to poll libc interceptors.
        const ns: f64 = 10e6;

        while { let __tmp_x = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone())); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        semasleep(Arc::new(Mutex::new(Some(ns as i64))));
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
    }
        // Sleep for an arbitrary-but-moderate interval to poll libc interceptors.
    { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
}

///go:nosplit
pub fn notetsleep_internal(n: Arc<Mutex<Option<note>>>, mut ns: Arc<Mutex<Option<i64>>>, mut gp: Arc<Mutex<Option<g>>>, mut deadline: Arc<Mutex<Option<i64>>>) -> bool {
        // gp and deadline are logically local variables, but they are written
        // as parameters so that the stack space they require is charged
        // to the caller.
        // This reduces the nosplit footprint of notetsleep_internal.
    { let new_val = getg().clone(); gp = new_val; };

        // Register for wakeup on n->waitm.
    if !internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize)))) {
                // Must be locked (got wakeup).
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = LOCKED as usize; __tmp_x != __tmp_y } {
        throw(Arc::new(Mutex::new(Some("notetsleep - waitm out of sync".to_string()))));
    }
        return true;
    }
        // Must be locked (got wakeup).
    if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
                // Queued. Sleep.
        { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr == 0 } {
        semasleep(Arc::new(Mutex::new(Some(-1 as i64))));
    } else {
                // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
        const ns: f64 = 10e6;

        while { let __tmp_x = semasleep(Arc::new(Mutex::new(Some(ns as i64)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
    }
                // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        return true;
    }

        // Queued. Sleep.
        // Sleep in arbitrary-but-moderate intervals to poll libc interceptors.
    { let new_val = { let __tmp_x = nanotime(); let __tmp_y = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *deadline.lock().unwrap() = Some(new_val); };
    loop {
                // Registered. Sleep.
        { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } && { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10e6 as i64; __tmp_x > __tmp_y } {
        { let new_val = 10e6 as i64; *ns.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = semasleep(Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
                // Acquired semaphore, semawakeup unregistered us.
                // Done.
        return true;
    }
                // Acquired semaphore, semawakeup unregistered us.
                // Done.
        if { let __nil_ptr = (*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone(); __nil_ptr != 0 } {
        asmcgocall(Arc::new(Mutex::new(Some((*(*cgo_yield.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(None)));
    }
        { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };

                // Interrupted or timed out. Still registered. Semaphore not acquired.
        { let new_val = { let __tmp_x = { let __v = (*deadline.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = nanotime(); __tmp_x - __tmp_y }; *ns.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        break
    }
    }

        // Registered. Sleep.
        // Acquired semaphore, semawakeup unregistered us.
        // Done.
        // Interrupted or timed out. Still registered. Semaphore not acquired.
        // Deadline hasn't arrived. Keep sleeping.
        // Deadline arrived. Still registered. Semaphore not acquired.
        // Want to give up and return, but have to unregister first,
        // so that any notewakeup racing with the return does not
        // try to grant us the semaphore when we don't expect it.
    loop {
        let mut v = internal_runtime_atomic::loaduintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()));
        { let _switch_val = v;
    if _switch_val == ({ let __v = Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&(*gp.lock().unwrap().as_ref().unwrap()).m.clone()) as usize))).lock().unwrap().as_ref().unwrap()) as usize))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) {
                        // No wakeup yet; unregister if possible.
            if internal_runtime_atomic::casuintptr(internal_runtime_atomic::GoPtr::local((*n.lock().unwrap().as_ref().unwrap()).key.clone()), Arc::new(Mutex::new(Some(v))), Arc::new(Mutex::new(Some(0 as usize)))) {
        return false;
    }
        } else if _switch_val == (LOCKED as usize) {
                        // Wakeup happened so semaphore is available.
                        // Grab it to avoid getting out of sync.
            { let new_val = true; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = semasleep(Arc::new(Mutex::new(Some(-1 as i64)))); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        throw(Arc::new(Mutex::new(Some("runtime: unable to acquire - semaphore out of sync".to_string()))));
    }
            { let new_val = false; *(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).blocked.lock().unwrap() = Some(new_val); };
            return true;
        } else {
            throw(Arc::new(Mutex::new(Some("runtime: unexpected waitm - semaphore out of sync".to_string()))));
        }
    }
    }
}

pub fn notetsleep(n: Arc<Mutex<Option<note>>>, ns: Arc<Mutex<Option<i64>>>) -> bool {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        throw(Arc::new(Mutex::new(Some("notetsleep not on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    notetsleep_internal(n.clone(), Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(0 as i64))))
}

/// same as runtime·notetsleep, but called on user g (not g0)
/// calls only nosplit functions between entersyscallblock/exitsyscall.
pub fn notetsleepg(n: Arc<Mutex<Option<note>>>, ns: Arc<Mutex<Option<i64>>>) -> bool {
    let mut gp = getg();
    if { let __left = gp.clone(); let __right = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).g0.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        throw(Arc::new(Mutex::new(Some("notetsleepg on g0".to_string()))));
    }
    semacreate({ let __field = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); __field });
    entersyscallblock();
    let mut ok = notetsleep_internal(n.clone(), Arc::new(Mutex::new(Some({ let __v = (*ns.lock().unwrap().as_ref().unwrap()).clone(); __v }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(0 as i64))));
    exitsyscall();
    ok
}

pub fn before_idle(__arg0: Arc<Mutex<Option<i64>>>, __arg1: Arc<Mutex<Option<i64>>>) -> (Arc<Mutex<Option<crate::runtime2::g>>>, bool) {
    return (Arc::new(Mutex::new(None)), false);
}

pub fn check_timeouts() {
}