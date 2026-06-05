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
use crate::lookup::*;
use crate::map::*;
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

/// A MethodSet is an ordered set of concrete or abstract (interface) methods;
/// a method is a [MethodVal] selection, and they are ordered by ascending m.Obj().Id().
/// The zero value for a MethodSet is a ready-to-use empty method set.
#[derive(Clone, Default)]
pub struct MethodSet {
    pub list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Selection>>>>>>>,
}

impl MethodSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { list: self.list.clone() }
    }
}

impl std::fmt::Display for MethodSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for MethodSet {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A methodSet is a set of methods and name collisions.
/// A collision indicates that multiple methods with the
/// same unique id, or a field with that id appeared.
#[derive(Clone, Default)]
pub struct methodSet(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Selection>>>>>>>);


pub(crate) static emptyMethodSet: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<MethodSet>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *emptyMethodSet.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *emptyMethodSet.lock().unwrap() = Some(Default::default());
}


impl MethodSet {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = self.len(); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("MethodSet {}".to_string())));
    }
        let mut buf: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}\n", format!("{}", "MethodSet {".to_string()))))));
        { let __range_holder = self.list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("\t{}\n", format!("&{}", (*f.lock().unwrap().as_ref().unwrap())))))));
    } }
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}\n", format!("{}", "}".to_string()))))));
        return (*buf.lock().unwrap().as_ref().unwrap()).string();
    }

    /// Len returns the number of methods in s.
    pub fn len(&self) -> i32 {
        ({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// At returns the i'th method in s for 0 <= i < s.Len().
    pub fn at(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::selection::Selection>>> {
        { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// Lookup returns the method with matching package and name, or nil if not found.
    pub fn lookup(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<crate::selection::Selection>>> {
        if { let __tmp_x = self.len(); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        let mut key = id(pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let key_closure_clone = key.clone(); let mut s_closure_clone = (*self).clone(); let mut i = sort::search(Arc::new(Mutex::new(Some(({ let __len_target = { let __field = s_closure_clone.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32))), Arc::new(Mutex::new(Some({ let mut s_closure_clone_closure_clone = s_closure_clone.clone(); Box::new(move |i: Arc<Mutex<Option<i32>>>| -> bool {
        let mut m = { let __seq = { let __seq_holder = s_closure_clone_closure_clone.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        return { let __tmp_x = (*(*(*m.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).id().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*key_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x >= __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }))));
        if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = self.list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut m = { let __seq = { let __seq_holder = self.list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if { let __tmp_x = (*(*(*m.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).id().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*key.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return m.clone();
    }
    }
        return Arc::new(Mutex::new(None));
    }
}

impl methodSet {
    /// Add adds all functions in list to the method set s.
    /// If multiples is set, every function in list appears multiple times
    /// and is treated as a collision.
    pub fn add(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>, index: Arc<Mutex<Option<Vec<i32>>>>, indirect: Arc<Mutex<Option<bool>>>, multiples: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<methodSet>>> {
        let mut __self = self.clone();
        if { let __tmp_x = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some(__self.clone())));
    }
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        { let new_val = __self.add_one((*f).clone(), concat(index.clone(), Arc::new(Mutex::new(Some(i as i32)))), Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = multiples.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    } }
        Arc::new(Mutex::new(Some(__self.clone())))
    }

    pub fn add_one(&self, f: Arc<Mutex<Option<Func>>>, index: Arc<Mutex<Option<Vec<i32>>>>, indirect: Arc<Mutex<Option<bool>>>, multiples: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<methodSet>>> {
        let mut __self = self.clone();
        if false {
        { let new_val = Arc::new(Mutex::new(Some(methodSet(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::selection::Selection>>>>::new()))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take().unwrap() }; __self = __moved_val; };
    }
        let mut key = { let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result };
                // if f is not in the set, add it
        if !{ let __v = (*multiples.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // TODO(gri) A found method may not be added because it's not in the method set
                // (!indirect && f.hasPtrRecv()). A 2nd method on the same level may be in the method
                // set and may not collide with the first one, thus leading to a false positive.
                // Is that possible? Investigate.
        {
        let (_, mut found) = { let __map = { let __map_holder = __self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&(*key.lock().unwrap().as_ref().unwrap()).clone())) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !found && ({ let __v = (*indirect.lock().unwrap().as_ref().unwrap()).clone(); __v } || !{ let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.has_ptr_recv(); __result }) {
            { let __map_key = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(Some(Selection { kind: Arc::new(Mutex::new(Some(crate::selection::SelectionKind(Arc::new(Mutex::new(Some(METHOD_VAL as i32))))))), recv: Arc::new(Mutex::new(None)), obj: Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(f.clone())) as Box<dyn Object + Send + Sync>))), index: index.clone(), indirect: Arc::new(Mutex::new(Some({ let __arg_holder = indirect.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))); (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            return Arc::new(Mutex::new(Some(__self.clone())));;
        }
    }
    }
                // TODO(gri) A found method may not be added because it's not in the method set
                // (!indirect && f.hasPtrRecv()). A 2nd method on the same level may be in the method
                // set and may not collide with the first one, thus leading to a false positive.
                // Is that possible? Investigate.
        { let __map_key = (*key.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = Arc::new(Mutex::new(None)); (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        Arc::new(Mutex::new(Some(__self.clone())))
    }
}

/// NewMethodSet returns the method set for the given type T.
/// It always returns a non-nil method set, even if it is empty.
pub fn new_method_set(T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<MethodSet>>> {
        // WARNING: The code in this function is extremely subtle - do not modify casually!
        //          This function and lookupFieldOrMethod should be kept in sync.
        // TODO(rfindley) confirm that this code is in sync with lookupFieldOrMethod
        //                with respect to type params.
        // Methods cannot be associated with a named pointer type.
        // (spec: "The type denoted by T is called the receiver base type;
        // it must not be a pointer or interface type and it must be declared
        // in the same package as the method.").
    {
        let mut t = as_named(T.clone());;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && is_pointer(Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(t.clone())) as Box<dyn Type + Send + Sync>)))) {
            return emptyMethodSet.clone();;
        }
    }

        // method set up to the current depth, allocated lazily
    let mut base: Arc<Mutex<Option<methodSet>>> = Arc::new(Mutex::new(Some(methodSet(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::selection::Selection>>>>::new())))))));

    let (mut typ, mut isPtr) = deref(T.clone());

        // *typ where typ is an interface has no methods.
    if isPtr && is_interface(typ.clone()) {
        return emptyMethodSet.clone();
    }

        // Start with typ as single entry at shallowest depth.
    let mut current = Arc::new(Mutex::new(Some(vec![crate::lookup::embeddedType { typ: typ.clone(), index: Arc::new(Mutex::new(None)), indirect: Arc::new(Mutex::new(Some(isPtr))), multiples: Arc::new(Mutex::new(Some(false))), ..Default::default() }])));

        // seen tracks named types that we have seen already, allocated lazily.
        // Used to avoid endless searches in case of recursive types.
        //
        // We must use a lookup on identity rather than a simple map[*Named]bool as
        // instantiated types may be identical but not equal.
    let mut seen: Arc<Mutex<Option<instanceLookup>>> = Arc::new(Mutex::new(Some(Default::default())));

        // collect methods at current depth
    while { let __tmp_x = ((*current.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut next: Arc<Mutex<Option<Vec<embeddedType>>>> = Arc::new(Mutex::new(None));

                // field and method sets at current depth, indexed by names (Id's), and allocated lazily
        let mut fset: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
        let mut mset: Arc<Mutex<Option<methodSet>>> = Arc::new(Mutex::new(Some(methodSet(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::selection::Selection>>>>::new())))))));

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
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = named.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.num_methods(); __result }; __tmp_x < __tmp_y } {
        { let new_val = (*mset.lock().unwrap().as_ref().unwrap()).add_one({ let __recv = named.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }, concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some({ let __selector_holder = e.indirect.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = e.multiples.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); mset = new_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
        }
    }
                // We have seen this type before, at a more shallow depth
                // (note that multiples of this type at the current depth
                // were consolidated before). The type at that depth shadows
                // this same type at the current depth, so we can ignore
                // this one.
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
        if { let __nil_result = (*fset.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::new()))); fset = new_val; };
    }
        { let __map_key = { let __map_key_holder = { let __recv = f.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.id(); __result }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some(true))); (*fset.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        let (mut typ, mut isPtr) = deref({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
        { let new_val = { let __append_target = next.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(embeddedType { typ: typ.clone(), index: concat({ let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(i as i32)))), indirect: Arc::new(Mutex::new(Some((*e.indirect.lock().unwrap().as_ref().unwrap()) || isPtr))), multiples: Arc::new(Mutex::new(Some({ let __selector_holder = e.multiples.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }); __append_target.clone() }; next = new_val; };
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        { let new_val = (*mset.lock().unwrap().as_ref().unwrap()).add({ let __field = (*{ let __recv = t.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }, { let __field = e.index.clone(); __field }, Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __selector_holder = e.multiples.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); mset = new_val; };;
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
                // Embedded fields are always of the form T or *T where
                // T is a type name. If typ appeared multiple times at
                // this depth, f.Type appears multiple times at the next
                // depth.
                // TODO(gri) optimization: ignore types that can't
                // have fields or methods (only Named, Struct, and
                // Interface types need to be considered).
                // Add methods and collisions at this depth to base if no entries with matching
                // names exist already.
        for (k, mut m) in { let __range_holder = { let __named_map_holder = mset.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        {
        let (_, mut found) = { let __map = { let __map_holder = { let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !found {
            if { let __map = { let __map_holder = fset.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&k)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        *m.lock().unwrap() = None;
    };
            if { let __map_holder = { let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none() } {
        { let new_val = Arc::new(Mutex::new(Some(methodSet(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::selection::Selection>>>>::new()))))))); base = new_val; };
    };
            { let __map_key = k.clone(); let __map_value = m.clone(); (*{ let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

                // Fields collide with methods of the same name at this depth.
                // collision
                // Add all (remaining) fields at this depth as collisions (since they will
                // hide any method further down) if no entries with matching names exist already.
        for (k, _) in { let __range_holder = fset.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        {
        let (_, mut found) = { let __map = { let __map_holder = { let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&k)) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if !found {
            if { let __map_holder = { let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.lock().unwrap(); (*__map_guard).is_none() } {
        { let new_val = Arc::new(Mutex::new(Some(methodSet(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::selection::Selection>>>>::new()))))))); base = new_val; };
    };
            { let __map_key = k.clone(); let __map_value = Arc::new(Mutex::new(None)); (*{ let __named_map = (*base.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }

                // collision
        { let new_val = consolidate_multiples(next.clone()); current = new_val; };
    }

        // embedded types found at current depth
        // field and method sets at current depth, indexed by names (Id's), and allocated lazily
        // we only care about the field names
        // If we have a named type, we may have associated methods.
        // Look for those first.
        // We have seen this type before, at a more shallow depth
        // (note that multiples of this type at the current depth
        // were consolidated before). The type at that depth shadows
        // this same type at the current depth, so we can ignore
        // this one.
        // Embedded fields are always of the form T or *T where
        // T is a type name. If typ appeared multiple times at
        // this depth, f.Type appears multiple times at the next
        // depth.
        // TODO(gri) optimization: ignore types that can't
        // have fields or methods (only Named, Struct, and
        // Interface types need to be considered).
        // Add methods and collisions at this depth to base if no entries with matching
        // names exist already.
        // Fields collide with methods of the same name at this depth.
        // collision
        // Add all (remaining) fields at this depth as collisions (since they will
        // hide any method further down) if no entries with matching names exist already.
        // collision
    if { let __tmp_x = ({ let __named_map_holder = base.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return emptyMethodSet.clone();
    }

        // collect methods
    let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Selection>>>>>>> = Arc::new(Mutex::new(None));
    for (_, m) in { let __range_holder = { let __named_map_holder = base.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if { let __nil_result = (*m.lock().unwrap()).is_some(); __nil_result } {
        { let __iface_handle = T.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*m.lock().unwrap().as_mut().unwrap()).recv.lock().unwrap() = __iface_value; };
        { let new_val = { let __append_target = list.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(m.clone()); __append_target.clone() }; list = new_val; };
    }
    }

        // sort by unique name
    let list_closure_clone = list.clone(); { let __sort_target = list_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>| -> bool {
        return { let __tmp_x = (*(*(*{ let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).id().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*(*{ let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).id().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i32))), Arc::new(Mutex::new(Some((__sort_j - 1) as i32)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    return Arc::new(Mutex::new(Some(MethodSet { list: list.clone(), ..Default::default() })));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for MethodSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
