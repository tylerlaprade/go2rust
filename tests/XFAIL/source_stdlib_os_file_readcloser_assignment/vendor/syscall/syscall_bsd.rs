use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoLocalPtrKey,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    syscall_darwin::{SockaddrDatalink, getdirentries, nametomib},
    syscall_unix::{Sockaddr, SockaddrInet4, SockaddrInet6, SockaddrUnix, clen, mmapper},
    zerrors_darwin_arm64::{A_F__I_N_E_T, A_F__I_N_E_T6, A_F__L_I_N_K, A_F__U_N_I_X, E_A_F_N_O_S_U_P_P_O_R_T, E_C_O_N_N_A_B_O_R_T_E_D, E_I_N_V_A_L, E_I_O},
    zsyscall_darwin_arm64::{accept_1, close, getcwd, mmap_1, munmap_1, recvmsg_1, sendmsg_1, sysctl_1},
    ztypes_darwin_arm64::{Iovec, Msghdr, PATH_MAX, RawSockaddr, RawSockaddrAny, RawSockaddrDatalink, RawSockaddrInet4, RawSockaddrInet6, RawSockaddrUnix, SIZEOF_SOCKADDR_ANY, SIZEOF_SOCKADDR_DATALINK, SIZEOF_SOCKADDR_INET4, SIZEOF_SOCKADDR_INET6, SIZEOF_SOCKADDR_UNIX, _C_int, _Socklen},
};

use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub const IMPLEMENTS_GETWD: bool = true;


pub(crate) const MASK: i32 = 0x7F;
pub(crate) const CORE: i32 = 0x80;
pub(crate) const SHIFT: i32 = 8;
pub(crate) const EXITED: i32 = 0;
pub(crate) const STOPPED: i32 = 0x7F;


pub(crate) static mapper: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::syscall_unix::mmapper>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *mapper.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *mapper.lock().unwrap() = Some({
        let __owner = Arc::new(Mutex::new(Some(crate::syscall_unix::mmapper {
            active: Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<u8>, Arc<Mutex<Option<Vec<u8>>>>>::new()))),
            mmap: Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>, __arg2: Arc<Mutex<Option<i32>>>, __arg3: Arc<Mutex<Option<i32>>>, __arg4: Arc<Mutex<Option<i32>>>, __arg5: Arc<Mutex<Option<i64>>>| -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { mmap_1(__arg0, __arg1, __arg2, __arg3, __arg4, __arg5) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))),
            munmap: Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> { munmap_1(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().mutex.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    });
}


pub(crate) fn __go_zero_globals() {
    *mapper.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
}


pub(crate) fn __go_init_order_9() {
    *mapper.lock().unwrap() = Some({
        let __owner = Arc::new(Mutex::new(Some(crate::syscall_unix::mmapper {
            active: Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<u8>, Arc<Mutex<Option<Vec<u8>>>>>::new()))),
            mmap: Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>, __arg2: Arc<Mutex<Option<i32>>>, __arg3: Arc<Mutex<Option<i32>>>, __arg4: Arc<Mutex<Option<i32>>>, __arg5: Arc<Mutex<Option<i64>>>| -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { mmap_1(__arg0, __arg1, __arg2, __arg3, __arg4, __arg5) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i64>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>))),
            munmap: Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> { munmap_1(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().mutex.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    });
}


impl crate::syscall_unix::SockaddrInet4 {
    pub fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 65535; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0 as u32))))))), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = SIZEOF_SOCKADDR_INET4 as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        { let new_val = A_F__I_N_E_T as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
        let mut p: GoPtr<[u8; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*self.raw.lock().unwrap().as_ref().unwrap()).port.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
        { let new_val = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.port.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
        { let new_val = { let __selector_holder = self.addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap() = Some(new_val); };
        (
            Arc::new(Mutex::new(Some(Arc::as_ptr(&self.raw.clone()) as usize))),
            Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.raw.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))))))),
            Arc::new(Mutex::new(None))
        )
    }
}

impl crate::syscall_unix::SockaddrInet6 {
    pub fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 65535; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0 as u32))))))), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = SIZEOF_SOCKADDR_INET6 as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        { let new_val = A_F__I_N_E_T6 as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
        let mut p: GoPtr<[u8; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*self.raw.lock().unwrap().as_ref().unwrap()).port.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
        { let new_val = (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.port.lock().unwrap().as_ref().unwrap()); let __tmp_y = 8; __tmp_x >> __tmp_y }) as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.with_mut(|__seq| { __seq[(0) as usize] = new_val; }); };
        { let new_val = (*Arc::new(Mutex::new(Some({ let __selector_holder = self.port.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u8))).lock().unwrap().as_ref().unwrap()).clone(); p.with_mut(|__seq| { __seq[(1) as usize] = new_val; }); };
        { let new_val = { let __selector_holder = self.zone_id.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).scope_id.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.addr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap() = Some(new_val); };
        (
            Arc::new(Mutex::new(Some(Arc::as_ptr(&self.raw.clone()) as usize))),
            Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.raw.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))))))),
            Arc::new(Mutex::new(None))
        )
    }
}

impl crate::syscall_unix::SockaddrUnix {
    pub fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut n = Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()).len() as i32)));
        if {
            let __go_cond_0 = {
                let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32);
                let __tmp_y = 104;
                __tmp_x >= __tmp_y
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y };
                __go_cond_1
            }
        } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0 as u32))))))), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = Arc::new(Mutex::new(Some(({ let __tmp_x = 3; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as u8))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.raw.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = __moved_val; };
        { let new_val = A_F__U_N_I_X as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*(*self.raw.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        (
            Arc::new(Mutex::new(Some(Arc::as_ptr(&self.raw.clone()) as usize))),
            Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.raw.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u32))))))),
            Arc::new(Mutex::new(None))
        )
    }
}

impl crate::syscall_darwin::SockaddrDatalink {
    pub fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = (*self.index.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0 as u32))))))), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
        { let new_val = { let __selector_holder = self.len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
        { let new_val = A_F__L_I_N_K as u8; *(*self.raw.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.r#type.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.nlen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).nlen.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.alen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).alen.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.slen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).slen.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = self.data.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.raw.lock().unwrap().as_ref().unwrap()).data.lock().unwrap() = Some(new_val); };
        (
            Arc::new(Mutex::new(Some(Arc::as_ptr(&self.raw.clone()) as usize))),
            Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(SIZEOF_SOCKADDR_DATALINK as u32))))))),
            Arc::new(Mutex::new(None))
        )
    }
}

pub fn getwd() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut buf: Arc<Mutex<Option<[u8; 1024]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let (_, mut err) = getcwd(Arc::new(Mutex::new(Some({
        let __seq_holder = buf.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    let mut n = clen(Arc::new(Mutex::new(Some({
        let __seq_holder = buf.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
        let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
        drop(__seq_guard);
        let __low = 0;
        let __high = __seq.len();
        let __max = __source_cap;
        let _slice = &__seq[__low..__high];
        let mut _v = Vec::with_capacity((__max - __low) as usize);
        _v.extend_from_slice(_slice);
        _v
    }))));
    if { let __tmp_x = n; let __tmp_y = 1; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    return (
        Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({
            let __seq_holder = buf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = (n) as usize;
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))),
        Arc::new(Mutex::new(None))
    );
}

pub fn read_dirent(fd: Arc<Mutex<Option<i32>>>, buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // Final argument is (basep *uintptr) and the syscall doesn't take nil.
        // 64 bits should be enough. (32 bits isn't even on 386). Since the
        // actual system call is getdirentries64, 64 is a good guess.
        // TODO(rsc): Can we use a single global basep for all calls?
    let mut base = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&Arc::new(Mutex::new(Some(u64::default())))) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } }));
    return getdirentries(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), buf.clone(), base.clone());
}

pub fn any_to_sockaddr(rsa: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrAny>) -> (Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    { let _switch_val = { let __v = (*{ let __ptr_value = rsa.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).family.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (A_F__L_I_N_K as u8) {
            let mut pp: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrDatalink> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(rsa.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            let mut sa = Arc::new(Mutex::new(Some(SockaddrDatalink::default())));
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).len.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.family.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).family.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.index.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.r#type.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.nlen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).nlen.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.alen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).alen.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.slen.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).slen.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.data.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).data.lock().unwrap() = Some(new_val); };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_darwin::SockaddrDatalinkPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else if _switch_val == (A_F__U_N_I_X as u8) {
            let mut pp: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrUnix> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(rsa.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            if { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 2 as u8; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __ptr_value = pp.borrow(); __ptr_value.as_ref().unwrap().len.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = SIZEOF_SOCKADDR_UNIX as u8; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
            let mut sa = Arc::new(Mutex::new(Some(SockaddrUnix::default())));
                        // Some BSDs include the trailing NUL in the length, whereas
                        // others do not. Work around this by subtracting the leading
                        // family and len. The path is then scanned to see if a NUL
                        // terminator still exists within the length.
            let mut n = Arc::new(Mutex::new(Some({
                let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap());
                let __tmp_y = 2;
                __tmp_x - __tmp_y
            })));
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if {
            let __tmp_x = { let __seq = { let __seq_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.path.clone()); __ptr_value }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
            let __tmp_y = 0 as i8;
            __tmp_x == __tmp_y
        } {
                // found early NUL; assume Len included the NUL
                // or was overestimating.
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *n.lock().unwrap() = Some(new_val); };
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                        // found early NUL; assume Len included the NUL
                        // or was overestimating.
            { let new_val = Arc::new(Mutex::new(Some(String::from_utf8((*{ let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*sa.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = __moved_val; };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrUnixPtr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else if _switch_val == (A_F__I_N_E_T as u8) {
            let mut pp: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrInet4> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(rsa.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet4::default())));
            let mut p: GoPtr<[u8; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.port.clone()); __ptr_value }.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            { let new_val = {
                let __tmp_x = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = 8;
                    __tmp_x << __tmp_y
                };
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            }; *(*sa.lock().unwrap().as_ref().unwrap()).port.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap() = Some(new_val); };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet4Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        } else if _switch_val == (A_F__I_N_E_T6 as u8) {
            let mut pp: GoPtr<crate::ztypes_darwin_arm64::RawSockaddrInet6> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(rsa.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            let mut sa = Arc::new(Mutex::new(Some(SockaddrInet6::default())));
            let mut p: GoPtr<[u8; 2]> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&{ let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.port.clone()); __ptr_value }.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
            { let new_val = {
                let __tmp_x = {
                    let __tmp_x = (*Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[(0) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap());
                    let __tmp_y = 8;
                    __tmp_x << __tmp_y
                };
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[(1) as usize].clone() } as i32))).lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            }; *(*sa.lock().unwrap().as_ref().unwrap()).port.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.scope_id.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).zone_id.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = { let __ptr_value = pp.with_mut(|__ptr_value| __ptr_value.addr.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*sa.lock().unwrap().as_ref().unwrap()).addr.lock().unwrap() = Some(new_val); };
            return (Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::SockaddrInet6Ptr(sa.clone())) as Box<dyn Sockaddr + Send + Sync>))), Arc::new(Mutex::new(None)));
        }
    }
        // Some BSDs include the trailing NUL in the length, whereas
        // others do not. Work around this by subtracting the leading
        // family and len. The path is then scanned to see if a NUL
        // terminator still exists within the length.
        // subtract leading Family, Len
        // found early NUL; assume Len included the NUL
        // or was overestimating.
    return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_A_F_N_O_S_U_P_P_O_R_T as usize))))) as Box<dyn StdError + Send + Sync>))));
}

pub fn accept(fd: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut nfd: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut sa: Arc<Mutex<Option<Box<dyn Sockaddr + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut rsa: Arc<Mutex<Option<RawSockaddrAny>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut len: Arc<Mutex<Option<_Socklen>>> = Arc::new(Mutex::new(Some(crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(SIZEOF_SOCKADDR_ANY as u32)))))));
    { let (__tmp_0, __tmp_1) = accept_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), rsa.clone(), len.clone()); *nfd.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*nfd.lock().unwrap().as_ref().unwrap()), sa.clone(), err.clone());
    }
    if ({ let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y }) && { let __tmp_x = (*len.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::ztypes_darwin_arm64::_Socklen(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x == __tmp_y } {
                // Accepted socket has no address.
                // This is likely due to a bug in xnu kernels,
                // where instead of ECONNABORTED error socket
                // is accepted, but has no address.
        close(Arc::new(Mutex::new(Some({ let __arg_holder = nfd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return (0, Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_C_O_N_N_A_B_O_R_T_E_D as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
        // Accepted socket has no address.
        // This is likely due to a bug in xnu kernels,
        // where instead of ECONNABORTED error socket
        // is accepted, but has no address.
    { let (__tmp_0, __tmp_1) = any_to_sockaddr(GoPtr::local(rsa.clone())); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *sa.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        close(Arc::new(Mutex::new(Some({ let __arg_holder = nfd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = 0; *nfd.lock().unwrap() = Some(new_val); };
    }
    return ((*nfd.lock().unwrap().as_ref().unwrap()), sa.clone(), err.clone());
}

pub fn recvmsg_raw(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, flags: Arc<Mutex<Option<i32>>>, rsa: Arc<Mutex<Option<RawSockaddrAny>>>) -> (i32, i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut oobn: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut recvflags: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut msg: Arc<Mutex<Option<Msghdr>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&rsa) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone(); (*msg.lock().unwrap().as_mut().unwrap()).name = new_val; };
    { let new_val = Arc::new(Mutex::new(Some(SIZEOF_SOCKADDR_ANY as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*msg.lock().unwrap().as_ref().unwrap()).namelen.lock().unwrap() = __moved_val; };
    let mut iov: Arc<Mutex<Option<Iovec>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new(p.clone(), (0) as usize)); (*iov.lock().unwrap().as_mut().unwrap()).base = new_val; };
        (*iov.lock().unwrap().as_mut().unwrap()).set_len(Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
    let mut dummy: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // receive at least one normal byte
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = GoPtr::local(dummy.clone().clone()); (*iov.lock().unwrap().as_mut().unwrap()).base = new_val; };
        (*iov.lock().unwrap().as_mut().unwrap()).set_len(Arc::new(Mutex::new(Some(1))));
    }
        { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new(oob.clone(), (0) as usize)); (*msg.lock().unwrap().as_mut().unwrap()).control = new_val; };
        (*msg.lock().unwrap().as_mut().unwrap()).set_controllen(Arc::new(Mutex::new(Some((*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
        // receive at least one normal byte
    { let new_val = iov.clone().clone(); (*msg.lock().unwrap().as_mut().unwrap()).iov = new_val; };
    { let new_val = 1 as i32; *(*msg.lock().unwrap().as_ref().unwrap()).iovlen.lock().unwrap() = Some(new_val); };
    {
        { let (__tmp_0, __tmp_1) = recvmsg_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), msg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return ((*n.lock().unwrap().as_ref().unwrap()), (*oobn.lock().unwrap().as_ref().unwrap()), (*recvflags.lock().unwrap().as_ref().unwrap()), err.clone());;
        }
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*msg.lock().unwrap().as_ref().unwrap()).controllen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *oobn.lock().unwrap() = __moved_val; };
    { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*msg.lock().unwrap().as_ref().unwrap()).flags.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *recvflags.lock().unwrap() = __moved_val; };
    return ((*n.lock().unwrap().as_ref().unwrap()), (*oobn.lock().unwrap().as_ref().unwrap()), (*recvflags.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn sendmsg_n_1(fd: Arc<Mutex<Option<i32>>>, p: Arc<Mutex<Option<Vec<u8>>>>, oob: Arc<Mutex<Option<Vec<u8>>>>, ptr: Arc<Mutex<Option<usize>>>, salen: Arc<Mutex<Option<_Socklen>>>, flags: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let mut msg: Arc<Mutex<Option<Msghdr>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = Arc::new(Mutex::new({ let __ptr = ptr.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone(); (*msg.lock().unwrap().as_mut().unwrap()).name = new_val; };
    { let new_val = Arc::new(Mutex::new(Some((*{ let __v = (*salen.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*msg.lock().unwrap().as_ref().unwrap()).namelen.lock().unwrap() = __moved_val; };
    let mut iov: Arc<Mutex<Option<Iovec>>> = Arc::new(Mutex::new(Some(Default::default())));
    if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new(p.clone(), (0) as usize)); (*iov.lock().unwrap().as_mut().unwrap()).base = new_val; };
        (*iov.lock().unwrap().as_mut().unwrap()).set_len(Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
    let mut dummy: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // send at least one normal byte
        if { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = GoPtr::local(dummy.clone().clone()); (*iov.lock().unwrap().as_mut().unwrap()).base = new_val; };
        (*iov.lock().unwrap().as_mut().unwrap()).set_len(Arc::new(Mutex::new(Some(1))));
    }
        { let new_val = GoPtr::slice_elem(GoSliceElemPtr::new(oob.clone(), (0) as usize)); (*msg.lock().unwrap().as_mut().unwrap()).control = new_val; };
        (*msg.lock().unwrap().as_mut().unwrap()).set_controllen(Arc::new(Mutex::new(Some((*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))));
    }
        // send at least one normal byte
    { let new_val = iov.clone().clone(); (*msg.lock().unwrap().as_mut().unwrap()).iov = new_val; };
    { let new_val = 1 as i32; *(*msg.lock().unwrap().as_ref().unwrap()).iovlen.lock().unwrap() = Some(new_val); };
    {
        { let (__tmp_0, __tmp_1) = sendmsg_1(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), msg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
    if { let __tmp_x = ((*oob.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };
    }
    return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
}

pub fn sysctl_uint32(name: Arc<Mutex<Option<String>>>) -> (u32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut value: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // Translate name to mib number.
    let (mut mib, __tmp_1) = nametomib(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0, err.clone());
    }

        // Read into buffer of that size.
    let mut n = Arc::new(Mutex::new(Some(4 as usize)));
    let mut buf = Arc::new(Mutex::new(Some(vec![0; (4) as usize])));
    {
        { let __rhs_holder = sysctl_1(
            mib.clone(),
            GoPtr::slice_elem(GoSliceElemPtr::new(buf.clone(), (0) as usize)),
            n.clone(),
            GoPtr::nil(),
            Arc::new(Mutex::new(Some(0 as usize)))
        ).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4 as usize; __tmp_x != __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::new(crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(E_I_O as usize))))) as Box<dyn StdError + Send + Sync>))));
    }
    return (
        { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u32>(unimplemented!("unsafe.Pointer conversion to u32")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v },
        Arc::new(Mutex::new(None))
    );
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
