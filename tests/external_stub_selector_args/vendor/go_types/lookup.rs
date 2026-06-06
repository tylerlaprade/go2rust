use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// embeddedType represents an embedded type
#[derive(Clone)]
pub struct embeddedType {
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub index: Arc<Mutex<Option<Vec<i32>>>>,
    pub indirect: Arc<Mutex<Option<bool>>>,
    pub multiples: Arc<Mutex<Option<bool>>>,
}

impl embeddedType {
    pub fn __go_value_clone(&self) -> Self {
        Self { typ: self.typ.clone(), index: self.index.clone(), indirect: { let __guard = self.indirect.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, multiples: { let __guard = self.multiples.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for embeddedType {
    fn default() -> Self {
        Self { typ: Arc::new(Mutex::new(None)), index: Arc::new(Mutex::new(None)), indirect: Arc::new(Mutex::new(Some(false))), multiples: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for embeddedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.typ.lock().unwrap().as_ref().unwrap()), format_slice(&self.index), (*self.indirect.lock().unwrap().as_ref().unwrap()), (*self.multiples.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for embeddedType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct instanceLookup {
    pub buf: Arc<Mutex<Option<[Arc<Mutex<Option<Named>>>; 3]>>>,
    pub m: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::named::Named>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Named>>>>>>>>>>>,
}

impl instanceLookup {
    pub fn __go_value_clone(&self) -> Self {
        Self { buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, m: self.m.clone() }
    }
}


impl Default for instanceLookup {
    fn default() -> Self {
        Self { buf: Arc::new(Mutex::new(Some(std::array::from_fn(|_| Arc::new(Mutex::new(None)))))), m: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for instanceLookup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", format_slice_wrapped(&self.buf), "<map>")
    }
}

impl GoJsonDecode for instanceLookup {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl instanceLookup {
    pub fn lookup(&self, inst: Arc<Mutex<Option<Named>>>) -> Arc<Mutex<Option<crate::named::Named>>> {
        { let __range_holder = self.buf.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && identical(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(inst.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
        return (*t).clone();
    }
    } }
        { let __range_holder = { let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new({ let __recv = inst.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        if identical(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(inst.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
        return (*t).clone();
    }
    } }
        return Arc::new(Mutex::new(None));
    }

    pub fn add(&mut self, inst: Arc<Mutex<Option<Named>>>) {
        { let __range_holder = self.buf.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, t) in __range_values.iter().enumerate() {
        if { let __nil_result = (*t.lock().unwrap()).is_none(); __nil_result } {
        (*self.buf.lock().unwrap().as_mut().unwrap())[(i) as usize] = inst.clone();
        return;
    }
    } }
        if { let __nil_target = self.m.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::named::Named>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::named::Named>>>>>>>>::new()))); self.m = new_val; };
    }
        let mut insts = { let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new({ let __recv = inst.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        { let __map_key = GoLocalPtrKey::new({ let __recv = inst.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.origin(); __result }.clone()); let __map_value = { let __append_target = insts.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(inst.clone()); __append_target.clone() }; (*self.m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
}

impl crate::check::Checker {
    /// missingMethod is like MissingMethod but accepts a *Checker as receiver,
    /// a comparator equivalent for type comparison, and a *string for error causes.
    /// The receiver may be nil if missingMethod is invoked through an exported
    /// API call (such as MissingMethod), i.e., when all methods have been type-
    /// checked.
    /// The underlying type of T must be an interface; T (rather than its under-
    /// lying type) is used for better error messages (reported through *cause).
    /// The comparator is used to compare signatures.
    /// If a method is missing and cause is not nil, *cause describes the error.
    pub fn missing_method(&mut self, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, r#static: Arc<Mutex<Option<bool>>>, equivalent: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::object::Func>>>, bool) {
    let mut method: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));
    let mut wrongType: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut methods = (*{ let __recv = ({
        let val = under(T.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone();
        if { let __tmp_x = ((*methods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }
        const ok: i32 = 0;
const notFound: i32 = 1;
const wrongName: i32 = 2;
const unexported: i32 = 3;
const wrongSig: i32 = 4;
const ambigSel: i32 = 5;
const ptrRecv: i32 = 6;
const field: i32 = 7;

        let mut state = Arc::new(Mutex::new(Some(ok)));
        let mut m: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));
        let mut f: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));
        {
        let (mut u, _) = ({
        let val = under(V.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });;
        if { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } {
            let mut tset = { let __recv = u.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_m in __range_values.iter() {
        { let new_val = (*__range_m).clone(); m = new_val; };
        { let (__tmp_0, __tmp_1) = { let __recv = tset.clone(); let __recv_ptr: *const crate::typeset::_TypeSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeset::_TypeSet }; let __result = unsafe { &*__recv_ptr }.lookup_method({ let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false)))); __result }; f = __tmp_1.clone(); };
        if { let __nil_result = (*f.lock().unwrap()).is_none(); __nil_result } {
        if !{ let __v = (*r#static.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        continue
    }
        { let new_val = 1; *state.lock().unwrap() = Some(new_val); };
        break
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = equivalent.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) } {
        { let new_val = 4; *state.lock().unwrap() = Some(new_val); };
        break
    }
    } };
        } else {
            { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_m in __range_values.iter() {
        { let new_val = (*__range_m).clone(); m = new_val; };
        let (mut obj, mut index, mut indirect) = lookup_field_or_method_impl(V.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false))));
        if { let __nil_result = (*obj.lock().unwrap()).is_none(); __nil_result } {
        if { let __nil_result = (*index.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = 5; *state.lock().unwrap() = Some(new_val); };
        } else if indirect {
            { let new_val = 6; *state.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = 1; *state.lock().unwrap() = Some(new_val); };
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_impl(V.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(true)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; };
            { let (__tmp_0, __tmp_1) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    }); f = __tmp_0.clone(); };
            if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 2; *state.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } {
        { let new_val = 3; *state.lock().unwrap() = Some(new_val); };
    }
    }
        }
        break
    }
        { let (__tmp_0, __tmp_1) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    }); f = __tmp_0.clone(); };
        if { let __nil_result = (*f.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = 7; *state.lock().unwrap() = Some(new_val); };
        break
    }
        if true {
        self.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(f.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)));
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = equivalent.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) } {
        { let new_val = 4; *state.lock().unwrap() = Some(new_val); };
        break
    }
    } };
        }
    }
                // check if m is ambiguous, on *V, or on V with case-folding
                /* fold case */
                // If the names are equal, f must be unexported
                // (otherwise the package wouldn't matter).
                // we must have a method (not a struct field)
                // methods may not have a fully set up signature yet
        if { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }
        if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
                // This method may be formatted in funcString below, so must have a fully
                // set up signature.
        if true {
        self.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(f.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)));
    }
    }
                // This method may be formatted in funcString below, so must have a fully
                // set up signature.
        '__go_switch_1: loop {
        { let _switch_val = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            if is_interface_ptr(V.clone()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "(".to_string())); __s.push_str(&format!("{}", (*self.interface_ptr_error(V.clone()).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s }; *cause.lock().unwrap() = Some(new_val); };
        } else if is_interface_ptr(T.clone()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "(".to_string())); __s.push_str(&format!("{}", (*self.interface_ptr_error(T.clone()).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s }; *cause.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(missing method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        }
        } else if _switch_val == (2) {
            let (mut fs, mut ms) = (self.func_string(f.clone(), Arc::new(Mutex::new(Some(false)))), self.func_string(m.clone(), Arc::new(Mutex::new(Some(false)))));
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(missing method %s)\n\t\thave %s\n\t\twant %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = fs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = ms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (3) {
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(unexported method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (4) {
            let (mut fs, mut ms) = (self.func_string(f.clone(), Arc::new(Mutex::new(Some(false)))), self.func_string(m.clone(), Arc::new(Mutex::new(Some(false)))));
            if { let __tmp_x = (*fs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ms.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Don't report "want Foo, have Foo".
                // Add package information to disambiguate (go.dev/issue/54258).
        { let __tmp_0 = self.func_string(f.clone(), Arc::new(Mutex::new(Some(true)))); let __tmp_1 = self.func_string(m.clone(), Arc::new(Mutex::new(Some(true)))); *fs.lock().unwrap() = __tmp_0.lock().unwrap().take(); *ms.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
                        // Don't report "want Foo, have Foo".
                        // Add package information to disambiguate (go.dev/issue/54258).
            if { let __tmp_x = (*fs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ms.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // We still have "want Foo, have Foo".
                // This is most likely due to different type parameters with
                // the same name appearing in the instantiated signatures
                // (go.dev/issue/61685).
                // Rather than reporting this misleading error cause, for now
                // just point out that the method signature is incorrect.
                // TODO(gri) should find a good way to report the root cause
        { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(wrong type for method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        break '__go_switch_1
    }
                        // We still have "want Foo, have Foo".
                        // This is most likely due to different type parameters with
                        // the same name appearing in the instantiated signatures
                        // (go.dev/issue/61685).
                        // Rather than reporting this misleading error cause, for now
                        // just point out that the method signature is incorrect.
                        // TODO(gri) should find a good way to report the root cause
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(wrong type for method %s)\n\t\thave %s\n\t\twant %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = fs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = ms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (5) {
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(ambiguous selector %s.%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (6) {
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(method %s has pointer receiver)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (7) {
            { let new_val = (*self.sprintf(Arc::new(Mutex::new(Some("(%s.%s is a field, not a method)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    };
        break;
    }
    }
                // This method may be formatted in funcString below, so must have a fully
                // set up signature.
                // Don't report "want Foo, have Foo".
                // Add package information to disambiguate (go.dev/issue/54258).
                // We still have "want Foo, have Foo".
                // This is most likely due to different type parameters with
                // the same name appearing in the instantiated signatures
                // (go.dev/issue/61685).
                // Rather than reporting this misleading error cause, for now
                // just point out that the method signature is incorrect.
                // TODO(gri) should find a good way to report the root cause
        return (m.clone(), { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x == __tmp_y });
    }

    /// hasAllMethods is similar to checkMissingMethod but instead reports whether all methods are present.
    /// If V is not a valid type, or if it is a struct containing embedded fields with invalid types, the
    /// result is true because it is not possible to say with certainty whether a method is missing or not
    /// (an embedded field may have the method in question).
    /// If the result is false and cause is not nil, *cause describes the error.
    /// Use hasAllMethods to avoid follow-on errors due to incorrect types.
    pub fn has_all_methods(&mut self, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, r#static: Arc<Mutex<Option<bool>>>, equivalent: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> bool {
        if !is_valid(V.clone()) {
        return true;
    }
                // we don't know anything about V, assume it implements T
        let (mut m, _) = self.missing_method(V.clone(), T.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r#static.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), equivalent.clone(), cause.clone());
        return { let __nil_result = (*m.lock().unwrap()).is_none(); __nil_result } || has_invalid_embedded_fields(V.clone(), Arc::new(Mutex::new(None)));
    }

    /// check may be nil.
    pub fn interface_ptr_error(&self, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
        assert(Arc::new(Mutex::new(Some(is_interface_ptr(T.clone())))));
        {
        let (mut p, _) = ({
        let val = under(T.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if is_type_param({ let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field }) {
            return self.sprintf(Arc::new(Mutex::new(Some("type %s is pointer to type parameter, not type parameter".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
        }
    }
        self.sprintf(Arc::new(Mutex::new(Some("type %s is pointer to interface, not interface".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = T.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))))
    }

    /// funcString returns a string of the form name + signature for f.
    /// check may be nil.
    pub fn func_string(&self, f: Arc<Mutex<Option<Func>>>, pkgInfo: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
        let mut buf = bytes::new_buffer_string({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });
        let mut qf: crate::typestring::Qualifier = Arc::new(Mutex::new(None));
        if true && !{ let __v = (*pkgInfo.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }))); qf = new_val; };
    }
        let mut w = new_type_writer(buf.clone(), qf.clone());
        { let new_val = pkgInfo.lock().unwrap().as_ref().unwrap().clone(); *(*w.lock().unwrap().as_ref().unwrap()).pkg_info.lock().unwrap() = Some(new_val); };
        { let new_val = false; *(*w.lock().unwrap().as_ref().unwrap()).param_names.lock().unwrap() = Some(new_val); };
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.signature(({
        let val = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
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
    })); __result };
        return { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.string(); __result };
    }

    /// assertableTo reports whether a value of type V can be asserted to have type T.
    /// The receiver may be nil if assertableTo is invoked through an exported API call
    /// (such as AssertableTo), i.e., when all methods have been type-checked.
    /// The underlying type of V must be an interface.
    /// If the result is false and cause is not nil, *cause describes the error.
    /// TODO(gri) replace calls to this function with calls to newAssertableTo.
    pub fn assertable_to(&mut self, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> bool {
                // no static check is required if T is an interface
                // spec: "If T is an interface type, x.(T) asserts that the
                //        dynamic type of x implements the interface T."
        if is_interface(T.clone()) {
        return true;
    }
                // TODO(gri) fix this for generalized interfaces
        self.has_all_methods(T.clone(), V.clone(), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, __arg1: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool { identical(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))), cause.clone())
    }

    /// newAssertableTo reports whether a value of type V can be asserted to have type T.
    /// It also implements behavior for interfaces that currently are only permitted
    /// in constraint position (we have not yet defined that behavior in the spec).
    /// The underlying type of V must be an interface.
    /// If the result is false and cause is not nil, *cause is set to the error cause.
    pub fn new_assertable_to(&mut self, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> bool {
                // no static check is required if T is an interface
                // spec: "If T is an interface type, x.(T) asserts that the
                //        dynamic type of x implements the interface T."
        if is_interface(T.clone()) {
        return true;
    }
        self.implements(T.clone(), V.clone(), Arc::new(Mutex::new(Some(false))), cause.clone())
    }
}

/// LookupFieldOrMethod looks up a field or method with given package and name
/// in T and returns the corresponding *Var or *Func, an index sequence, and a
/// bool indicating if there were any pointer indirections on the path to the
/// field or method. If addressable is set, T is the type of an addressable
/// variable (only matters for method lookups). T must not be nil.
///
/// The last index entry is the field or method index in the (possibly embedded)
/// type where the entry was found, either:
///
///  1. the list of declared methods of a named type; or
///  2. the list of all methods (method set) of an interface type; or
///  3. the list of fields of a struct type.
///
/// The earlier index entries are the indices of the embedded struct fields
/// traversed to get to the found entry, starting at depth 0.
///
/// If no entry is found, a nil object is returned. In this case, the returned
/// index and indirect values have the following meaning:
///
///   - If index != nil, the index sequence points to an ambiguous entry
///     (the same name appeared more than once at the same embedding level).
///
///   - If indirect is set, a method with a pointer receiver type was found
///     but there was no pointer on the path from the actual receiver type to
///     the method's formal receiver base type, nor was the receiver addressable.
pub fn lookup_field_or_method(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, addressable: Arc<Mutex<Option<bool>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, Arc<Mutex<Option<Vec<i32>>>>, bool) {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut index: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut indirect: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    if { let __nil_result = (*T.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("LookupFieldOrMethod on nil type".to_string()) as Box<dyn Any + Send + Sync>);
    }
    lookup_field_or_method_1(T.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = addressable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))))
}

/// lookupFieldOrMethod is like LookupFieldOrMethod but with the additional foldCase parameter
/// (see Object.sameId for the meaning of foldCase).
pub fn lookup_field_or_method_1(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, addressable: Arc<Mutex<Option<bool>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, Arc<Mutex<Option<Vec<i32>>>>, bool) {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut index: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut indirect: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // Methods cannot be associated to a named pointer type.
        // (spec: "The type denoted by T is called the receiver base type;
        // it must not be a pointer or interface type and it must be declared
        // in the same package as the method.").
        // Thus, if we have a named pointer type, proceed with the underlying
        // pointer type but discard the result if it is a method since we would
        // not have found it for T (see also go.dev/issue/8590).
    {
        let mut t = as_named(T.clone());;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            {
        let (mut p, _) = ({
        let val = { let __recv = t.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.underlying(); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_impl(Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(p.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(false))), pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *index.lock().unwrap() = __moved_tmp_1; *indirect.lock().unwrap() = Some(__tmp_2); };;
            {
        let (_, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });;
        if ok {
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), false);;
        }
    };
            return (obj.clone(), index.clone(), (*indirect.lock().unwrap().as_ref().unwrap()));;
        }
    };
        }
    }

    { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_impl(T.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = addressable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *index.lock().unwrap() = __moved_tmp_1; *indirect.lock().unwrap() = Some(__tmp_2); };

        // If we didn't find anything and if we have a type parameter with a core type,
        // see if there is a matching field (but not a method, those need to be declared
        // explicitly in the constraint). If the constraint is a named pointer type (see
        // above), we are ok here because only fields are accepted as results.
    const enableTParamFieldLookup: bool = false;

    if enableTParamFieldLookup && { let __nil_result = (*obj.lock().unwrap()).is_none(); __nil_result } && is_type_param(T.clone()) {
        {
        let mut t = core_type(T.clone());;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_impl(t.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = addressable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *index.lock().unwrap() = __moved_tmp_1; *indirect.lock().unwrap() = Some(__tmp_2); };;
            {
        let (_, mut ok) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
        }
    });;
        if !ok {
            { let __tmp_0 = None; let __tmp_1 = None; let __tmp_2 = false; *obj.lock().unwrap() = __tmp_0; *index.lock().unwrap() = __tmp_1; *indirect.lock().unwrap() = Some(__tmp_2); };;
        }
    };
        }
    }
    }
        // accept fields (variables) only
    return (obj.clone(), index.clone(), (*indirect.lock().unwrap().as_ref().unwrap()));
}

/// lookupFieldOrMethodImpl is the implementation of lookupFieldOrMethod.
/// Notably, in contrast to lookupFieldOrMethod, it won't find struct fields
/// in base types of defined (*Named) pointer types T. For instance, given
/// the declaration:
///
///	type T *struct{f int}
///
/// lookupFieldOrMethodImpl won't find the field f in the defined (*Named) type T
/// (methods on T are not permitted in the first place).
///
/// Thus, lookupFieldOrMethodImpl should only be called by lookupFieldOrMethod
/// and missingMethod (the latter doesn't care about struct fields).
///
/// The resulting object may not be fully type-checked.
pub fn lookup_field_or_method_impl(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, addressable: Arc<Mutex<Option<bool>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, Arc<Mutex<Option<Vec<i32>>>>, bool) {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut index: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    let mut indirect: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        // WARNING: The code in this function is extremely subtle - do not modify casually!
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        return (obj.clone(), index.clone(), (*indirect.lock().unwrap().as_ref().unwrap()));
    }

        // blank fields/methods are never found
        // Importantly, we must not call under before the call to deref below (nor
        // does deref call under), as doing so could incorrectly result in finding
        // methods of the pointer base type when T is a (*Named) pointer type.
    let (mut typ, mut isPtr) = deref(T.clone());

        // *typ where typ is an interface (incl. a type parameter) has no methods.
    if isPtr {
        {
        let (_, mut ok) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });;
        if ok {
            return (obj.clone(), index.clone(), (*indirect.lock().unwrap().as_ref().unwrap()));;
        }
    }
    }

        // Start with typ as single entry at shallowest depth.
    let mut current = Arc::new(Mutex::new(Some(vec![embeddedType { typ: typ.clone(), index: Arc::new(Mutex::new(None)), indirect: Arc::new(Mutex::new(Some(isPtr))), multiples: Arc::new(Mutex::new(Some(false))), ..Default::default() }])));

        // seen tracks named types that we have seen already, allocated lazily.
        // Used to avoid endless searches in case of recursive types.
        //
        // We must use a lookup on identity rather than a simple map[*Named]bool as
        // instantiated types may be identical but not equal.
    let mut seen: Arc<Mutex<Option<instanceLookup>>> = Arc::new(Mutex::new(Some(Default::default())));

        // search current depth
    while { let __tmp_x = ((*current.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut next: Arc<Mutex<Option<Vec<embeddedType>>>> = Arc::new(Mutex::new(None));

                // look for (pkg, name) in all types at current depth
        { let __range_holder = current.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        let mut typ = e.typ.clone();
                // If we have a named type, we may have associated methods.
                // Look for those first.
        {
        let mut named = as_named(typ.clone());;
        if { let __nil_result = (*named.lock().unwrap()).is_some(); __nil_result } {
            {
        let mut alt = (*seen.lock().unwrap().as_ref().unwrap()).lookup(named.clone());;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            continue;
        }
    };
            (*seen.lock().unwrap().as_mut().unwrap()).add(named.clone());;
            {
        let (mut i, mut m) = { let __recv = named.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.lookup_method(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if { let __nil_result = (*m.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(i)))); index = new_val; };;
            if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } || (*e.multiples.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), index.clone(), false);
    };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };;
            { let new_val = { let __selector_holder = e.indirect.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *indirect.lock().unwrap() = Some(new_val); };;
            continue;
        }
    };
        }
    }
                // We have seen this type before, at a more shallow depth
                // (note that multiples of this type at the current depth
                // were consolidated before). The type at that depth shadows
                // this same type at the current depth, so we can ignore
                // this one.
                // look for a matching attached method
                // potential match
                // caution: method may not have a proper signature yet
                // collision
                // we can't have a matching field or interface method
        {
    let _ts_subject = under(typ.clone()).clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        if { let __recv = f.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.same_id(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        { let new_val = concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(i as i32)))); index = new_val; };
        if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } || (*e.multiples.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), index.clone(), false);
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(f.clone())) as Box<dyn Object + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };
        { let new_val = { let __selector_holder = e.indirect.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *indirect.lock().unwrap() = Some(new_val); };
        continue
    }
        if { let __nil_result = (*obj.lock().unwrap()).is_none(); __nil_result } && (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let (mut typ, mut isPtr) = deref({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
        { let new_val = { let __append_target = next.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(embeddedType { typ: typ.clone(), index: concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(i as i32)))), indirect: Arc::new(Mutex::new(Some((*e.indirect.lock().unwrap().as_ref().unwrap()) || isPtr))), multiples: Arc::new(Mutex::new(Some({ let __selector_holder = e.multiples.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }); __append_target.clone() }; next = new_val; };
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        {
        let (mut i, mut m) = { let __recv = { let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).lookup_method(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if { let __nil_result = (*m.lock().unwrap()).is_some(); __nil_result } {
            assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));;
            { let new_val = concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(i)))); index = new_val; };;
            if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } || (*e.multiples.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(None)), index.clone(), false);
    };
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };;
            { let new_val = { let __selector_holder = e.indirect.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *indirect.lock().unwrap() = Some(new_val); };;
        }
    };
    }
    }
    } }

                // If we have a named type, we may have associated methods.
                // Look for those first.
                // We have seen this type before, at a more shallow depth
                // (note that multiples of this type at the current depth
                // were consolidated before). The type at that depth shadows
                // this same type at the current depth, so we can ignore
                // this one.
                // look for a matching attached method
                // potential match
                // caution: method may not have a proper signature yet
                // collision
                // we can't have a matching field or interface method
                // look for a matching field and collect embedded types
                // collision
                // we can't have a matching interface method
                // Collect embedded struct fields for searching the next
                // lower depth, but only if we have not seen a match yet
                // (if we have a match it is either the desired field or
                // we have a name collision on the same depth; in either
                // case we don't need to look further).
                // Embedded fields are always of the form T or *T where
                // T is a type name. If e.typ appeared multiple times at
                // this depth, f.typ appears multiple times at the next
                // depth.
                // TODO(gri) optimization: ignore types that can't
                // have fields or methods (only Named, Struct, and
                // Interface types need to be considered).
                // look for a matching method (interface may be a type parameter)
                // collision
        if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } {
                // found a potential match
                // spec: "A method call x.m() is valid if the method set of (the type of) x
                //        contains m and the argument list can be assigned to the parameter
                //        list of m. If x is addressable and &x's method set contains m, x.m()
                //        is shorthand for (&x).m()".
        {
        let (mut f, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });;
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
            if { let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.has_ptr_recv(); __result } && !{ let __v = (*indirect.lock().unwrap().as_ref().unwrap()).clone(); __v } && !{ let __v = (*addressable.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), true);
    };
        }
    }
                // determine if method has a pointer receiver
                // pointer/addressable receiver required
        return (obj.clone(), index.clone(), (*indirect.lock().unwrap().as_ref().unwrap()));
    }

                // found a potential match
                // spec: "A method call x.m() is valid if the method set of (the type of) x
                //        contains m and the argument list can be assigned to the parameter
                //        list of m. If x is addressable and &x's method set contains m, x.m()
                //        is shorthand for (&x).m()".
                // determine if method has a pointer receiver
                // pointer/addressable receiver required
        { let new_val = consolidate_multiples(next.clone()); current = new_val; };
    }

        // embedded types found at current depth
        // look for (pkg, name) in all types at current depth
        // If we have a named type, we may have associated methods.
        // Look for those first.
        // We have seen this type before, at a more shallow depth
        // (note that multiples of this type at the current depth
        // were consolidated before). The type at that depth shadows
        // this same type at the current depth, so we can ignore
        // this one.
        // look for a matching attached method
        // potential match
        // caution: method may not have a proper signature yet
        // collision
        // we can't have a matching field or interface method
        // look for a matching field and collect embedded types
        // collision
        // we can't have a matching interface method
        // Collect embedded struct fields for searching the next
        // lower depth, but only if we have not seen a match yet
        // (if we have a match it is either the desired field or
        // we have a name collision on the same depth; in either
        // case we don't need to look further).
        // Embedded fields are always of the form T or *T where
        // T is a type name. If e.typ appeared multiple times at
        // this depth, f.typ appears multiple times at the next
        // depth.
        // TODO(gri) optimization: ignore types that can't
        // have fields or methods (only Named, Struct, and
        // Interface types need to be considered).
        // look for a matching method (interface may be a type parameter)
        // collision
        // found a potential match
        // spec: "A method call x.m() is valid if the method set of (the type of) x
        //        contains m and the argument list can be assigned to the parameter
        //        list of m. If x is addressable and &x's method set contains m, x.m()
        //        is shorthand for (&x).m()".
        // determine if method has a pointer receiver
        // pointer/addressable receiver required
    return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), false);
}

/// consolidateMultiples collects multiple list entries with the same type
/// into a single entry marked as containing multiples. The result is the
/// consolidated list.
pub fn consolidate_multiples(list: Arc<Mutex<Option<Vec<embeddedType>>>>) -> Arc<Mutex<Option<Vec<embeddedType>>>> {
    if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return list.clone();
    }

        // at most one entry - nothing to do
    let mut n = Arc::new(Mutex::new(Some(0)));
    let mut prev = Arc::new(Mutex::new(Some(BTreeMap::<GoTypeInterfaceKey, Arc<Mutex<Option<i32>>>>::new())));
    { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for e in __range_values.iter() {
        {
        let (mut i, mut found) = lookup_type(prev.clone(), { let __field = e.typ.clone(); __field });;
        if found {
            { let new_val = true; *{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.multiples.lock().unwrap() = Some(new_val); };;
        } else {
            { let __map_key = GoTypeInterfaceKey::new(e.typ.clone()); let __map_value = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).clone()))); (*prev.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            (*list.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = e.clone();;
            { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        }
    }
    } }
        // ignore this entry
    return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })));
}

pub fn lookup_type(m: Arc<Mutex<Option<BTreeMap<GoTypeInterfaceKey, Arc<Mutex<Option<i32>>>>>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> (i32, bool) {
        // fast path: maybe the types are equal
    {
        let (mut i, mut found) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoTypeInterfaceKey::new(typ.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if found {
            return ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, true);;
        }
    }

    for (__range_key, i) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let t = __range_key.value();
        if identical(t.clone(), typ.clone()) {
        return ((*i.lock().unwrap().as_mut().unwrap()), true);
    }
    }

    (0, false)
}

/// hasInvalidEmbeddedFields reports whether T is a struct (or a pointer to a struct) that contains
/// (directly or indirectly) embedded fields with invalid types.
pub fn has_invalid_embedded_fields(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut seen: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::r#struct::Struct>, Arc<Mutex<Option<bool>>>>>>>) -> bool {
    {
        let (mut S, _) = ({
        let val = under(deref_struct_ptr(T.clone()).clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#struct::StructPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
        }
    });;
        if { let __nil_result = (*S.lock().unwrap()).is_some(); __nil_result } && !{ let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(S.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
            if { let __nil_result = (*seen.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::r#struct::Struct>, Arc<Mutex<Option<bool>>>>::new()))); seen = new_val; };
    };
            { let __map_key = GoLocalPtrKey::new(S.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            { let __range_holder = (*S.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        if (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()) && (!is_valid({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) || has_invalid_embedded_fields({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, seen.clone())) {
        return true;
    }
    } };
        }
    }
    false
}

pub fn is_interface_ptr(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
    let (mut p, _) = ({
        let val = under(T.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });
    return { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } && is_interface({ let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field });
}

/// deref dereferences typ if it is a *Pointer (but not a *Named type
/// with an underlying pointer type!) and returns its base and true.
/// Otherwise it returns (typ, false).
pub fn deref(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, bool) {
    {
        let (mut p, _) = ({
        let val = unalias(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
            if { let __iface_handle = { let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        if DEBUG {
        std::panic::panic_any(Box::new("pointer with nil base type (possibly due to an invalid cyclic declaration)".to_string()) as Box<dyn Any + Send + Sync>);
    }
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), true);
    };
            return ({ let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field }, true);;
        }
    }
        // p.base should never be nil, but be conservative
    return (typ.clone(), false);
}

/// derefStructPtr dereferences typ if it is a (named or unnamed) pointer to a
/// (named or unnamed) struct and returns its base. Otherwise it returns typ.
pub fn deref_struct_ptr(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    {
        let (mut p, _) = ({
        let val = under(typ.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
            {
        let (_, mut ok) = ({
        let val = under({ let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field }).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#struct::StructPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::r#struct::Struct>)), false)
        }
    });;
        if ok {
            return { let __field = (*p.lock().unwrap().as_ref().unwrap()).base.clone(); __field };;
        }
    };
        }
    }
    return typ.clone();
}

/// concat returns the result of concatenating list and i.
/// The result does not share its underlying array with list.
pub fn concat(list: Arc<Mutex<Option<Vec<i32>>>>, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<i32>>>> {
    let mut t: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    { let new_val = { let __append_target = t.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = list.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; t = new_val; };
    return { let __append_target = t.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*i.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() };
}

/// fieldIndex returns the index for the field with matching package and name, or a value < 0.
/// See Object.sameId for the meaning of foldCase.
pub fn field_index(fields: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> i32 {
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        { let __range_holder = fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        if { let __recv = f.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.same_id(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        return i as i32;
    }
    } }
    }
    -(1)
}

/// methodIndex returns the index of and method with matching package and name, or (-1, nil).
/// See Object.sameId for the meaning of foldCase.
pub fn method_index(methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> (i32, Arc<Mutex<Option<crate::object::Func>>>) {
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, m) in __range_values.iter().enumerate() {
        if { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.same_id(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        return (i as i32, (*m).clone());
    }
    } }
    }
    (-(1), Arc::new(Mutex::new(None)))
}

pub fn __go_nil_recv_crate__check___checker_missing_method(check: Arc<Mutex<Option<Checker>>>, V: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, r#static: Arc<Mutex<Option<bool>>>, equivalent: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>, cause: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Func>>>, bool) {
    let mut method: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));
    let mut wrongType: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut methods = (*{ let __recv = ({
        let val = under(T.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone();
    if { let __tmp_x = ((*methods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }

    const ok: i32 = 0;
const notFound: i32 = 1;
const wrongName: i32 = 2;
const unexported: i32 = 3;
const wrongSig: i32 = 4;
const ambigSel: i32 = 5;
const ptrRecv: i32 = 6;
const field: i32 = 7;


    let mut state = Arc::new(Mutex::new(Some(ok)));
    let mut m: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));
    let mut f: Arc<Mutex<Option<Func>>> = Arc::new(Mutex::new(None));

    {
        let (mut u, _) = ({
        let val = under(V.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::interface::InterfacePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::interface::Interface>)), false)
        }
    });;
        if { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } {
            let mut tset = { let __recv = u.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_m in __range_values.iter() {
        { let new_val = (*__range_m).clone(); m = new_val; };
        { let (__tmp_0, __tmp_1) = { let __recv = tset.clone(); let __recv_ptr: *const crate::typeset::_TypeSet = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typeset::_TypeSet }; let __result = unsafe { &*__recv_ptr }.lookup_method({ let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false)))); __result }; f = __tmp_1.clone(); };
        if { let __nil_result = (*f.lock().unwrap()).is_none(); __nil_result } {
        if !{ let __v = (*r#static.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        continue
    }
        { let new_val = 1; *state.lock().unwrap() = Some(new_val); };
        break
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = equivalent.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) } {
        { let new_val = 4; *state.lock().unwrap() = Some(new_val); };
        break
    }
    } };
        } else {
            { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for __range_m in __range_values.iter() {
        { let new_val = (*__range_m).clone(); m = new_val; };
        let (mut obj, mut index, mut indirect) = lookup_field_or_method_impl(V.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(false))));
        if { let __nil_result = (*obj.lock().unwrap()).is_none(); __nil_result } {
        if { let __nil_result = (*index.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = 5; *state.lock().unwrap() = Some(new_val); };
        } else if indirect {
            { let new_val = 6; *state.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = 1; *state.lock().unwrap() = Some(new_val); };
            { let (__tmp_0, __tmp_1, __tmp_2) = lookup_field_or_method_impl(V.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(true)))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *obj.lock().unwrap() = __moved_tmp_0; };
            { let (__tmp_0, __tmp_1) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    }); f = __tmp_0.clone(); };
            if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 2; *state.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y } {
        { let new_val = 3; *state.lock().unwrap() = Some(new_val); };
    }
    }
        }
        break
    }
        { let (__tmp_0, __tmp_1) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    }); f = __tmp_0.clone(); };
        if { let __nil_result = (*f.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = 7; *state.lock().unwrap() = Some(new_val); };
        break
    }
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(f.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None))); __result };
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = equivalent.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) } {
        { let new_val = 4; *state.lock().unwrap() = Some(new_val); };
        break
    }
    } };
        }
    }

        // check if m is ambiguous, on *V, or on V with case-folding
        /* fold case */
        // If the names are equal, f must be unexported
        // (otherwise the package wouldn't matter).
        // we must have a method (not a struct field)
        // methods may not have a fully set up signature yet
    if { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), false);
    }

    if { let __nil_result = (*cause.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_result = (*f.lock().unwrap()).is_some(); __nil_result } {
                // This method may be formatted in funcString below, so must have a fully
                // set up signature.
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(f.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None))); __result };
    }
    }
                // This method may be formatted in funcString below, so must have a fully
                // set up signature.
        '__go_switch_2: loop {
        { let _switch_val = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            if is_interface_ptr(V.clone()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "(".to_string())); __s.push_str(&format!("{}", (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.interface_ptr_error(V.clone()); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s }; *cause.lock().unwrap() = Some(new_val); };
        } else if is_interface_ptr(T.clone()) {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", "(".to_string())); __s.push_str(&format!("{}", (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.interface_ptr_error(T.clone()); __result }.lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ")".to_string())); __s }; *cause.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(missing method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        }
        } else if _switch_val == (2) {
            let (mut fs, mut ms) = ({ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(f.clone(), Arc::new(Mutex::new(Some(false)))); __result }, { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(m.clone(), Arc::new(Mutex::new(Some(false)))); __result });
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(missing method %s)\n\t\thave %s\n\t\twant %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = fs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = ms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (3) {
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(unexported method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (4) {
            let (mut fs, mut ms) = ({ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(f.clone(), Arc::new(Mutex::new(Some(false)))); __result }, { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(m.clone(), Arc::new(Mutex::new(Some(false)))); __result });
            if { let __tmp_x = (*fs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ms.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // Don't report "want Foo, have Foo".
                // Add package information to disambiguate (go.dev/issue/54258).
        { let __tmp_0 = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(f.clone(), Arc::new(Mutex::new(Some(true)))); __result }; let __tmp_1 = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.func_string(m.clone(), Arc::new(Mutex::new(Some(true)))); __result }; *fs.lock().unwrap() = __tmp_0.lock().unwrap().take(); *ms.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
                        // Don't report "want Foo, have Foo".
                        // Add package information to disambiguate (go.dev/issue/54258).
            if { let __tmp_x = (*fs.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*ms.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
                // We still have "want Foo, have Foo".
                // This is most likely due to different type parameters with
                // the same name appearing in the instantiated signatures
                // (go.dev/issue/61685).
                // Rather than reporting this misleading error cause, for now
                // just point out that the method signature is incorrect.
                // TODO(gri) should find a good way to report the root cause
        { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(wrong type for method %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        break '__go_switch_2
    }
                        // We still have "want Foo, have Foo".
                        // This is most likely due to different type parameters with
                        // the same name appearing in the instantiated signatures
                        // (go.dev/issue/61685).
                        // Rather than reporting this misleading error cause, for now
                        // just point out that the method signature is incorrect.
                        // TODO(gri) should find a good way to report the root cause
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(wrong type for method %s)\n\t\thave %s\n\t\twant %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = fs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = ms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (5) {
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(ambiguous selector %s.%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (6) {
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(method %s has pointer receiver)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (7) {
            { let new_val = (*{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.sprintf(Arc::new(Mutex::new(Some("(%s.%s is a field, not a method)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = V.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result }.lock().unwrap().as_ref().unwrap()).clone(); *cause.lock().unwrap() = Some(new_val); };
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    };
        break;
    }
    }

        // This method may be formatted in funcString below, so must have a fully
        // set up signature.
        // Don't report "want Foo, have Foo".
        // Add package information to disambiguate (go.dev/issue/54258).
        // We still have "want Foo, have Foo".
        // This is most likely due to different type parameters with
        // the same name appearing in the instantiated signatures
        // (go.dev/issue/61685).
        // Rather than reporting this misleading error cause, for now
        // just point out that the method signature is incorrect.
        // TODO(gri) should find a good way to report the root cause
    return (m.clone(), { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 6; __tmp_x == __tmp_y });
}

impl GoValueClone for embeddedType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for instanceLookup {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
