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
use crate::package::*;
use crate::pointer::*;
use crate::predicates::*;
use crate::recording::*;
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
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A declInfo describes a package-level const, type, var, or func declaration.
#[derive(Clone)]
pub struct declInfo {
    pub file: Arc<Mutex<Option<Scope>>>,
    pub version: Arc<Mutex<Option<goVersion>>>,
    pub lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>,
    pub vtyp: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub inherited: Arc<Mutex<Option<bool>>>,
    pub tdecl: Arc<Mutex<Option<go_ast::r#mod::TypeSpec>>>,
    pub fdecl: Arc<Mutex<Option<go_ast::r#mod::FuncDecl>>>,
    pub deps: Arc<Mutex<Option<BTreeMap<GoObjectInterfaceKey, Arc<Mutex<Option<bool>>>>>>>,
}

impl declInfo {
    pub fn __go_value_clone(&self) -> Self {
        Self { file: self.file.clone(), version: { let __guard = self.version.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, lhs: self.lhs.clone(), vtyp: self.vtyp.clone(), init: self.init.clone(), inherited: { let __guard = self.inherited.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tdecl: self.tdecl.clone(), fdecl: self.fdecl.clone(), deps: self.deps.clone() }
    }
}


impl Default for declInfo {
    fn default() -> Self {
        Self { file: Arc::new(Mutex::new(None)), version: Arc::new(Mutex::new(Some(crate::version::goVersion(Arc::new(Mutex::new(Some(String::new()))))))), lhs: Arc::new(Mutex::new(None)), vtyp: Arc::new(Mutex::new(None)), init: Arc::new(Mutex::new(None)), inherited: Arc::new(Mutex::new(Some(false))), tdecl: Arc::new(Mutex::new(None)), fdecl: Arc::new(Mutex::new(None)), deps: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for declInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {}}}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.version.lock().unwrap().as_ref().unwrap()), format_slice_wrapped(&self.lhs), (*self.vtyp.lock().unwrap().as_ref().unwrap()), (*self.init.lock().unwrap().as_ref().unwrap()), (*self.inherited.lock().unwrap().as_ref().unwrap()), { let __guard = self.tdecl.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.fdecl.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_map(&self.deps))
    }
}

impl GoJsonDecode for declInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl declInfo {
    /// hasInitializer reports whether the declared object has an initialization
    /// expression or function body.
    pub fn has_initializer(&self) -> bool {
        return { let __iface_handle = { let __field = self.init.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } || { let __nil_target = self.fdecl.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __nil_target = (*self.fdecl.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    /// addDep adds obj to the set of objects d's init expression depends on.
    pub fn add_dep(&mut self, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        let mut m = self.deps.clone();
        if { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoObjectInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))); m = new_val; };
        { let new_val = m.clone(); self.deps = new_val; };
    }
        { let __map_key = GoObjectInterfaceKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

impl crate::check::Checker {
    /// arityMatch checks that the lhs and rhs of a const or var decl
    /// have the appropriate number of names and init exprs. For const
    /// decls, init is the value spec providing the init exprs; for
    /// var decls, init is nil (the init exprs are in s in this case).
    pub fn arity_match(&self, s: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>, init: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>) {
        let mut l = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        let mut r = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        if { let __nil_result = (*init.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = ({ let __len_target = { let __field = (*init.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *r.lock().unwrap() = Some(new_val); };
    }
        const code: i32 = WRONG_ASSIGN_COUNT;

        if { let __nil_result = (*init.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // var decl w/o init expr
            if { let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(code as i32))))))), Arc::new(Mutex::new(Some("missing type or init expr".to_string()))));
    }
        } else if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
            if { let __tmp_x = ({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
                // init exprs from s
        let mut n = { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(code as i32))))))), Arc::new(Mutex::new(Some("extra init expr %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
                // init exprs "inherited"
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(s.clone())) as Box<dyn positioner + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(code as i32))))))); let __method_arg2 = Arc::new(Mutex::new(Some("extra init expr at %s".to_string()))); self.errorf(__method_arg0, __method_arg1, __method_arg2, Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*self.fset.lock().unwrap().as_ref().unwrap()).position({ let __recv = init.clone(); let __recv_ptr: *const go_ast::r#mod::ValueSpec = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::ValueSpec }; let __result = unsafe { &*__recv_ptr }.pos(); __result }); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))) };
    }
        } else if { let __tmp_x = { let __v = (*l.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } && ({ let __nil_result = (*init.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y }) {
            let mut n = { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(n.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(code as i32))))))), Arc::new(Mutex::new(Some("missing init expr for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(n.clone()) as Box<dyn Any + Send + Sync>]))));
        }
    }

    /// declarePkgObj declares obj in the package scope, records its ident -> obj mapping,
    /// and updates check.objMap. The object must not be a function or method.
    pub fn declare_pkg_obj(&mut self, ident: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, d: Arc<Mutex<Option<declInfo>>>) {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y }))));
                // spec: "A package-scope or file-scope identifier with name init
                // may only be declared to be a function with this (func()) signature."
        if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "init".to_string(); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(ident.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INIT_DECL as i32))))))), Arc::new(Mutex::new(Some("cannot declare init - must be func".to_string()))));
        return;
    }
                // spec: "The main package must have package name main and declare
                // a function main that takes no arguments and returns no value."
        if { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "main".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*self.pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "main".to_string(); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(ident.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_MAIN_DECL as i32))))))), Arc::new(Mutex::new(Some("cannot declare main - must be func".to_string()))));
        return;
    }
        { let __method_arg0 = { let __field = (*self.pkg.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = ident.clone(); let __method_arg2 = obj.clone(); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
        { let __map_key = GoObjectInterfaceKey::new(obj.clone()); let __map_value = d.clone(); (*self.obj_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        (*obj.lock().unwrap().as_mut().unwrap()).set_order(Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.obj_map.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32))));
    }

    /// filename returns a filename suitable for debugging output.
    pub fn filename(&self, fileNo: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
        let mut file = { let __seq = { let __seq_holder = self.files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*fileNo.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        {
        let mut pos = { let __recv = file.clone(); let __recv_ptr: *const go_ast::r#mod::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::File }; let __result = unsafe { &*__recv_ptr }.pos(); __result };;
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) {
            return { let __recv = (*self.fset.lock().unwrap().as_ref().unwrap()).file(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __recv_value = __recv.borrow(); let __result = (*__recv_value.as_ref().unwrap()).name(); __result };;
        }
    }
        Arc::new(Mutex::new(Some(format!("file[{}]", { let __v = (*fileNo.lock().unwrap().as_ref().unwrap()).clone(); __v }))))
    }

    pub fn import_package(&mut self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, path: Arc<Mutex<Option<String>>>, dir: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<crate::package::Package>>> {
                // If we already have a package for the given (path, dir)
                // pair, use it instead of doing a full import.
                // Checker.impMap only caches packages that are marked Complete
                // or fake (dummy packages for failed imports). Incomplete but
                // non-fake packages do require an import to complete them.
        let mut key = Arc::new(Mutex::new(Some(importKey { path: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), dir: Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
        let mut imp = { let __map = { let __map_holder = self.imp_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*key.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        if { let __nil_result = (*imp.lock().unwrap()).is_some(); __nil_result } {
        return imp.clone();
    }
                // no package yet => import it
        if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "C".to_string(); __tmp_x == __tmp_y } && ((*(*self.conf.lock().unwrap().as_ref().unwrap()).fake_import_c.lock().unwrap().as_ref().unwrap()) || (*(*self.conf.lock().unwrap().as_ref().unwrap()).go115_uses_cgo.lock().unwrap().as_ref().unwrap())) {
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).fake_import_c.lock().unwrap().as_ref().unwrap()) && (*(*self.conf.lock().unwrap().as_ref().unwrap()).go115_uses_cgo.lock().unwrap().as_ref().unwrap()) {
        self.error(at.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_IMPORT_PATH as i32))))))), Arc::new(Mutex::new(Some("cannot use FakeImportC and go115UsesCgo together".to_string()))));
    }
        { let new_val = new_package(Arc::new(Mutex::new(Some("C".to_string()))), Arc::new(Mutex::new(Some("C".to_string())))).clone(); imp = new_val; };
        { let new_val = true; *(*imp.lock().unwrap().as_ref().unwrap()).fake.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*self.conf.lock().unwrap().as_ref().unwrap()).go115_uses_cgo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*imp.lock().unwrap().as_ref().unwrap()).cgo.lock().unwrap() = Some(new_val); };
    } else {
                // ordinary import
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
        let mut importer = (*self.conf.lock().unwrap().as_ref().unwrap()).importer.clone();;
        if { let __nil_result = (*importer.lock().unwrap()).is_none(); __nil_result } {
            { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Config.Importer not installed"))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };;
        } else {
        let (mut importerFrom, mut ok) = ({
        let val = importer.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            (Arc::new(Mutex::new(None::<Box<dyn ImporterFrom + Send + Sync>>)), false)
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn ImporterFrom + Send + Sync>>)), false)
        }
    });;
        if ok {
            { let (__tmp_0, __tmp_1) = (*importerFrom.lock().unwrap().as_ref().unwrap()).import_from(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(crate::api::ImportMode(Arc::new(Mutex::new(Some(0 as i32)))))))); imp = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
            if { let __nil_result = (*imp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Config.Importer.ImportFrom({}, {}, 0) returned nil but no error", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    };
        } else {
            { let (__tmp_0, __tmp_1) = (*importer.lock().unwrap().as_ref().unwrap()).import(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); imp = __tmp_0.clone(); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
            if { let __nil_result = (*imp.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("Config.Importer.Import({}) returned nil but no error", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    };
        }
    }
    }
                // make sure we have a valid package name
                // (errors here can only happen through manipulation of packages after creation)
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && { let __nil_result = (*imp.lock().unwrap()).is_some(); __nil_result } && ({ let __tmp_x = { let __selector_holder = (*imp.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*imp.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y }) {
        { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid package name: {:?}", (*{ let __field = (*imp.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()))))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        *imp.lock().unwrap() = None;
    }
                // create fake package below
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        self.errorf(at.clone(), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BROKEN_IMPORT as i32))))))), Arc::new(Mutex::new(Some("could not import %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::api::Error>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }]))));
        if { let __nil_result = (*imp.lock().unwrap()).is_none(); __nil_result } {
                // create a new fake package
                // come up with a sensible package name (heuristic)
        let mut name = { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        {
        let mut i = Arc::new(Mutex::new(Some((*name.lock().unwrap().as_ref().unwrap()).len() as i32)));;
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };;
        }
    }
        {
        let mut i = Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "/".to_string(); __s.rfind(&__substr).map(|__i| __i as i32).unwrap_or(-1) })));;
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };;
        }
    }
        { let new_val = new_package(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); imp = new_val; };
    }
                // create a new fake package
                // come up with a sensible package name (heuristic)
                // continue to use the package as best as we can
        { let new_val = true; *(*imp.lock().unwrap().as_ref().unwrap()).fake.lock().unwrap() = Some(new_val); };
    }
    }
                // package scope is not populated
                // ordinary import
                // make sure we have a valid package name
                // (errors here can only happen through manipulation of packages after creation)
                // create fake package below
                // create a new fake package
                // come up with a sensible package name (heuristic)
                // continue to use the package as best as we can
                // avoid follow-up lookup failures
                // package should be complete or marked fake, but be cautious
        if (*{ let __field = (*imp.lock().unwrap().as_ref().unwrap()).complete.clone(); __field }.lock().unwrap().as_ref().unwrap()) || (*{ let __field = (*imp.lock().unwrap().as_ref().unwrap()).fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __map_key = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = imp.clone(); (*self.imp_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
                // Once we've formatted an error message, keep the pkgPathMap
                // up-to-date on subsequent imports. It is used for package
                // qualification in error messages.
        if { let __nil_target = self.pkg_path_map.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.mark_imports(imp.clone());
    }
        return imp.clone();
    }
                // Once we've formatted an error message, keep the pkgPathMap
                // up-to-date on subsequent imports. It is used for package
                // qualification in error messages.
                // something went wrong (importer may have returned incomplete package without error)
        return Arc::new(Mutex::new(None));
    }

    /// collectObjects collects all file and package objects and inserts them
    /// into their respective scopes. It also performs imports and associates
    /// methods with receiver base type names.
    pub fn collect_objects(&mut self) {
        let mut pkg = self.pkg.clone();
                // pkgImports is the set of packages already imported by any package file seen
                // so far. Used to avoid duplicate entries in pkg.imports. Allocate and populate
                // it (pkg.imports may not be empty if we are checking test files incrementally).
                // Note that pkgImports is keyed by package (and thus package path), not by an
                // importKey value. Two different importKey values may map to the same package
                // which is why we cannot use the check.impMap here.
        let mut pkgImports = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::package::Package>, Arc<Mutex<Option<bool>>>>::new())));
        { let __range_holder = (*pkg.lock().unwrap().as_ref().unwrap()).imports.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for imp in __range_values.iter() {
        { let __map_key = GoLocalPtrKey::new(imp.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*pkgImports.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
        type methodInfo = AnonymousStruct1;
                // method
                // true if pointer receiver
                // receiver type name
        let mut methods: Arc<Mutex<Option<Vec<methodInfo>>>> = Arc::new(Mutex::new(None));
        let mut fileScopes: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::scope::Scope>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = self.files.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        { let __range_holder = self.files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (fileNo, file) in __range_values.iter().enumerate() {
        { let new_val = as_go_version(Arc::new(Mutex::new(Some({ let __map = { let __map_holder = self.versions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(file.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*self.environment.lock().unwrap().as_ref().unwrap()).version.lock().unwrap() = __moved_val; };
                // The package identifier denotes the current package,
                // but there is no corresponding package object.
        self.record_def({ let __field = (*file.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(None)));
                // Use the actual source file extent rather than *ast.File extent since the
                // latter doesn't include comments which appear at the start or end of the file.
                // Be conservative and use the *ast.File extent if we don't have a *token.File.
        let (mut pos, mut end) = ({ let __recv = file.clone(); let __recv_ptr: *const go_ast::r#mod::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::File }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __recv = file.clone(); let __recv_ptr: *const go_ast::r#mod::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::File }; let __result = unsafe { &*__recv_ptr }.end(); __result });
        {
        let mut f: GoPtr<go_token::position::File> = { let __go_ptr = (*self.fset.lock().unwrap().as_ref().unwrap()).file({ let __recv = file.clone(); let __recv_ptr: *const go_ast::r#mod::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::File }; let __result = unsafe { &*__recv_ptr }.pos(); __result }).clone(); match __go_ptr { go_token::GoPtr::Nil => GoPtr::nil(), go_token::GoPtr::Local(__value) => GoPtr::local(__value.clone()), go_token::GoPtr::Raw(__addr) => GoPtr::raw(__addr), go_token::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), go_token::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };;
        if !f.is_nil() {
            { let __tmp_0 = go_token::position::Pos(Arc::new(Mutex::new(Some({ let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result } as i32)))); let __tmp_1 = go_token::position::Pos(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).base(); __result }; let __tmp_y = { let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).size(); __result }; __tmp_x + __tmp_y } as i32)))); *pos.lock().unwrap() = Some(__tmp_0); *end.lock().unwrap() = Some(__tmp_1); };;
        }
    }
        let mut fileScope = new_scope({ let __field = (*pkg.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), self.filename(Arc::new(Mutex::new(Some(fileNo as i32)))));
        (*fileScopes.lock().unwrap().as_mut().unwrap())[(fileNo) as usize] = fileScope.clone();
        self.record_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FilePtr(file.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), fileScope.clone());
                // determine file directory, necessary to resolve imports
                // FileName may be "" (typically for tests) in which case
                // we get "." as the directory which is what we would want.
        let mut fileDir = dir(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*self.fset.lock().unwrap().as_ref().unwrap()).position((*(*file.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos()).lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut check_closure_clone = (*self).clone(); let fileDir_closure_clone = fileDir.clone(); let fileScope_closure_clone = fileScope.clone(); let mut methods_closure_clone = methods.clone(); let pkg_closure_clone = pkg.clone(); let pkgImports_closure_clone = pkgImports.clone(); { let mut __recv = check_closure_clone.clone(); let __method_arg0 = { let __field = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); Box::new(move |mut d: Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>| {
        {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::importDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::importDecl>()).unwrap().clone())));
        if { let __tmp_x = { let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return;
    };
        let (mut path, mut err) = validated_import_path(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        check_closure_clone_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr((*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).path.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_IMPORT_PATH as i32))))))), Arc::new(Mutex::new(Some("invalid import path (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![{ let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => panic!("nil error-to-any lowering requires nil interface representation"), Some(__err) => if let Some(typed_val) = __err.downcast_ref::<crate::api::Error>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_errorString>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<errors_joinError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<fmt_wrapErrors>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_PanicNilError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_TypeAssertionError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_boundsError>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorAddressString>() { go_box_any_with_metadata(typed_val.clone(), "struct", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_errorString>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<runtime_plainError>() { go_box_any_with_metadata(typed_val.clone(), "basic", true) } else if let Some(typed_val) = __err.downcast_ref::<strconv_NumError>() { go_box_any_with_metadata(typed_val.clone(), "pointer", true) } else { panic!("type info required: error-to-any for unknown dynamic error type") } } }]))));
        return;
    };
        let mut imp = check_closure_clone_closure_clone.import_package(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr((*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).path.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = fileDir_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*imp.lock().unwrap()).is_none(); __nil_result } {
        return;
    };
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*imp.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __nil_target = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *name.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "C".to_string(); __tmp_x == __tmp_y } {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(IMPORT_C_RENAMED as i32))))))), Arc::new(Mutex::new(Some("cannot rename import \"C\"".to_string()))));
        return;
    }
    };
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "init".to_string(); __tmp_x == __tmp_y } {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ImportSpecPtr((*d.lock().unwrap().as_ref().unwrap()).spec.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INIT_DECL as i32))))))), Arc::new(Mutex::new(Some("cannot import package as init - init must be a func".to_string()))));
        return;
    };
        if !{ let __map = { let __map_holder = pkgImports_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(imp.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        { let __map_key = GoLocalPtrKey::new(imp.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*pkgImports_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let new_val = { let __append_target = (*pkg_closure_clone.lock().unwrap().as_ref().unwrap()).imports.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(imp.clone()); __append_target.clone() }; (*pkg_closure_clone.lock().unwrap().as_mut().unwrap()).imports = new_val; };
    };
        let mut pkgName = new_pkg_name((*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).pos(), pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), imp.clone());;
        if { let __nil_target = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        check_closure_clone_closure_clone.record_def({ let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(pkgName.clone())) as Box<dyn Object + Send + Sync>))));
    } else {
        check_closure_clone_closure_clone.record_implicit(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ImportSpecPtr((*d.lock().unwrap().as_ref().unwrap()).spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(pkgName.clone())) as Box<dyn Object + Send + Sync>))));
    };
        if (*{ let __field = (*imp.lock().unwrap().as_ref().unwrap()).fake.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __map_key = GoLocalPtrKey::new(pkgName.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*check_closure_clone_closure_clone.used_pkg_names.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    };
        { let new_val = { let __append_target = check_closure_clone_closure_clone.imports.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(pkgName.clone()); __append_target.clone() }; check_closure_clone_closure_clone.imports = new_val; };;
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".".to_string(); __tmp_x == __tmp_y } {
        if { let __nil_target = check_closure_clone_closure_clone.dot_import_map.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<crate::check::dotImportKey, Arc<Mutex<Option<crate::object::PkgName>>>>::new()))); check_closure_clone_closure_clone.dot_import_map = new_val; };
    }
        for (name, obj) in { let __range_holder = (*(*imp.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if go_token::is_exported(Arc::new(Mutex::new(Some(name.clone())))) {
        {
        let mut alt = { let __recv = fileScope_closure_clone.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.lookup(Arc::new(Mutex::new(Some(name.clone())))); __result };;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            let mut err = check_closure_clone_closure_clone.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s redeclared in this block".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*alt.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(alt.clone()); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
        } else {
            { let __recv = fileScope_closure_clone.clone(); let __recv_ptr: *mut crate::scope::Scope = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::scope::Scope }; let __result = unsafe { &mut *__recv_ptr }.insert_1(Arc::new(Mutex::new(Some(name.clone()))), obj.clone()); __result };;
            { let __map_key = dotImportKey { scope: fileScope_closure_clone.clone(), name: Arc::new(Mutex::new(Some(name.clone()))), ..Default::default() }; let __map_value = pkgName.clone(); (*check_closure_clone_closure_clone.dot_import_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }
    }
    } else {
        check_closure_clone_closure_clone.declare(fileScope_closure_clone.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(pkgName.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::constDecl>()).is_some() {
        let mut d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::constDecl>()).unwrap().clone())));
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        let mut obj = new_const({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)), go_constant::make_int64(Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).iota.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64)))).clone());
        let mut init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
    }
        let mut d = Arc::new(Mutex::new(Some(declInfo { file: fileScope_closure_clone.clone(), version: Arc::new(Mutex::new(Some({ let __selector_holder = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), vtyp: { let __field = (*d.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, init: init.clone(), inherited: Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).inherited.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
        check_closure_clone_closure_clone.declare_pkg_obj((*name).clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::ConstPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), d.clone());
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::varDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::varDecl>()).unwrap().clone())));
        let mut lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));;
        let mut d1: Arc<Mutex<Option<declInfo>>> = Arc::new(Mutex::new(None));;
        if { let __tmp_x = (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(declInfo { file: fileScope_closure_clone.clone(), version: Arc::new(Mutex::new(Some({ let __selector_holder = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), lhs: lhs.clone(), vtyp: { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, init: { let __seq = { let __seq_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), ..Default::default() }))).clone(); d1 = new_val; };
    };
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        let mut obj = new_var({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));
        (*lhs.lock().unwrap().as_mut().unwrap())[(i) as usize] = obj.clone();
        let mut di = d1.clone();
        if { let __nil_result = (*di.lock().unwrap()).is_none(); __nil_result } {
        let mut init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
    }
        { let new_val = Arc::new(Mutex::new(Some(declInfo { file: fileScope_closure_clone.clone(), version: Arc::new(Mutex::new(Some({ let __selector_holder = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), vtyp: { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, init: init.clone(), ..Default::default() }))).clone(); di = new_val; };
    }
        check_closure_clone_closure_clone.declare_pkg_obj((*name).clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), di.clone());
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::typeDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::typeDecl>()).unwrap().clone())));
        let mut obj = new_type_name((*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos(), pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));;
        { let __method_arg0 = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg2 = Arc::new(Mutex::new(Some(declInfo { file: fileScope_closure_clone.clone(), version: Arc::new(Mutex::new(Some({ let __selector_holder = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), tdecl: { let __field = (*d.lock().unwrap().as_ref().unwrap()).spec.clone(); __field }, ..Default::default() }))); check_closure_clone_closure_clone.declare_pkg_obj(__method_arg0, __method_arg1, __method_arg2) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::funcDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::decl::funcDecl>()).unwrap().clone())));
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        let mut obj = new_func((*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos(), pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)));;
        let mut hasTParamError = Arc::new(Mutex::new(Some(false)));;
        if { let __tmp_x = (*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __nil_target = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        check_closure_clone_closure_clone.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldListPtr((*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).recv.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_RECV as i32))))))), Arc::new(Mutex::new(Some("method has no receiver".to_string()))));
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "init".to_string(); __tmp_x == __tmp_y } || ({ let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "main".to_string(); __tmp_x == __tmp_y } && { let __tmp_x = { let __selector_holder = (*check_closure_clone_closure_clone.pkg.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "main".to_string(); __tmp_x == __tmp_y }) {
        let mut code = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_INIT_DECL as i32)))))));
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "main".to_string(); __tmp_x == __tmp_y } {
        { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_MAIN_DECL as i32)))); *code.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = (*(*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x != __tmp_y } {
        check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr({ let __seq = { let __seq_holder = (*(*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("func %s must have no type parameters".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = true; *hasTParamError.lock().unwrap() = Some(new_val); };
    }
        {
        let mut t = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).r#type.clone();;
        if { let __tmp_x = (*(*t.lock().unwrap().as_ref().unwrap()).params.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x != __tmp_y } || { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).results.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("func %s must have no arguments and no return values".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        }
    }
    }
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "init".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*pkg_closure_clone.lock().unwrap().as_ref().unwrap()).scope.clone(); (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).parent = new_val; };
        check_closure_clone_closure_clone.record_def({ let __field = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
        if { let __nil_target = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        check_closure_clone_closure_clone.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_INIT_BODY as i32))))))), Arc::new(Mutex::new(Some("missing function body".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
    } else {
        check_closure_clone_closure_clone.declare({ let __field = (*pkg_closure_clone.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }, { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    } else {
        let (mut ptr, mut base, _) = check_closure_clone_closure_clone.unpack_recv({ let __field = (*{ let __seq = { let __seq_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, Arc::new(Mutex::new(Some(false))));
        {
        let (mut recv, _) = ({
        let val = base.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if { let __nil_result = (*recv.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
            { let __append_target = methods_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(methodInfo { obj: obj.clone(), ptr: Arc::new(Mutex::new(Some(ptr))), recv: recv.clone(), ..Default::default() }); __append_target.clone() };;
        }
    }
        check_closure_clone_closure_clone.record_def({ let __field = (*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
    };
        let _ = { let __tmp_x = (*(*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x != __tmp_y } && !{ let __v = (*hasTParamError.lock().unwrap().as_ref().unwrap()).clone(); __v } && check_closure_clone_closure_clone.verify_versionf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr({ let __seq = { let __seq_holder = (*(*(*(*d.lock().unwrap().as_ref().unwrap()).decl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type parameter".to_string()))), Arc::new(Mutex::new(Some(vec![]))));;
        let mut info = Arc::new(Mutex::new(Some(declInfo { file: fileScope_closure_clone.clone(), version: Arc::new(Mutex::new(Some({ let __selector_holder = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), fdecl: { let __field = (*d.lock().unwrap().as_ref().unwrap()).decl.clone(); __field }, ..Default::default() })));;
        { let __map_key = GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>)))); let __map_value = info.clone(); (*check_closure_clone_closure_clone.obj_map.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        { let __recv = obj.clone(); let __recv_ptr: *mut crate::object::Func = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::object::Func }; let __result = unsafe { &mut *__recv_ptr }.set_order(Arc::new(Mutex::new(Some(({ let __len_target = { let __field = check_closure_clone_closure_clone.obj_map.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as u32)))); __result };;
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }))); __recv.walk_decls(__method_arg0, __method_arg1) };
    } }
                // The package identifier denotes the current package,
                // but there is no corresponding package object.
                // Use the actual source file extent rather than *ast.File extent since the
                // latter doesn't include comments which appear at the start or end of the file.
                // Be conservative and use the *ast.File extent if we don't have a *token.File.
                // determine file directory, necessary to resolve imports
                // FileName may be "" (typically for tests) in which case
                // we get "." as the directory which is what we would want.
                // import package
                // error reported by parser
                // local name overrides imported package name
                // match 1.17 cmd/compile (not prescribed by spec)
                // add package to list of explicit imports
                // (this functionality is provided as a convenience
                // for clients; it is not needed for type-checking)
                // in a dot-import, the dot represents the package
                // match 1.17 cmd/compile (not prescribed by spec)
                // add import to file scope
                // dot-import
                // merge imported scope with file scope
                // Note: Avoid eager resolve(name, obj) here, so we only
                // resolve dot-imported objects as needed.
                // A package scope may contain non-exported objects,
                // do not import them!
                // declare dot-imported object
                // (Do not use check.declare because it modifies the object
                // via Object.setScopePos, which leads to a race condition;
                // the object may be imported into more than one file scope
                // concurrently. See go.dev/issue/32154.)
                // declare imported package object in file scope
                // (no need to provide s.Name since we called check.recordDef earlier)
                // declare all constants
                // If there's exactly one rhs initializer, use
                // the same declInfo d1 for all lhs variables
                // so that each lhs variable depends on the same
                // rhs initializer (n:1 var declaration).
                // The lhs elements are only set up after the for loop below,
                // but that's ok because declareVar only collects the declInfo
                // for a later phase.
                // declare all variables
                // individual assignments
                // signature set later
                // avoid duplicate type parameter errors
                // regular function
                // treat as function
                // TODO(rFindley) Should this be a hard error?
                // don't declare init functions in the package scope - they are invisible
                // init functions must have a body
                // TODO(gri) make this error message consistent with the others above
                // method
                // TODO(rFindley) earlier versions of this code checked that methods
                //                have no type parameters, but this is checked later
                //                when type checking the function type. Confirm that
                //                we don't need to check tparams here.
                // (Methods with invalid receiver cannot be associated to a type, and
                // methods with blank _ names are never found; no need to collect any
                // of them. They will still be type-checked with all the other functions.)
                // Methods are not package-level objects but we still track them in the
                // object map so that we can handle them like regular functions (if the
                // receiver is invalid); also we need their fdecl info when associating
                // them with their receiver base type, below.
                // verify that objects in package and file scopes have different names
        { let __range_holder = fileScopes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for scope in __range_values.iter() {
        for (name, mut obj) in { let __range_holder = (*scope.lock().unwrap().as_ref().unwrap()).elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        {
        let mut alt = (*(*pkg.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some(name.clone()))));;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            { let __iface_handle = resolve(Arc::new(Mutex::new(Some(name.clone()))), obj.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };;
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32))))))));;
            {
        let (mut pkg, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::PkgNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::PkgName>)), false)
        }
    });;
        if ok {
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s already declared through import of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*alt.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __recv = pkg.clone(); let __recv_ptr: *const crate::object::PkgName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::PkgName }; let __result = unsafe { &*__recv_ptr }.imported(); __result }.clone()) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(pkg.clone())) as Box<dyn Object + Send + Sync>)))); __result };;
        } else {
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s already declared through dot-import of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*alt.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new((*obj.lock().unwrap().as_ref().unwrap()).pkg().clone()) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(obj.clone()); __result };;
        }
    };
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
        }
    }
    }
    } }
                // TODO(gri) dot-imported objects don't have a position; addAltDecl won't print anything
                // Now that we have all package scope objects and all methods,
                // associate methods with receiver base type name where possible.
                // Ignore methods that have an invalid receiver. They will be
                // type-checked later, with regular functions.
        if { let __nil_result = (*methods.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Func>>>>>>>>::new()))); self.methods = new_val; };
        for i in 0..(({ let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut m: Option<GoSliceElemPtr<methodInfo>> = Some(GoSliceElemPtr::new(methods.clone(), (i) as usize));
                // Determine the receiver base type and associate m with it.
        let (mut ptr, mut base) = self.resolve_base_type_name(Arc::new(Mutex::new(Some({ let __selector_holder = (*m.as_ref().unwrap().borrow().as_ref().unwrap()).ptr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), { let __field = (*m.as_ref().unwrap().borrow().as_ref().unwrap()).recv.clone(); __field });
        if { let __nil_result = (*base.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = ptr; *(*(*m.as_ref().unwrap().borrow().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).has_ptr_recv_.lock().unwrap() = Some(new_val); };
        { let __map_key = GoLocalPtrKey::new(base.clone()); let __map_value = { let __slice = { let __map_holder = self.methods.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&GoLocalPtrKey::new(base.clone())).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __field = (*m.as_ref().unwrap().borrow().as_ref().unwrap()).obj.clone(); __field }); __slice.clone() }; (*self.methods.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    }
    }

    /// unpackRecv unpacks a receiver type expression and returns its components: ptr indicates
    /// whether rtyp is a pointer receiver, base is the receiver base type expression stripped
    /// of its type parameters (if any), and tparams are its type parameter names, if any. The
    /// type parameters are only unpacked if unpackParams is set. For instance, given the rtyp
    ///
    ///	*T[A, _]
    ///
    /// ptr is true, base is T, and tparams is [A, _] (assuming unpackParams is set).
    /// Note that base may not be a *ast.Ident for erroneous programs.
    pub fn unpack_recv(&self, rtyp: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, unpackParams: Arc<Mutex<Option<bool>>>) -> (bool, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>>) {
    let mut ptr: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut base: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<go_ast::r#mod::Ident>>>>>>> = Arc::new(Mutex::new(None));

                // unpack receiver type
        { let __iface_handle = go_ast::unparen(rtyp.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *base.lock().unwrap() = __iface_value; };
        {
        let (mut t, _) = ({
        let val = base.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::StarExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
        }
    });;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = true; *ptr.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = go_ast::unparen({ let __field = (*t.lock().unwrap().as_ref().unwrap()).x.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *base.lock().unwrap() = __iface_value; };;
        }
    }
                // unpack type parameters, if any
        {
    let _ts_subject = base.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexExprPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IndexListExprPtr>()).is_some() {
        let mut ix = unpack_indexed_expr(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn go_ast::r#mod::Node + Send + Sync>))));;
        { let __iface_handle = { let __field = (*ix.lock().unwrap().as_ref().unwrap()).x.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *base.lock().unwrap() = __iface_value; };;
        if { let __v = (*unpackParams.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __range_holder = (*ix.lock().unwrap().as_ref().unwrap()).indices.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut arg in __range_values.iter().cloned() {
        let mut par: Arc<Mutex<Option<go_ast::r#mod::Ident>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = arg.clone();
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
        let arg = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IdentPtr>()).unwrap().0.clone();
        { let new_val = arg.clone(); par = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExprPtr>()).is_some() {
        let arg = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadExprPtr>()).unwrap().0.clone();
    } else if _ts_is_nil {
        let arg = _ts_subject.clone();
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*ix.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("parameterized receiver contains nil parameters".to_string()))));;
    } else {
        let arg = _ts_subject.clone();
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DECL as i32))))))), Arc::new(Mutex::new(Some("receiver type parameter %s must be an identifier".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = arg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
    }
    }
        if { let __nil_result = (*par.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { name_pos: (*arg.lock().unwrap().as_ref().unwrap()).pos(), name: Arc::new(Mutex::new(Some("_".to_string()))), ..Default::default() }))).clone(); par = new_val; };
    }
        { let new_val = { let __append_target = tparams.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(par.clone()); __append_target.clone() }; tparams = new_val; };
    } }
    };
    }
    }
                // ignore - error already reported by parser
        return ((*ptr.lock().unwrap().as_ref().unwrap()), base.clone(), tparams.clone());
    }

    /// resolveBaseTypeName returns the non-alias base type name for the given name, and whether
    /// there was a pointer indirection to get to it. The base type name must be declared
    /// in package scope, and there can be at most one pointer indirection. Traversals
    /// through generic alias types are not permitted. If no such type name exists, the
    /// returned base is nil.
    pub fn resolve_base_type_name(&self, mut ptr: Arc<Mutex<Option<bool>>>, mut name: Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> (bool, Arc<Mutex<Option<crate::object::TypeName>>>) {
    let mut ptr_: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut base: Arc<Mutex<Option<TypeName>>> = Arc::new(Mutex::new(None));

                // Algorithm: Starting from name, which is expected to denote a type,
                // we follow that type through non-generic alias declarations until
                // we reach a non-alias type name.
        let mut seen: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<bool>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
        while { let __nil_result = (*name.lock().unwrap()).is_some(); __nil_result } {
                // name must denote an object found in the current package scope
                // (note that dot-imported objects are not in the package scope!)
        let mut obj = (*(*self.pkg.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __nil_result = (*obj.lock().unwrap()).is_none(); __nil_result } {
        break
    }

                // the object must be a type name...
        let (mut tname, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
        }
    });
        if { let __nil_result = (*tname.lock().unwrap()).is_none(); __nil_result } {
        break
    }

                // ... which we have not seen before
        if { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(tname.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        break
    }

                // we're done if tdecl describes a defined type (not an alias)
        let mut tdecl = (*{ let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(tname.clone())) as Box<dyn Object + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).tdecl.clone();
        if !go_token::position::Pos::is_valid(&(*(*tdecl.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap())) {
        return ({ let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v }, tname.clone());
    }

                // an alias must not be generic
                // (importantly, we must not collect such methods - was https://go.dev/issue/70417)
        if { let __nil_target = (*tdecl.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        break
    }

                // otherwise, remember this type name and continue resolving
        if { let __nil_result = (*seen.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::TypeName>, Arc<Mutex<Option<bool>>>>::new()))); seen = new_val; };
    }
        { let __map_key = GoLocalPtrKey::new(tname.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

                // The go/parser keeps parentheses; strip them, if any.
        let mut typ = go_ast::unparen({ let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field });

                // dereference a pointer type
        {
        let (mut pexpr, _) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::StarExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::StarExpr>)), false)
        }
    });;
        if { let __nil_result = (*pexpr.lock().unwrap()).is_some(); __nil_result } {
            if { let __v = (*ptr.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break
    };
            { let new_val = true; *ptr.lock().unwrap() = Some(new_val); };;
            { let __iface_handle = go_ast::unparen({ let __field = (*pexpr.lock().unwrap().as_ref().unwrap()).x.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *typ.lock().unwrap() = __iface_value; };;
        }
    }

                // if we've already seen a pointer, we're done
                // continue with pointer base type
                // After dereferencing, typ must be a locally defined type name.
                // Referring to other packages (qualified identifiers) or going
                // through instantiated types (index expressions) is not permitted,
                // so we can ignore those.
        { let (__tmp_0, __tmp_1) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    }); name = __tmp_0.clone(); };
        if { let __nil_result = (*name.lock().unwrap()).is_none(); __nil_result } {
        break
    }
    }
                // name must denote an object found in the current package scope
                // (note that dot-imported objects are not in the package scope!)
                // the object must be a type name...
                // ... which we have not seen before
                // we're done if tdecl describes a defined type (not an alias)
                // must exist for objects in package scope
                // an alias must not be generic
                // (importantly, we must not collect such methods - was https://go.dev/issue/70417)
                // otherwise, remember this type name and continue resolving
                // The go/parser keeps parentheses; strip them, if any.
                // dereference a pointer type
                // if we've already seen a pointer, we're done
                // continue with pointer base type
                // After dereferencing, typ must be a locally defined type name.
                // Referring to other packages (qualified identifiers) or going
                // through instantiated types (index expressions) is not permitted,
                // so we can ignore those.
                // no base type found
        (false, Arc::new(Mutex::new(None)))
    }

    /// packageObjects typechecks all package objects, but not function bodies.
    pub fn package_objects(&mut self) {
                // process package objects in source order for reproducible results
        let mut objList: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); (({ let __len_target = { let __field = self.obj_map.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));
        let mut i = Arc::new(Mutex::new(Some(0)));
        for (__range_key, _) in { let __range_holder = self.obj_map.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let obj = __range_key.value();
        (*objList.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = obj.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        slices::sort_func::<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>, Box<dyn Object + Send + Sync>>(objList.clone(), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, b: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>| -> i32 {
        cmp::compare::<u32>((*a.lock().unwrap().as_ref().unwrap()).order(), (*b.lock().unwrap().as_ref().unwrap()).order())
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> i32 + Send + Sync>))));
                // add new methods to already type-checked types (from a prior Checker.Files call)
        { let __range_holder = objList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut obj in __range_values.iter().cloned() {
        {
        let (mut obj, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
        }
    });;
        if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } && { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
            self.collect_methods(obj.clone());;
        }
    }
    } }
        if false && (*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) {
                // With Alias nodes we can process declarations in any order.
                //
                // TODO(adonovan): unfortunately, Alias nodes
                // (GODEBUG=gotypesalias=1) don't entirely resolve
                // problems with cycles. For example, in
                // GOROOT/test/typeparam/issue50259.go,
                //
                // 	type T[_ any] struct{}
                // 	type A T[B]
                // 	type B = T[A]
                //
                // TypeName A has Type Named during checking, but by
                // the time the unified export data is written out,
                // its Type is Invalid.
                //
                // Investigate and reenable this branch.
        { let __range_holder = objList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        self.obj_decl(obj.clone(), Arc::new(Mutex::new(None)));
    } }
    } else {
                // Without Alias nodes, we process non-alias type declarations first, followed by
                // alias declarations, and then everything else. This appears to avoid most situations
                // where the type of an alias is needed before it is available.
                // There may still be cases where this is not good enough (see also go.dev/issue/25838).
                // In those cases Checker.ident will report an error ("invalid use of type alias").
        let mut aliasList: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeName>>>>>>> = Arc::new(Mutex::new(None));
        let mut othersList: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
                // phase 1: non-alias type declarations
        { let __range_holder = objList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        {
        let (mut tname, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
        }
    });;
        if { let __nil_result = (*tname.lock().unwrap()).is_some(); __nil_result } {
            if go_token::position::Pos::is_valid(&(*(*(*{ let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(tname.clone())) as Box<dyn Object + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).tdecl.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap())) {
        { let new_val = { let __append_target = aliasList.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(tname.clone()); __append_target.clone() }; aliasList = new_val; };
    } else {
        self.obj_decl(obj.clone(), Arc::new(Mutex::new(None)));
    };
        } else {
            { let new_val = { let __append_target = othersList.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*obj).clone()); __append_target.clone() }; othersList = new_val; };;
        }
    }
    } }
                // phase 2: alias type declarations
        { let __range_holder = aliasList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        self.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)));
    } }
                // phase 3: all other declarations
        { let __range_holder = othersList.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        self.obj_decl(obj.clone(), Arc::new(Mutex::new(None)));
    } }
    }
                // With Alias nodes we can process declarations in any order.
                //
                // TODO(adonovan): unfortunately, Alias nodes
                // (GODEBUG=gotypesalias=1) don't entirely resolve
                // problems with cycles. For example, in
                // GOROOT/test/typeparam/issue50259.go,
                //
                // 	type T[_ any] struct{}
                // 	type A T[B]
                // 	type B = T[A]
                //
                // TypeName A has Type Named during checking, but by
                // the time the unified export data is written out,
                // its Type is Invalid.
                //
                // Investigate and reenable this branch.
                // Without Alias nodes, we process non-alias type declarations first, followed by
                // alias declarations, and then everything else. This appears to avoid most situations
                // where the type of an alias is needed before it is available.
                // There may still be cases where this is not good enough (see also go.dev/issue/25838).
                // In those cases Checker.ident will report an error ("invalid use of type alias").
                // everything that's not a type
                // phase 1: non-alias type declarations
                // phase 2: alias type declarations
                // phase 3: all other declarations
                // At this point we may have a non-empty check.methods map; this means that not all
                // entries were deleted at the end of typeDecl because the respective receiver base
                // types were not found. In that case, an error was reported when declaring those
                // methods. We can now safely discard this map.
        { let new_val = Arc::new(Mutex::new(None)); self.methods = new_val; };
    }

    /// unusedImports checks for unused imports.
    pub fn unused_imports(&self) {
                // If function bodies are not checked, packages' uses are likely missing - don't check.
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).ignore_func_bodies.lock().unwrap().as_ref().unwrap()) {
        return;
    }
                // spec: "It is illegal (...) to directly import a package without referring to
                // any of its exported identifiers. To import a package solely for its side-effects
                // (initialization), use the blank identifier as explicit package name."
        { let __range_holder = self.imports.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } && !{ let __map = { let __map_holder = self.used_pkg_names.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(obj.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        self.error_unused_pkg((*obj).clone());
    }
    } }
    }

    pub fn error_unused_pkg(&self, obj: Arc<Mutex<Option<PkgName>>>) {
                // If the package was imported with a name other than the final
                // import path element, show it explicitly in the error message.
                // Note that this handles both renamed imports and imports of
                // packages containing unconventional package declarations.
                // Note that this uses / always, even on Windows, because Go import
                // paths always use forward slashes.
        let mut path = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_ref().unwrap()).imported.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut elem = { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        {
        let mut i = Arc::new(Mutex::new(Some({ let __s = (*elem.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "/".to_string(); __s.rfind(&__substr).map(|__i| __i as i32).unwrap_or(-1) })));;
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*elem.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *elem.lock().unwrap() = __moved_val; };;
        }
    }
        if { let __tmp_x = { let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ".".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*elem.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(obj.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_IMPORT as i32))))))), Arc::new(Mutex::new(Some("%q imported and not used".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::PkgNamePtr(obj.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_IMPORT as i32))))))), Arc::new(Mutex::new(Some("%q imported as %s and not used".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
    }
}

pub fn validated_import_path(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut s, mut err) = strconv::unquote({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), err.clone());
    }
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("empty string"))))));
    }
    const illegalChars: &'static str = "!\"#$%&'()*,:;<=>?[\\]^{|}`\u{fffd}";

    for (_, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if !unicode::is_graphic(Arc::new(Mutex::new(Some(r as i32)))) || unicode::is_space(Arc::new(Mutex::new(Some(r as i32)))) || strings::contains_rune("!\"#$%&'()*,:;<=>?[\\]^{|}`\u{fffd}".to_string(), r) {
        return ({ let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("invalid character U+{:04X}", r as u32))))));
    }
    }
    return ({ let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
}

/// dir makes a good-faith attempt to return the directory
/// portion of path. If path is empty, the result is ".".
/// (Per the go/build package dependency tests, we cannot import
/// path/filepath and simply use filepath.Dir.)
pub fn dir(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut i = strings::last_index_any({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, "/\\".to_string());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x > __tmp_y } {
            return Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() })));;
        }
    }

        // i <= 0
    Arc::new(Mutex::new(Some(".".to_string())))
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub obj: Arc<Mutex<Option<Func>>>,
    pub ptr: Arc<Mutex<Option<bool>>>,
    pub recv: Arc<Mutex<Option<go_ast::r#mod::Ident>>>,
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


impl GoValueClone for declInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
