use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::badlinkname_unix::*;
use crate::bpf_bsd::*;
use crate::dirent::*;
use crate::env_unix::*;
use crate::exec_unix::*;
use crate::flock_bsd::*;
use crate::forkpipe::*;
use crate::linkname_bsd::*;
use crate::linkname_darwin::*;
use crate::linkname_libc::*;
use crate::linkname_unix::*;
use crate::net::*;
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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct SysProcAttr {
    pub chroot: Arc<Mutex<Option<String>>>,
    pub credential: Arc<Mutex<Option<Credential>>>,
    pub ptrace: Arc<Mutex<Option<bool>>>,
    pub setsid: Arc<Mutex<Option<bool>>>,
    pub setpgid: Arc<Mutex<Option<bool>>>,
    pub setctty: Arc<Mutex<Option<bool>>>,
    pub noctty: Arc<Mutex<Option<bool>>>,
    pub ctty: Arc<Mutex<Option<i32>>>,
    pub foreground: Arc<Mutex<Option<bool>>>,
    pub pgid: Arc<Mutex<Option<i32>>>,
}

impl SysProcAttr {
    pub fn __go_value_clone(&self) -> Self {
        Self { chroot: { let __guard = self.chroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, credential: self.credential.clone(), ptrace: { let __guard = self.ptrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, setsid: { let __guard = self.setsid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, setpgid: { let __guard = self.setpgid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, setctty: { let __guard = self.setctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, noctty: { let __guard = self.noctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ctty: { let __guard = self.ctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, foreground: { let __guard = self.foreground.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pgid: { let __guard = self.pgid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for SysProcAttr {
    fn default() -> Self {
        Self { chroot: Arc::new(Mutex::new(Some(String::new()))), credential: Arc::new(Mutex::new(None)), ptrace: Arc::new(Mutex::new(Some(false))), setsid: Arc::new(Mutex::new(Some(false))), setpgid: Arc::new(Mutex::new(Some(false))), setctty: Arc::new(Mutex::new(Some(false))), noctty: Arc::new(Mutex::new(Some(false))), ctty: Arc::new(Mutex::new(Some(0))), foreground: Arc::new(Mutex::new(Some(false))), pgid: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for SysProcAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {}}}", (*self.chroot.lock().unwrap().as_ref().unwrap()), { let __guard = self.credential.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptrace.lock().unwrap().as_ref().unwrap()), (*self.setsid.lock().unwrap().as_ref().unwrap()), (*self.setpgid.lock().unwrap().as_ref().unwrap()), (*self.setctty.lock().unwrap().as_ref().unwrap()), (*self.noctty.lock().unwrap().as_ref().unwrap()), (*self.ctty.lock().unwrap().as_ref().unwrap()), (*self.foreground.lock().unwrap().as_ref().unwrap()), (*self.pgid.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for SysProcAttr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Chroot") {
            out.chroot = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ptrace") {
            out.ptrace = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Setsid") {
            out.setsid = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Setpgid") {
            out.setpgid = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Setctty") {
            out.setctty = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Noctty") {
            out.noctty = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Ctty") {
            out.ctty = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Foreground") {
            out.foreground = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Pgid") {
            out.pgid = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


impl GoValueClone for SysProcAttr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
