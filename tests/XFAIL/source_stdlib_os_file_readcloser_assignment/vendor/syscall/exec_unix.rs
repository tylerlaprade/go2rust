use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{exec_libc2::{SysProcAttr}, syscall_unix::{Errno}, zerrors_darwin_arm64::{F_D__C_L_O_E_X_E_C, F__G_E_T_F_L, F__S_E_T_F_D, F__S_E_T_F_L, O__N_O_N_B_L_O_C_K}, zsyscall_darwin_arm64::{fcntl}};

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Credential holds user and group identities to be assumed
/// by a child process started by [StartProcess].
#[derive(Debug, Clone)]
pub struct Credential {
    pub uid: Arc<Mutex<Option<u32>>>,
    pub gid: Arc<Mutex<Option<u32>>>,
    pub groups: Arc<Mutex<Option<Vec<u32>>>>,
    pub no_set_groups: Arc<Mutex<Option<bool>>>,
}

impl Credential {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.uid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.gid.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.groups.clone();
        let __go_clone_3_0 = { let __guard = self.no_set_groups.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            uid: __go_clone_0_0,
            gid: __go_clone_1_0,
            groups: __go_clone_2_0,
            no_set_groups: __go_clone_3_0,
        }
    }
}


impl Default for Credential {
    fn default() -> Self {
        Self { uid: Arc::new(Mutex::new(Some(0))), gid: Arc::new(Mutex::new(Some(0))), groups: Arc::new(Mutex::new(None)), no_set_groups: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Credential {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.uid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.gid.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.groups));
        let __go_fmt_3 = format!("{}", (*self.no_set_groups.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for Credential {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Uid") {
            out.uid = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Gid") {
            out.gid = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Groups") {
            out.groups = <Arc<Mutex<Option<Vec<u32>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("NoSetGroups") {
            out.no_set_groups = <Arc<Mutex<Option<bool>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// ProcAttr holds attributes that will be applied to a new process started
/// by [StartProcess].
#[derive(Debug, Clone)]
pub struct ProcAttr {
    pub dir: Arc<Mutex<Option<String>>>,
    pub env: Arc<Mutex<Option<Vec<String>>>>,
    pub files: Arc<Mutex<Option<Vec<usize>>>>,
    pub sys: Arc<Mutex<Option<SysProcAttr>>>,
}

impl ProcAttr {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.env.clone();
        let __go_clone_2_0 = self.files.clone();
        let __go_clone_3_0 = self.sys.clone();
        Self {
            dir: __go_clone_0_0,
            env: __go_clone_1_0,
            files: __go_clone_2_0,
            sys: __go_clone_3_0,
        }
    }
}


impl Default for ProcAttr {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(String::new()))), env: Arc::new(Mutex::new(None)), files: Arc::new(Mutex::new(None)), sys: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for ProcAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.dir.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.env));
        let __go_fmt_2 = format!("{}", format_slice(&self.files));
        let __go_fmt_3 = format!("{}", { let __guard = self.sys.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for ProcAttr {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Dir") {
            out.dir = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Env") {
            out.env = <Arc<Mutex<Option<Vec<String>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Files") {
            out.files = <Arc<Mutex<Option<Vec<usize>>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static ForkLock: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::rwmutex::RWMutex>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zeroProcAttr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<ProcAttr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static zeroSysProcAttr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::exec_libc2::SysProcAttr>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static execveLibc: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<crate::syscall_unix::Errno>>> + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static execveDarwin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static execveOpenBSD: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>, Arc<Mutex<Option<Arc<Mutex<Option<u8>>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ForkLock.lock().unwrap() = Some(Default::default());
    *zeroProcAttr.lock().unwrap() = Some(Default::default());
    *zeroSysProcAttr.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *ForkLock.lock().unwrap() = Some(Default::default());
    *zeroProcAttr.lock().unwrap() = Some(Default::default());
    *zeroSysProcAttr.lock().unwrap() = Some(Default::default());
}


pub fn close_on_exec(fd: Arc<Mutex<Option<i32>>>) {
    fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(1))));
}

pub fn set_nonblock(fd: Arc<Mutex<Option<i32>>>, nonblocking: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    let (mut flag, __tmp_1) = fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(0)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    if { let __tmp_x = ({ let __tmp_x = { let __tmp_x = flag; let __tmp_y = 4; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y }); let __tmp_y = { let __v = (*nonblocking.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    if { let __v = (*nonblocking.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __rhs = 4; flag = flag | __rhs; };
    } else {
        { let __rhs = 4; flag = flag & ! __rhs; };
    }
    { let (__tmp_0, __tmp_1) = fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(flag)))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    err.clone()
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Credential {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ProcAttr {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
