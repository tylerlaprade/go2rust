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
