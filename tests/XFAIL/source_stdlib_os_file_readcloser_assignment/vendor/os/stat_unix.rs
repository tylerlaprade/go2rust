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
use crate::stat_darwin::*;
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


impl crate::types::File {
    /// Stat returns the [FileInfo] structure describing file.
    /// If there is an error, it will be of type [*PathError].
    pub fn stat(&self) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), ErrInvalid.clone());
    }
        let mut fs: Arc<Mutex<Option<fileStat>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut err = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).fstat((*fs.lock().unwrap().as_ref().unwrap()).sys.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), self.wrap_err(Arc::new(Mutex::new(Some("stat".to_string()))), err.clone()));
    }
        fill_file_stat_from_sys(fs.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        return (Arc::new(Mutex::new(Some(Box::new((*fs.clone().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io_fs::r#mod::FileInfo + Send + Sync>))), Arc::new(Mutex::new(None)));
    }
}

/// statNolog stats a file with no test logging.
pub fn stat_nolog(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut fs: Arc<Mutex<Option<fileStat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let fs_closure_clone = fs.clone(); let name_closure_clone = name.clone(); let mut err = ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        return syscall::stat(Arc::new(Mutex::new(Some({ let __arg_holder = name_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*fs_closure_clone.lock().unwrap().as_ref().unwrap()).sys.clone());
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("stat".to_string()))), path: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: err.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
    fill_file_stat_from_sys(fs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return (Arc::new(Mutex::new(Some(Box::new((*fs.clone().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io_fs::r#mod::FileInfo + Send + Sync>))), Arc::new(Mutex::new(None)));
}

/// lstatNolog lstats a file with no test logging.
pub fn lstat_nolog(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut fs: Arc<Mutex<Option<fileStat>>> = Arc::new(Mutex::new(Some(Default::default())));
    let fs_closure_clone = fs.clone(); let name_closure_clone = name.clone(); let mut err = ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        return syscall::lstat(Arc::new(Mutex::new(Some({ let __arg_holder = name_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*fs_closure_clone.lock().unwrap().as_ref().unwrap()).sys.clone());
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("lstat".to_string()))), path: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: err.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }
    fill_file_stat_from_sys(fs.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return (Arc::new(Mutex::new(Some(Box::new((*fs.clone().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io_fs::r#mod::FileInfo + Send + Sync>))), Arc::new(Mutex::new(None)));
}