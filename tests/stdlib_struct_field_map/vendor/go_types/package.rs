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

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Package describes a Go package.
#[derive(Clone)]
pub struct Package {
    pub path: Arc<Mutex<Option<String>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub scope: Arc<Mutex<Option<Scope>>>,
    pub imports: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Package>>>>>>>,
    pub complete: Arc<Mutex<Option<bool>>>,
    pub fake: Arc<Mutex<Option<bool>>>,
    pub cgo: Arc<Mutex<Option<bool>>>,
    pub go_version: Arc<Mutex<Option<String>>>,
}

impl Package {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scope: self.scope.clone(), imports: self.imports.clone(), complete: { let __guard = self.complete.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, fake: { let __guard = self.fake.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, cgo: { let __guard = self.cgo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, go_version: { let __guard = self.go_version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Package {
    fn default() -> Self {
        Self { path: Arc::new(Mutex::new(Some(String::new()))), name: Arc::new(Mutex::new(Some(String::new()))), scope: Arc::new(Mutex::new(None)), imports: Arc::new(Mutex::new(None)), complete: Arc::new(Mutex::new(Some(false))), fake: Arc::new(Mutex::new(Some(false))), cgo: Arc::new(Mutex::new(Some(false))), go_version: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Package {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Package {
    /// Path returns the package path.
    pub fn path(&self) -> Arc<Mutex<Option<String>>> {
        return self.path.clone();
    }

    /// Name returns the package name.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    /// SetName sets the package name.
    pub fn set_name(&mut self, name: Arc<Mutex<Option<String>>>) {
        { let new_val = name.lock().unwrap().as_ref().unwrap().clone(); *self.name.lock().unwrap() = Some(new_val); };
    }

    /// GoVersion returns the minimum Go version required by this package.
    /// If the minimum version is unknown, GoVersion returns the empty string.
    /// Individual source files may specify a different minimum Go version,
    /// as reported in the [go/ast.File.GoVersion] field.
    pub fn go_version(&self) -> Arc<Mutex<Option<String>>> {
        return self.go_version.clone();
    }

    /// Scope returns the (complete or incomplete) package scope
    /// holding the objects declared at package level (TypeNames,
    /// Consts, Vars, and Funcs).
    /// For a nil pkg receiver, Scope returns the Universe scope.
    pub fn scope(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        if true {
        return self.scope.clone();
    }
        (*Universe.lock().unwrap().as_ref().unwrap()).clone()
    }

    /// A package is complete if its scope contains (at least) all
    /// exported objects; otherwise it is incomplete.
    pub fn complete(&self) -> bool {
        return (*self.complete.lock().unwrap().as_ref().unwrap());
    }

    /// MarkComplete marks a package as complete.
    pub fn mark_complete(&mut self) {
        { let new_val = true; *self.complete.lock().unwrap() = Some(new_val); };
    }

    /// Imports returns the list of packages directly imported by
    /// pkg; the list is in source order.
    ///
    /// If pkg was loaded from export data, Imports includes packages that
    /// provide package-level objects referenced by pkg. This may be more or
    /// less than the set of packages directly imported by pkg's source code.
    ///
    /// If pkg uses cgo and the FakeImportC configuration option
    /// was enabled, the imports list may contain a fake "C" package.
    pub fn imports(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Package>>>>>>> {
        return self.imports.clone();
    }

    /// SetImports sets the list of explicitly imported packages to list.
    /// It is the caller's responsibility to make sure list elements are unique.
    pub fn set_imports(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Package>>>>>>>) {
        { let new_val = list.clone(); self.imports = new_val; };
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("package {} ({:?})", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.path.lock().unwrap().as_ref().unwrap())))))
    }
}

/// NewPackage returns a new Package for the given package path and name.
/// The package is not complete and contains no explicit imports.
pub fn new_package(path: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Package>>> {
    let mut scope = new_scope({ let __arg_holder = Universe.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(format!("package {:?}", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v })))));
    return Arc::new(Mutex::new(Some(Package { path: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), scope: scope.clone(), ..Default::default() })));
}

impl GoValueClone for Package {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
