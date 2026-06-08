use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_libc2::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
use crate::rlimit::*;
use crate::rlimit_darwin::*;
use crate::route_bsd::*;
use crate::route_darwin::*;
use crate::sockcmsg_unix::*;
use crate::sockcmsg_unix_other::*;
use crate::r#mod::*;
use crate::syscall_bsd::*;
use crate::syscall_darwin::*;
use crate::syscall_darwin_arm64::*;
use crate::syscall_unix::*;
use crate::time_nofake::*;
use crate::timestruct::*;
use crate::zerrors_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;
use crate::ztypes_darwin_arm64::*;

use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn accept_1(s: Arc<Mutex<Option<i32>>>, rsa: Arc<Mutex<Option<RawSockaddrAny>>>, addrlen: Arc<Mutex<Option<_Socklen>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut fd: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_accept_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&rsa) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&addrlen) as usize))).lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fd.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*fd.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_accept_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn getsockopt(s: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, name: Arc<Mutex<Option<i32>>>, val: Arc<Mutex<Option<usize>>>, vallen: Arc<Mutex<Option<_Socklen>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_getsockopt_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*level.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&vallen) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_getsockopt_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn setsockopt(s: Arc<Mutex<Option<i32>>>, level: Arc<Mutex<Option<i32>>>, name: Arc<Mutex<Option<i32>>>, val: Arc<Mutex<Option<usize>>>, vallen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_setsockopt_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*level.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*val.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*vallen.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_setsockopt_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn shutdown(s: Arc<Mutex<Option<i32>>>, how: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_shutdown_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*how.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_shutdown_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn recvfrom_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, from: Arc<Mutex<Option<RawSockaddrAny>>>, fromlen: Arc<Mutex<Option<_Socklen>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_recvfrom_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some((*flags.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&from) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&fromlen) as usize))).lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_recvfrom_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn sendto_1(s: Arc<Mutex<Option<i32>>>, buf: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, to: Arc<Mutex<Option<usize>>>, addrlen: Arc<Mutex<Option<_Socklen>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (_, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_sendto_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some((*flags.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*to.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*{ let __v = (*addrlen.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_sendto_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn recvmsg_1(s: Arc<Mutex<Option<i32>>>, msg: Arc<Mutex<Option<Msghdr>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_recvmsg_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&msg) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*flags.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_recvmsg_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn sendmsg_1(s: Arc<Mutex<Option<i32>>>, msg: Arc<Mutex<Option<Msghdr>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_sendmsg_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&msg) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*flags.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_sendmsg_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fcntl(fd: Arc<Mutex<Option<i32>>>, cmd: Arc<Mutex<Option<i32>>>, arg: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut val: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fcntl_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*cmd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*arg.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *val.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*val.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_fcntl_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn chmod(path: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = byte_ptr_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); _p0 = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_chmod_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&_p0) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*mode.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_chmod_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn close(fd: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_close_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_close_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn closedir(dir: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_closedir_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*dir.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_closedir_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn dup(fd: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut nfd: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_dup_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *nfd.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*nfd.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_dup_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn libc_dup2_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fchdir(fd: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fchdir_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_fchdir_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fchmod(fd: Arc<Mutex<Option<i32>>>, mode: Arc<Mutex<Option<u32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fchmod_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*mode.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_fchmod_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fchown(fd: Arc<Mutex<Option<i32>>>, uid: Arc<Mutex<Option<i32>>>, gid: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fchown_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*uid.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*gid.lock().unwrap().as_ref().unwrap()) as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_fchown_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fsync(fd: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fsync_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_fsync_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn ftruncate(fd: Arc<Mutex<Option<i32>>>, length: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_ftruncate_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_ftruncate_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn getrlimit(which: Arc<Mutex<Option<i32>>>, lim: Arc<Mutex<Option<Rlimit>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = raw_syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_getrlimit_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*which.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&lim) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_getrlimit_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn open(path: Arc<Mutex<Option<String>>>, mode: Arc<Mutex<Option<i32>>>, perm: Arc<Mutex<Option<u32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut fd: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = byte_ptr_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); _p0 = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*fd.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_open_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&_p0) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*mode.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*perm.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fd.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*fd.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_open_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn pread_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, offset: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_pread_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some((*offset.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_pread_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn pwrite_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, offset: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_pwrite_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some((*offset.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_pwrite_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn read_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_read_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_read_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn readdir_r(dir: Arc<Mutex<Option<usize>>>, entry: Arc<Mutex<Option<Dirent>>>, result: Arc<Mutex<Option<Arc<Mutex<Option<Dirent>>>>>>) -> Arc<Mutex<Option<crate::syscall_unix::Errno>>> {
    let mut res: Arc<Mutex<Option<Errno>>> = Arc::new(Mutex::new(Some(Default::default())));

    let (mut r0, _, _) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_readdir_r_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*dir.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&entry) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&result) as usize))).lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(r0 as usize)))); *res.lock().unwrap() = Some(new_val); };
    res.clone()
}

pub fn libc_readdir_r_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn seek(fd: Arc<Mutex<Option<i32>>>, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut newoffset: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_x(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_lseek_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*offset.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*whence.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *newoffset.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*newoffset.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_lseek_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn setrlimit_1(which: Arc<Mutex<Option<i32>>>, lim: GoPtr<crate::ztypes_darwin_arm64::Rlimit>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = raw_syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_setrlimit_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*which.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(lim.addr()))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_setrlimit_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn write_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_write_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_write_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn mmap_1(addr: Arc<Mutex<Option<usize>>>, length: Arc<Mutex<Option<usize>>>, prot: Arc<Mutex<Option<i32>>>, flag: Arc<Mutex<Option<i32>>>, fd: Arc<Mutex<Option<i32>>>, pos: Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ret: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall6_x(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_mmap_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*prot.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*flag.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()) as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *ret.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*ret.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_mmap_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn munmap_1(addr: Arc<Mutex<Option<usize>>>, length: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_munmap_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*addr.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*length.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_munmap_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn execve(path: Arc<Mutex<Option<u8>>>, argv: Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>, envp: Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = raw_syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_execve_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&path) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&argv) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&envp) as usize))).lock().unwrap().as_ref().unwrap()) as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_execve_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn sysctl_1(mib: Arc<Mutex<Option<Vec<_C_int>>>>, old: GoPtr<u8>, oldlen: Arc<Mutex<Option<usize>>>, new: GoPtr<u8>, newlen: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*mib.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = mib.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (_, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_sysctl_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*mib.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(old.addr()))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&oldlen) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(new.addr()))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*newlen.lock().unwrap().as_ref().unwrap()) as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_sysctl_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn openat(fd: Arc<Mutex<Option<i32>>>, path: Arc<Mutex<Option<String>>>, flags: Arc<Mutex<Option<i32>>>, perm: Arc<Mutex<Option<u32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut fdret: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = byte_ptr_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); _p0 = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*fdret.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    let (mut r0, _, mut e1) = syscall6_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_openat_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&_p0) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*flags.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*perm.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *fdret.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*fdret.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_openat_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn getcwd(buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(Arc::as_ptr(&_zero.clone()) as usize))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *_p0.lock().unwrap() = __moved_val; };
    }
    let (mut r0, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_getcwd_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*_p0.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = Arc::new(Mutex::new(Some(r0 as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_getcwd_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn fstat(fd: Arc<Mutex<Option<i32>>>, stat: Arc<Mutex<Option<Stat_t>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fstat_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&stat) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_fstat_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn lstat(path: Arc<Mutex<Option<String>>>, stat: Arc<Mutex<Option<Stat_t>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = byte_ptr_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); _p0 = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_lstat_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&_p0) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&stat) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_lstat_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn stat(path: Arc<Mutex<Option<String>>>, stat: Arc<Mutex<Option<Stat_t>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut _p0: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = byte_ptr_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); _p0 = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    let (_, _, mut e1) = syscall_1(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_stat_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&_p0) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some(Arc::as_ptr(&stat) as usize))).lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    err.clone()
}

pub fn libc_stat_trampoline() {
    unimplemented!("Go function declaration has no body");
}
