use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{error::{ErrInvalid, PathError}, file_posix::{ignoring_e_i_n_t_r}, stat_darwin::{fill_file_stat_from_sys}, types::{File, FileInfo}, types_unix::{fileStat}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

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