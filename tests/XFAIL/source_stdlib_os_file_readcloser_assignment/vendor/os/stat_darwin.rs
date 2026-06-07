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
use crate::getwd::*;
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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


pub fn fill_file_stat_from_sys(fs: Arc<Mutex<Option<fileStat>>>, name: Arc<Mutex<Option<String>>>) {
    { let new_val = internal_filepathlite::base(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*fs.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = __moved_val; };
    { let new_val = { let __selector_holder = (*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*fs.lock().unwrap().as_ref().unwrap()).size.lock().unwrap() = Some(new_val); };
    { let new_val = time::unix((*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mtimespec.lock().unwrap().as_ref().unwrap()).unix()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*fs.lock().unwrap().as_ref().unwrap()).mod_time.lock().unwrap() = __moved_val; };
    { let new_val = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0777 as u16; __tmp_x & __tmp_y } as u32)))); *(*fs.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    { let _switch_val = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_F_M_T as u16; __tmp_x & __tmp_y };
    if _switch_val == (syscall::S__I_F_B_L_K as u16) || _switch_val == (syscall::S__I_F_W_H_T as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DEVICE as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_C_H_R as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some((MODE_DEVICE as u32 | MODE_CHAR_DEVICE as u32) as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_D_I_R as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DIR as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_I_F_O as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_NAMED_PIPE as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_L_N_K as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SYMLINK as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_R_E_G as u16) {
        } else if _switch_val == (syscall::S__I_F_S_O_C_K as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SOCKET as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        }
    }
        // nothing to do
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_G_I_D as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETGID as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_U_I_D as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETUID as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_V_T_X as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_STICKY as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
}