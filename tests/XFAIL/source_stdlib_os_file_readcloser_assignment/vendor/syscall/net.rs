use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
use crate::zsyscall_darwin_arm64::*;
use crate::zsysnum_darwin_arm64::*;
use crate::ztypes_darwin_arm64::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// A RawConn is a raw network connection.
pub trait RawConn: std::fmt::Display + Any {
    fn __go_clone_box_raw_conn(&self) -> Box<dyn RawConn + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_raw_conn(&self, other: &(dyn RawConn + Send + Sync)) -> bool;
    fn control(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn read(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (bool) + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn write(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (bool) + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
}

impl Clone for Box<dyn RawConn + Send + Sync> {
    fn clone(&self) -> Self {
        RawConn::__go_clone_box_raw_conn(self.as_ref())
    }
}