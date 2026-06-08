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

use std::sync::{Arc, Mutex};

pub(crate) const MIN_HEAP_ALIGN: i32 = 8;
pub(crate) const __MAX_SMALL_SIZE: i32 = 32768;
pub(crate) const SMALL_SIZE_DIV: i32 = 8;
pub(crate) const SMALL_SIZE_MAX: i32 = 1024;
pub(crate) const LARGE_SIZE_DIV: i32 = 128;
pub(crate) const __NUM_SIZE_CLASSES: i32 = 68;
pub(crate) const __PAGE_SHIFT: i32 = 13;
pub(crate) const MAX_OBJS_PER_SPAN: i32 = 1024;


pub(crate) static class_to_size: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u16; 68]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static class_to_allocnpages: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 68]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static class_to_divmagic: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u32; 68]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static size_to_class8: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 129]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static size_to_class128: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 249]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *class_to_size.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *class_to_allocnpages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *class_to_divmagic.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *size_to_class8.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *size_to_class128.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *class_to_size.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 8 as u16, 16 as u16, 24 as u16, 32 as u16, 48 as u16, 64 as u16, 80 as u16, 96 as u16, 112 as u16, 128 as u16, 144 as u16, 160 as u16, 176 as u16, 192 as u16, 208 as u16, 224 as u16, 240 as u16, 256 as u16, 288 as u16, 320 as u16, 352 as u16, 384 as u16, 416 as u16, 448 as u16, 480 as u16, 512 as u16, 576 as u16, 640 as u16, 704 as u16, 768 as u16, 896 as u16, 1024 as u16, 1152 as u16, 1280 as u16, 1408 as u16, 1536 as u16, 1792 as u16, 2048 as u16, 2304 as u16, 2688 as u16, 3072 as u16, 3200 as u16, 3456 as u16, 4096 as u16, 4864 as u16, 5376 as u16, 6144 as u16, 6528 as u16, 6784 as u16, 6912 as u16, 8192 as u16, 9472 as u16, 9728 as u16, 10240 as u16, 10880 as u16, 12288 as u16, 13568 as u16, 14336 as u16, 16384 as u16, 18432 as u16, 19072 as u16, 20480 as u16, 21760 as u16, 24576 as u16, 27264 as u16, 28672 as u16, 32768 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
    *class_to_allocnpages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 2 as u8, 1 as u8, 2 as u8, 1 as u8, 2 as u8, 1 as u8, 3 as u8, 2 as u8, 3 as u8, 1 as u8, 3 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 1 as u8, 7 as u8, 6 as u8, 5 as u8, 4 as u8, 3 as u8, 5 as u8, 7 as u8, 2 as u8, 9 as u8, 7 as u8, 5 as u8, 8 as u8, 3 as u8, 10 as u8, 7 as u8, 4 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    *class_to_divmagic.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 8 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 16 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 24 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 32 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 48 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 64 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 80 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 96 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 112 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 128 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 144 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 160 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 176 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 192 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 208 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 224 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 240 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 256 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 288 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 320 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 352 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 384 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 416 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 448 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 480 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 512 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 576 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 640 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 704 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 768 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 896 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1024 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1152 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1280 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1408 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1536 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1792 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2048 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2304 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2688 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3072 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3200 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3456 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 4096 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 4864 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 5376 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6144 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6528 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6784 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6912 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 8192 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 9472 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 9728 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 10240 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 10880 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 12288 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 13568 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 14336 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 16384 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 18432 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 19072 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 20480 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 21760 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 24576 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 27264 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 28672 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 32768 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32]))).lock().unwrap().as_ref().unwrap()).clone());
    *size_to_class8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 5 as u8, 6 as u8, 6 as u8, 7 as u8, 7 as u8, 8 as u8, 8 as u8, 9 as u8, 9 as u8, 10 as u8, 10 as u8, 11 as u8, 11 as u8, 12 as u8, 12 as u8, 13 as u8, 13 as u8, 14 as u8, 14 as u8, 15 as u8, 15 as u8, 16 as u8, 16 as u8, 17 as u8, 17 as u8, 18 as u8, 18 as u8, 19 as u8, 19 as u8, 19 as u8, 19 as u8, 20 as u8, 20 as u8, 20 as u8, 20 as u8, 21 as u8, 21 as u8, 21 as u8, 21 as u8, 22 as u8, 22 as u8, 22 as u8, 22 as u8, 23 as u8, 23 as u8, 23 as u8, 23 as u8, 24 as u8, 24 as u8, 24 as u8, 24 as u8, 25 as u8, 25 as u8, 25 as u8, 25 as u8, 26 as u8, 26 as u8, 26 as u8, 26 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
    *size_to_class128.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([32 as u8, 33 as u8, 34 as u8, 35 as u8, 36 as u8, 37 as u8, 37 as u8, 38 as u8, 38 as u8, 39 as u8, 39 as u8, 40 as u8, 40 as u8, 40 as u8, 41 as u8, 41 as u8, 41 as u8, 42 as u8, 43 as u8, 43 as u8, 44 as u8, 44 as u8, 44 as u8, 44 as u8, 44 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 46 as u8, 46 as u8, 46 as u8, 46 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 48 as u8, 48 as u8, 48 as u8, 49 as u8, 49 as u8, 50 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 53 as u8, 53 as u8, 54 as u8, 54 as u8, 54 as u8, 54 as u8, 55 as u8, 55 as u8, 55 as u8, 55 as u8, 55 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 61 as u8, 61 as u8, 61 as u8, 61 as u8, 61 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *class_to_size.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *class_to_allocnpages.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *class_to_divmagic.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *size_to_class8.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *size_to_class128.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_69() {
    *class_to_size.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u16, 8 as u16, 16 as u16, 24 as u16, 32 as u16, 48 as u16, 64 as u16, 80 as u16, 96 as u16, 112 as u16, 128 as u16, 144 as u16, 160 as u16, 176 as u16, 192 as u16, 208 as u16, 224 as u16, 240 as u16, 256 as u16, 288 as u16, 320 as u16, 352 as u16, 384 as u16, 416 as u16, 448 as u16, 480 as u16, 512 as u16, 576 as u16, 640 as u16, 704 as u16, 768 as u16, 896 as u16, 1024 as u16, 1152 as u16, 1280 as u16, 1408 as u16, 1536 as u16, 1792 as u16, 2048 as u16, 2304 as u16, 2688 as u16, 3072 as u16, 3200 as u16, 3456 as u16, 4096 as u16, 4864 as u16, 5376 as u16, 6144 as u16, 6528 as u16, 6784 as u16, 6912 as u16, 8192 as u16, 9472 as u16, 9728 as u16, 10240 as u16, 10880 as u16, 12288 as u16, 13568 as u16, 14336 as u16, 16384 as u16, 18432 as u16, 19072 as u16, 20480 as u16, 21760 as u16, 24576 as u16, 27264 as u16, 28672 as u16, 32768 as u16]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_70() {
    *class_to_allocnpages.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 2 as u8, 1 as u8, 2 as u8, 1 as u8, 2 as u8, 1 as u8, 3 as u8, 2 as u8, 3 as u8, 1 as u8, 3 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 6 as u8, 1 as u8, 7 as u8, 6 as u8, 5 as u8, 4 as u8, 3 as u8, 5 as u8, 7 as u8, 2 as u8, 9 as u8, 7 as u8, 5 as u8, 8 as u8, 3 as u8, 10 as u8, 7 as u8, 4 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_71() {
    *class_to_divmagic.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 8 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 16 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 24 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 32 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 48 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 64 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 80 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 96 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 112 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 128 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 144 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 160 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 176 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 192 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 208 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 224 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 240 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 256 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 288 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 320 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 352 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 384 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 416 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 448 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 480 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 512 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 576 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 640 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 704 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 768 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 896 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1024 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1152 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1280 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1408 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1536 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 1792 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2048 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2304 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 2688 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3072 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3200 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 3456 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 4096 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 4864 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 5376 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6144 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6528 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6784 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 6912 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 8192 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 9472 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 9728 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 10240 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 10880 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 12288 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 13568 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 14336 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 16384 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 18432 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 19072 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 20480 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 21760 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 24576 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 27264 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 28672 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32, { let __tmp_x = { let __tmp_x = !(0 as u32) as u32; let __tmp_y = 32768 as u32; __tmp_x / __tmp_y } as u32; let __tmp_y = 1 as u32; __tmp_x + __tmp_y } as u32]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_72() {
    *size_to_class8.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0 as u8, 1 as u8, 2 as u8, 3 as u8, 4 as u8, 5 as u8, 5 as u8, 6 as u8, 6 as u8, 7 as u8, 7 as u8, 8 as u8, 8 as u8, 9 as u8, 9 as u8, 10 as u8, 10 as u8, 11 as u8, 11 as u8, 12 as u8, 12 as u8, 13 as u8, 13 as u8, 14 as u8, 14 as u8, 15 as u8, 15 as u8, 16 as u8, 16 as u8, 17 as u8, 17 as u8, 18 as u8, 18 as u8, 19 as u8, 19 as u8, 19 as u8, 19 as u8, 20 as u8, 20 as u8, 20 as u8, 20 as u8, 21 as u8, 21 as u8, 21 as u8, 21 as u8, 22 as u8, 22 as u8, 22 as u8, 22 as u8, 23 as u8, 23 as u8, 23 as u8, 23 as u8, 24 as u8, 24 as u8, 24 as u8, 24 as u8, 25 as u8, 25 as u8, 25 as u8, 25 as u8, 26 as u8, 26 as u8, 26 as u8, 26 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 27 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 28 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 29 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 30 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 31 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8, 32 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_73() {
    *size_to_class128.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([32 as u8, 33 as u8, 34 as u8, 35 as u8, 36 as u8, 37 as u8, 37 as u8, 38 as u8, 38 as u8, 39 as u8, 39 as u8, 40 as u8, 40 as u8, 40 as u8, 41 as u8, 41 as u8, 41 as u8, 42 as u8, 43 as u8, 43 as u8, 44 as u8, 44 as u8, 44 as u8, 44 as u8, 44 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 45 as u8, 46 as u8, 46 as u8, 46 as u8, 46 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 47 as u8, 48 as u8, 48 as u8, 48 as u8, 49 as u8, 49 as u8, 50 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 51 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 52 as u8, 53 as u8, 53 as u8, 54 as u8, 54 as u8, 54 as u8, 54 as u8, 55 as u8, 55 as u8, 55 as u8, 55 as u8, 55 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 56 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 57 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 58 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 59 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 60 as u8, 61 as u8, 61 as u8, 61 as u8, 61 as u8, 61 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 62 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 63 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 64 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 65 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 66 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8, 67 as u8]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
