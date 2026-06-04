use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_register_embedded_owner, go_strconv_format_float, go_strconv_format_int};

use crate::alias::*;
use crate::api::*;
use crate::api_predicates::*;
use crate::array::*;
use crate::assignments::*;
use crate::badlinkname::*;
use crate::basic::*;
use crate::builtins::*;
use crate::call::*;
use crate::chan::*;
use crate::check::*;
use crate::r#const::*;
use crate::context::*;
use crate::conversions::*;
use crate::decl::*;
use crate::errors::*;
use crate::errsupport::*;
use crate::eval::*;
use crate::expr::*;
use crate::exprstring::*;
use crate::format::*;
use crate::gccgosizes::*;
use crate::gcsizes::*;
use crate::index::*;
use crate::infer::*;
use crate::initorder::*;
use crate::instantiate::*;
use crate::interface::*;
use crate::iter::*;
use crate::labels::*;
use crate::literals::*;
use crate::lookup::*;
use crate::map::*;
use crate::methodset::*;
use crate::mono::*;
use crate::named::*;
use crate::object::*;
use crate::objset::*;
use crate::operand::*;
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
use crate::resolver::*;
use crate::r#return::*;
use crate::scope::*;
use crate::scope2::*;
use crate::selection::*;
use crate::signature::*;
use crate::sizes::*;
use crate::slice::*;
use crate::stmt::*;
use crate::r#struct::*;
use crate::subst::*;
use crate::termlist::*;
use crate::tuple::*;
use crate::r#type::*;
use crate::typelists::*;
use crate::typeparam::*;
use crate::typeset::*;
use crate::typestring::*;
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A goVersion is a Go language version string of the form "go1.%d"
/// where d is the minor version number. goVersion strings don't
/// contain release numbers ("go1.20.1" is not a valid goVersion).
#[derive(Debug, Clone, Default)]
pub struct goVersion(pub Arc<Mutex<Option<String>>>);

impl Display for goVersion {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for goVersion {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}


#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<ast_Ident>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { obj: self.obj.clone(), ptr: { let __guard = self.ptr.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: self.recv.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { obj: Arc::new(Mutex::new(None)), ptr: Arc::new(Mutex::new(Some(false))), recv: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.obj.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.ptr.lock().unwrap().as_ref().unwrap()), { let __guard = self.recv.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub name: Arc<Mutex<Option<String>>>,
    pub kind: Arc<Mutex<Option<BasicKind>>>,
    pub val: Arc<Mutex<Option<constant_Value>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: self.val.clone() }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(0))))))), val: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub name: Arc<Mutex<Option<String>>>,
    pub nargs: Arc<Mutex<Option<i32>>>,
    pub variadic: Arc<Mutex<Option<bool>>>,
    pub kind: Arc<Mutex<Option<exprKind>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nargs: { let __guard = self.nargs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, variadic: { let __guard = self.variadic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.nargs.lock().unwrap().as_ref().unwrap()), (*self.variadic.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static go1_9: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_13: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_14: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_17: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_18: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_20: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_21: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_22: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go1_23: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static go_current: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<goVersion>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *go1_9.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_13.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_14.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_17.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_18.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_20.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_21.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_22.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_23.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go_current.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_9.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.9".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_13.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.13".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_14.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.14".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_17.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.17".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_18.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.18".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_20.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.20".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_21.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.21".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_22.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.22".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go1_23.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.23".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
    *go_current.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some(format!("go1.{}", goversion::VERSION))))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *go1_9.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_13.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_14.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_17.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_18.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_20.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_21.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_22.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go1_23.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
    *go_current.lock().unwrap() = Some(goVersion(Arc::new(Mutex::new(Some(String::new())))));
}


pub(crate) fn __go_init_order_17() {
    *go1_9.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.9".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_18() {
    *go1_13.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.13".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_19() {
    *go1_14.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.14".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_20() {
    *go1_17.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.17".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_21() {
    *go1_18.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.18".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_22() {
    *go1_20.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.20".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_23() {
    *go1_21.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.21".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_24() {
    *go1_22.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.22".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_25() {
    *go1_23.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some("go1.23".to_string())))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_26() {
    *go_current.lock().unwrap() = Some((*as_go_version(Arc::new(Mutex::new(Some(format!("go1.{}", goversion::VERSION))))).lock().unwrap().as_ref().unwrap()).clone());
}


impl goVersion {
    /// isValid reports whether v is a valid Go version.
    pub fn is_valid(&self) -> bool {
        return { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y };
    }

    /// cmp returns -1, 0, or +1 depending on whether x < y, x == y, or x > y,
    /// interpreted as Go versions.
    pub fn cmp(&self, y: Arc<Mutex<Option<goVersion>>>) -> i32 {
        version::compare(Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()).to_string()))), Arc::new(Mutex::new(Some({ let __v = (*y.lock().unwrap().as_ref().unwrap()).clone(); __v }.to_string()))))
    }
}

impl crate::check::Checker {
    /// allowVersion reports whether the current effective Go version
    /// (which may vary from one file to another) is allowed to use the
    /// feature version (want).
    pub fn allow_version(&self, want: Arc<Mutex<Option<goVersion>>>) -> bool {
        !(*(*self.environment.lock().unwrap().as_ref().unwrap()).version.lock().unwrap().as_ref().unwrap()).is_valid() || { let __tmp_x = (*(*self.environment.lock().unwrap().as_ref().unwrap()).version.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some({ let __arg_holder = want.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y }
    }

    /// verifyVersionf is like allowVersion but also accepts a format string and arguments
    /// which are used to report a version error if allowVersion returns false.
    pub fn verify_versionf(&self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, v: Arc<Mutex<Option<goVersion>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> bool {
        if !self.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        self.version_errorf(at.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone());
        return false;
    }
        true
    }
}

/// asGoVersion returns v as a goVersion (e.g., "go1.20.1" becomes "go1.20").
/// If v is not a valid Go version, the result is the empty string.
pub fn as_go_version(v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<goVersion>>> {
    Arc::new(Mutex::new(Some(goVersion(Arc::new(Mutex::new(Some((*version::lang(v.clone()).lock().unwrap().as_ref().unwrap()).clone())))))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
