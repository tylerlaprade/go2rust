use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::dir::*;
use crate::dir_darwin::*;
use crate::eloop_other::*;
use crate::env::*;
use crate::error::*;
use crate::error_errno::*;
use crate::exec::*;
use crate::exec_nohandle::*;
use crate::exec_posix::*;
use crate::exec_unix::*;
use crate::executable::*;
use crate::executable_darwin::*;
use crate::file::*;
use crate::file_open_unix::*;
use crate::file_posix::*;
use crate::file_unix::*;
use crate::path::*;
use crate::path_unix::*;
use crate::pidfd_other::*;
use crate::pipe_unix::*;
use crate::proc::*;
use crate::rawconn::*;
use crate::removeall_at::*;
use crate::root::*;
use crate::root_nonwindows::*;
use crate::root_openat::*;
use crate::root_unix::*;
use crate::stat::*;
use crate::stat_darwin::*;
use crate::stat_unix::*;
use crate::sticky_bsd::*;
use crate::sys::*;
use crate::sys_bsd::*;
use crate::sys_unix::*;
use crate::tempfile::*;
use crate::types::*;
use crate::types_unix::*;
use crate::wait_unimp::*;
use crate::zero_copy_posix::*;
use crate::zero_copy_stub::*;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) static getwdCache: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *getwdCache.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *getwdCache.lock().unwrap() = Some(Default::default());
}


/// Getwd returns an absolute path name corresponding to the
/// current directory. If the current directory can be
/// reached via multiple paths (due to symbolic links),
/// Getwd may return any one of them.
///
/// On Unix platforms, if the environment variable PWD
/// provides an absolute name, and it is a name of the
/// current directory, it is returned.
pub fn getwd() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut dir: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if { let __tmp_x = "darwin".to_string(); let __tmp_y = "windows".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "plan9".to_string(); __tmp_x == __tmp_y } {
                // Use syscall.Getwd directly for
                //   - plan9: see reasons in CL 89575;
                //   - windows: syscall implementation is sufficient,
                //     and we should not rely on $PWD.
        { let (__tmp_0, __tmp_1) = syscall::getwd(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        return ({ let __owned = dir.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, new_syscall_error(Arc::new(Mutex::new(Some("getwd".to_string()))), err.clone()));
    }

        // Use syscall.Getwd directly for
        //   - plan9: see reasons in CL 89575;
        //   - windows: syscall implementation is sufficient,
        //     and we should not rely on $PWD.
        // Clumsy but widespread kludge:
        // if $PWD is set and matches ".", use it.
    let mut dot: FileInfo = Arc::new(Mutex::new(None));
    { let new_val = getenv(Arc::new(Mutex::new(Some("PWD".to_string())))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_val; };
    if { let __tmp_x = ((*dir.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*dir.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = stat_nolog(Arc::new(Mutex::new(Some(".".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *dot.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
        let (mut d, mut err) = stat_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && same_file(dot.clone(), d.clone()) {
        return ({ let __owned = dir.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
    }

        // If err is ENAMETOOLONG here, the syscall.Getwd below will
        // fail with the same error, too, but let's give it a try
        // anyway as the fallback code is much slower.
        // If the operating system provides a Getwd call, use it.
    if syscall::IMPLEMENTS_GETWD {
        { let (__tmp_0, __tmp_1) = ignoring_e_i_n_t_r2::<String>(Arc::new(Mutex::new(Some(Box::new(move || -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) { syscall::getwd() }) as Box<dyn FnMut() -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
                // Linux returns ENAMETOOLONG if the result is too long.
                // Some BSD systems appear to return EINVAL.
                // FreeBSD systems appear to use ENOMEM
                // Solaris appears to use ERANGE.
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_N_A_M_E_T_O_O_L_O_N_G as usize)).unwrap_or(false); !__matched } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_V_A_L as usize)).unwrap_or(false); !__matched } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (ERR_E_R_A_N_G_E as usize)).unwrap_or(false); !__matched } && { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (ERR_E_N_O_M_E_M as usize)).unwrap_or(false); !__matched } {
        return ({ let __owned = dir.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, new_syscall_error(Arc::new(Mutex::new(Some("getwd".to_string()))), err.clone()));
    }
    }

        // Linux returns ENAMETOOLONG if the result is too long.
        // Some BSD systems appear to return EINVAL.
        // FreeBSD systems appear to use ENOMEM
        // Solaris appears to use ERANGE.
        // We're trying to find our way back to ".".
    if { let __nil_result = (*dot.lock().unwrap()).is_none(); __nil_result } {
        { let (__tmp_0, __tmp_1) = stat_nolog(Arc::new(Mutex::new(Some(".".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *dot.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    }

        // Apply same kludge but to cached dir instead of $PWD.
    (*getwdCache.lock().unwrap().as_ref().unwrap()).mutex.lock();
    { let new_val = { let __selector_holder = (*getwdCache.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *dir.lock().unwrap() = Some(new_val); };
    (*getwdCache.lock().unwrap().as_ref().unwrap()).mutex.unlock();
    if { let __tmp_x = ((*dir.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut d, mut err) = stat_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && same_file(dot.clone(), d.clone()) {
        return ({ let __owned = dir.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
    }

        // Root is a special case because it has no parent
        // and ends in a slash.
    let (mut root, __tmp_1) = stat_nolog(Arc::new(Mutex::new(Some("/".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
                // Can't stat root - no hope.
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
        // Can't stat root - no hope.
    if same_file(root.clone(), dot.clone()) {
        return (Arc::new(Mutex::new(Some("/".to_string()))), Arc::new(Mutex::new(None)));
    }

        // General algorithm: find name in parent
        // and then find name of parent. Each iteration
        // adds /name to the beginning of dir.
    { let new_val = "".to_string(); *dir.lock().unwrap() = Some(new_val); };
    let mut parent = Arc::new(Mutex::new(Some("..".to_string())));
    loop {
        if { let __tmp_x = ((*parent.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1024; __tmp_x >= __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), new_syscall_error(Arc::new(Mutex::new(Some("getwd".to_string()))), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_A_M_E_T_O_O_L_O_N_G as usize))))) as Box<dyn StdError + Send + Sync>)))));
    }
        let (mut fd, mut err) = open_dir_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = parent.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }

        loop {
        let (mut names, mut err) = { let __recv = fd.clone(); let __recv_ptr: *const crate::types::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::types::File }; let __result = unsafe { &*__recv_ptr }.readdirnames(Arc::new(Mutex::new(Some(100)))); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = fd.clone(); let __recv_ptr: *const crate::types::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::types::File }; let __result = unsafe { &*__recv_ptr }.close(); __result };
                // Readdirnames can return io.EOF or other error.
                // In any case, we're here because syscall.Getwd
                // is not implemented or failed with ENAMETOOLONG,
                // so return the most sensible error.
        if syscall::IMPLEMENTS_GETWD {
        return (Arc::new(Mutex::new(Some("".to_string()))), new_syscall_error(Arc::new(Mutex::new(Some("getwd".to_string()))), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_N_A_M_E_T_O_O_L_O_N_G as usize))))) as Box<dyn StdError + Send + Sync>)))));
    }
        return (Arc::new(Mutex::new(Some("".to_string()))), new_syscall_error(Arc::new(Mutex::new(Some("getwd".to_string()))), Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(ERR_E_N_O_S_Y_S as usize))))) as Box<dyn StdError + Send + Sync>)))));
    }
                // Readdirnames can return io.EOF or other error.
                // In any case, we're here because syscall.Getwd
                // is not implemented or failed with ENAMETOOLONG,
                // so return the most sensible error.
        { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        let (mut d, _) = lstat_nolog(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*parent.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", name)); __s }))));
        if same_file(d.clone(), dot.clone()) {
        { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", name)); __s.push_str(&format!("{}", { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s }; *dir.lock().unwrap() = Some(new_val); };
        // TODO: unsupported goto found
    }
    } }
    }

                // Readdirnames can return io.EOF or other error.
                // In any case, we're here because syscall.Getwd
                // is not implemented or failed with ENAMETOOLONG,
                // so return the most sensible error.
        let (mut pd, __tmp_1) = { let __recv = fd.clone(); let __recv_ptr: *const crate::types::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::types::File }; let __result = unsafe { &*__recv_ptr }.stat(); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        { let __recv = fd.clone(); let __recv_ptr: *const crate::types::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::types::File }; let __result = unsafe { &*__recv_ptr }.close(); __result };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
        if same_file(pd.clone(), root.clone()) {
        break
    }

                // Set up for next round.
        { let __iface_handle = pd.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *dot.lock().unwrap() = __iface_value; };
        { let new_val = format!("{}{}", "../".to_string(), { let __v = (*parent.lock().unwrap().as_ref().unwrap()).clone(); __v }); *parent.lock().unwrap() = Some(new_val); };
    }

        // Sanity check
        // Readdirnames can return io.EOF or other error.
        // In any case, we're here because syscall.Getwd
        // is not implemented or failed with ENAMETOOLONG,
        // so return the most sensible error.
        // Set up for next round.
        // Save answer as hint to avoid the expensive path next time.
    (*getwdCache.lock().unwrap().as_ref().unwrap()).mutex.lock();
    { let new_val = dir.lock().unwrap().as_ref().unwrap().clone(); *(*getwdCache.lock().unwrap().as_ref().unwrap()).dir.lock().unwrap() = Some(new_val); };
    (*getwdCache.lock().unwrap().as_ref().unwrap()).mutex.unlock();

    return ({ let __owned = dir.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    unreachable!()
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub mutex: sync::mutex::Mutex,
    pub dir: Arc<Mutex<Option<String>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { mutex: self.mutex.clone(), dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}

impl AnonymousStruct1 {
    pub fn lock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.lock()
    }

    pub fn try_lock(&mut self) -> bool {
        let embedded_ref = &mut self.mutex;
        embedded_ref.try_lock()
    }

    pub fn unlock(&mut self) {
        let embedded_ref = &mut self.mutex;
        embedded_ref.unlock()
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { mutex: Default::default(), dir: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.dir.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type getwdCache = AnonymousStruct1;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
