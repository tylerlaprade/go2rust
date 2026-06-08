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
use crate::types_unix::*;
use crate::wait_unimp::*;
use crate::zero_copy_posix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::types::File {
    pub fn write_to_1(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut written: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut handled: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        (0, false, Arc::new(Mutex::new(None)))
    }

    pub fn read_from_1(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut handled: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        (0, false, Arc::new(Mutex::new(None)))
    }
}