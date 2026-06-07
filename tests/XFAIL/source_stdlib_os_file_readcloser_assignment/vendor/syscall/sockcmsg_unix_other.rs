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
use crate::net::*;
use crate::rlimit::*;
use crate::rlimit_darwin::*;
use crate::route_bsd::*;
use crate::route_darwin::*;
use crate::sockcmsg_unix::*;
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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub r#type: Arc<Mutex<Option<u8>>>,
    pub nlen: Arc<Mutex<Option<u8>>>,
    pub alen: Arc<Mutex<Option<u8>>>,
    pub slen: Arc<Mutex<Option<u8>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nlen: { let __guard = self.nlen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, alen: { let __guard = self.alen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, slen: { let __guard = self.slen.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { r#type: Arc::new(Mutex::new(Some(0))), nlen: Arc::new(Mutex::new(Some(0))), alen: Arc::new(Mutex::new(Some(0))), slen: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.r#type.lock().unwrap().as_ref().unwrap()), (*self.nlen.lock().unwrap().as_ref().unwrap()), (*self.alen.lock().unwrap().as_ref().unwrap()), (*self.slen.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
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
        Ok(out)
    }
}
