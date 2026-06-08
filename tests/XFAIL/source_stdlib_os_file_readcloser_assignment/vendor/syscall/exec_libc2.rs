use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{exec_unix::{Credential}};

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
        let __go_clone_0_0 = { let __guard = self.chroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.credential.clone();
        let __go_clone_2_0 = { let __guard = self.ptrace.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.setsid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.setpgid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.setctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.noctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = { let __guard = self.ctty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_8_0 = { let __guard = self.foreground.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_9_0 = { let __guard = self.pgid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            chroot: __go_clone_0_0,
            credential: __go_clone_1_0,
            ptrace: __go_clone_2_0,
            setsid: __go_clone_3_0,
            setpgid: __go_clone_4_0,
            setctty: __go_clone_5_0,
            noctty: __go_clone_6_0,
            ctty: __go_clone_7_0,
            foreground: __go_clone_8_0,
            pgid: __go_clone_9_0,
        }
    }
}


impl Default for SysProcAttr {
    fn default() -> Self {
        Self { chroot: Arc::new(Mutex::new(Some(String::new()))), credential: Arc::new(Mutex::new(None)), ptrace: Arc::new(Mutex::new(Some(false))), setsid: Arc::new(Mutex::new(Some(false))), setpgid: Arc::new(Mutex::new(Some(false))), setctty: Arc::new(Mutex::new(Some(false))), noctty: Arc::new(Mutex::new(Some(false))), ctty: Arc::new(Mutex::new(Some(0))), foreground: Arc::new(Mutex::new(Some(false))), pgid: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for SysProcAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.chroot.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.credential.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.ptrace.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.setsid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.setpgid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.setctty.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.noctty.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", (*self.ctty.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_8 = format!("{}", (*self.foreground.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_9 = format!("{}", (*self.pgid.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9)
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
