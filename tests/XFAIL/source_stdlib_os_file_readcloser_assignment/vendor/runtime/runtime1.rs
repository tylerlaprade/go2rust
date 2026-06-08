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

pub(crate) const TRACEBACK_CRASH: i32 = 1 << 0;
pub(crate) const TRACEBACK_ALL: i32 = 1 << 1;
pub(crate) const TRACEBACK_SHIFT: i32 = 2;


#[derive(Clone)]
pub struct dbgVar {
    pub name: Arc<Mutex<Option<String>>>,
    pub value: Arc<Mutex<Option<i32>>>,
    pub atomic: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub def: Arc<Mutex<Option<i32>>>,
}

impl dbgVar {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, value: self.value.clone(), atomic: self.atomic.clone(), def: { let __guard = self.def.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for dbgVar {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), value: Arc::new(Mutex::new(None)), atomic: Arc::new(Mutex::new(None)), def: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for dbgVar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), { let __guard = self.value.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.atomic.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.def.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for dbgVar {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static traceback_cache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static traceback_env: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static argc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static argv: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static test_z64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static test_x64: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<u64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static debug: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct25>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static dbgvars: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<dbgVar>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *traceback_cache.lock().unwrap() = Some(0);
    *traceback_env.lock().unwrap() = Some(0);
    *argc.lock().unwrap() = Some(0);
    *argv.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *test_z64.lock().unwrap() = Some(0);
    *test_x64.lock().unwrap() = Some(0);
    *debug.lock().unwrap() = Some(Default::default());
    *dbgvars.lock().unwrap() = Some(vec![]);
    *traceback_cache.lock().unwrap() = Some(((2 as u32) << (TRACEBACK_SHIFT as u32)) as u32);
    *dbgvars.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("adaptivestackstart".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).adaptivestackstart.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asyncpreemptoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asynctimerchan".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("cgocheck".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).cgocheck.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("clobberfree".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dataindependenttiming".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dataindependenttiming.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("disablethp".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).disablethp.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dontfreezetheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("efence".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).efence.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gccheckmark".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcpacertrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcshrinkstackoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcshrinkstackoff.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcstoptheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gctrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("harddecommit".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).harddecommit.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("inittrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).inittrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("invalidptr".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("madvdontneed".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).madvdontneed.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).panicnil.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("profstackdepth".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone().clone(), def: Arc::new(Mutex::new(Some(128 as i32))), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("runtimecontentionstacks".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).runtime_contention_stacks.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("sbrk".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scavtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scheddetail".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scheddetail.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("schedtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).schedtrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceadvanceperiod".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceallocfree".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).traceallocfree.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracecheckstackownership".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).trace_check_stack_ownership.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracebackancestors".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracebackancestors.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracefpunwindoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracefpunwindoff.clone().clone(), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *traceback_cache.lock().unwrap() = Some(0);
    *traceback_env.lock().unwrap() = Some(0);
    *argc.lock().unwrap() = Some(0);
    *argv.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *test_z64.lock().unwrap() = Some(0);
    *test_x64.lock().unwrap() = Some(0);
    *debug.lock().unwrap() = Some(Default::default());
    *dbgvars.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_62() {
    *traceback_cache.lock().unwrap() = Some(((2 as u32) << (TRACEBACK_SHIFT as u32)) as u32);
}


pub(crate) fn __go_init_order_63() {
    *dbgvars.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("adaptivestackstart".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).adaptivestackstart.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asyncpreemptoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).asyncpreemptoff.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("asynctimerchan".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).asynctimerchan.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("cgocheck".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).cgocheck.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("clobberfree".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).clobberfree.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dataindependenttiming".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dataindependenttiming.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("disablethp".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).disablethp.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("dontfreezetheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).dontfreezetheworld.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("efence".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).efence.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gccheckmark".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gccheckmark.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcpacertrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcpacertrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcshrinkstackoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcshrinkstackoff.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gcstoptheworld".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gcstoptheworld.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("gctrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).gctrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("harddecommit".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).harddecommit.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("inittrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).inittrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("invalidptr".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).invalidptr.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("madvdontneed".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).madvdontneed.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("panicnil".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).panicnil.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("profstackdepth".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).profstackdepth.clone().clone(), def: Arc::new(Mutex::new(Some(128 as i32))), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("runtimecontentionstacks".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).runtime_contention_stacks.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("sbrk".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).sbrk.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scavtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scavtrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("scheddetail".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).scheddetail.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("schedtrace".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).schedtrace.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceadvanceperiod".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).traceadvanceperiod.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("traceallocfree".to_string()))), atomic: (*debug.lock().unwrap().as_ref().unwrap()).traceallocfree.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracecheckstackownership".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).trace_check_stack_ownership.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracebackancestors".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracebackancestors.clone().clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(dbgVar { name: Arc::new(Mutex::new(Some("tracefpunwindoff".to_string()))), value: (*debug.lock().unwrap().as_ref().unwrap()).tracefpunwindoff.clone().clone(), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
}


/// gotraceback returns the current traceback settings.
///
/// If level is 0, suppress all tracebacks.
/// If level is 1, show tracebacks, but exclude runtime frames.
/// If level is 2, show tracebacks including runtime frames.
/// If all is set, print all goroutine stacks. Otherwise, print just the current goroutine.
/// If crash is set, crash (core dump, etc) after tracebacking.
///
///go:nosplit
pub fn gotraceback() -> (i32, bool, bool) {
    let mut level: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut all: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut crash: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut gp = getg();
    let mut t = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::local(traceback_cache.clone()));
    { let new_val = { let __tmp_x = { let __tmp_x = t; let __tmp_y = TRACEBACK_CRASH as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }; *crash.lock().unwrap() = Some(new_val); };
    { let new_val = { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_USER as u32)))); __tmp_x >= __tmp_y } || { let __tmp_x = { let __tmp_x = t; let __tmp_y = TRACEBACK_ALL as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y }; *all.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*(*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).traceback.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *level.lock().unwrap() = __moved_val; };
    } else if { let __tmp_x = { let __selector_holder = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32)))); __tmp_x >= __tmp_y } {
        { let new_val = 2 as i32; *level.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = t; let __tmp_y = TRACEBACK_SHIFT; __tmp_x >> __tmp_y }) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *level.lock().unwrap() = __moved_val; };
    }
        // Always include runtime frames in runtime throws unless
        // otherwise overridden by m.traceback.
    return ((*level.lock().unwrap().as_ref().unwrap()), (*all.lock().unwrap().as_ref().unwrap()), (*crash.lock().unwrap().as_ref().unwrap()));
}

pub fn environ() -> Arc<Mutex<Option<Vec<String>>>> {
    envs.clone()
}

///go:nosplit
pub fn acquirem() -> Arc<Mutex<Option<crate::runtime2::m>>> {
    let mut gp = getg();
    { let __target = (*(*gp.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).locks.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    return (*gp.lock().unwrap().as_ref().unwrap()).m.clone();
}

///go:nosplit
pub fn releasem(mp: GoPtr<crate::runtime2::m>) {
    let mut gp = getg();
    { let __target = { let __ptr_value = mp.with_mut(|__ptr_value| __ptr_value.locks.clone()); __ptr_value }.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    if { let __tmp_x = (*{ let __ptr_value = mp.borrow(); __ptr_value.as_ref().unwrap().locks.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } && (*{ let __field = (*gp.lock().unwrap().as_ref().unwrap()).preempt.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
                // restore the preemption request in case we've cleared it in newstack
        { let new_val = STACK_PREEMPT as usize; *(*gp.lock().unwrap().as_ref().unwrap()).stackguard0.lock().unwrap() = Some(new_val); };
    }
}

#[derive(Clone)]
pub struct AnonymousStruct25 {
    pub cgocheck: Arc<Mutex<Option<i32>>>,
    pub clobberfree: Arc<Mutex<Option<i32>>>,
    pub disablethp: Arc<Mutex<Option<i32>>>,
    pub dontfreezetheworld: Arc<Mutex<Option<i32>>>,
    pub efence: Arc<Mutex<Option<i32>>>,
    pub gccheckmark: Arc<Mutex<Option<i32>>>,
    pub gcpacertrace: Arc<Mutex<Option<i32>>>,
    pub gcshrinkstackoff: Arc<Mutex<Option<i32>>>,
    pub gcstoptheworld: Arc<Mutex<Option<i32>>>,
    pub gctrace: Arc<Mutex<Option<i32>>>,
    pub invalidptr: Arc<Mutex<Option<i32>>>,
    pub madvdontneed: Arc<Mutex<Option<i32>>>,
    pub runtime_contention_stacks: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub scavtrace: Arc<Mutex<Option<i32>>>,
    pub scheddetail: Arc<Mutex<Option<i32>>>,
    pub schedtrace: Arc<Mutex<Option<i32>>>,
    pub tracebackancestors: Arc<Mutex<Option<i32>>>,
    pub asyncpreemptoff: Arc<Mutex<Option<i32>>>,
    pub harddecommit: Arc<Mutex<Option<i32>>>,
    pub adaptivestackstart: Arc<Mutex<Option<i32>>>,
    pub tracefpunwindoff: Arc<Mutex<Option<i32>>>,
    pub traceadvanceperiod: Arc<Mutex<Option<i32>>>,
    pub trace_check_stack_ownership: Arc<Mutex<Option<i32>>>,
    pub profstackdepth: Arc<Mutex<Option<i32>>>,
    pub dataindependenttiming: Arc<Mutex<Option<i32>>>,
    pub malloc: Arc<Mutex<Option<bool>>>,
    pub inittrace: Arc<Mutex<Option<i32>>>,
    pub sbrk: Arc<Mutex<Option<i32>>>,
    pub traceallocfree: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub panicnil: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
    pub asynctimerchan: Arc<Mutex<Option<internal_runtime_atomic::types::Int32>>>,
}
impl AnonymousStruct25 {
    pub fn __go_value_clone(&self) -> Self {
        Self { cgocheck: { let __guard = self.cgocheck.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, clobberfree: { let __guard = self.clobberfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, disablethp: { let __guard = self.disablethp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dontfreezetheworld: { let __guard = self.dontfreezetheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, efence: { let __guard = self.efence.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gccheckmark: { let __guard = self.gccheckmark.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcpacertrace: { let __guard = self.gcpacertrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcshrinkstackoff: { let __guard = self.gcshrinkstackoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gcstoptheworld: { let __guard = self.gcstoptheworld.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gctrace: { let __guard = self.gctrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, invalidptr: { let __guard = self.invalidptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, madvdontneed: { let __guard = self.madvdontneed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, runtime_contention_stacks: { let __guard = self.runtime_contention_stacks.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scavtrace: { let __guard = self.scavtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scheddetail: { let __guard = self.scheddetail.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, schedtrace: { let __guard = self.schedtrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracebackancestors: { let __guard = self.tracebackancestors.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asyncpreemptoff: { let __guard = self.asyncpreemptoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, harddecommit: { let __guard = self.harddecommit.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, adaptivestackstart: { let __guard = self.adaptivestackstart.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tracefpunwindoff: { let __guard = self.tracefpunwindoff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceadvanceperiod: { let __guard = self.traceadvanceperiod.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, trace_check_stack_ownership: { let __guard = self.trace_check_stack_ownership.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, profstackdepth: { let __guard = self.profstackdepth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dataindependenttiming: { let __guard = self.dataindependenttiming.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, malloc: { let __guard = self.malloc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inittrace: { let __guard = self.inittrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sbrk: { let __guard = self.sbrk.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, traceallocfree: { let __guard = self.traceallocfree.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, panicnil: { let __guard = self.panicnil.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, asynctimerchan: { let __guard = self.asynctimerchan.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct25 {
    fn default() -> Self {
        Self { cgocheck: Arc::new(Mutex::new(Some(0))), clobberfree: Arc::new(Mutex::new(Some(0))), disablethp: Arc::new(Mutex::new(Some(0))), dontfreezetheworld: Arc::new(Mutex::new(Some(0))), efence: Arc::new(Mutex::new(Some(0))), gccheckmark: Arc::new(Mutex::new(Some(0))), gcpacertrace: Arc::new(Mutex::new(Some(0))), gcshrinkstackoff: Arc::new(Mutex::new(Some(0))), gcstoptheworld: Arc::new(Mutex::new(Some(0))), gctrace: Arc::new(Mutex::new(Some(0))), invalidptr: Arc::new(Mutex::new(Some(0))), madvdontneed: Arc::new(Mutex::new(Some(0))), runtime_contention_stacks: Arc::new(Mutex::new(Some(Default::default()))), scavtrace: Arc::new(Mutex::new(Some(0))), scheddetail: Arc::new(Mutex::new(Some(0))), schedtrace: Arc::new(Mutex::new(Some(0))), tracebackancestors: Arc::new(Mutex::new(Some(0))), asyncpreemptoff: Arc::new(Mutex::new(Some(0))), harddecommit: Arc::new(Mutex::new(Some(0))), adaptivestackstart: Arc::new(Mutex::new(Some(0))), tracefpunwindoff: Arc::new(Mutex::new(Some(0))), traceadvanceperiod: Arc::new(Mutex::new(Some(0))), trace_check_stack_ownership: Arc::new(Mutex::new(Some(0))), profstackdepth: Arc::new(Mutex::new(Some(0))), dataindependenttiming: Arc::new(Mutex::new(Some(0))), malloc: Arc::new(Mutex::new(Some(false))), inittrace: Arc::new(Mutex::new(Some(0))), sbrk: Arc::new(Mutex::new(Some(0))), traceallocfree: Arc::new(Mutex::new(Some(Default::default()))), panicnil: Arc::new(Mutex::new(Some(Default::default()))), asynctimerchan: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for AnonymousStruct25 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", (*self.cgocheck.lock().unwrap().as_ref().unwrap()), (*self.clobberfree.lock().unwrap().as_ref().unwrap()), (*self.disablethp.lock().unwrap().as_ref().unwrap()), (*self.dontfreezetheworld.lock().unwrap().as_ref().unwrap()), (*self.efence.lock().unwrap().as_ref().unwrap()), (*self.gccheckmark.lock().unwrap().as_ref().unwrap()), (*self.gcpacertrace.lock().unwrap().as_ref().unwrap()), (*self.gcshrinkstackoff.lock().unwrap().as_ref().unwrap()), (*self.gcstoptheworld.lock().unwrap().as_ref().unwrap()), (*self.gctrace.lock().unwrap().as_ref().unwrap()), (*self.invalidptr.lock().unwrap().as_ref().unwrap()), (*self.madvdontneed.lock().unwrap().as_ref().unwrap()), (*self.runtime_contention_stacks.lock().unwrap().as_ref().unwrap()), (*self.scavtrace.lock().unwrap().as_ref().unwrap()), (*self.scheddetail.lock().unwrap().as_ref().unwrap()), (*self.schedtrace.lock().unwrap().as_ref().unwrap()), (*self.tracebackancestors.lock().unwrap().as_ref().unwrap()), (*self.asyncpreemptoff.lock().unwrap().as_ref().unwrap()), (*self.harddecommit.lock().unwrap().as_ref().unwrap()), (*self.adaptivestackstart.lock().unwrap().as_ref().unwrap()), (*self.tracefpunwindoff.lock().unwrap().as_ref().unwrap()), (*self.traceadvanceperiod.lock().unwrap().as_ref().unwrap()), (*self.trace_check_stack_ownership.lock().unwrap().as_ref().unwrap()), (*self.profstackdepth.lock().unwrap().as_ref().unwrap()), (*self.dataindependenttiming.lock().unwrap().as_ref().unwrap()), (*self.malloc.lock().unwrap().as_ref().unwrap()), (*self.inittrace.lock().unwrap().as_ref().unwrap()), (*self.sbrk.lock().unwrap().as_ref().unwrap()), (*self.traceallocfree.lock().unwrap().as_ref().unwrap()), (*self.panicnil.lock().unwrap().as_ref().unwrap()), (*self.asynctimerchan.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct25 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type debug = AnonymousStruct25;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for dbgVar {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
