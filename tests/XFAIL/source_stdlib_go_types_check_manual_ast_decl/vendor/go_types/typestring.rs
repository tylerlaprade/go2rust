use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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
use crate::typeterm::*;
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Qualifier controls how named package-level objects are printed in
/// calls to [TypeString], [ObjectString], and [SelectionString].
///
/// These three formatting routines call the Qualifier for each
/// package-level object O, and if the Qualifier returns a non-empty
/// string p, the object is printed in the form p.O.
/// If it returns an empty string, only the object name O is printed.
///
/// Using a nil Qualifier is equivalent to using (*[Package]).Path: the
/// object is qualified by the import path, e.g., "encoding/json.Marshal".
pub type Qualifier = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>>>>;


#[derive(Clone)]
pub struct typeWriter {
    pub buf: Arc<Mutex<Option<bytes_Buffer>>>,
    pub seen: Arc<Mutex<Option<BTreeMap<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>>>>,
    pub qf: Qualifier,
    pub ctxt: Arc<Mutex<Option<Context>>>,
    pub tparams: Arc<Mutex<Option<TypeParamList>>>,
    pub param_names: Arc<Mutex<Option<bool>>>,
    pub tp_subscripts: Arc<Mutex<Option<bool>>>,
    pub pkg_info: Arc<Mutex<Option<bool>>>,
}

impl typeWriter {
    pub fn __go_value_clone(&self) -> Self {
        Self { buf: self.buf.clone(), seen: self.seen.clone(), qf: self.qf.clone(), ctxt: self.ctxt.clone(), tparams: self.tparams.clone(), param_names: { let __guard = self.param_names.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, tp_subscripts: { let __guard = self.tp_subscripts.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg_info: { let __guard = self.pkg_info.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for typeWriter {
    fn default() -> Self {
        Self { buf: Arc::new(Mutex::new(None)), seen: Arc::new(Mutex::new(None)), qf: Arc::new(Mutex::new(None)), ctxt: Arc::new(Mutex::new(None)), tparams: Arc::new(Mutex::new(None)), param_names: Arc::new(Mutex::new(Some(false))), tp_subscripts: Arc::new(Mutex::new(Some(false))), pkg_info: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for typeWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_map(&self.seen), "<func>", { let __guard = self.ctxt.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.tparams.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.param_names.lock().unwrap().as_ref().unwrap()), (*self.tp_subscripts.lock().unwrap().as_ref().unwrap()), (*self.pkg_info.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for typeWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
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


impl typeWriter {
    pub fn byte(&self, mut b: Arc<Mutex<Option<u8>>>) {
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (' ' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = ('#' as i32) as u8; *b.lock().unwrap() = Some(new_val); };
    }
        (*self.buf.lock().unwrap().as_mut().unwrap()).write_byte({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        return;
    }
        (*self.buf.lock().unwrap().as_mut().unwrap()).write_byte({ let __arg_holder = b.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (',' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (';' as i32) as u8; __tmp_x == __tmp_y } {
        (*self.buf.lock().unwrap().as_mut().unwrap()).write_byte((' ' as i32) as u8);
    }
    }

    pub fn string(&self, s: Arc<Mutex<Option<String>>>) {
        (*self.buf.lock().unwrap().as_mut().unwrap()).write_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    }

    pub fn error(&self, msg: Arc<Mutex<Option<String>>>) {
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        std::panic::panic_any(Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>);
    }
        (*self.buf.lock().unwrap().as_mut().unwrap()).write_string({ let mut __s = String::new(); __s.push_str(&format!("{}", "<".to_string())); __s.push_str(&format!("{}", { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", ">".to_string())); __s });
    }

    pub fn typ(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __map = { let __map_holder = self.seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoTypeInterfaceKey::new(typ.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        self.error(Arc::new(Mutex::new(Some(format!("{}{}", "cycle to ".to_string(), (*go_type_name(typ.clone()).lock().unwrap().as_ref().unwrap()))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            { let __map_key = GoTypeInterfaceKey::new(typ.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
            let typ_defer_captured = typ.clone(); let mut w_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __map_handle = w_defer_captured.seen.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoTypeInterfaceKey::new(typ_defer_captured.clone())); };
    }));
            '__go_switch_1: loop {
    {
    let _ts_subject = typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let t = _ts_subject.clone();
        self.error(Arc::new(Mutex::new(Some("nil".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        {
        let (mut obj, _) = ({
        let val = (*(*(*Unsafe.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).clone();
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
        if (*obj.lock().unwrap()).is_some() {
            self.type_name(obj.clone());;
            break '__go_switch_1;
        }
    }
    };
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        self.byte(Arc::new(Mutex::new(Some(('[' as i32) as u8))));;
        self.string(Arc::new(Mutex::new(Some(go_strconv_format_int((*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()) as i64, 10 as i32)))));;
        self.byte(Arc::new(Mutex::new(Some((']' as i32) as u8))));;
        self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        self.string(Arc::new(Mutex::new(Some("[]".to_string()))));;
        self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        self.string(Arc::new(Mutex::new(Some("struct{".to_string()))));;
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.byte(Arc::new(Mutex::new(Some((';' as i32) as u8))));
    }
        let mut pkgAnnotate = Arc::new(Mutex::new(Some(false)));
        if { let __nil_target = self.qf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && (*self.pkg_info.clone().lock().unwrap().as_ref().unwrap()) && !is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        { let new_val = true; *pkgAnnotate.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.pkg_info.lock().unwrap() = Some(new_val); };
    }
        if !(*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
    }
        self.typ((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone());
        if { let __v = (*pkgAnnotate.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.string(Arc::new(Mutex::new(Some(" /* package ".to_string()))));
        self.string((*(*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.lock().unwrap().as_ref().unwrap()).path());
        self.string(Arc::new(Mutex::new(Some(" */ ".to_string()))));
    }
        {
        let mut tag = { let __recv = t.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.tag(Arc::new(Mutex::new(Some(i as i32)))); __result };;
        if { let __tmp_x = (*tag.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));;
            self.string(Arc::new(Mutex::new(Some(format!("{:?}", (*tag.lock().unwrap().as_ref().unwrap()).clone())))));;
        }
    }
    } };
        self.byte(Arc::new(Mutex::new(Some(('}' as i32) as u8))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        self.byte(Arc::new(Mutex::new(Some(('*' as i32) as u8))));;
        self.typ((*t.lock().unwrap().as_ref().unwrap()).base.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).unwrap().0.clone();
        self.tuple(t.clone(), Arc::new(Mutex::new(Some(false))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        self.string(Arc::new(Mutex::new(Some("func".to_string()))));;
        self.signature(t.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __recv = t.clone(); let __recv_ptr: *const crate::union::Union = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::union::Union }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some("empty union".to_string()))));
        break '__go_switch_1
    };
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, t) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.string(Arc::new(Mutex::new(Some(" | ".to_string()))));
    }
        if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        self.byte(Arc::new(Mutex::new(Some(('~' as i32) as u8))));
    }
        self.typ((*t.lock().unwrap().as_ref().unwrap()).typ.clone());
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if { let __left_wrapper = crate::interface::InterfacePtr(t.clone()); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = { let __recv = { let __recv_holder = (*universeAnyAlias.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.r#type(); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).underlying(); __result }.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        self.string(Arc::new(Mutex::new(Some("any".to_string()))));
        break '__go_switch_1
    }
        if { let __left_wrapper = crate::interface::InterfacePtr(t.clone()); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = (*as_named((*universeComparable.lock().unwrap().as_ref().unwrap()).r#type().clone()).lock().unwrap().as_ref().unwrap()).underlying.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        self.string(Arc::new(Mutex::new(Some("interface{comparable}".to_string()))));
        break '__go_switch_1
    }
    };
        if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).implicit.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        if { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        self.typ({ let __seq = { let __seq_holder = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone());
        break '__go_switch_1
    }
        self.string(Arc::new(Mutex::new(Some("/* implicit */ ".to_string()))));
    };
        self.string(Arc::new(Mutex::new(Some("interface{".to_string()))));;
        let mut first = Arc::new(Mutex::new(Some(true)));;
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.type_set({ let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result });
    } else {
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        if !{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some((';' as i32) as u8))));
    }
        { let new_val = false; *first.lock().unwrap() = Some(new_val); };
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        self.signature(({
        let val = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }));
    } }
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for typ in __range_values.iter() {
        if !{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some((';' as i32) as u8))));
    }
        { let new_val = false; *first.lock().unwrap() = Some(new_val); };
        self.typ(typ.clone());
    } }
    };
        self.byte(Arc::new(Mutex::new(Some(('}' as i32) as u8))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        self.string(Arc::new(Mutex::new(Some("map[".to_string()))));;
        self.typ((*t.lock().unwrap().as_ref().unwrap()).key.clone());;
        self.byte(Arc::new(Mutex::new(Some((']' as i32) as u8))));;
        self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));;
        let mut parens: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));;
        { let _switch_val = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::chan::ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32))))) {
            { let new_val = "chan ".to_string(); *s.lock().unwrap() = Some(new_val); };
            {
        let (mut c, _) = ({
        let val = (*t.lock().unwrap().as_ref().unwrap()).elem.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
        }
    });;
        if (*c.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*c.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(RECV_ONLY as i32)))); __tmp_x == __tmp_y } {
            { let new_val = true; *parens.lock().unwrap() = Some(new_val); };;
        }
    }
        } else if _switch_val == (crate::chan::ChanDir(Arc::new(Mutex::new(Some(SEND_ONLY as i32))))) {
            { let new_val = "chan<- ".to_string(); *s.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (crate::chan::ChanDir(Arc::new(Mutex::new(Some(RECV_ONLY as i32))))) {
            { let new_val = "<-chan ".to_string(); *s.lock().unwrap() = Some(new_val); };
        } else {
            self.error(Arc::new(Mutex::new(Some("unknown channel direction".to_string()))));
        }
    };
        self.string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __v = (*parens.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some(('(' as i32) as u8))));
    };
        self.typ((*t.lock().unwrap().as_ref().unwrap()).elem.clone());;
        if { let __v = (*parens.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some((')' as i32) as u8))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __method_arg0 = Arc::new(Mutex::new(Some(((*self.ctxt.lock().unwrap().as_mut().unwrap()).get_i_d(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>))))).to_string()))); self.string(__method_arg0) };
    };
        self.type_name({ let __field = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); __field });;
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.type_list((*(*(*t.lock().unwrap().as_ref().unwrap()).inst.lock().unwrap().as_ref().unwrap()).targs.lock().unwrap().as_ref().unwrap()).list());
    } else if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __recv = { let __recv = t.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        self.t_param_list({ let __recv = { let __recv = t.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result });
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        if { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.error(Arc::new(Mutex::new(Some("unnamed type parameter".to_string()))));
        break '__go_switch_1
    };
        {
        let mut i = slices::index::<Vec<Arc<Mutex<Option<crate::typeparam::TypeParam>>>>, crate::typeparam::TypeParam>((*self.tparams.lock().unwrap().as_ref().unwrap()).list(), t.clone());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            self.string(Arc::new(Mutex::new(Some(format!("${}", i)))));;
        } else {
            self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
            if (*self.tp_subscripts.clone().lock().unwrap().as_ref().unwrap()) || { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.string(subscript(Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).id.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))));
    };
            if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && (*{ let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result }.lock().unwrap()).is_some() {
        if IS_TYPES2 {
        self.string(Arc::new(Mutex::new(Some(format!(" /* with {} declared at {} */", (*(*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.lock().unwrap().as_ref().unwrap()).clone(), (*(*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).pos().lock().unwrap().as_ref().unwrap()))))));
    } else {
        self.string(Arc::new(Mutex::new(Some("/* type parameter */".to_string()))));
    }
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).unwrap().0.clone();
        self.type_name({ let __field = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); __field });;
        {
        let mut list = (*(*t.lock().unwrap().as_ref().unwrap()).targs.lock().unwrap().as_ref().unwrap()).list();;
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
            self.type_list(list.clone());;
        } else if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __recv = { let __recv = t.clone(); let __recv_ptr: *mut crate::alias::Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::alias::Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        self.t_param_list({ let __recv = { let __recv = t.clone(); let __recv_ptr: *mut crate::alias::Alias = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::alias::Alias }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result });
    }
    };
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.typ(unalias((*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone()).clone());
    };
    } else {
        let t = _ts_subject.clone();
        self.string((*t.lock().unwrap().as_ref().unwrap()).string());;
    }
    };
    break;
}

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

    /// typeSet writes a canonical hash for an interface type set.
    pub fn type_set(&mut self, s: Arc<Mutex<Option<_TypeSet>>>) {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }))));
        let mut first = Arc::new(Mutex::new(Some(true)));
        { let __range_holder = (*s.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        if !{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some((';' as i32) as u8))));
    }
        { let new_val = false; *first.lock().unwrap() = Some(new_val); };
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        self.signature(({
        let val = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }));
    } }
        if (*(*s.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).is_all() {
        } else if (*(*s.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).is_empty() {
            self.string((*(*s.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).string());
        } else {
            let mut termHashes: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
            { let __range_holder = { let __named_slice = (*(*s.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for term in __range_values.iter() {
                // terms are not canonically sorted, so we sort their hashes instead.
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
        if (*{ let __field = (*term.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(('~' as i32) as u8);
    }
        { let __recv = new_type_hasher(buf.clone(), { let __field = self.ctxt.clone(); __field }); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).typ((*term.lock().unwrap().as_ref().unwrap()).typ.clone()); __result };
        { let new_val = { let __append_target = termHashes.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*(*buf.lock().unwrap().as_mut().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; termHashes = new_val; };
    } }
                        // terms are not canonically sorted, so we sort their hashes instead.
            slices::sort::<Vec<String>, String>(termHashes.clone());
            if !{ let __v = (*first.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.byte(Arc::new(Mutex::new(Some((';' as i32) as u8))));
    }
            self.string(strings::join(termHashes.clone(), Arc::new(Mutex::new(Some("|".to_string())))));
        }
    }

    pub fn type_list(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>) {
        self.byte(Arc::new(Mutex::new(Some(('[' as i32) as u8))));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.byte(Arc::new(Mutex::new(Some((',' as i32) as u8))));
    }
        self.typ(typ.clone());
    } }
        self.byte(Arc::new(Mutex::new(Some((']' as i32) as u8))));
    }

    pub fn t_param_list(&mut self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) {
        self.byte(Arc::new(Mutex::new(Some(('[' as i32) as u8))));
        let mut prev: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tpar) in __range_values.iter().enumerate() {
                // Determine the type parameter and its constraint.
                // list is expected to hold type parameter names,
                // but don't crash if that's not the case.
        if (*tpar.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some("nil type parameter".to_string()))));
        continue
    }
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __left_holder = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = prev.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } {
                // bound changed - write previous one before advancing
        self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
        self.typ(prev.clone());
    }
                // bound changed - write previous one before advancing
        self.byte(Arc::new(Mutex::new(Some((',' as i32) as u8))));
    }
                // bound changed - write previous one before advancing
        { let __iface_handle = (*tpar.lock().unwrap().as_ref().unwrap()).bound.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *prev.lock().unwrap() = (*__iface_guard).clone(); };
        self.typ(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(tpar.clone())) as Box<dyn Type + Send + Sync>))));
    } }
                // Determine the type parameter and its constraint.
                // list is expected to hold type parameter names,
                // but don't crash if that's not the case.
                // bound changed - write previous one before advancing
        if (*prev.lock().unwrap()).is_some() {
        self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
        self.typ(prev.clone());
    }
        self.byte(Arc::new(Mutex::new(Some((']' as i32) as u8))));
    }

    pub fn type_name(&self, obj: Arc<Mutex<Option<TypeName>>>) {
        { let __method_arg0 = package_prefix({ let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, self.qf.clone()); self.string(__method_arg0) };
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }

    pub fn tuple(&mut self, tup: Arc<Mutex<Option<Tuple>>>, variadic: Arc<Mutex<Option<bool>>>) {
        self.byte(Arc::new(Mutex::new(Some(('(' as i32) as u8))));
        if (*tup.lock().unwrap()).is_some() {
        { let __range_holder = (*tup.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        self.byte(Arc::new(Mutex::new(Some((',' as i32) as u8))));
    }
                // parameter names are ignored for type identity and thus type hashes
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } && { let __tmp_x = { let __selector_holder = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && (*self.param_names.clone().lock().unwrap().as_ref().unwrap()) {
        self.string(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
    }
        let mut typ = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        if { let __v = (*variadic.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (i as i32); let __tmp_y = ({ let __tmp_x = (({ let __len_target = { let __field = (*tup.lock().unwrap().as_ref().unwrap()).vars.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x == __tmp_y } {
        {
        let (mut s, mut ok) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::slice::SlicePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
        }
    });;
        if ok {
            self.string(Arc::new(Mutex::new(Some("...".to_string()))));;
            { let __iface_handle = (*s.lock().unwrap().as_ref().unwrap()).elem.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            {
        let (mut t, _) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_none() || { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::basic::BasicKind(Arc::new(Mutex::new(Some(STRING as i32)))); __tmp_x != __tmp_y } {
            self.error(Arc::new(Mutex::new(Some("expected string type".to_string()))));;
            continue;
        }
    };
            self.typ(typ.clone());;
            self.string(Arc::new(Mutex::new(Some("...".to_string()))));;
            continue;
        }
    }
    }
                // special case:
                // append(s, "foo"...) leads to signature func([]byte, string...)
        self.typ(typ.clone());
    } }
    }
                // parameter names are ignored for type identity and thus type hashes
                // special case:
                // append(s, "foo"...) leads to signature func([]byte, string...)
        self.byte(Arc::new(Mutex::new(Some((')' as i32) as u8))));
    }

    pub fn signature(&mut self, sig: Arc<Mutex<Option<Signature>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if { let __tmp_x = { let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.tparams.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        { let new_val = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }.clone(); self.tparams = new_val; };
        let mut w_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        *w_defer_captured.tparams.lock().unwrap() = None;
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
        self.t_param_list({ let __recv = { let __recv = sig.clone(); let __recv_ptr: *mut crate::signature::Signature = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::signature::Signature }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result });
    }
            self.tuple({ let __field = (*sig.lock().unwrap().as_ref().unwrap()).params.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*sig.lock().unwrap().as_ref().unwrap()).variadic.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            let mut n = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).len();
            if { let __tmp_x = n; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // no result
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                        // no result
            self.byte(Arc::new(Mutex::new(Some((' ' as i32) as u8))));
            if { let __tmp_x = n; let __tmp_y = 1; __tmp_x == __tmp_y } && ({ let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x == __tmp_y }) {
                // single unnamed result (if type hashing, name must be ignored)
        self.typ((*{ let __seq = { let __seq_holder = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone());
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                        // single unnamed result (if type hashing, name must be ignored)
                        // multiple or named result(s)
            self.tuple({ let __field = (*sig.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some(false))));

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
}

/// RelativeTo returns a [Qualifier] that fully qualifies members of
/// all packages other than pkg.
pub fn relative_to(pkg: Arc<Mutex<Option<Package>>>) -> Qualifier {
    if (*pkg.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }
    let pkg_closure_clone = pkg.clone(); return Arc::new(Mutex::new(Some(Box::new(move |other: Arc<Mutex<Option<Package>>>| -> Arc<Mutex<Option<String>>> {
        if { let __left = pkg_closure_clone.clone(); let __right = other.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        { let __recv = other.clone(); let __recv_ptr: *const crate::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::package::Package }; let __result = unsafe { &*__recv_ptr }.path(); __result }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
}

/// TypeString returns the string representation of typ.
/// The [Qualifier] controls the printing of
/// package-level objects, and may be nil.
pub fn type_string(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, qf: Qualifier) -> Arc<Mutex<Option<String>>> {
    let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    write_type(buf.clone(), typ.clone(), qf.clone());
    return (*buf.lock().unwrap().as_mut().unwrap()).string();
}

/// WriteType writes the string representation of typ to buf.
/// The [Qualifier] controls the printing of
/// package-level objects, and may be nil.
pub fn write_type(buf: Arc<Mutex<Option<bytes_Buffer>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, qf: Qualifier) {
    { let __recv = new_type_writer(buf.clone(), qf.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).typ(typ.clone()); __result };
}

/// WriteSignature writes the representation of the signature sig to buf,
/// without a leading "func" keyword. The [Qualifier] controls the printing
/// of package-level objects, and may be nil.
pub fn write_signature(buf: Arc<Mutex<Option<bytes_Buffer>>>, sig: Arc<Mutex<Option<Signature>>>, qf: Qualifier) {
    { let __recv = new_type_writer(buf.clone(), qf.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).signature(sig.clone()); __result };
}

pub fn new_type_writer(buf: Arc<Mutex<Option<bytes_Buffer>>>, qf: Qualifier) -> Arc<Mutex<Option<typeWriter>>> {
    Arc::new(Mutex::new(Some(typeWriter { buf: buf.clone(), seen: Arc::new(Mutex::new(Some(BTreeMap::<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))), qf: qf.clone(), ctxt: Default::default(), tparams: Default::default(), param_names: Arc::new(Mutex::new(Some(true))), tp_subscripts: Arc::new(Mutex::new(Some(false))), pkg_info: Arc::new(Mutex::new(Some(false))), ..Default::default() })))
}

pub fn new_type_hasher(buf: Arc<Mutex<Option<bytes_Buffer>>>, ctxt: Arc<Mutex<Option<Context>>>) -> Arc<Mutex<Option<typeWriter>>> {
    assert(Arc::new(Mutex::new(Some((*ctxt.lock().unwrap()).is_some()))));
    Arc::new(Mutex::new(Some(typeWriter { buf: buf.clone(), seen: Arc::new(Mutex::new(Some(BTreeMap::<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>::new()))), qf: Default::default(), ctxt: ctxt.clone(), tparams: Default::default(), param_names: Arc::new(Mutex::new(Some(false))), tp_subscripts: Arc::new(Mutex::new(Some(false))), pkg_info: Arc::new(Mutex::new(Some(false))), ..Default::default() })))
}

/// subscript returns the decimal (utf8) representation of x using subscript digits.
pub fn subscript(mut x: Arc<Mutex<Option<u64>>>) -> Arc<Mutex<Option<String>>> {
    const w: i32 = "\u{2080}".len() as i32;

    let mut buf: Arc<Mutex<Option<[u8; 96]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut i = Arc::new(Mutex::new(Some((*buf.lock().unwrap().as_ref().unwrap()).len() as i32)));
    loop {
        { let __rhs = 3; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        unicode_utf8::encode_rune(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))), Arc::new(Mutex::new(Some({ let __tmp_x = ('\u{2080}' as i32); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10 as u64; __tmp_x % __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))));
        { let __rhs = 10 as u64; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        break
    }
    }
        // '₀' == U+2080
    return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}

impl GoValueClone for typeWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
