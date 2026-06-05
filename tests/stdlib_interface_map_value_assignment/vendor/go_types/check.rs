use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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
use crate::version::*;

use internal_types_errors::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const DEBUG: bool = false;


/// exprInfo stores information about an untyped expression.
#[derive(Clone)]
pub struct exprInfo {
    pub is_lhs: Arc<Mutex<Option<bool>>>,
    pub mode: Arc<Mutex<Option<operandMode>>>,
    pub typ: Arc<Mutex<Option<Basic>>>,
    pub val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>,
}

impl exprInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { is_lhs: { let __guard = self.is_lhs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone(), val: self.val.clone() }
    }
}


impl Default for exprInfo {
    fn default() -> Self {
        Self { is_lhs: Arc::new(Mutex::new(Some(false))), mode: Arc::new(Mutex::new(Some(crate::operand::operandMode(Arc::new(Mutex::new(Some(0))))))), typ: Arc::new(Mutex::new(None)), val: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for exprInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.is_lhs.lock().unwrap().as_ref().unwrap()), (*self.mode.lock().unwrap().as_ref().unwrap()), { let __guard = self.typ.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for exprInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An environment represents the environment within which an object is
/// type-checked.
#[derive(Clone)]
pub struct environment {
    pub decl: Arc<Mutex<Option<declInfo>>>,
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub version: Arc<Mutex<Option<goVersion>>>,
    pub iota: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>,
    pub errpos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>,
    pub in_t_param_list: Arc<Mutex<Option<bool>>>,
    pub sig: Arc<Mutex<Option<Signature>>>,
    pub is_panic: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::CallExpr>, Arc<Mutex<Option<bool>>>>>>>,
    pub has_label: Arc<Mutex<Option<bool>>>,
    pub has_call_or_recv: Arc<Mutex<Option<bool>>>,
    pub expr_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl environment {
    pub fn __go_value_clone(&self) -> Self {
        Self { decl: self.decl.clone(), scope: self.scope.clone(), version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, iota: self.iota.clone(), errpos: self.errpos.clone(), in_t_param_list: { let __guard = self.in_t_param_list.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, sig: self.sig.clone(), is_panic: self.is_panic.clone(), has_label: { let __guard = self.has_label.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_call_or_recv: { let __guard = self.has_call_or_recv.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, expr_pos: { let __guard = self.expr_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for environment {
    fn default() -> Self {
        Self { decl: Arc::new(Mutex::new(None)), scope: Arc::new(Mutex::new(None)), version: Arc::new(Mutex::new(Some(crate::version::goVersion(Arc::new(Mutex::new(Some(String::new()))))))), iota: Arc::new(Mutex::new(None)), errpos: Arc::new(Mutex::new(None)), in_t_param_list: Arc::new(Mutex::new(Some(false))), sig: Arc::new(Mutex::new(None)), is_panic: Arc::new(Mutex::new(None)), has_label: Arc::new(Mutex::new(Some(false))), has_call_or_recv: Arc::new(Mutex::new(Some(false))), expr_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for environment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.decl.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.version.lock().unwrap().as_ref().unwrap()), (*self.iota.lock().unwrap().as_ref().unwrap()), (*self.errpos.lock().unwrap().as_ref().unwrap()), (*self.in_t_param_list.lock().unwrap().as_ref().unwrap()), { let __guard = self.sig.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_map(&self.is_panic), (*self.has_label.lock().unwrap().as_ref().unwrap()), (*self.has_call_or_recv.lock().unwrap().as_ref().unwrap()), (*self.expr_pos.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for environment {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An importKey identifies an imported package by import path and source directory
/// (directory containing the file containing the import). In practice, the directory
/// may always be the same, or may not matter. Given an (import path, directory), an
/// importer must always return the same package (but given two different import paths,
/// an importer may still return the same package by mapping them to the same package
/// paths).
#[derive(Debug, Clone)]
pub struct importKey {
    pub path: Arc<Mutex<Option<String>>>,
    pub dir: Arc<Mutex<Option<String>>>,
}

impl importKey {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for importKey {
    fn default() -> Self {
        Self { path: Arc::new(Mutex::new(Some(String::new()))), dir: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for importKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.path.lock().unwrap().as_ref().unwrap()), (*self.dir.lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for importKey {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = self.path.lock().unwrap(); let __right = other.path.lock().unwrap(); __left.as_ref() == __right.as_ref() }
                && { let __left = self.dir.lock().unwrap(); let __right = other.dir.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl Eq for importKey {}

impl PartialOrd for importKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for importKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            let __left = { self.path.lock().unwrap().as_ref().cloned() };
            let __right = { other.path.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        {
            let __left = { self.dir.lock().unwrap().as_ref().cloned() };
            let __right = { other.dir.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl GoJsonDecode for importKey {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A dotImportKey describes a dot-imported object in the given scope.
#[derive(Clone)]
pub struct dotImportKey {
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl dotImportKey {
    pub fn __go_value_clone(&self) -> Self {
        Self { scope: self.scope.clone(), name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for dotImportKey {
    fn default() -> Self {
        Self { scope: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for dotImportKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", { let __guard = self.scope.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for dotImportKey {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left_some = self.scope.lock().unwrap().is_some(); let __right_some = other.scope.lock().unwrap().is_some(); (!__left_some && !__right_some) || (__left_some && __right_some && Arc::ptr_eq(&self.scope, &other.scope)) }
                && { let __left = self.name.lock().unwrap(); let __right = other.name.lock().unwrap(); __left.as_ref() == __right.as_ref() }
        )
    }
}

impl Eq for dotImportKey {}

impl PartialOrd for dotImportKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for dotImportKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        {
            let __left_some = self.scope.lock().unwrap().is_some();
            let __right_some = other.scope.lock().unwrap().is_some();
            let __ord = match (__left_some, __right_some) {
                (false, false) => std::cmp::Ordering::Equal,
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                (true, true) => (Arc::as_ptr(&self.scope) as usize).cmp(&(Arc::as_ptr(&other.scope) as usize)),
            };
            match __ord {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        {
            let __left = { self.name.lock().unwrap().as_ref().cloned() };
            let __right = { other.name.lock().unwrap().as_ref().cloned() };
            match __left.cmp(&__right) {
                std::cmp::Ordering::Equal => {}
                __ord => return __ord,
            }
        }
        std::cmp::Ordering::Equal
    }
}

impl GoJsonDecode for dotImportKey {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An action describes a (delayed) action.
#[derive(Clone)]
pub struct action {
    pub version: Arc<Mutex<Option<goVersion>>>,
    pub f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>,
    pub desc: Arc<Mutex<Option<actionDesc>>>,
}

impl action {
    pub fn __go_value_clone(&self) -> Self {
        Self { version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, f: self.f.clone(), desc: self.desc.clone() }
    }
}


impl Default for action {
    fn default() -> Self {
        Self { version: Arc::new(Mutex::new(Some(crate::version::goVersion(Arc::new(Mutex::new(Some(String::new()))))))), f: Arc::new(Mutex::new(None)), desc: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.version.lock().unwrap().as_ref().unwrap()), "<func>", { let __guard = self.desc.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for action {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An actionDesc provides information on an action.
/// For debugging only.
#[derive(Clone)]
pub struct actionDesc {
    pub pos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>,
    pub format: Arc<Mutex<Option<String>>>,
    pub args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>,
}

impl actionDesc {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: self.pos.clone(), format: { let __guard = self.format.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, args: self.args.clone() }
    }
}


impl Default for actionDesc {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(None)), format: Arc::new(Mutex::new(Some(String::new()))), args: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for actionDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.format.lock().unwrap().as_ref().unwrap()), format_any_slice(&self.args))
    }
}

impl GoJsonDecode for actionDesc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Checker maintains the state of the type checker.
/// It must be created with [NewChecker].
#[derive(Clone)]
pub struct Checker {
    pub conf: Arc<Mutex<Option<Config>>>,
    pub ctxt: Arc<Mutex<Option<Context>>>,
    pub fset: Arc<Mutex<Option<go_token::position::FileSet>>>,
    pub pkg: Arc<Mutex<Option<Package>>>,
    pub info: Arc<Mutex<Option<Info>>>,
    pub next_i_d: Arc<Mutex<Option<u64>>>,
    pub obj_map: Arc<Mutex<Option<BTreeMap<GoObjectInterfaceKey, Arc<Mutex<Option<declInfo>>>>>>>,
    pub imp_map: Arc<Mutex<Option<BTreeMap<importKey, Arc<Mutex<Option<Package>>>>>>>,
    pub pkg_path_map: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>>>>>,
    pub seen_pkg_map: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::package::Package>, Arc<Mutex<Option<bool>>>>>>>,
    pub files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::File>>>>>>>,
    pub versions: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<go_ast::r#mod::File>, Arc<Mutex<Option<String>>>>>>>,
    pub imports: Arc<Mutex<Option<Vec<Arc<Mutex<Option<PkgName>>>>>>>,
    pub dot_import_map: Arc<Mutex<Option<BTreeMap<dotImportKey, Arc<Mutex<Option<PkgName>>>>>>>,
    pub broken_aliases: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<bool>>>>>>>,
    pub union_type_sets: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<_TypeSet>>>>>>>,
    pub used_vars: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::Var>, Arc<Mutex<Option<bool>>>>>>>,
    pub used_pkg_names: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::PkgName>, Arc<Mutex<Option<bool>>>>>>>,
    pub mono: Arc<Mutex<Option<monoGraph>>>,
    pub first_err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
    pub methods: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>>>>>,
    pub untyped: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn go_ast::r#mod::Expr + Send + Sync>>, Arc<Mutex<Option<exprInfo>>>>>>>,
    pub delayed: Arc<Mutex<Option<Vec<action>>>>,
    pub obj_path: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>,
    pub cleaners: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn cleaner + Send + Sync>>>>>>>>,
    pub environment: Arc<Mutex<Option<environment>>>,
    pub indent: Arc<Mutex<Option<i32>>>,
}

impl Checker {
    pub fn __go_value_clone(&self) -> Self {
        Self { conf: self.conf.clone(), ctxt: self.ctxt.clone(), fset: self.fset.clone(), pkg: self.pkg.clone(), info: self.info.clone(), next_i_d: { let __guard = self.next_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, obj_map: self.obj_map.clone(), imp_map: self.imp_map.clone(), pkg_path_map: self.pkg_path_map.clone(), seen_pkg_map: self.seen_pkg_map.clone(), files: self.files.clone(), versions: self.versions.clone(), imports: self.imports.clone(), dot_import_map: self.dot_import_map.clone(), broken_aliases: self.broken_aliases.clone(), union_type_sets: self.union_type_sets.clone(), used_vars: self.used_vars.clone(), used_pkg_names: self.used_pkg_names.clone(), mono: { let __guard = self.mono.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, first_err: self.first_err.clone(), methods: self.methods.clone(), untyped: self.untyped.clone(), delayed: self.delayed.clone(), obj_path: self.obj_path.clone(), cleaners: self.cleaners.clone(), environment: { let __guard = self.environment.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, indent: { let __guard = self.indent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Checker {
    fn default() -> Self {
        Self { conf: Arc::new(Mutex::new(None)), ctxt: Arc::new(Mutex::new(None)), fset: Arc::new(Mutex::new(None)), pkg: Arc::new(Mutex::new(None)), info: Arc::new(Mutex::new(None)), next_i_d: Arc::new(Mutex::new(Some(0))), obj_map: Arc::new(Mutex::new(None)), imp_map: Arc::new(Mutex::new(None)), pkg_path_map: Arc::new(Mutex::new(None)), seen_pkg_map: Arc::new(Mutex::new(None)), files: Arc::new(Mutex::new(None)), versions: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), dot_import_map: Arc::new(Mutex::new(None)), broken_aliases: Arc::new(Mutex::new(None)), union_type_sets: Arc::new(Mutex::new(None)), used_vars: Arc::new(Mutex::new(None)), used_pkg_names: Arc::new(Mutex::new(None)), mono: Arc::new(Mutex::new(Some(monoGraph::default()))), first_err: Arc::new(Mutex::new(None)), methods: Arc::new(Mutex::new(None)), untyped: Arc::new(Mutex::new(None)), delayed: Arc::new(Mutex::new(None)), obj_path: Arc::new(Mutex::new(None)), cleaners: Arc::new(Mutex::new(None)), environment: Arc::new(Mutex::new(Some(environment::default()))), indent: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Checker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.conf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.ctxt.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.fset.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.pkg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.info.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.next_i_d.lock().unwrap().as_ref().unwrap()), format_map(&self.obj_map), format_map(&self.imp_map), "<map>", format_map(&self.seen_pkg_map), format_slice_wrapped(&self.files), format_map(&self.versions), format_slice_wrapped(&self.imports), format_map(&self.dot_import_map), format_map(&self.broken_aliases), format_map(&self.union_type_sets), format_map(&self.used_vars), format_map(&self.used_pkg_names), (*self.mono.lock().unwrap().as_ref().unwrap()), (*self.first_err.lock().unwrap().as_ref().unwrap()), "<map>", format_map(&self.untyped), format_slice(&self.delayed), format_slice_wrapped_stringer(&self.obj_path), format_slice_wrapped_stringer(&self.cleaners), (*self.environment.lock().unwrap().as_ref().unwrap()), (*self.indent.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Checker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub trait cleaner: std::fmt::Display + Any {
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool;
    fn cleanup(&mut self);
}

impl Clone for Box<dyn cleaner + Send + Sync> {
    fn clone(&self) -> Self {
        cleaner::__go_clone_box_cleaner(self.as_ref())
    }
}

/// A bailout panic is used for early termination.
#[derive(Debug, Clone, Default)]
pub struct bailout {
}

impl bailout {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for bailout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for bailout {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static nopos: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<go_token::position::Pos>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static noposn: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<crate::errors::atPos>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static gotypesalias: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<internal_godebug::r#mod::Setting>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static _aliasAny: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i32>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *nopos.lock().unwrap() = Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))));
    *noposn.lock().unwrap() = Some(crate::errors::atPos(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))))));
    *gotypesalias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *_aliasAny.lock().unwrap() = Some(0);
    *noposn.lock().unwrap() = Some(crate::errors::atPos(Arc::new(Mutex::new(Some((*nopos.lock().unwrap().as_ref().unwrap()).clone())))));
    *gotypesalias.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("gotypesalias".to_string())))));
}


pub(crate) fn __go_zero_globals() {
    *nopos.lock().unwrap() = Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))));
    *noposn.lock().unwrap() = Some(crate::errors::atPos(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))))));
    *gotypesalias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *_aliasAny.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_1() {
    *noposn.lock().unwrap() = Some(crate::errors::atPos(Arc::new(Mutex::new(Some((*nopos.lock().unwrap().as_ref().unwrap()).clone())))));
}


pub(crate) fn __go_init_order_2() {
    *gotypesalias.lock().unwrap() = Some(internal_godebug::new(Arc::new(Mutex::new(Some("gotypesalias".to_string())))));
}


impl environment {
    /// lookupScope looks up name in the current environment and if an object
    /// is found it returns the scope containing the object and the object.
    /// Otherwise it returns (nil, nil).
    ///
    /// Note that obj.Parent() may be different from the returned scope if the
    /// object was inserted into the scope and already had a parent at that
    /// time (see Scope.Insert). This can only happen for dot-imported objects
    /// whose parent is the scope of the package that exported them.
    pub fn lookup_scope(&self, name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::scope::Scope>>>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        let mut s = self.scope.clone();
    while (*s.lock().unwrap()).is_some() {
        {
        let mut obj = { let __recv = s.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*obj.lock().unwrap()).is_some() && (!go_token::position::Pos::is_valid(&(*self.expr_pos.lock().unwrap().as_ref().unwrap())) || { let __tmp_x = cmp_pos((*obj.lock().unwrap().as_ref().unwrap()).scope_pos(), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.expr_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))); let __tmp_y = 0; __tmp_x <= __tmp_y }) {
            return (s.clone(), obj.clone());;
        }
    }
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).parent.clone(); s = new_val; };
    }
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    /// lookup is like lookupScope but it only returns the object (or nil).
    pub fn lookup(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        let (_, mut obj) = self.lookup_scope(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        return obj.clone();
    }
}

impl action {
    /// If debug is set, describef sets a printf-formatted description for action a.
    /// Otherwise, it is a no-op.
    pub fn describef(&mut self, pos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        if DEBUG {
        { let new_val = Arc::new(Mutex::new(Some(actionDesc { pos: pos.clone(), format: Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args: args.clone(), ..Default::default() }))).clone(); self.desc = new_val; };
    }
    }
}

impl Checker {
    /// addDeclDep adds the dependency edge (check.decl -> to) if check.decl exists
    pub fn add_decl_dep(&self, to: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        let mut from = (*self.environment.lock().unwrap().as_ref().unwrap()).decl.clone();
        if (*from.lock().unwrap()).is_none() {
        return;
    }
                // not in a package-level init expression
        {
        let (_, mut found) = { let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(to.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !found {
            return;;
        }
    }
                // to is not a package-level object
        { let __recv = from.clone(); let __recv_ptr: *mut crate::resolver::declInfo = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::resolver::declInfo }; let __result = unsafe { &mut *__recv_ptr }.add_dep(to.clone()); __result };
    }

    /// brokenAlias records that alias doesn't have a determined type yet.
    /// It also sets alias.typ to Typ[Invalid].
    /// Not used if check.conf._EnableAlias is set.
    pub fn broken_alias(&mut self, alias: Arc<Mutex<Option<TypeName>>>) {
        assert(Arc::new(Mutex::new(Some(!(*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap())))));
        if { let __nil_target = self.broken_aliases.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<bool>>>>::new()))); self.broken_aliases = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(alias.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.broken_aliases.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*alias.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// validAlias records that alias has the valid type typ (possibly Typ[Invalid]).
    pub fn valid_alias(&self, alias: Arc<Mutex<Option<TypeName>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some(!(*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap())))));
        { let __map_handle = self.broken_aliases.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(alias.clone())); };
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*alias.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    /// isBrokenAlias reports whether alias doesn't have a determined type yet.
    pub fn is_broken_alias(&self, alias: Arc<Mutex<Option<TypeName>>>) -> bool {
        assert(Arc::new(Mutex::new(Some(!(*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap())))));
        { let __map = { let __map_holder = self.broken_aliases.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(alias.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) }
    }

    pub fn remember_untyped(&mut self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, lhs: Arc<Mutex<Option<bool>>>, mode: Arc<Mutex<Option<operandMode>>>, typ: Arc<Mutex<Option<Basic>>>, val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) {
        let mut m = self.untyped.clone();
        if (*m.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn go_ast::r#mod::Expr + Send + Sync>>, Arc<Mutex<Option<exprInfo>>>>::new()))); m = new_val; };
        { let new_val = m.clone(); self.untyped = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(e.clone()); let __map_value = Arc::new(Mutex::new(Some(exprInfo { is_lhs: Arc::new(Mutex::new(Some({ let __arg_holder = lhs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), mode: Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), val: val.clone(), ..Default::default() }))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    /// later pushes f on to the stack of actions that will be processed later;
    /// either at the end of the current statement, or in case of a local constant
    /// or variable declaration, before the constant or variable is in scope
    /// (so that f still sees the scope before any new declarations).
    /// later returns the pushed action so one can provide a description
    /// via action.describef for debugging, if desired.
    pub fn later(&mut self, f: Arc<Mutex<Option<Box<dyn FnMut() -> () + Send + Sync>>>>) -> Option<GoSliceElemPtr<action>> {
        let mut i = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        { let new_val = { let __append_target = self.delayed.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(action { version: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), f: f.clone(), ..Default::default() }); __append_target.clone() }; self.delayed = new_val; };
        return Some(GoSliceElemPtr::new(self.delayed.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
    }

    /// push pushes obj onto the object path and returns its index in the path.
    pub fn push(&mut self, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> i32 {
        { let new_val = { let __append_target = self.obj_path.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(obj.clone()); __append_target.clone() }; self.obj_path = new_val; };
        return { let __tmp_x = (({ let __len_target = { let __field = self.obj_path.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y };
    }

    /// pop pops and returns the topmost object from the object path.
    pub fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.obj_path.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut obj = { let __seq = { let __seq_holder = self.obj_path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        (*self.obj_path.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = Arc::new(Mutex::new(None));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.obj_path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); self.obj_path = new_val; };
        return obj.clone();
    }

    /// needsCleanup records objects/types that implement the cleanup method
    /// which will be called at the end of type-checking.
    pub fn needs_cleanup(&mut self, c: Arc<Mutex<Option<Box<dyn cleaner + Send + Sync>>>>) {
        { let new_val = { let __append_target = self.cleaners.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(c.clone()); __append_target.clone() }; self.cleaners = new_val; };
    }

    /// initFiles initializes the files-specific portion of checker.
    /// The provided files must all belong to the same package.
    pub fn init_files(&mut self, files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::File>>>>>>>) {
                // start with a clean slate (check.Files may be called multiple times)
                // TODO(gri): what determines which fields are zeroed out here, vs at the end
                // of checkFiles?
        *self.files.lock().unwrap() = None;
        *self.imports.lock().unwrap() = None;
        { let new_val = Arc::new(Mutex::new(None)); self.dot_import_map = new_val; };
        *self.first_err.lock().unwrap() = None;
        { let new_val = Arc::new(Mutex::new(None)); self.methods = new_val; };
        { let new_val = Arc::new(Mutex::new(None)); self.untyped = new_val; };
        *self.delayed.lock().unwrap() = None;
        *self.obj_path.lock().unwrap() = None;
        *self.cleaners.lock().unwrap() = None;
                // We must initialize usedVars and usedPkgNames both here and in NewChecker,
                // because initFiles is not called in the CheckExpr or Eval codepaths, yet we
                // want to free this memory at the end of Files ('used' predicates are
                // only needed in the context of a given file).
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::Var>, Arc<Mutex<Option<bool>>>>::new()))); self.used_vars = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::PkgName>, Arc<Mutex<Option<bool>>>>::new()))); self.used_pkg_names = new_val; };
                // determine package name and collect valid files
        let mut pkg = self.pkg.clone();
        { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for file in __range_values.iter() {
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*file.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    {
        let _switch_val = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == "".to_string()) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *(*pkg.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    } else {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*file.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BLANK_PKG_NAME as i32))))))), Arc::new(Mutex::new(Some("invalid package name _".to_string()))));
    }
            _fallthrough = true;
        }
        if !_matched && (_switch_val == (*name.lock().unwrap().as_ref().unwrap()).clone()) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __append_target = self.files.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*file).clone()); __append_target.clone() }; self.files = new_val; };
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*file.lock().unwrap().as_ref().unwrap()).package.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISMATCHED_PKG_NAME as i32))))))), Arc::new(Mutex::new(Some("package %s; expected package %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        }
    }
    } }
                // ignore this file
                // reuse Info.FileVersions if provided
        let mut versions = (*self.info.lock().unwrap().as_ref().unwrap()).file_versions.clone();
        if (*versions.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<go_ast::r#mod::File>, Arc<Mutex<Option<String>>>>::new()))); versions = new_val; };
    }
        { let new_val = versions.clone(); self.versions = new_val; };
        let mut pkgVersion = as_go_version(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.conf.lock().unwrap().as_ref().unwrap()).go_version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if (*pkgVersion.lock().unwrap().as_ref().unwrap()).is_valid() && { let __tmp_x = ((*files.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = (*pkgVersion.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some({ let __arg_holder = go_current.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FilePtr({ let __seq = { let __seq_holder = files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TOO_NEW as i32))))))), Arc::new(Mutex::new(Some("package requires newer Go version %v (application built with %v)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*pkgVersion.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*go_current.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
                // determine Go version for each file
        { let __range_holder = self.files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for file in __range_values.iter() {
                // use unaltered Config.GoVersion by default
                // (This version string may contain dot-release numbers as in go1.20.1,
                // unlike file versions which are Go language versions only, if valid.)
        let mut v = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.conf.lock().unwrap().as_ref().unwrap()).go_version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
                // If the file specifies a version, use max(fileVersion, go1.21).
        {
        let mut fileVersion = as_go_version(Arc::new(Mutex::new(Some({ let __selector_holder = (*file.lock().unwrap().as_ref().unwrap()).go_version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if (*fileVersion.lock().unwrap().as_ref().unwrap()).is_valid() {
            { let new_val = Arc::new(Mutex::new(Some((*version_max(Arc::new(Mutex::new(Some({ let __arg_holder = fileVersion.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_21.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).to_string()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *v.lock().unwrap() = __moved_val; };;
            if { let __tmp_x = (*fileVersion.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some({ let __arg_holder = go_current.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*file.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(TOO_NEW as i32))))))), Arc::new(Mutex::new(Some("file requires newer Go version %v (application built with %v)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*fileVersion.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>, Box::new((*go_current.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    };
        }
    }
                // Go 1.21 introduced the feature of setting the go.mod
                // go line to an early version of Go and allowing //go:build lines
                // to set the Go version in a given file. Versions Go 1.21 and later
                // can be set backwards compatibly as that was the first version
                // files with go1.21 or later build tags could be built with.
                //
                // Set the version to max(fileVersion, go1.21): That will allow a
                // downgrade to a version before go1.22, where the for loop semantics
                // change was made, while being backwards compatible with versions of
                // go before the new //go:build semantics were introduced.
                // Report a specific error for each tagged file that's too new.
                // (Normally the build system will have filtered files by version,
                // but clients can present arbitrary files to the type checker.)
                // Use position of 'package [p]' for types/types2 consistency.
                // (Ideally we would use the //build tag itself.)
        { let __map_key = GoLocalPtrKey::new(file.clone()); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*versions.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    }

    pub fn handle_bailout(&self, err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        {
    let _ts_subject = go_recover().clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_is_nil || _ts_val.and_then(|__v| __v.downcast_ref::<bailout>()).is_some() {
        let p = _ts_subject.clone();
        drop(_ts_guard);
        { let new_val = { let __err_handle = self.first_err.clone(); let mut __err_guard = __err_handle.lock().unwrap(); __err_guard.take() }; *err.lock().unwrap() = new_val; };;
    } else {
        let p = _ts_subject.clone();
        drop(_ts_guard);
        std::panic::panic_any({ let __any_holder = p.clone(); let __any_guard = __any_holder.lock().unwrap(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) });;
    }
    }
    }

    /// Files checks the provided files as part of the checker's package.
    pub fn files(&mut self, files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::File>>>>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __left = self.pkg.clone(); let __right = (*Unsafe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // Defensive handling for Unsafe, which cannot be type checked, and must
                // not be mutated. See https://go.dev/issue/61212 for an example of where
                // Unsafe is passed to NewChecker.
        {
        *err.lock().unwrap() = None;;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
    }
                        // Defensive handling for Unsafe, which cannot be type checked, and must
                        // not be mutated. See https://go.dev/issue/61212 for an example of where
                        // Unsafe is passed to NewChecker.
                        // Avoid early returns here! Nearly all errors can be
                        // localized to a piece of syntax and needn't prevent
                        // type-checking of the rest of the package.
            let mut check_defer_captured = self.clone(); let err_defer_captured = err.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.handle_bailout(err_defer_captured.clone());
    }));
            self.check_files(files.clone());
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return err.clone();
    }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                err.clone()
            }
        }
    }

    /// checkFiles type-checks the specified files. Errors are reported as
    /// a side effect, not by returning early, to ensure that well-formed
    /// syntax is properly type annotated even in a package containing
    /// errors.
    pub fn check_files(&mut self, files: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::File>>>>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Ensure that _EnableAlias is consistent among concurrent type checking
                        // operations. See the documentation of [_aliasAny] for details.
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = sync_atomic::add_int32(_aliasAny.clone(), Arc::new(Mutex::new(Some(1 as i32)))); let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        std::panic::panic_any(Box::new("EnableAlias set while !EnableAlias type checking is ongoing".to_string()) as Box<dyn Any + Send + Sync>);
    }
        __defer_stack.push(Box::new(move || {
        sync_atomic::add_int32(_aliasAny.clone(), Arc::new(Mutex::new(Some(-1 as i32))));
    }));
    } else {
        if { let __tmp_x = sync_atomic::add_int32(_aliasAny.clone(), Arc::new(Mutex::new(Some(-1 as i32)))); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        std::panic::panic_any(Box::new("!EnableAlias set while EnableAlias type checking is ongoing".to_string()) as Box<dyn Any + Send + Sync>);
    }
        __defer_stack.push(Box::new(move || {
        sync_atomic::add_int32(_aliasAny.clone(), Arc::new(Mutex::new(Some(1 as i32))));
    }));
    }
            let mut check_closure_clone = (*self).clone(); let mut print = Arc::new(Mutex::new(Some(Box::new(move |msg: Arc<Mutex<Option<String>>>| {
        if (*(*check_closure_clone.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        println!();
        println!("{}", format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync>)));
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== initFiles ==".to_string())))) };
            self.init_files(files.clone());
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== collectObjects ==".to_string())))) };
            self.collect_objects();
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== packageObjects ==".to_string())))) };
            self.package_objects();
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== processDelayed ==".to_string())))) };
            self.process_delayed(Arc::new(Mutex::new(Some(0))));
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== cleanup ==".to_string())))) };
            self.cleanup();
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== initOrder ==".to_string())))) };
            self.init_order();
            if !(*(*self.conf.lock().unwrap().as_ref().unwrap()).disable_unused_import_check.lock().unwrap().as_ref().unwrap()) {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== unusedImports ==".to_string())))) };
        self.unused_imports();
    }
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = print.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("== recordUntyped ==".to_string())))) };
            self.record_untyped();
            if { let __nil_target = self.first_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
                // TODO(mdempsky): Ensure monomorph is safe when errors exist.
        self.monomorph();
    }
                        // TODO(mdempsky): Ensure monomorph is safe when errors exist.
            { let new_val = { let __selector_holder = (*self.conf.lock().unwrap().as_ref().unwrap()).go_version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.pkg.lock().unwrap().as_ref().unwrap()).go_version.lock().unwrap() = Some(new_val); };
            { let new_val = true; *(*self.pkg.lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };
                        // no longer needed - release memory
            *self.imports.lock().unwrap() = None;
            { let new_val = Arc::new(Mutex::new(None)); self.dot_import_map = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.pkg_path_map = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.seen_pkg_map = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.broken_aliases = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.union_type_sets = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.used_vars = new_val; };
            { let new_val = Arc::new(Mutex::new(None)); self.used_pkg_names = new_val; };
            *self.ctxt.lock().unwrap() = None;

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
            }
        }));
        std::panic::set_hook(__go_previous_panic_hook);
        match __go_panic_result {
            Ok(__go_value) => __go_value,
            Err(__go_panic_payload) => {
                go_store_panic_payload(__go_panic_payload);
                while let Some(f) = __defer_stack.pop() {
                    f();
                }
                go_resume_unrecovered_panic();
                ()
            }
        }
    }

    /// processDelayed processes all delayed actions pushed after top.
    pub fn process_delayed(&mut self, top: Arc<Mutex<Option<i32>>>) {
                // If each delayed action pushes a new action, the
                // stack will continue to grow during this loop.
                // However, it is only processing functions (which
                // are processed in a delayed fashion) that may
                // add more actions (such as nested functions), so
                // this is a sufficiently bounded process.
        let mut savedVersion = Arc::new(Mutex::new(Some({ let __selector_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut i = { let __owned = top.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut a: Option<GoSliceElemPtr<action>> = Some(GoSliceElemPtr::new(self.delayed.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        if { let __nil_target = (*a.as_ref().unwrap().borrow().as_ref().unwrap()).desc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.trace((*(*(*a.as_ref().unwrap().borrow().as_ref().unwrap()).desc.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some(format!("{}{}", "-- ".to_string(), (*(*(*a.as_ref().unwrap().borrow().as_ref().unwrap()).desc.lock().unwrap().as_ref().unwrap()).format.lock().unwrap().as_ref().unwrap()).clone())))), (*(*a.as_ref().unwrap().borrow().as_ref().unwrap()).desc.lock().unwrap().as_ref().unwrap()).args.clone());
    } else {
        self.trace(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("-- delayed %p".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new((*a.as_ref().unwrap().borrow().as_ref().unwrap()).f.clone()) as Box<dyn Any + Send + Sync>]))));
    }
    }
        { let new_val = { let __selector_holder = (*a.as_ref().unwrap().borrow().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*self.environment.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
        { let __f_holder = (*a.as_ref().unwrap().borrow().as_ref().unwrap()).f.clone(); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };

        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        println!();
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // reestablish the effective Go version captured earlier
                // may append to check.delayed
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x <= __tmp_y }))));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.delayed.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*top.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))); self.delayed = new_val; };
        { let new_val = savedVersion.lock().unwrap().as_ref().unwrap().clone(); *(*self.environment.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = Some(new_val); };
    }

    /// cleanup runs cleanup for all collected cleaners.
    pub fn cleanup(&mut self) {
                // Don't use a range clause since Named.cleanup may add more cleaners.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.cleaners.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __recv = { let __seq = { let __seq_holder = self.cleaners.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).cleanup(); __result };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        *self.cleaners.lock().unwrap() = None;
    }

    /// go/types doesn't support recording of types directly in the AST.
    /// dummy function to match types2 code.
    pub fn record_type_and_value_in_syntax(&self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, mode: Arc<Mutex<Option<operandMode>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) {
    }

    /// go/types doesn't support recording of types directly in the AST.
    /// dummy function to match types2 code.
    pub fn record_comma_ok_types_in_syntax(&self, x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, t0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, t1: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
    }

    pub fn object_of(&self, id: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.info.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.object_of(id)
    }

    pub fn pkg_name_of(&self, imp: Arc<Mutex<Option<go_ast::r#mod::ImportSpec>>>) -> Arc<Mutex<Option<crate::object::PkgName>>> {
        // Forward to embedded type's method
        let embedded = self.info.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg_name_of(imp)
    }

    pub fn type_of(&self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.info.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.type_of(e)
    }

    pub fn lookup(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.environment.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.lookup(name)
    }

    pub fn lookup_scope(&self, name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::scope::Scope>>>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.environment.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.lookup_scope(name)
    }

    pub fn record_types(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.info.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.record_types()
    }
}

impl cleaner for Checker {
    fn cleanup(&mut self) {
        Checker::cleanup(self)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Checker>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CheckerPtr(pub Arc<Mutex<Option<Checker>>>);

impl std::fmt::Display for CheckerPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl cleaner for CheckerPtr {
    fn cleanup(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Checker::cleanup(__recv)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CheckerPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn alias_any() -> bool {
    let mut v = { let __recv_holder = (*gotypesalias.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).value(); __result };
    let mut useAlias = Arc::new(Mutex::new(Some({ let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "0".to_string(); __tmp_x != __tmp_y })));
    let mut inuse = sync_atomic::load_int32(_aliasAny.clone());
    if { let __tmp_x = inuse; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*useAlias.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = inuse; let __tmp_y = 0 as i32; __tmp_x > __tmp_y }); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("gotypealias mutated during type checking, gotypesalias={}, inuse={}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }, inuse)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
    }
    return { let __v = (*useAlias.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// NewChecker returns a new [Checker] instance for a given package.
/// [Package] files may be added incrementally via checker.Files.
pub fn new_checker(mut conf: Arc<Mutex<Option<Config>>>, fset: Arc<Mutex<Option<go_token::position::FileSet>>>, pkg: Arc<Mutex<Option<Package>>>, mut info: Arc<Mutex<Option<Info>>>) -> Arc<Mutex<Option<Checker>>> {
        // make sure we have a configuration
    if (*conf.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(Config::default()))).clone(); conf = new_val; };
    }

        // make sure we have an info struct
    if (*info.lock().unwrap()).is_none() {
        { let new_val = Arc::new(Mutex::new(Some(Info::default()))).clone(); info = new_val; };
    }

        // Note: clients may call NewChecker with the Unsafe package, which is
        // globally shared and must not be mutated. Therefore NewChecker must not
        // mutate *pkg.
        //
        // (previously, pkg.goVersion was mutated here: go.dev/issue/61212)
        // In go/types, conf._EnableAlias is controlled by gotypesalias.
    { let new_val = { let __tmp_x = (*{ let __recv_holder = (*gotypesalias.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).value(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "0".to_string(); __tmp_x != __tmp_y }; *(*conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap() = Some(new_val); };

    Arc::new(Mutex::new(Some(Checker { conf: conf.clone(), ctxt: { let __field = (*conf.lock().unwrap().as_ref().unwrap()).context.clone(); __field }, fset: fset.clone(), pkg: pkg.clone(), info: info.clone(), obj_map: Arc::new(Mutex::new(Some(BTreeMap::<GoObjectInterfaceKey, Arc<Mutex<Option<crate::resolver::declInfo>>>>::new()))), imp_map: Arc::new(Mutex::new(Some(BTreeMap::<importKey, Arc<Mutex<Option<crate::package::Package>>>>::new()))), used_vars: Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::Var>, Arc<Mutex<Option<bool>>>>::new()))), used_pkg_names: Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::PkgName>, Arc<Mutex<Option<bool>>>>::new()))), next_i_d: Default::default(), pkg_path_map: Default::default(), seen_pkg_map: Default::default(), files: Default::default(), versions: Default::default(), imports: Default::default(), dot_import_map: Default::default(), broken_aliases: Default::default(), union_type_sets: Default::default(), mono: Arc::new(Mutex::new(Some(monoGraph::default()))), first_err: Default::default(), methods: Default::default(), untyped: Default::default(), delayed: Default::default(), obj_path: Default::default(), cleaners: Default::default(), environment: Arc::new(Mutex::new(Some(environment::default()))), indent: Default::default() })))
}

pub fn version_max(a: Arc<Mutex<Option<goVersion>>>, b: Arc<Mutex<Option<goVersion>>>) -> Arc<Mutex<Option<crate::version::goVersion>>> {
    if { let __tmp_x = (*a.lock().unwrap().as_ref().unwrap()).cmp(Arc::new(Mutex::new(Some({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x < __tmp_y } {
        return { let __owned = b.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    return { let __owned = a.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// instantiatedIdent determines the identifier of the type instantiated in expr.
/// Helper function for recordInstance in recording.go.
pub fn instantiated_ident(expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<go_ast::r#mod::Ident>>> {
    let mut selOrIdent: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
    {
    let _ts_subject = expr.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).unwrap().0.clone();
        { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *selOrIdent.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let e = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).unwrap().0.clone();
        { let __iface_handle = { let __field = (*e.lock().unwrap().as_ref().unwrap()).x.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *selOrIdent.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() {
        let e = _ts_subject.clone();
        { let __iface_handle = e.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *selOrIdent.lock().unwrap() = (*__iface_guard).clone(); };;
    }
    }
        // only exists in go/ast, not syntax
    {
    let _ts_subject = selOrIdent.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        return x.clone();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectorExprPtr>()).unwrap().0.clone();
        return (*x.lock().unwrap().as_ref().unwrap()).sel.clone();;
    }
    }

        // extra debugging of go.dev/issue/63933
    std::panic::panic_any(Box::new({ let __v = sprintf(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some("instantiated ident not found; please report: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = expr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for exprInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for environment {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for importKey {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for dotImportKey {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for action {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for actionDesc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Checker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for bailout {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
