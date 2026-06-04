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
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


impl crate::check::Checker {
    /// validType verifies that the given type does not "expand" indefinitely
    /// producing a cycle in the type graph.
    /// (Cycles involving alias types, as in "type A = [10]A" are detected
    /// earlier, via the objDecl cycle detection mechanism.)
    pub fn valid_type(&mut self, typ: Arc<Mutex<Option<Named>>>) {
        self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    /// validType0 checks if the given type is valid. If typ is a type parameter
    /// its value is looked up in the type argument list of the instantiated
    /// (enclosing) type, if it exists. Otherwise the type parameter must be from
    /// an enclosing function and can be ignored.
    /// The nest list describes the stack (the "nest in memory") of types which
    /// contain (or embed in the case of interfaces) other types. For instance, a
    /// struct named S which contains a field of named type F contains (the memory
    /// of) F in S, leading to the nest S->F. If a type appears in its own nest
    /// (say S->F->S) we have an invalid recursive type. The path list is the full
    /// path of named types in a cycle, it is only needed for error reporting.
    pub fn valid_type0(&mut self, mut pos: Arc<Mutex<Option<token_Pos>>>, mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, nest: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Named>>>>>>>, path: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Named>>>>>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(typ.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        { let __iface_handle = unalias(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        {
        let (mut t, _) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::named::NamedPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::named::Named>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::named::Named>)), false)
        }
    });;
        if (*t.lock().unwrap()).is_some() && { let __nil_target = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = token_Pos((*(*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.lock().unwrap().as_ref().unwrap()).0 as i32); *pos.lock().unwrap() = Some(new_val); };;
        }
    }
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        self.trace(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("validType(%s) nest %v, path %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = path_string(make_obj_list(nest.clone())); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = path_string(make_obj_list(path.clone())); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                /* obj should always exist but be conservative */
        {
    let _ts_subject = typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let t = typ.clone();
        if DEBUG {
        panic!("validType0(nil)");
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*t.lock().unwrap().as_ref().unwrap()).elem.clone(), nest.clone(), path.clone());
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if !self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), nest.clone(), path.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        if !self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*t.lock().unwrap().as_ref().unwrap()).typ.clone(), nest.clone(), path.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for etyp in __range_values.iter() {
        if !self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), etyp.clone(), nest.clone(), path.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        if !is_valid({ let __recv = t.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.underlying(); __result }.clone()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    };
        { let __range_holder = nest.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        if identical(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(e.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
        assert(Arc::new(Mutex::new(Some({ let __left = (*(*t.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));
        assert(Arc::new(Mutex::new(Some({ let __left = (*(*{ let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*t.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*{ let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = (*__iface_guard).clone(); };
        { let __range_holder = path.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (start, p) in __range_values.iter().enumerate() {
        if identical(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(p.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
        self.cycle_error(make_obj_list(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(start) as usize..].to_vec() })))), Arc::new(Mutex::new(Some(0))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    }
    } }
        panic!("cycle start not found");
    }
    } };
        if !self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*{ let __recv = t.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone(), { let __append_target = nest.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(t.clone()); __append_target.clone() }, { let __append_target = path.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(t.clone()); __append_target.clone() }) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return false;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        {
        let mut d = Arc::new(Mutex::new(Some({ let __tmp_x = ((*nest.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));;
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            let mut inst = { let __seq = { let __seq_holder = nest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();;
            { let __range_holder = { let __recv = { let __recv = inst.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, tparam) in __range_values.iter().enumerate() {
        if { let __left = t.clone(); let __right = tparam.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } && { let __tmp_x = i as i32; let __tmp_y = { let __recv = { let __recv = inst.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; __tmp_x < __tmp_y } {
        let mut targ = { let __recv = { let __recv = inst.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(i as i32)))); __result };
        let mut res = self.valid_type0(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), targ.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = nest.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() }))), path.clone());
        (*nest.lock().unwrap().as_mut().unwrap())[({ let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = inst.clone();
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return res;
    }
    }
    } };
        }
    };
    }
    }
                // We should never see a nil type but be conservative and panic
                // only in debug mode.
                // TODO(gri) The optimization below is incorrect (see go.dev/issue/65711):
                //           in that issue `type A[P any] [1]P` is a valid type on its own
                //           and the (uninstantiated) A is recorded in check.valids. As a
                //           consequence, when checking the remaining declarations, which
                //           are not valid, the validity check ends prematurely because A
                //           is considered valid, even though its validity depends on the
                //           type argument provided to it.
                //
                //           A correct optimization is important for pathological cases.
                //           Keep code around for reference until we found an optimization.
                //
                // // Exit early if we already know t is valid.
                // // This is purely an optimization but it prevents excessive computation
                // // times in pathological cases such as testdata/fixedbugs/issue6977.go.
                // // (Note: The valids map could also be allocated locally, once for each
                // // validType call.)
                // if check.valids.lookup(t) != nil {
                // 	break
                // }
                // Don't report a 2nd error if we already know the type is invalid
                // (e.g., if a cycle was detected earlier, via under).
                // Note: ensure that t.orig is fully resolved by calling Underlying().
                // If the current type t is also found in nest, (the memory of) t is
                // embedded in itself, indicating an invalid recursive type.
                // We have a cycle. If t != t.Origin() then t is an instance of
                // the generic type t.Origin(). Because t is in the nest, t must
                // occur within the definition (RHS) of the generic type t.Origin(),
                // directly or indirectly, after expansion of the RHS.
                // Therefore t.Origin() must be invalid, no matter how it is
                // instantiated since the instantiation t of t.Origin() happens
                // inside t.Origin()'s RHS and thus is always the same and always
                // present.
                // Therefore we can mark the underlying of both t and t.Origin()
                // as invalid. If t is not an instance of a generic type, t and
                // t.Origin() are the same.
                // Furthermore, because we check all types in a package for validity
                // before type checking is complete, any exported type that is invalid
                // will have an invalid underlying type and we can't reach here with
                // such a type (invalid types are excluded above).
                // Thus, if we reach here with a type t, both t and t.Origin() (if
                // different in the first place) must be from the current package;
                // they cannot have been imported.
                // Therefore it is safe to change their underlying types; there is
                // no chance for a race condition (the types of the current package
                // are not yet available to other goroutines).
                // Find the starting point of the cycle and report it.
                // Because each type in nest must also appear in path (see invariant below),
                // type t must be in path since it was found in nest. But not every type in path
                // is in nest. Specifically t may appear in path with an earlier index than the
                // index of t in nest. Search again.
                // No cycle was found. Check the RHS of t.
                // Every type added to nest is also added to path; thus every type that is in nest
                // must also be in path (invariant). But not every type in path is in nest, since
                // nest may be pruned (see below, *TypeParam case).
                // see TODO above
                // check.valids.add(t) // t is valid
                // A type parameter stands for the type (argument) it was instantiated with.
                // Check the corresponding type argument for validity if we are in an
                // instantiated type.
                // the type instance
                // Find the corresponding type argument for the type parameter
                // and proceed with checking that type argument.
                // The type parameter and type argument lists should
                // match in length but be careful in case of errors.
                // The type argument must be valid in the enclosing
                // type (where inst was instantiated), hence we must
                // check targ's validity in the type nest excluding
                // the current (instantiated) type (see the example
                // at the end of this file).
                // For error reporting we keep the full path.
                // The check.validType0 call with nest[:d] may have
                // overwritten the entry at the current depth d.
                // Restore the entry (was issue go.dev/issue/66323).
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return true;
    }
    }
}

/// makeObjList returns the list of type name objects for the given
/// list of named types.
pub fn make_obj_list(tlist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Named>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> {
    let mut olist: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*tlist.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = tlist.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, t) in __range_values.iter().enumerate() {
        (*olist.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr((*t.lock().unwrap().as_ref().unwrap()).obj.clone())) as Box<dyn Object + Send + Sync>)));
    } }
    return olist.clone();
}