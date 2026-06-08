use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    defs_darwin_arm64::{keventt, pthread, pthreadattr, pthreadcond, pthreadcondattr, pthreadmutex, pthreadmutexattr, stackt, timespec, usigactiont},
    mfinal::{keep_alive},
    os_darwin::{__S_S__D_I_S_A_B_L_E, sigset},
    stubs_arm64::{asmcgocall_no_g},
    sys_libc::{libc_call},
};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_attr_init(attr: Arc<Mutex<Option<pthreadattr>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_attr_init_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(attr.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_attr_init_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_attr_getstacksize(attr: Arc<Mutex<Option<pthreadattr>>>, size: Arc<Mutex<Option<usize>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_attr_getstacksize_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(attr.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(size.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_attr_getstacksize_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_attr_setdetachstate(attr: Arc<Mutex<Option<pthreadattr>>>, state: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_attr_setdetachstate_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(attr.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_attr_setdetachstate_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_create(attr: Arc<Mutex<Option<pthreadattr>>>, start: Arc<Mutex<Option<usize>>>, arg: Arc<Mutex<Option<usize>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_create_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(attr.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_create_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn raise(sig_local: Arc<Mutex<Option<u32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(raise_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&sig_local.clone()) as usize))));
}

pub fn raise_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_self() -> Arc<Mutex<Option<crate::defs_darwin_arm64::pthread>>> {
    let mut t: Arc<Mutex<Option<pthread>>> = Arc::new(Mutex::new(Some(Default::default())));

    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_self_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&t.clone()) as usize))));
    t.clone()
}

pub fn pthread_self_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_kill(t: Arc<Mutex<Option<pthread>>>, sig_local: Arc<Mutex<Option<u32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_kill_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&t.clone()) as usize))));
    ()
}

pub fn pthread_kill_trampoline() {
    unimplemented!("Go function declaration has no body");
}


/// mmap is used to do low-level memory allocation via mmap. Don't allow stack
/// splits, since this function (used by sysAlloc) is called in a lot of low-level
/// parts of the runtime and callers often assume it won't acquire any locks.
///
///go:nosplit
pub fn mmap(addr: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, prot: Arc<Mutex<Option<i32>>>, flags: Arc<Mutex<Option<i32>>>, fd: Arc<Mutex<Option<i32>>>, off: Arc<Mutex<Option<u32>>>) -> (Arc<Mutex<Option<usize>>>, i32) {
    let mut args = Arc::new(Mutex::new(Some(AnonymousStruct34 { addr: Arc::new(Mutex::new(Some({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), n: Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), prot: Arc::new(Mutex::new(Some({ let __arg_holder = prot.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), flags: Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), fd: Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), off: Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ret1: Default::default(), ret2: Arc::new(Mutex::new(Some(0))) })));
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(mmap_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&args.clone()) as usize))));
    return ({ let __return_value_0 = Arc::new(Mutex::new(Some({ let __selector_holder = (*args.lock().unwrap().as_ref().unwrap()).ret1.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __return_value_0 }, (*(*args.lock().unwrap().as_ref().unwrap()).ret2.lock().unwrap().as_ref().unwrap()));
}

pub fn mmap_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn munmap(addr: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(munmap_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&addr.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
}

pub fn munmap_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn madvise(addr: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>, flags: Arc<Mutex<Option<i32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(madvise_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&addr.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
}

pub fn madvise_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn mlock(addr: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<usize>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(mlock_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&addr.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = addr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
}

pub fn mlock_trampoline() {
    unimplemented!("Go function declaration has no body");
}


/// This is exported via linkname to assembly in runtime/cgo.
///
///go:nosplit
///go:cgo_unsafe_args
///go:linkname exit
pub fn exit(code: Arc<Mutex<Option<i32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(exit_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&code.clone()) as usize))));
}

pub fn exit_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn usleep(usec: Arc<Mutex<Option<u32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(usleep_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&usec.clone()) as usize))));
}

pub fn usleep_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn usleep_no_g(usec: Arc<Mutex<Option<u32>>>) {
    asmcgocall_no_g(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(usleep_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&usec.clone()) as usize))));
}

///go:nosplit
///go:cgo_unsafe_args
pub fn write1(fd: Arc<Mutex<Option<usize>>>, p: Arc<Mutex<Option<usize>>>, n: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(write_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&fd.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn write_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn nanotime1() -> i64 {
    let mut r: Arc<Mutex<Option<AnonymousStruct35>>> = Arc::new(Mutex::new(Some(Default::default())));
        // raw timer
        // conversion factors. nanoseconds = t * numer / denom.
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(nanotime_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&r.clone()) as usize))));

        // Note: Apple seems unconcerned about overflow here. See
        // https://developer.apple.com/library/content/qa/qa1398/_index.html
        // Note also, numer == denom == 1 is common.
    let mut t = Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).t.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).numer.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x != __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).numer.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
    }
    if { let __tmp_x = (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).denom.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u32; __tmp_x != __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*r.lock().unwrap().as_ref().unwrap()).denom.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }
    return { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn nanotime_trampoline() {
    unimplemented!("Go function declaration has no body");
}


/// walltime should be an internal detail,
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - gitee.com/quant1x/gox
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname walltime
///go:nosplit
///go:cgo_unsafe_args
pub fn walltime() -> (i64, i32) {
    let mut t: Arc<Mutex<Option<timespec>>> = Arc::new(Mutex::new(Some(Default::default())));
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(walltime_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&t.clone()) as usize))));
    return (
        (*(*t.lock().unwrap().as_ref().unwrap()).tv_sec.lock().unwrap().as_ref().unwrap()),
        (*Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).tv_nsec.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    );
}

pub fn walltime_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn sigaction(sig_local: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<usigactiont>>>, old: Arc<Mutex<Option<usigactiont>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(sigaction_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&sig_local.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(new.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(old.clone()) as Box<dyn Any + Send + Sync>))));
}

pub fn sigaction_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn sigprocmask(how: Arc<Mutex<Option<u32>>>, new: Arc<Mutex<Option<sigset>>>, old: Arc<Mutex<Option<sigset>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(sigprocmask_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&how.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(new.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(old.clone()) as Box<dyn Any + Send + Sync>))));
}

pub fn sigprocmask_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn sigaltstack(new: Arc<Mutex<Option<stackt>>>, old: Arc<Mutex<Option<stackt>>>) {
    if { let __nil_result = (*new.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).ss_flags.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = __S_S__D_I_S_A_B_L_E as i32; __tmp_x & __tmp_y }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = (*{ let __field = (*new.lock().unwrap().as_ref().unwrap()).ss_size.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Despite the fact that Darwin's sigaltstack man page says it ignores the size
                // when SS_DISABLE is set, it doesn't. sigaltstack returns ENOMEM
                // if we don't give it a reasonable size.
                // ref: http://lists.llvm.org/pipermail/llvm-commits/Week-of-Mon-20140421/214296.html
        { let new_val = 32768 as usize; *(*new.lock().unwrap().as_ref().unwrap()).ss_size.lock().unwrap() = Some(new_val); };
    }
        // Despite the fact that Darwin's sigaltstack man page says it ignores the size
        // when SS_DISABLE is set, it doesn't. sigaltstack returns ENOMEM
        // if we don't give it a reasonable size.
        // ref: http://lists.llvm.org/pipermail/llvm-commits/Week-of-Mon-20140421/214296.html
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(sigaltstack_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(new.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(new.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(old.clone()) as Box<dyn Any + Send + Sync>))));
}

pub fn sigaltstack_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn raiseproc(sig_local: Arc<Mutex<Option<u32>>>) {
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(raiseproc_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&sig_local.clone()) as usize))));
}

pub fn raiseproc_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn fcntl(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    let mut ret: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut errno: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

    let mut args = Arc::new(Mutex::new(Some(AnonymousStruct36 { fd: Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), cmd: Arc::new(Mutex::new(Some({ let __arg_holder = cmd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), arg: Arc::new(Mutex::new(Some({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ret: Arc::new(Mutex::new(Some(0 as i32))), errno: Arc::new(Mutex::new(Some(0 as i32))) })));
    libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(fcntl_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&args.clone()) as usize))));
    return ((*(*args.lock().unwrap().as_ref().unwrap()).ret.lock().unwrap().as_ref().unwrap()), (*(*args.lock().unwrap().as_ref().unwrap()).errno.lock().unwrap().as_ref().unwrap()));
}

pub fn fcntl_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn kqueue() -> i32 {
    let mut v = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(kqueue_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(None)));
    v
}

pub fn kqueue_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn kevent(kq_local: Arc<Mutex<Option<i32>>>, ch: GoPtr<crate::defs_darwin_arm64::keventt>, nch: Arc<Mutex<Option<i32>>>, ev: GoPtr<crate::defs_darwin_arm64::keventt>, nev: Arc<Mutex<Option<i32>>>, ts: Arc<Mutex<Option<timespec>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(kevent_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&kq_local.clone()) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(ch.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(ev.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(ts.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn kevent_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_mutex_init(m: Arc<Mutex<Option<pthreadmutex>>>, attr: Arc<Mutex<Option<pthreadmutexattr>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_mutex_init_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(m.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(m.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_mutex_init_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_mutex_lock(m: Arc<Mutex<Option<pthreadmutex>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_mutex_lock_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(m.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(m.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_mutex_lock_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_mutex_unlock(m: Arc<Mutex<Option<pthreadmutex>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_mutex_unlock_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(m.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(m.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_mutex_unlock_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_cond_init(c: Arc<Mutex<Option<pthreadcond>>>, attr: Arc<Mutex<Option<pthreadcondattr>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_cond_init_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(c.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(attr.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_cond_init_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_cond_wait(c: Arc<Mutex<Option<pthreadcond>>>, m: Arc<Mutex<Option<pthreadmutex>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_cond_wait_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(c.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(m.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_cond_wait_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_cond_timedwait_relative_np(c: Arc<Mutex<Option<pthreadcond>>>, m: Arc<Mutex<Option<pthreadmutex>>>, t: Arc<Mutex<Option<timespec>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_cond_timedwait_relative_np_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(c.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(m.clone()) as Box<dyn Any + Send + Sync>))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(t.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_cond_timedwait_relative_np_trampoline() {
    unimplemented!("Go function declaration has no body");
}


///go:nosplit
///go:cgo_unsafe_args
pub fn pthread_cond_signal(c: Arc<Mutex<Option<pthreadcond>>>) -> i32 {
    let mut ret = libc_call(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(pthread_cond_signal_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(c.clone())))) as usize))));
    keep_alive(Arc::new(Mutex::new(Some(Box::new(c.clone()) as Box<dyn Any + Send + Sync>))));
    ret
}

pub fn pthread_cond_signal_trampoline() {
    unimplemented!("Go function declaration has no body");
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct34 {
    pub addr: Arc<Mutex<Option<usize>>>,
    pub n: Arc<Mutex<Option<usize>>>,
    pub prot: Arc<Mutex<Option<i32>>>,
    pub flags: Arc<Mutex<Option<i32>>>,
    pub fd: Arc<Mutex<Option<i32>>>,
    pub off: Arc<Mutex<Option<u32>>>,
    pub ret1: Arc<Mutex<Option<usize>>>,
    pub ret2: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct34 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.addr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.prot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_1 = { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_2 = { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.off.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.ret1.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.ret2.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            addr: __go_clone_0_0,
            n: __go_clone_1_0,
            prot: __go_clone_2_0,
            flags: __go_clone_2_1,
            fd: __go_clone_2_2,
            off: __go_clone_3_0,
            ret1: __go_clone_4_0,
            ret2: __go_clone_5_0,
        }
    }
}


impl Default for AnonymousStruct34 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            addr: __go_default_0_0,
            n: __go_default_1_0,
            prot: __go_default_2_0,
            flags: __go_default_2_1,
            fd: __go_default_2_2,
            off: __go_default_3_0,
            ret1: __go_default_4_0,
            ret2: __go_default_5_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct34 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.addr.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.prot.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.flags.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.fd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.off.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.ret1.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.ret2.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7)
    }
}

impl GoJsonDecode for AnonymousStruct34 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct35 {
    pub t: Arc<Mutex<Option<i64>>>,
    pub numer: Arc<Mutex<Option<u32>>>,
    pub denom: Arc<Mutex<Option<u32>>>,
}
impl AnonymousStruct35 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.t.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.numer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_1 = { let __guard = self.denom.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            t: __go_clone_0_0,
            numer: __go_clone_1_0,
            denom: __go_clone_1_1,
        }
    }
}


impl Default for AnonymousStruct35 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            t: __go_default_0_0,
            numer: __go_default_1_0,
            denom: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct35 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.t.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.numer.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.denom.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for AnonymousStruct35 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct36 {
    pub fd: Arc<Mutex<Option<i32>>>,
    pub cmd: Arc<Mutex<Option<i32>>>,
    pub arg: Arc<Mutex<Option<i32>>>,
    pub ret: Arc<Mutex<Option<i32>>>,
    pub errno: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct36 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.fd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.cmd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_2 = { let __guard = self.arg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ret.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_1 = { let __guard = self.errno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            fd: __go_clone_0_0,
            cmd: __go_clone_0_1,
            arg: __go_clone_0_2,
            ret: __go_clone_1_0,
            errno: __go_clone_1_1,
        }
    }
}


impl Default for AnonymousStruct36 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_2 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            fd: __go_default_0_0,
            cmd: __go_default_0_1,
            arg: __go_default_0_2,
            ret: __go_default_1_0,
            errno: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for AnonymousStruct36 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.fd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.cmd.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.arg.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.ret.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.errno.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for AnonymousStruct36 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}
