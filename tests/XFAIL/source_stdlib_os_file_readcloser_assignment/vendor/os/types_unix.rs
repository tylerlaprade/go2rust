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
use crate::stat_unix::*;
use crate::sticky_bsd::*;
use crate::sys::*;
use crate::sys_bsd::*;
use crate::sys_unix::*;
use crate::tempfile::*;
use crate::types::*;
use crate::wait_unimp::*;
use crate::zero_copy_posix::*;
use crate::zero_copy_stub::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A fileStat is the implementation of FileInfo returned by Stat and Lstat.
#[derive(Clone)]
pub struct fileStat {
    pub name: Arc<Mutex<Option<String>>>,
    pub size: Arc<Mutex<Option<i64>>>,
    pub mode: FileMode,
    pub mod_time: Arc<Mutex<Option<time::r#mod::Time>>>,
    pub sys: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Stat_t>>>,
}

impl fileStat {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, size: { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mod_time: { let __guard = self.mod_time.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sys: { let __guard = self.sys.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for fileStat {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), size: Arc::new(Mutex::new(Some(0))), mode: Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0))))))), mod_time: Arc::new(Mutex::new(Some(Default::default()))), sys: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for fileStat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.size.lock().unwrap().as_ref().unwrap()), (*self.mode.lock().unwrap().as_ref().unwrap()), (*self.mod_time.lock().unwrap().as_ref().unwrap()), (*self.sys.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for fileStat {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl fileStat {
    pub fn size(&self) -> i64 {
        return (*self.size.lock().unwrap().as_ref().unwrap());
    }

    pub fn mode(&self) -> Arc<Mutex<Option<io_fs::r#mod::FileMode>>> {
        return self.mode.clone();
    }

    pub fn mod_time(&self) -> Arc<Mutex<Option<time::r#mod::Time>>> {
        return self.mod_time.clone();
    }

    pub fn sys(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(self.sys.clone().clone()) as Box<dyn Any + Send + Sync>)))
    }
}

#[derive(Clone)]
pub struct fileStatPtr(pub Arc<Mutex<Option<fileStat>>>);

impl std::fmt::Display for fileStatPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io_fs::r#mod::FileInfo for fileStatPtr {
    fn is_dir(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::is_dir(__recv)
    }
    fn mod_time(&self) -> Arc<Mutex<Option<time::r#mod::Time>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::mod_time(__recv)
    }
    fn mode(&self) -> Arc<Mutex<Option<io_fs::r#mod::FileMode>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::mode(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::name(__recv)
    }
    fn size(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::size(__recv)
    }
    fn sys(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        fileStat::sys(__recv)
    }
    fn __go_clone_box_file_info(&self) -> Box<dyn io_fs::r#mod::FileInfo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io_fs::r#mod::FileInfo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_file_info(&self, other: &(dyn io_fs::r#mod::FileInfo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<fileStatPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn same_file_1(fs1: Arc<Mutex<Option<fileStat>>>, fs2: Arc<Mutex<Option<fileStat>>>) -> bool {
    return { let __tmp_x = (*(*(*fs1.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).dev.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*(*fs2.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).dev.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = (*(*(*fs1.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).ino.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*(*fs2.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).ino.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
}

impl GoValueClone for fileStat {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
