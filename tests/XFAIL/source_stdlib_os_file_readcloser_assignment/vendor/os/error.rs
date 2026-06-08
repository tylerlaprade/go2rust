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

use crate::{error_errno::{syscallErrorType}, file::{LinkError}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub trait timeout: std::fmt::Display + Any {
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool;
    fn timeout(&self) -> bool;
}

impl Clone for Box<dyn timeout + Send + Sync> {
    fn clone(&self) -> Self {
        timeout::__go_clone_box_timeout(self.as_ref())
    }
}

/// PathError records an error and the operation and file path that caused it.
pub type PathError = Arc<Mutex<Option<io_fs::r#mod::PathError>>>;


/// SyscallError records an error from a specific system call.
#[derive(Clone)]
pub struct SyscallError {
    pub syscall: Arc<Mutex<Option<String>>>,
    pub err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
}

impl SyscallError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.syscall.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.err.clone();
        Self {
            syscall: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}


impl Default for SyscallError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            syscall: __go_default_0_0,
            err: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for SyscallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for SyscallError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}


pub static ErrInvalid: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrPermission: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNotExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrClosed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNoDeadline: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrDeadlineExceeded: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrInvalid.lock().unwrap() = None;
    *ErrPermission.lock().unwrap() = None;
    *ErrExist.lock().unwrap() = None;
    *ErrNotExist.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
    *ErrNoDeadline.lock().unwrap() = None;
    *ErrDeadlineExceeded.lock().unwrap() = None;
    { let __rhs_holder = io_fs::ErrInvalid.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrInvalid.lock().unwrap() = new_val; }
    { let __rhs_holder = io_fs::ErrPermission.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrPermission.lock().unwrap() = new_val; }
    { let __rhs_holder = io_fs::ErrExist.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrExist.lock().unwrap() = new_val; }
    { let __rhs_holder = io_fs::ErrNotExist.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotExist.lock().unwrap() = new_val; }
    { let __rhs_holder = io_fs::ErrClosed.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosed.lock().unwrap() = new_val; }
    { let __rhs_holder = err_no_deadline().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoDeadline.lock().unwrap() = new_val; }
    { let __rhs_holder = err_deadline_exceeded().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrDeadlineExceeded.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrInvalid.lock().unwrap() = None;
    *ErrPermission.lock().unwrap() = None;
    *ErrExist.lock().unwrap() = None;
    *ErrNotExist.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
    *ErrNoDeadline.lock().unwrap() = None;
    *ErrDeadlineExceeded.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = io_fs::ErrInvalid.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrInvalid.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = io_fs::ErrPermission.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrPermission.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = io_fs::ErrExist.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrExist.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    { let __rhs_holder = io_fs::ErrNotExist.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotExist.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = io_fs::ErrClosed.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosed.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = err_no_deadline().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoDeadline.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_6() {
    { let __rhs_holder = err_deadline_exceeded().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrDeadlineExceeded.lock().unwrap() = new_val; }
}


impl SyscallError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.syscall.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ": ".to_string()));
            __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", self.err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.err.clone()
    }

    /// Timeout reports whether this error represents a timeout.
    pub fn timeout(&self) -> bool {
        let (mut t, mut ok) = ({
        let val = self.err.clone().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<SyscallError>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn timeout + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<internal_poll::fd::DeadlineExceededError>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn timeout + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<internal_poll::fd::errNetClosing>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn timeout + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<io_fs::r#mod::PathError>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn timeout + Send + Sync>))), true)
            } else if let Some(typed_val) = any_val.downcast_ref::<syscall::syscall_unix::Errno>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn timeout + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn timeout + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn timeout + Send + Sync>>)), false)
        }
    });
        return ok && (*t.lock().unwrap().as_ref().unwrap()).timeout();
    }
}

impl StdError for SyscallError {}


impl timeout for SyscallError {
    fn timeout(&self) -> bool {
        SyscallError::timeout(self)
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SyscallError>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct SyscallErrorPtr(pub Arc<Mutex<Option<SyscallError>>>);

impl std::fmt::Display for SyscallErrorPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl timeout for SyscallErrorPtr {
    fn timeout(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        SyscallError::timeout(__recv)
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<SyscallErrorPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl timeout for internal_poll::fd::DeadlineExceededError {
    fn timeout(&self) -> bool {
        self.timeout()
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<internal_poll::fd::DeadlineExceededError>() {
            false
        } else {
            false
        }
    }
}

impl timeout for internal_poll::fd::errNetClosing {
    fn timeout(&self) -> bool {
        self.timeout()
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<internal_poll::fd::errNetClosing>() {
            false
        } else {
            false
        }
    }
}

impl timeout for io_fs::r#mod::PathError {
    fn timeout(&self) -> bool {
        self.timeout()
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<io_fs::r#mod::PathError>() {
            false
        } else {
            false
        }
    }
}

impl timeout for syscall::syscall_unix::Errno {
    fn timeout(&self) -> bool {
        self.timeout()
    }
    fn __go_clone_box_timeout(&self) -> Box<dyn timeout + Send + Sync> {
        Box::new(self.clone()) as Box<dyn timeout + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_timeout(&self, other: &(dyn timeout + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<syscall::syscall_unix::Errno>() {
            false
        } else {
            false
        }
    }
}

pub fn err_no_deadline() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_poll::ErrNoDeadline.clone()
}

/// errDeadlineExceeded returns the value for os.ErrDeadlineExceeded.
/// This error comes from the internal/poll package, which is also
/// used by package net. Doing it this way ensures that the net
/// package will return os.ErrDeadlineExceeded for an exceeded deadline,
/// as documented by net.Conn.SetDeadline, without requiring any extra
/// work in the net package and without requiring the internal/poll
/// package to import os (which it can't, because that would be circular).
pub fn err_deadline_exceeded() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_poll::ErrDeadlineExceeded.clone()
}

/// NewSyscallError returns, as an error, a new [SyscallError]
/// with the given system call name and error details.
/// As a convenience, if err is nil, NewSyscallError returns nil.
pub fn new_syscall_error(syscall: Arc<Mutex<Option<String>>>, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some(Box::new(SyscallError { syscall: Arc::new(Mutex::new(Some({ let __arg_holder = syscall.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: err.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
}

/// IsNotExist returns a boolean indicating whether its argument is known to
/// report that a file or directory does not exist. It is satisfied by
/// [ErrNotExist] as well as some syscall errors.
///
/// This function predates [errors.Is]. It only supports errors returned by
/// the os package. New code should use errors.Is(err, fs.ErrNotExist).
pub fn is_not_exist(err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> bool {
    underlying_error_is(err.clone(), ErrNotExist.clone())
}

pub fn underlying_error_is(mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>, target: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> bool {
        // Note that this function is not errors.Is:
        // underlyingError only unwraps the specific error-wrapping types
        // that it historically did, not all errors implementing Unwrap().
    { let __rhs_holder = underlying_error(err.clone()).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    if { let __left = err.clone(); let __right = target.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return true;
    }

        // To preserve prior behavior, only examine syscall errors.
    let (mut e, mut ok) = ({
        let val = err.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<syscall::syscall_unix::Errno>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });
    return ok && syscall::syscall_unix::Errno::is(&(*e.lock().unwrap().as_ref().unwrap()), target.clone());
}

/// underlyingError returns the underlying error for known os error types.
pub fn underlying_error(mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    {
    let _ts_subject = err.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val = _ts_guard.as_ref();
    if _ts_val.and_then(|__v| __v.downcast_ref::<io_fs::r#mod::PathError>()).is_some() {
        let err = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<io_fs::r#mod::PathError>()).unwrap().clone())));
        drop(_ts_guard);
        return (*err.lock().unwrap().as_ref().unwrap()).err.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::file::LinkError>()).is_some() {
        let err = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::file::LinkError>()).unwrap().clone())));
        drop(_ts_guard);
        return (*err.lock().unwrap().as_ref().unwrap()).err.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<SyscallError>()).is_some() {
        let err = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<SyscallError>()).unwrap().clone())));
        drop(_ts_guard);
        return (*err.lock().unwrap().as_ref().unwrap()).err.clone();;
    }
    }
    err.clone()
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for SyscallError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
