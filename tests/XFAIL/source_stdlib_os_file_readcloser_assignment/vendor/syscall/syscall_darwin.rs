use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{exec_unix::{execveDarwin}, r#mod::{byte_slice_from_string}, syscall_unix::{Errno, errno_err}, zerrors_darwin_arm64::{C_T_L__M_A_X_N_A_M_E, O__R_D_O_N_L_Y}, zsyscall_darwin_arm64::{close, closedir, execve, libc_dup2_trampoline, openat, readdir_r, seek, sysctl_1}, ztypes_darwin_arm64::{Dirent, RawSockaddrDatalink, _C_int}};

use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SockaddrDatalink {
    pub len: Arc<Mutex<Option<u8>>>,
    pub family: Arc<Mutex<Option<u8>>>,
    pub index: Arc<Mutex<Option<u16>>>,
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub nlen: Arc<Mutex<Option<u8>>>,
    pub alen: Arc<Mutex<Option<u8>>>,
    pub slen: Arc<Mutex<Option<u8>>>,
    pub data: Arc<Mutex<Option<[i8; 12]>>>,
    pub raw: Arc<Mutex<Option<RawSockaddrDatalink>>>,
}

impl SockaddrDatalink {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.family.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.nlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.alen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.slen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.raw.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            len: __go_clone_0_0,
            family: __go_clone_1_0,
            index: __go_clone_2_0,
            r#type: __go_clone_3_0,
            nlen: __go_clone_4_0,
            alen: __go_clone_5_0,
            slen: __go_clone_6_0,
            data: __go_clone_7_0,
            raw: __go_clone_8_0,
        }
    }
}


impl Default for SockaddrDatalink {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_7_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let __go_default_8_0 = Arc::new(Mutex::new(Some(RawSockaddrDatalink::default())));
        Self {
            len: __go_default_0_0,
            family: __go_default_1_0,
            index: __go_default_2_0,
            r#type: __go_default_3_0,
            nlen: __go_default_4_0,
            alen: __go_default_5_0,
            slen: __go_default_6_0,
            data: __go_default_7_0,
            raw: __go_default_8_0,
        }
    }
}

impl std::fmt::Display for SockaddrDatalink {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.len.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.family.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.nlen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.alen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.slen.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", format_slice(&self.data));
        let __go_fmt_8 = format!("{}", (*self.raw.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8)
    }
}

impl GoJsonDecode for SockaddrDatalink {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Family") {
            out.family = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Index") {
            out.index = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Type") {
            out.r#type = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Nlen") {
            out.nlen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Alen") {
            out.alen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Slen") {
            out.slen = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Data") {
            out.data = <Arc<Mutex<Option<[i8; 12]>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static dupTrampoline: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *dupTrampoline.lock().unwrap() = Some(0);
    *dupTrampoline.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_dup2_trampoline.clone()) as Box<dyn Any + Send + Sync>)))));
}


pub(crate) fn __go_zero_globals() {
    *dupTrampoline.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_2() {
    *dupTrampoline.lock().unwrap() = Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_dup2_trampoline.clone()) as Box<dyn Any + Send + Sync>)))));
}


impl SockaddrDatalink {
}

impl Sockaddr for SockaddrDatalink {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        SockaddrDatalink::sockaddr(self)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrDatalink>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SockaddrDatalinkPtr(pub Arc<Mutex<Option<SockaddrDatalink>>>);

impl std::fmt::Display for SockaddrDatalinkPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Sockaddr for SockaddrDatalinkPtr {
    fn sockaddr(&mut self) -> (Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<crate::ztypes_darwin_arm64::_Socklen>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        SockaddrDatalink::sockaddr(__recv)
    }
    fn __go_clone_box_sockaddr(&self) -> Box<dyn Sockaddr + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Sockaddr + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_sockaddr(&self, other: &(dyn Sockaddr + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SockaddrDatalinkPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// Translate "kern.hostname" to []_C_int{0,1,2,3}.
pub fn nametomib(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<crate::ztypes_darwin_arm64::_C_int>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut mib: Arc<Mutex<Option<Vec<_C_int>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    const siz: usize = std::mem::size_of::<crate::ztypes_darwin_arm64::_C_int>();


        // NOTE(rsc): It seems strange to set the buffer to have
        // size CTL_MAXNAME+2 but use only CTL_MAXNAME
        // as the size. I don't know why the +2 is here, but the
        // kernel uses +2 for its own implementation of this function.
        // I am scared that if we don't include the +2 here, the kernel
        // will silently write 2 words farther than we specify
        // and we'll get memory corruption.
    let mut buf: Arc<Mutex<Option<[_C_int; 14]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| crate::ztypes_darwin_arm64::_C_int(Arc::new(Mutex::new(Some(0))))))));
    let mut n = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(C_T_L__M_A_X_N_A_M_E as usize))).lock().unwrap().as_ref().unwrap()) as usize; let __tmp_y = siz as usize; __tmp_x * __tmp_y })));

    let mut p: GoPtr<u8> = GoPtr::raw({ let __ptr = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    let (mut bytes, __tmp_1) = byte_slice_from_string(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }

        // Magic sysctl: "setting" 0.3 to a string name
        // lets you read back the array of integers form.
    {
        { let __rhs_holder = sysctl_1(Arc::new(Mutex::new(Some(vec![crate::ztypes_darwin_arm64::_C_int(Arc::new(Mutex::new(Some(0 as i32)))), crate::ztypes_darwin_arm64::_C_int(Arc::new(Mutex::new(Some(3 as i32))))]))), p.clone(), n.clone(), GoPtr::slice_elem(GoSliceElemPtr::new(bytes.clone(), (0) as usize)), Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()).len() as usize)))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (Arc::new(Mutex::new(None)), err.clone());;
        }
    }
    return (Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = (0) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = siz as usize; __tmp_x / __tmp_y }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(None)));
}

fn __go_init_0() {
    { let new_val = Box::new(move |__arg0: Arc<Mutex<Option<u8>>>, __arg1: Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>, __arg2: Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>| -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> { execve(__arg0, __arg1, __arg2) }) as Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>; *execveDarwin.lock().unwrap() = Some(new_val); };
}

pub fn fdopendir(fd: Arc<Mutex<Option<i32>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut dir: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut r0, _, mut e1) = syscall_ptr(Arc::new(Mutex::new(Some(internal_abi::func_p_c_a_b_i0(Arc::new(Mutex::new(Some(Box::new(libc_fdopendir_trampoline.clone()) as Box<dyn Any + Send + Sync>))))))), Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some(0 as usize))), Arc::new(Mutex::new(Some(0 as usize))));
    { let new_val = r0; *dir.lock().unwrap() = Some(new_val); };
    if { let __tmp_x = (*e1.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*dir.lock().unwrap().as_ref().unwrap()), err.clone());
}

pub fn libc_fdopendir_trampoline() {
    unimplemented!("Go function declaration has no body");
}


pub fn getdirentries(fd: Arc<Mutex<Option<i32>>>, mut buf: Arc<Mutex<Option<Vec<u8>>>>, basep: Arc<Mutex<Option<usize>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // Simulate Getdirentries using fdopendir/readdir_r/closedir.
                // We store the number of entries to skip in the seek
                // offset of fd. See issue #31368.
                // It's not the full required semantics, but should handle the case
                // of calling Getdirentries or ReadDirent repeatedly.
                // It won't handle assigning the results of lseek to *basep, or handle
                // the directory being edited underfoot.
        let (mut skip, __tmp_1) = seek(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0 as i64))), Arc::new(Mutex::new(Some(1)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }

                // We need to duplicate the incoming file descriptor
                // because the caller expects to retain control of it, but
                // fdopendir expects to take control of its argument.
                // Just Dup'ing the file descriptor is not enough, as the
                // result shares underlying state. Use openat to make a really
                // new file descriptor referring to the same directory.
        let (mut fd2, __tmp_1) = openat(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(".".to_string()))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(0 as u32)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
        let (mut d, __tmp_1) = fdopendir(Arc::new(Mutex::new(Some(fd2)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        close(Arc::new(Mutex::new(Some(fd2))));
        {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
        let d_defer_captured = d.clone(); __defer_stack.push(Box::new(move || {
        closedir(Arc::new(Mutex::new(Some(d_defer_captured))));
    }));

        let mut cnt: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        loop {
        let mut entry: Arc<Mutex<Option<Dirent>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut entryp: Arc<Mutex<Option<Dirent>>> = Arc::new(Mutex::new(None));
        let mut e = readdir_r(Arc::new(Mutex::new(Some(d))), entry.clone(), Arc::new(Mutex::new(Some(entryp.clone()))));
        if { let __tmp_x = (*e.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        {
        { let __rhs_holder = errno_err(Arc::new(Mutex::new(Some({ let __arg_holder = e.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
        if { let __nil_result = (*entryp.lock().unwrap()).is_none(); __nil_result } {
        break
    }
        if { let __tmp_x = skip; let __tmp_y = 0 as i64; __tmp_x > __tmp_y } {
        { skip -= 1; }
        { let mut guard = cnt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        continue
    }
        let mut reclen = Arc::new(Mutex::new(Some({ let __selector_holder = (*entry.lock().unwrap().as_ref().unwrap()).reclen.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32)));
        if { let __tmp_x = ({ let __v = (*reclen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x > __tmp_y } {
                // Not enough room. Return for now.
                // The counter will let us know where we should start up again.
                // Note: this strategy for suspending in the middle and
                // restarting is O(n^2) in the length of the directory. Oh well.
        break
    }

                // Not enough room. Return for now.
                // The counter will let us know where we should start up again.
                // Note: this strategy for suspending in the middle and
                // restarting is O(n^2) in the length of the directory. Oh well.
                // Copy entry into return buffer.
        { let _src = { let __copy_src_holder = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*buf.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*reclen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
        { let __rhs = (*reclen.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = cnt.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // Not enough room. Return for now.
                // The counter will let us know where we should start up again.
                // Note: this strategy for suspending in the middle and
                // restarting is O(n^2) in the length of the directory. Oh well.
                // Copy entry into return buffer.
                // Set the seek offset of the input fd to record
                // how many files we've already returned.
        { let (__tmp_0, __tmp_1) = seek(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = cnt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }

        {
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }));
    std::panic::set_hook(__go_previous_panic_hook);
    match __go_panic_result {
        Ok(__go_value) => __go_value,
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                f();
            }
            go_resume_unrecovered_panic();
            ((*n.lock().unwrap().as_ref().unwrap()), err.clone())
        }
    }
}

/// Implemented in the runtime package (runtime/sys_darwin.go)
pub fn syscall_1(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn syscall6_1(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>, a4: Arc<Mutex<Option<usize>>>, a5: Arc<Mutex<Option<usize>>>, a6: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn syscall6_x(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>, a4: Arc<Mutex<Option<usize>>>, a5: Arc<Mutex<Option<usize>>>, a6: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn raw_syscall_1(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}


pub fn syscall_ptr(r#fn: Arc<Mutex<Option<usize>>>, a1: Arc<Mutex<Option<usize>>>, a2: Arc<Mutex<Option<usize>>>, a3: Arc<Mutex<Option<usize>>>) -> (usize, usize, Arc<Mutex<Option<crate::syscall_unix::Errno>>>) {
    unimplemented!("Go function declaration has no body");
}


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}


impl GoValueClone for SockaddrDatalink {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
