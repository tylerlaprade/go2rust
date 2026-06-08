use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use crate::{
    dir::{DirEntry, readdirMode},
    dir_darwin::{dirInfo},
    error::{ErrClosed, PathError},
    file_posix::{chmod_1},
    file_unix::{epipecheck, new_file, open_file_nolog},
    rawconn::{new_raw_conn, rawConn},
    stat::{lstat, stat},
    types::{File, FileInfo, FileMode, MODE_STICKY},
};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const O__R_D_O_N_L_Y: i32 = syscall::O__R_D_O_N_L_Y as i32;
pub const O__W_R_O_N_L_Y: i32 = syscall::O__W_R_O_N_L_Y as i32;
pub const O__R_D_W_R: i32 = syscall::O__R_D_W_R as i32;
pub const O__A_P_P_E_N_D: i32 = syscall::O__A_P_P_E_N_D as i32;
pub const O__C_R_E_A_T_E: i32 = syscall::O__C_R_E_A_T as i32;
pub const O__E_X_C_L: i32 = syscall::O__E_X_C_L as i32;
pub const O__S_Y_N_C: i32 = syscall::O__S_Y_N_C as i32;
pub const O__T_R_U_N_C: i32 = syscall::O__T_R_U_N_C as i32;


pub const S_E_E_K__S_E_T: i32 = 0;
pub const S_E_E_K__C_U_R: i32 = 1;
pub const S_E_E_K__E_N_D: i32 = 2;


/// LinkError records an error during a link or symlink or rename
/// system call and the paths that caused it.
#[derive(Clone)]
pub struct LinkError {
    pub op: Arc<Mutex<Option<String>>>,
    pub old: Arc<Mutex<Option<String>>>,
    pub new: Arc<Mutex<Option<String>>>,
    pub err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
}

impl LinkError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.op.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.old.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.new.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.err.clone();
        Self {
            op: __go_clone_0_0,
            old: __go_clone_1_0,
            new: __go_clone_2_0,
            err: __go_clone_3_0,
        }
    }
}


impl Default for LinkError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        Self {
            op: __go_default_0_0,
            old: __go_default_1_0,
            new: __go_default_2_0,
            err: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for LinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for LinkError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Op") {
            out.op = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Old") {
            out.old = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("New") {
            out.new = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// noReadFrom can be embedded alongside another type to
/// hide the ReadFrom method of that other type.
#[derive(Debug, Clone, Default)]
pub struct noReadFrom {
}

impl noReadFrom {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for noReadFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for noReadFrom {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// fileWithoutReadFrom implements all the methods of *File other
/// than ReadFrom. This is used to permit ReadFrom to call io.Copy
/// without leading to a recursive call to ReadFrom.
#[derive(Clone)]
pub struct fileWithoutReadFrom {
    pub no_read_from: Arc<Mutex<Option<noReadFrom>>>,
    pub file: Arc<Mutex<Option<File>>>,
}

impl fileWithoutReadFrom {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_read_from.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.file.clone();
        Self {
            no_read_from: __go_clone_0_0,
            file: __go_clone_1_0,
        }
    }
}


impl Default for fileWithoutReadFrom {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noReadFrom::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            no_read_from: __go_default_0_0,
            file: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for fileWithoutReadFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_read_from.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for fileWithoutReadFrom {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// noWriteTo can be embedded alongside another type to
/// hide the WriteTo method of that other type.
#[derive(Debug, Clone, Default)]
pub struct noWriteTo {
}

impl noWriteTo {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for noWriteTo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for noWriteTo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// fileWithoutWriteTo implements all the methods of *File other
/// than WriteTo. This is used to permit WriteTo to call io.Copy
/// without leading to a recursive call to WriteTo.
#[derive(Clone)]
pub struct fileWithoutWriteTo {
    pub no_write_to: Arc<Mutex<Option<noWriteTo>>>,
    pub file: Arc<Mutex<Option<File>>>,
}

impl fileWithoutWriteTo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.no_write_to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.file.clone();
        Self {
            no_write_to: __go_clone_0_0,
            file: __go_clone_1_0,
        }
    }
}


impl Default for fileWithoutWriteTo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(noWriteTo::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            no_write_to: __go_default_0_0,
            file: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for fileWithoutWriteTo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.no_write_to.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for fileWithoutWriteTo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static Stdin: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::types::File>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Stdout: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::types::File>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Stderr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::types::File>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errWriteAtInAppendMode: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errPathEscapes: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static lstat_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static checkWrapErr: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Stdin.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Stdout.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Stderr.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *errWriteAtInAppendMode.lock().unwrap() = None;
    *errPathEscapes.lock().unwrap() = None;
    *checkWrapErr.lock().unwrap() = Some(false);
    *Stdin.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stdin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stdin".to_string())))));
    *Stdout.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stdout.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stdout".to_string())))));
    *Stderr.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stderr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stderr".to_string())))));
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("os: invalid use of WriteAt on file opened with O_APPEND".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errWriteAtInAppendMode.lock().unwrap() = new_val; }
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("path escapes from parent".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errPathEscapes.lock().unwrap() = new_val; }
    *lstat_1.lock().unwrap() = Some(Box::new(lstat));
    *checkWrapErr.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *Stdin.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Stdout.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Stderr.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *errWriteAtInAppendMode.lock().unwrap() = None;
    *errPathEscapes.lock().unwrap() = None;
    *checkWrapErr.lock().unwrap() = Some(false);
}


pub(crate) fn __go_init_order_10() {
    *Stdin.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stdin.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stdin".to_string())))));
}


pub(crate) fn __go_init_order_11() {
    *Stdout.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stdout.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stdout".to_string())))));
}


pub(crate) fn __go_init_order_12() {
    *Stderr.lock().unwrap() = Some(new_file(Arc::new(Mutex::new(Some({ let __selector_holder = syscall::Stderr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("/dev/stderr".to_string())))));
}


pub(crate) fn __go_init_order_13() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("os: invalid use of WriteAt on file opened with O_APPEND".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errWriteAtInAppendMode.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_14() {
    { let __rhs_holder = errors::new(Arc::new(Mutex::new(Some("path escapes from parent".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errPathEscapes.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_15() {
    *lstat_1.lock().unwrap() = Some(Box::new(lstat));
}


pub(crate) fn __go_init_order_16() {
    *checkWrapErr.lock().unwrap() = Some(false);
}


impl crate::types::File {
    /// Name returns the name of the file as presented to Open.
    ///
    /// It is safe to call Name after [Close].
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return (*self.file.lock().unwrap().as_ref().unwrap()).name.clone();
    }

    /// Read reads up to len(b) bytes from the File and stores them in b.
    /// It returns the number of bytes read and any error encountered.
    /// At end of file, Read returns 0, io.EOF.
    pub fn read(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("read".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        let (__tmp_0, mut e) = self.read_1(b.clone()); *n.lock().unwrap() = Some(__tmp_0);;
        return (
            { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v },
            self.wrap_err(Arc::new(Mutex::new(Some("read".to_string()))), e.clone())
        );
    }

    /// ReadAt reads len(b) bytes from the File starting at byte offset off.
    /// It returns the number of bytes read and the error, if any.
    /// ReadAt always returns a non-nil error when n < len(b).
    /// At end of file, that error is io.EOF.
    pub fn read_at(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>, mut off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("read".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (
            0,
            Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("readat".to_string()))), path: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: errors::new(Arc::new(Mutex::new(Some("negative offset".to_string())))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
        );
    }
        while { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut m, mut e) = self.pread(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = self.wrap_err(Arc::new(Mutex::new(Some("read".to_string()))), e.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break
    }
        { let __rhs = m; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = b.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (m) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); b = new_val; };
        { let __rhs = (*Arc::new(Mutex::new(Some(m as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// ReadFrom implements io.ReaderFrom.
    pub fn read_from(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("write".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        let (__tmp_0, mut handled, mut e) = self.read_from_1(r.clone()); *n.lock().unwrap() = Some(__tmp_0);;
        if !handled {
        return generic_read_from(Arc::new(Mutex::new(Some(self.clone()))), r.clone());
    }
                // without wrapping
        return (
            { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v },
            self.wrap_err(Arc::new(Mutex::new(Some("write".to_string()))), e.clone())
        );
    }

    /// Write writes len(b) bytes from b to the File.
    /// It returns the number of bytes written and an error, if any.
    /// Write returns a non-nil error when n != len(b).
    pub fn write(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("write".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        let (__tmp_0, mut e) = self.write_1(b.clone()); *n.lock().unwrap() = Some(__tmp_0);;
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 0; *n.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        { let __rhs_holder = io::ErrShortWrite.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        epipecheck(Arc::new(Mutex::new(Some(self.clone()))), e.clone());
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = self.wrap_err(Arc::new(Mutex::new(Some("write".to_string()))), e.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// WriteAt writes len(b) bytes to the File starting at byte offset off.
    /// It returns the number of bytes written and an error, if any.
    /// WriteAt returns a non-nil error when n != len(b).
    ///
    /// If file was opened with the O_APPEND flag, WriteAt returns an error.
    pub fn write_at(&self, mut b: Arc<Mutex<Option<Vec<u8>>>>, mut off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("write".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        if (*(*self.file.lock().unwrap().as_ref().unwrap()).append_mode.clone().lock().unwrap().as_ref().unwrap()) {
        return (0, errWriteAtInAppendMode.clone());
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (
            0,
            Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("writeat".to_string()))), path: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: errors::new(Arc::new(Mutex::new(Some("negative offset".to_string())))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
        );
    }
        while { let __tmp_x = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut m, mut e) = self.pwrite(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = self.wrap_err(Arc::new(Mutex::new(Some("write".to_string()))), e.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break
    }
        { let __rhs = m; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = b.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = (m) as usize;
            let __high = __seq.len();
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); b = new_val; };
        { let __rhs = (*Arc::new(Mutex::new(Some(m as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// WriteTo implements io.WriterTo.
    pub fn write_to(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("read".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        let (__tmp_0, mut handled, mut e) = self.write_to_1(w.clone()); *n.lock().unwrap() = Some(__tmp_0);;
        if handled {
        return (
            { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v },
            self.wrap_err(Arc::new(Mutex::new(Some("read".to_string()))), e.clone())
        );
    }
        generic_write_to(Arc::new(Mutex::new(Some(self.clone()))), w.clone())
    }

    /// Seek sets the offset for the next Read or Write on file to offset, interpreted
    /// according to whence: 0 means relative to the origin of the file, 1 means
    /// relative to the current offset, and 2 means relative to the end.
    /// It returns the new offset and an error, if any.
    /// The behavior of Seek on a file opened with O_APPEND is not specified.
    pub fn seek(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ret: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("seek".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (0, err.clone());;
        }
    }
        let (mut r, mut e) = self.seek_1(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = whence.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } && !(*(*self.file.lock().unwrap().as_ref().unwrap()).dirinfo.lock().unwrap().as_mut().unwrap()).load().is_nil() && { let __tmp_x = r; let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_S_D_I_R as usize))))) as Box<dyn StdError + Send + Sync>; *e.lock().unwrap() = Some(new_val); };
    }
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return (
            0,
            self.wrap_err(Arc::new(Mutex::new(Some("seek".to_string()))), e.clone())
        );
    }
        (r, Arc::new(Mutex::new(None)))
    }

    /// WriteString is like Write, but writes the contents of string s rather than
    /// a slice of bytes.
    pub fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut b = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result };
        return self.write(b.clone());
    }

    /// wrapErr wraps an error that occurred during an operation on an open file.
    /// It passes io.EOF through unchanged, otherwise converts
    /// poll.ErrFileClosing to ErrClosed and wraps the error in a PathError.
    pub fn wrap_err(&self, op: Arc<Mutex<Option<String>>>, mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } || { let __left = err.clone(); let __right = io::EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return err.clone();
    }
        if { let __left = err.clone(); let __right = internal_poll::ErrFileClosing.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        { let __rhs_holder = ErrClosed.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    } else if (*checkWrapErr.lock().unwrap().as_ref().unwrap()) && errors::is(err.clone(), Arc::new(Mutex::new(Some((*internal_poll::ErrFileClosing.lock().unwrap().as_ref().unwrap()).clone())))) {
        std::panic::panic_any(Box::new(format!("{}{}", "unexpected error wrapping poll.ErrFileClosing: ".to_string(), (*Arc::new(Mutex::new(Some(format!("{}", err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap()))) as Box<dyn Any + Send + Sync>);
    }
        Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), path: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: err.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
    }

    /// Chmod changes the mode of the file to mode.
    /// If there is an error, it will be of type *PathError.
    pub fn chmod(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.chmod_1(Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// SetDeadline sets the read and write deadlines for a File.
    /// It is equivalent to calling both SetReadDeadline and SetWriteDeadline.
    ///
    /// Only some kinds of files support setting a deadline. Calls to SetDeadline
    /// for files that do not support deadlines will return ErrNoDeadline.
    /// On most systems ordinary files do not support deadlines, but pipes do.
    ///
    /// A deadline is an absolute time after which I/O operations fail with an
    /// error instead of blocking. The deadline applies to all future and pending
    /// I/O, not just the immediately following call to Read or Write.
    /// After a deadline has been exceeded, the connection can be refreshed
    /// by setting a deadline in the future.
    ///
    /// If the deadline is exceeded a call to Read or Write or to other I/O
    /// methods will return an error that wraps ErrDeadlineExceeded.
    /// This can be tested using errors.Is(err, os.ErrDeadlineExceeded).
    /// That error implements the Timeout method, and calling the Timeout
    /// method will return true, but there are other possible errors for which
    /// the Timeout will return true even if the deadline has not been exceeded.
    ///
    /// An idle timeout can be implemented by repeatedly extending
    /// the deadline after successful Read or Write calls.
    ///
    /// A zero value for t means I/O operations will not time out.
    pub fn set_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.set_deadline_1(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// SetReadDeadline sets the deadline for future Read calls and any
    /// currently-blocked Read call.
    /// A zero value for t means Read will not time out.
    /// Not all files support setting deadlines; see SetDeadline.
    pub fn set_read_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.set_read_deadline_1(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// SetWriteDeadline sets the deadline for any future Write calls and any
    /// currently-blocked Write call.
    /// Even if Write times out, it may return n > 0, indicating that
    /// some of the data was successfully written.
    /// A zero value for t means Write will not time out.
    /// Not all files support setting deadlines; see SetDeadline.
    pub fn set_write_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.set_write_deadline_1(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// SyscallConn returns a raw file.
    /// This implements the syscall.Conn interface.
    pub fn syscall_conn(&self) -> (Arc<Mutex<Option<Box<dyn syscall::net::RawConn + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("SyscallConn".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return (Arc::new(Mutex::new(None)), err.clone());;
        }
    }
        { let (__return_tmp_0, __return_tmp_1) = new_raw_conn(Arc::new(Mutex::new(Some(self.clone())))); let __return_slot_0 = Arc::new(Mutex::new(Some(Box::new((*__return_tmp_0.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn syscall::net::RawConn + Send + Sync>))); (__return_slot_0, __return_tmp_1) }
    }
}

impl LinkError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.op.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", " ".to_string()));
            __s.push_str(&format!("{}", (*self.old.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", " ".to_string()));
            __s.push_str(&format!("{}", (*self.new.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ": ".to_string()));
            __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", self.err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.err.clone()
    }
}

impl StdError for LinkError {}


impl noReadFrom {
    /// ReadFrom hides another ReadFrom method.
    /// It should never be called.
    pub fn read_from(&self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("can't happen".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

impl io::r#mod::ReaderFrom for noReadFrom {
    /// ReadFrom hides another ReadFrom method.
    /// It should never be called.
    fn read_from(&self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("can't happen".to_string()) as Box<dyn Any + Send + Sync>);
    }
    fn __go_clone_box_reader_from(&self) -> Box<dyn io::r#mod::ReaderFrom + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::ReaderFrom + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader_from(&self, other: &(dyn io::r#mod::ReaderFrom + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noReadFrom>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct noReadFromPtr(pub Arc<Mutex<Option<noReadFrom>>>);

impl std::fmt::Display for noReadFromPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::ReaderFrom for noReadFromPtr {
    fn read_from(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        noReadFrom::read_from(__recv, r)
    }
    fn __go_clone_box_reader_from(&self) -> Box<dyn io::r#mod::ReaderFrom + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::ReaderFrom + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader_from(&self, other: &(dyn io::r#mod::ReaderFrom + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noReadFromPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl noWriteTo {
    /// WriteTo hides another WriteTo method.
    /// It should never be called.
    pub fn write_to(&self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("can't happen".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

impl io::r#mod::WriterTo for noWriteTo {
    /// WriteTo hides another WriteTo method.
    /// It should never be called.
    fn write_to(&mut self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        std::panic::panic_any(Box::new("can't happen".to_string()) as Box<dyn Any + Send + Sync>);
    }
    fn __go_clone_box_writer_to(&self) -> Box<dyn io::r#mod::WriterTo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::WriterTo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer_to(&self, other: &(dyn io::r#mod::WriterTo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noWriteTo>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct noWriteToPtr(pub Arc<Mutex<Option<noWriteTo>>>);

impl std::fmt::Display for noWriteToPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::WriterTo for noWriteToPtr {
    fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        noWriteTo::write_to(__recv, w)
    }
    fn __go_clone_box_writer_to(&self) -> Box<dyn io::r#mod::WriterTo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::WriterTo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer_to(&self, other: &(dyn io::r#mod::WriterTo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<noWriteToPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl fileWithoutReadFrom {
    pub fn chdir(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chdir()
    }

    pub fn chmod(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chmod(mode)
    }

    pub fn chown(&self, uid: Arc<Mutex<Option<i32>>>, gid: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chown(uid, gid)
    }

    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.close()
    }

    pub fn fd(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.fd()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn read(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(b)
    }

    pub fn read_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_at(b, off)
    }

    pub fn read_dir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_dir(n)
    }

    pub fn read_from(&self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.no_read_from.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from(__arg0)
    }

    pub fn readdir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdir(n)
    }

    pub fn readdirnames(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdirnames(n)
    }

    pub fn seek(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.seek(offset, whence)
    }

    pub fn set_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_deadline(t)
    }

    pub fn set_read_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_read_deadline(t)
    }

    pub fn set_write_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_write_deadline(t)
    }

    pub fn stat(&self) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.stat()
    }

    pub fn sync(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.sync()
    }

    pub fn syscall_conn(&self) -> (Arc<Mutex<Option<Box<dyn syscall::net::RawConn + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.syscall_conn()
    }

    pub fn truncate(&self, size: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.truncate(size)
    }

    pub fn write(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write(b)
    }

    pub fn write_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_at(b, off)
    }

    pub fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_string(s)
    }

    pub fn write_to(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to(w)
    }

    pub fn check_valid(&self, op: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.check_valid(op)
    }

    pub fn chmod_1(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chmod_1(mode)
    }

    pub fn pread(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pread(b, off)
    }

    pub fn pwrite(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pwrite(b, off)
    }

    pub fn read_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_1(b)
    }

    pub fn read_from_1(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from_1(r)
    }

    pub fn readdir_1(&self, n: Arc<Mutex<Option<i32>>>, mode: Arc<Mutex<Option<readdirMode>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdir_1(n, mode)
    }

    pub fn seek_1(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.seek_1(offset, whence)
    }

    pub fn set_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_deadline_1(t)
    }

    pub fn set_read_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_read_deadline_1(t)
    }

    pub fn set_write_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_write_deadline_1(t)
    }

    pub fn wrap_err(&self, op: Arc<Mutex<Option<String>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.wrap_err(op, err)
    }

    pub fn write_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_1(b)
    }

    pub fn write_to_1(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to_1(w)
    }
}

impl fileWithoutWriteTo {
    pub fn chdir(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chdir()
    }

    pub fn chmod(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chmod(mode)
    }

    pub fn chown(&self, uid: Arc<Mutex<Option<i32>>>, gid: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chown(uid, gid)
    }

    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.close()
    }

    pub fn fd(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.fd()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn read(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(b)
    }

    pub fn read_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_at(b, off)
    }

    pub fn read_dir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_dir(n)
    }

    pub fn read_from(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from(r)
    }

    pub fn readdir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdir(n)
    }

    pub fn readdirnames(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdirnames(n)
    }

    pub fn seek(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.seek(offset, whence)
    }

    pub fn set_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_deadline(t)
    }

    pub fn set_read_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_read_deadline(t)
    }

    pub fn set_write_deadline(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_write_deadline(t)
    }

    pub fn stat(&self) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.stat()
    }

    pub fn sync(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.sync()
    }

    pub fn syscall_conn(&self) -> (Arc<Mutex<Option<Box<dyn syscall::net::RawConn + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.syscall_conn()
    }

    pub fn truncate(&self, size: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.truncate(size)
    }

    pub fn write(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write(b)
    }

    pub fn write_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_at(b, off)
    }

    pub fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_string(s)
    }

    pub fn write_to(&self, __arg0: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.no_write_to.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to(__arg0)
    }

    pub fn check_valid(&self, op: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.check_valid(op)
    }

    pub fn chmod_1(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chmod_1(mode)
    }

    pub fn pread(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pread(b, off)
    }

    pub fn pwrite(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pwrite(b, off)
    }

    pub fn read_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_1(b)
    }

    pub fn read_from_1(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from_1(r)
    }

    pub fn readdir_1(&self, n: Arc<Mutex<Option<i32>>>, mode: Arc<Mutex<Option<readdirMode>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.readdir_1(n, mode)
    }

    pub fn seek_1(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.seek_1(offset, whence)
    }

    pub fn set_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_deadline_1(t)
    }

    pub fn set_read_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_read_deadline_1(t)
    }

    pub fn set_write_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.set_write_deadline_1(t)
    }

    pub fn wrap_err(&self, op: Arc<Mutex<Option<String>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.wrap_err(op, err)
    }

    pub fn write_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_1(b)
    }

    pub fn write_to_1(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.file.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to_1(w)
    }
}

pub fn generic_read_from(f: Arc<Mutex<Option<File>>>, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    io::copy(Arc::new(Mutex::new(Some(Box::new(fileWithoutReadFrom { file: f.clone(), no_read_from: Arc::new(Mutex::new(Some(noReadFrom::default()))) }) as Box<dyn io::r#mod::Writer + Send + Sync>))), r.clone())
}

pub fn generic_write_to(f: Arc<Mutex<Option<File>>>, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    io::copy(w.clone(), Arc::new(Mutex::new(Some(Box::new(fileWithoutWriteTo { file: f.clone(), no_write_to: Arc::new(Mutex::new(Some(noWriteTo::default()))) }) as Box<dyn io::r#mod::Reader + Send + Sync>))))
}

/// setStickyBit adds ModeSticky to the permission bits of path, non atomic.
pub fn set_sticky_bit(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let (mut fi, mut err) = stat(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return err.clone();
    }
    return chmod(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = (*(*fi.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_STICKY as u32)))); __tmp_x | __tmp_y }))));
}

/// Open opens the named file for reading. If successful, methods on
/// the returned file can be used for reading; the associated file
/// descriptor has mode O_RDONLY.
/// If there is an error, it will be of type *PathError.
pub fn open(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::types::File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    open_file(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32))))))))
}

/// OpenFile is the generalized open call; most users will use Open
/// or Create instead. It opens the named file with specified flag
/// (O_RDONLY etc.). If the file does not exist, and the O_CREATE flag
/// is passed, it is created with mode perm (before umask);
/// the containing directory must exist. If successful,
/// methods on the returned File can be used for I/O.
/// If there is an error, it will be of type *PathError.
pub fn open_file(name: Arc<Mutex<Option<String>>>, flag: Arc<Mutex<Option<i32>>>, perm: FileMode) -> (Arc<Mutex<Option<crate::types::File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    internal_testlog::open(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut f, mut err) = open_file_nolog(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = flag.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = perm.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }
    { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*flag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y }; *(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).append_mode.lock().unwrap() = Some(new_val); };

    return (f.clone(), Arc::new(Mutex::new(None)));
}

/// Chmod changes the mode of the named file to mode.
/// If the file is a symbolic link, it changes the mode of the link's target.
/// If there is an error, it will be of type *PathError.
///
/// A different subset of the mode bits are used, depending on the
/// operating system.
///
/// On Unix, the mode's permission bits, ModeSetuid, ModeSetgid, and
/// ModeSticky are used.
///
/// On Windows, only the 0o200 bit (owner writable) of mode is used; it
/// controls whether the file's read-only attribute is set or cleared.
/// The other bits are currently unused. For compatibility with Go 1.12
/// and earlier, use a non-zero mode. Use mode 0o400 for a read-only
/// file and 0o600 for a readable+writable file.
///
/// On Plan 9, the mode's permission bits, ModeAppend, ModeExclusive,
/// and ModeTemporary are used.
pub fn chmod(name: Arc<Mutex<Option<String>>>, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    chmod_1(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for LinkError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for noReadFrom {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for fileWithoutReadFrom {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for noWriteTo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for fileWithoutWriteTo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
