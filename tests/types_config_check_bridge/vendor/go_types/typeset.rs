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
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A _TypeSet represents the type set of an interface.
/// Because of existing language restrictions, methods can be "factored out"
/// from the terms. The actual type set is the intersection of the type set
/// implied by the methods and the type set described by the terms and the
/// comparable bit. To test whether a type is included in a type set
/// ("implements" relation), the type must implement all methods _and_ be
/// an element of the type set described by the terms and the comparable bit.
/// If the term list describes the set of all types and comparable is true,
/// only comparable types are meant; in all other cases comparable is false.
#[derive(Clone)]
pub struct _TypeSet {
    pub methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>,
    pub terms: Arc<Mutex<Option<termlist>>>,
    pub comparable: Arc<Mutex<Option<bool>>>,
}

impl _TypeSet {
    pub fn __go_value_clone(&self) -> Self {
        Self { methods: self.methods.clone(), terms: self.terms.clone(), comparable: { let __guard = self.comparable.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for _TypeSet {
    fn default() -> Self {
        Self { methods: Arc::new(Mutex::new(None)), terms: Arc::new(Mutex::new(Some(Default::default()))), comparable: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for _TypeSet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for _TypeSet {
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


pub(crate) static topTypeSet: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<_TypeSet>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static invalidTypeSet: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<_TypeSet>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *topTypeSet.lock().unwrap() = Some(Default::default());
    *invalidTypeSet.lock().unwrap() = Some(Default::default());
    *topTypeSet.lock().unwrap() = Some(_TypeSet { terms: allTermlist.clone(), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *topTypeSet.lock().unwrap() = Some(Default::default());
    *invalidTypeSet.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_11() {
    *topTypeSet.lock().unwrap() = Some(_TypeSet { terms: allTermlist.clone(), ..Default::default() });
}


impl _TypeSet {
    /// IsEmpty reports whether s is the empty set.
    pub fn is_empty(&self) -> bool {
        (*self.terms.lock().unwrap().as_ref().unwrap()).is_empty()
    }

    /// IsAll reports whether s is the set of all types (corresponding to the empty interface).
    pub fn is_all(&self) -> bool {
        self.is_method_set() && { let __tmp_x = (({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y }
    }

    /// IsMethodSet reports whether the interface t is fully described by its method set.
    pub fn is_method_set(&self) -> bool {
        !((*self.comparable.clone().lock().unwrap().as_ref().unwrap())) && (*self.terms.lock().unwrap().as_ref().unwrap()).is_all()
    }

    /// IsComparable reports whether each type in the set is comparable.
    pub fn is_comparable(&self, seen: Arc<Mutex<Option<BTreeMap<GoTypeInterfaceKey, Arc<Mutex<Option<bool>>>>>>>) -> bool {
        if (*self.terms.lock().unwrap().as_ref().unwrap()).is_all() {
        return (*self.comparable.lock().unwrap().as_ref().unwrap());
    }
        let seen_closure_clone = seen.clone(); return self.is(Arc::new(Mutex::new(Some(Box::new(move |t: Arc<Mutex<Option<term>>>| -> bool {
        return { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } && comparable_type({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some(false))), seen_closure_clone.clone(), Arc::new(Mutex::new(None)));
    }) as Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>))));
    }

    /// NumMethods returns the number of methods available.
    pub fn num_methods(&self) -> i32 {
        ({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Method returns the i'th method of s for 0 <= i < s.NumMethods().
    /// The methods are ordered by their unique ID.
    pub fn method(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
        { let __seq = { let __seq_holder = self.methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// LookupMethod returns the index of and method with matching package and name, or (-1, nil).
    pub fn lookup_method(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> (i32, Arc<Mutex<Option<crate::object::Func>>>) {
        method_index({ let __field = self.methods.clone(); __field }, pkg.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if self.is_empty() {
            return Arc::new(Mutex::new(Some("\u{2205}".to_string())));
        } else if self.is_all() {
            return Arc::new(Mutex::new(Some("\u{1d4e4}".to_string())));
        }
        let mut hasMethods = Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y })));
        let mut hasTerms = self.has_terms();
        let mut buf: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*buf.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some(('{' as i32) as u8))));
        if (*self.comparable.clone().lock().unwrap().as_ref().unwrap()) {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("comparable".to_string()))));
        if { let __v = (*hasMethods.lock().unwrap().as_ref().unwrap()).clone(); __v } || hasTerms {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("; ".to_string()))));
    }
    }
        { let __range_holder = self.methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, m) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("; ".to_string()))));
    }
        (*buf.lock().unwrap().as_mut().unwrap()).write_string({ let __recv = m.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.string(); __result });
    } }
        if { let __v = (*hasMethods.lock().unwrap().as_ref().unwrap()).clone(); __v } && hasTerms {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("; ".to_string()))));
    }
        if hasTerms {
        (*buf.lock().unwrap().as_mut().unwrap()).write_string((*self.terms.lock().unwrap().as_ref().unwrap()).string());
    }
        (*buf.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("}".to_string()))));
        return (*buf.lock().unwrap().as_ref().unwrap()).string();
    }

    /// hasTerms reports whether s has specific type terms.
    pub fn has_terms(&self) -> bool {
        !(*self.terms.lock().unwrap().as_ref().unwrap()).is_empty() && !(*self.terms.lock().unwrap().as_ref().unwrap()).is_all()
    }

    /// subsetOf reports whether s1 ⊆ s2.
    pub fn subset_of(&self, s2: Arc<Mutex<Option<_TypeSet>>>) -> bool {
        (*self.terms.lock().unwrap().as_ref().unwrap()).subset_of({ let __field = (*s2.lock().unwrap().as_ref().unwrap()).terms.clone(); __field })
    }

    /// typeset is an iterator over the (type/underlying type) pairs in s.
    /// If s has no specific terms, typeset calls yield with (nil, nil).
    /// In any case, typeset is guaranteed to call yield at least once.
    pub fn typeset(&self, r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) {
        if !self.has_terms() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))) };
        return;
    }
        { let __range_holder = { let __named_slice = (*self.terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
                // Unalias(x) == under(x) for ~x terms
        let mut u = unalias({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
        if !(*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __iface_handle = under(u.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *u.lock().unwrap() = __iface_value; };
    }
        if DEBUG {
        assert(Arc::new(Mutex::new(Some(identical(u.clone(), under(u.clone()).clone())))));
    }
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, u.clone()) } {
        break
    }
    } }
    }

    /// is calls f with the specific type terms of s and reports whether
    /// all calls to f returned true. If there are no specific terms, is
    /// returns the result of f(nil).
    pub fn is(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync>>>>) -> bool {
        if !self.has_terms() {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(None))) };
    }
        { let __range_holder = { let __named_slice = (*self.terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<term>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*t).clone()) } {
        return false;
    }
    } }
        true
    }
}

/// computeInterfaceTypeSet may be called with check == nil.
pub fn compute_interface_type_set(check: Arc<Mutex<Option<Checker>>>, mut pos: Arc<Mutex<Option<go_token::position::Pos>>>, ityp: Arc<Mutex<Option<Interface>>>) -> Arc<Mutex<Option<_TypeSet>>> {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let __go_previous_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if { let __nil_target = (*ityp.lock().unwrap().as_ref().unwrap()).tset.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*ityp.lock().unwrap().as_ref().unwrap()).tset.clone();
    }
    }

                // If the interface is not fully set up yet, the type set will
                // not be complete, which may lead to errors when using the
                // type set (e.g. missing method). Don't compute a partial type
                // set (and don't store it!), so that we still compute the full
                // type set eventually. Instead, return the top type set and
                // let any follow-on errors play out.
                //
                // TODO(gri) Consider recording when this happens and reporting
                // it as an error (but only if there were no other errors so
                // to not have unnecessary follow-on errors).
        if !(*{ let __field = (*ityp.lock().unwrap().as_ref().unwrap()).complete.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return topTypeSet.clone();
    }
    }

        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && (*(*(*check.lock().unwrap().as_ref().unwrap()).conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
                // Types don't generally have position information.
                // If we don't have a valid pos provided, try to use
                // one close enough.
        if !go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) && { let __tmp_x = (({ let __len_target = { let __field = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*{ let __seq = { let __seq_holder = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *pos.lock().unwrap() = Some(new_val); };
    }
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.trace(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("-- type set for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(ityp.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        { let __target = (*check.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let check_defer_captured = check.clone(); let ityp_defer_captured = ityp.clone(); let pos_defer_captured = pos.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = (*check_defer_captured.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __recv = check_defer_captured.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.trace(Arc::new(Mutex::new(Some({ let __arg_holder = pos_defer_captured.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("=> %s ".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __recv = ityp_defer_captured.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }

                // Types don't generally have position information.
                // If we don't have a valid pos provided, try to use
                // one close enough.
                // An infinitely expanding interface (due to a cycle) is detected
                // elsewhere (Checker.validType), so here we simply assume we only
                // have valid interfaces. Mark the interface as complete to avoid
                // infinite recursion if the validType check occurs later for some
                // reason.
        { let new_val = Arc::new(Mutex::new(Some(_TypeSet { terms: allTermlist.clone(), ..Default::default() }))).clone(); (*ityp.lock().unwrap().as_mut().unwrap()).tset = new_val; };

        let mut unionSets: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<_TypeSet>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::new())));
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } {
        if { let __nil_target = (*check.lock().unwrap().as_ref().unwrap()).union_type_sets.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<_TypeSet>>>>::new()))); (*check.lock().unwrap().as_mut().unwrap()).union_type_sets = new_val; };
    }
        { let new_val = (*check.lock().unwrap().as_ref().unwrap()).union_type_sets.clone(); unionSets = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<_TypeSet>>>>::new()))); unionSets = new_val; };
    }

                // Methods of embedded interfaces are collected unchanged; i.e., the identity
                // of a method I.m's Func Object of an interface I is the same as that of
                // the method m in an interface that embeds interface I. On the other hand,
                // if a method is embedded via multiple overlapping embedded interfaces, we
                // don't provide a guarantee which "original m" got chosen for the embedding
                // interface. See also go.dev/issue/34421.
                //
                // If we don't care to provide this identity guarantee anymore, instead of
                // reusing the original method in embeddings, we can clone the method's Func
                // Object and give it the position of a corresponding embedded interface. Then
                // we can get rid of the mpos map below and simply use the cloned method's
                // position.
        let mut seen: Arc<Mutex<Option<objset>>> = Arc::new(Mutex::new(Some(crate::objset::objset(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new())))))));
        let mut allMethods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>> = Arc::new(Mutex::new(None));
        let mut mpos = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::object::Func>, Arc<Mutex<Option<go_token::position::Pos>>>>::new())));
        let mut allMethods_closure_clone = allMethods.clone(); let check_closure_clone = check.clone(); let mpos_closure_clone = mpos.clone(); let seen_closure_clone = seen.clone(); let mut addMethod = Arc::new(Mutex::new(Some(Box::new(move |pos: Arc<Mutex<Option<go_token::position::Pos>>>, m: Arc<Mutex<Option<Func>>>, explicit: Arc<Mutex<Option<bool>>>| {
        let mut other = (*seen_closure_clone.lock().unwrap().as_mut().unwrap()).insert(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))));
    if { let __nil_result = (*other.lock().unwrap()).is_none(); __nil_result } {
            { let __append_target = allMethods_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(m.clone()); __append_target.clone() };
            { let __map_key = GoLocalPtrKey::new(m.clone()); let __map_value = Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))); (*mpos_closure_clone.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        } else if { let __v = (*explicit.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            if { let __nil_result = (*check_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        let mut err = { let __recv = check_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32)))))))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate method %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __map = { let __map_holder = mpos_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(({
        let val = other.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| go_token::position::Pos(Arc::new(Mutex::new(Some(0))))) }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("other declaration of method %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
        } else {
            if { let __nil_result = (*check_closure_clone.lock().unwrap()).is_some(); __nil_result } {
        let check_closure_clone_closure_clone = check_closure_clone.clone(); let m_closure_clone = m.clone(); let mpos_closure_clone_closure_clone = mpos_closure_clone.clone(); let other_closure_clone = other.clone(); let pos_closure_clone = pos.clone(); { let __recv = { let __recv = check_closure_clone_closure_clone.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.later(Arc::new(Mutex::new(Some({ let check_closure_clone_closure_clone_closure_clone = check_closure_clone_closure_clone.clone(); let m_closure_clone_closure_clone = m_closure_clone.clone(); let pos_closure_clone_closure_clone = pos_closure_clone.clone(); Box::new(move || {
        if go_token::position::Pos::is_valid(&(*pos_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap())) && !{ let __recv = check_closure_clone_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = go1_14.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } || !identical({ let __field = (*(*m_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, (*other_closure_clone.lock().unwrap().as_ref().unwrap()).r#type().clone()) {
        let mut err = { let __recv = check_closure_clone_closure_clone_closure_clone.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32)))))))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate method %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*m_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __map = { let __map_holder = mpos_closure_clone_closure_clone.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(({
        let val = other_closure_clone.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| go_token::position::Pos(Arc::new(Mutex::new(Some(0))))) }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("other declaration of method %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*m_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
    }) as Box<dyn FnMut() -> () + Send + Sync> })))); __result }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos_closure_clone.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate method check for %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*m_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
        }
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Func>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync>)));

                // We have a duplicate method name in an embedded (not explicitly declared) method.
                // Check method signatures after all types are computed (go.dev/issue/33656).
                // If we're pre-go1.14 (overlapping embeddings are not permitted), report that
                // error here as well (even though we could do it eagerly) because it's the same
                // error message.
        { let __range_holder = (*ityp.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Func>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = addMethod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Func>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), (*m).clone(), Arc::new(Mutex::new(Some(true)))) };
    } }

                // collect embedded elements
        let mut allTerms = Arc::new(Mutex::new(Some((*allTermlist.lock().unwrap().as_ref().unwrap()).clone())));
        let mut allComparable = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = (*ityp.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, typ) in __range_values.iter().enumerate() {
                // The embedding position is nil for imported interfaces.
                // We don't need to do version checks in those cases.
        let mut pos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0)))))));
        if { let __nil_target = (*ityp.lock().unwrap().as_ref().unwrap()).embed_pos.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*{ let __seq = ({ let __v = (*(*ityp.lock().unwrap().as_ref().unwrap()).embed_pos.lock().unwrap().as_ref().unwrap()).clone(); __v }); __seq[(i) as usize].clone() }.0.lock().unwrap().as_ref().unwrap()))))); *pos.lock().unwrap() = Some(new_val); };
    }
        let mut comparable: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let mut terms: Arc<Mutex<Option<termlist>>> = Arc::new(Mutex::new(Some(Default::default())));
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(!is_type_param(typ.clone())))));;
        let mut tset = compute_interface_type_set(check.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), u.clone());;
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) && { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.is_imported_constraint(typ.clone()); __result } && !{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("embedding constraint interface %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(typ.clone()) as Box<dyn Any + Send + Sync>])))); __result } {
        continue
    };
        { let new_val = { let __selector_holder = (*tset.lock().unwrap().as_ref().unwrap()).comparable.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *comparable.lock().unwrap() = Some(new_val); };;
        { let __range_holder = (*tset.lock().unwrap().as_ref().unwrap()).methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Func>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> = { let mut __f_guard = addMethod.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<Func>>>, Arc<Mutex<Option<bool>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(pos.clone(), (*m).clone(), Arc::new(Mutex::new(Some(false)))) };
    } };
        { let new_val = { let __selector_holder = (*tset.lock().unwrap().as_ref().unwrap()).terms.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *terms.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).is_some() {
        let u = _ts_val.and_then(|__v| __v.downcast_ref::<crate::union::UnionPtr>()).unwrap().0.clone();
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) && { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && !{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("embedding interface element %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(u.clone()) as Box<dyn Any + Send + Sync>])))); __result } {
        continue
    };
        let mut tset = compute_union_type_set(check.clone(), unionSets.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), u.clone());;
        if { let __left = tset.clone(); let __right = invalidTypeSet.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        continue
    };
        assert(Arc::new(Mutex::new(Some(!(*{ let __field = (*tset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap())))));;
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = (*tset.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y }))));;
        { let new_val = { let __selector_holder = (*tset.lock().unwrap().as_ref().unwrap()).terms.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *terms.lock().unwrap() = Some(new_val); };;
    } else {
        let u = _ts_subject.clone();
        if !is_valid(u.clone()) {
        continue
    };
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) && { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } && !{ let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("embedding non-interface type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(typ.clone()) as Box<dyn Any + Send + Sync>])))); __result } {
        continue
    };
        { let new_val = termlist(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(crate::typeterm::term { tilde: Arc::new(Mutex::new(Some(false))), typ: typ.clone(), ..Default::default() })))])))); *terms.lock().unwrap() = Some(new_val); };;
    }
    }
                // For now we don't permit type parameters as constraints.
                // If typ is local, an error was already reported where typ is specified/defined.
                // use embedding position pos rather than m.pos
                // ignore invalid unions
                // The type set of an interface is the intersection of the type sets of all its elements.
                // Due to language restrictions, only embedded interfaces can add methods, they are handled
                // separately. Here we only need to intersect the term lists and comparable bits.
        { let (__tmp_0, __tmp_1) = intersect_term_lists(Arc::new(Mutex::new(Some({ let __arg_holder = allTerms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = allComparable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = terms.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = comparable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *allTerms.lock().unwrap() = __moved_tmp_0; *allComparable.lock().unwrap() = Some(__tmp_1); };
    } }

                // The embedding position is nil for imported interfaces.
                // We don't need to do version checks in those cases.
                // embedding position
                // For now we don't permit type parameters as constraints.
                // If typ is local, an error was already reported where typ is specified/defined.
                // use embedding position pos rather than m.pos
                // ignore invalid unions
                // The type set of an interface is the intersection of the type sets of all its elements.
                // Due to language restrictions, only embedded interfaces can add methods, they are handled
                // separately. Here we only need to intersect the term lists and comparable bits.
        { let new_val = allComparable.lock().unwrap().as_ref().unwrap().clone(); *(*(*ityp.lock().unwrap().as_ref().unwrap()).tset.lock().unwrap().as_ref().unwrap()).comparable.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = ((*allMethods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x != __tmp_y } {
        sort_methods(allMethods.clone());
        { let new_val = allMethods.clone(); (*(*ityp.lock().unwrap().as_ref().unwrap()).tset.lock().unwrap().as_mut().unwrap()).methods = new_val; };
    }
        { let new_val = allTerms.lock().unwrap().as_ref().unwrap().clone(); *(*(*ityp.lock().unwrap().as_ref().unwrap()).tset.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap() = Some(new_val); };

        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*ityp.lock().unwrap().as_ref().unwrap()).tset.clone();
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
            Arc::new(Mutex::new(None))
        }
    }
}

/// intersectTermLists computes the intersection of two term lists and respective comparable bits.
/// xcomp, ycomp are valid only if xterms.isAll() and yterms.isAll() respectively.
pub fn intersect_term_lists(xterms: Arc<Mutex<Option<termlist>>>, xcomp: Arc<Mutex<Option<bool>>>, yterms: Arc<Mutex<Option<termlist>>>, ycomp: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<crate::termlist::termlist>>>, bool) {
    let mut terms = (*xterms.lock().unwrap().as_ref().unwrap()).intersect(yterms.clone());

        // If one of xterms or yterms is marked as comparable,
        // the result must only include comparable types.
    let mut comp = Arc::new(Mutex::new(Some({ let __v = (*xcomp.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __v = (*ycomp.lock().unwrap().as_ref().unwrap()).clone(); __v })));
    if { let __v = (*comp.lock().unwrap().as_ref().unwrap()).clone(); __v } && !(*terms.lock().unwrap().as_ref().unwrap()).is_all() {
                // only keep comparable terms
        let mut i = Arc::new(Mutex::new(Some(0)));
        { let __range_holder = { let __named_slice = (*terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        if comparable_type({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None))) {
        (*{ let __named_slice = (*terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = t.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
                /* strictly comparable */
        { let new_val = crate::termlist::termlist(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*terms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })))); *terms.lock().unwrap() = Some(new_val); };
        if !(*terms.lock().unwrap().as_ref().unwrap()).is_all() {
        { let new_val = false; *comp.lock().unwrap() = Some(new_val); };
    }
    }
        // only keep comparable terms
        /* strictly comparable */
    assert(Arc::new(Mutex::new(Some(!{ let __v = (*comp.lock().unwrap().as_ref().unwrap()).clone(); __v } || (*terms.lock().unwrap().as_ref().unwrap()).is_all()))));
    return (terms.clone(), { let __v = (*comp.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

pub fn compare_func(a: Arc<Mutex<Option<Func>>>, b: Arc<Mutex<Option<Func>>>) -> i32 {
    { let __recv = a.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.cmp((*b.lock().unwrap().as_ref().unwrap()).object.clone()); __result }
}

pub fn sort_methods(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) {
    slices::sort_func::<Vec<Arc<Mutex<Option<crate::object::Func>>>>, crate::object::Func>(list.clone(), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::object::Func>>>, __arg1: Arc<Mutex<Option<crate::object::Func>>>| -> i32 { compare_func(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>, Arc<Mutex<Option<crate::object::Func>>>) -> i32 + Send + Sync>))));
}

pub fn assert_sorted_methods(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) {
    if !DEBUG {
        std::panic::panic_any(Box::new("assertSortedMethods called outside debug mode".to_string()) as Box<dyn Any + Send + Sync>);
    }
    if !slices::is_sorted_func::<Vec<Arc<Mutex<Option<crate::object::Func>>>>, crate::object::Func>(list.clone(), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<crate::object::Func>>>, __arg1: Arc<Mutex<Option<crate::object::Func>>>| -> i32 { compare_func(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>, Arc<Mutex<Option<crate::object::Func>>>) -> i32 + Send + Sync>)))) {
        std::panic::panic_any(Box::new("methods not sorted".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

/// computeUnionTypeSet may be called with check == nil.
/// The result is &invalidTypeSet if the union overflows.
pub fn compute_union_type_set(check: Arc<Mutex<Option<Checker>>>, unionSets: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::union::Union>, Arc<Mutex<Option<_TypeSet>>>>>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>, utyp: Arc<Mutex<Option<Union>>>) -> Arc<Mutex<Option<_TypeSet>>> {
    {
        let (mut tset, _) = { let __map = { let __map_holder = unionSets.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(utyp.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if { let __nil_result = (*tset.lock().unwrap()).is_some(); __nil_result } {
            return tset.clone();;
        }
    }

        // avoid infinite recursion (see also computeInterfaceTypeSet)
    { let __map_key = GoLocalPtrKey::new(utyp.clone()); let __map_value = Arc::new(Mutex::new(Some(_TypeSet::default()))); (*unionSets.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };

    let mut allTerms: Arc<Mutex<Option<termlist>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = (*utyp.lock().unwrap().as_ref().unwrap()).terms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut t in __range_values.iter().cloned() {
        let mut terms: Arc<Mutex<Option<termlist>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut u = under({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
        {
        let (mut ui, _) = ({
        let val = u.clone();
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
        if { let __nil_result = (*ui.lock().unwrap()).is_some(); __nil_result } {
            assert(Arc::new(Mutex::new(Some(!is_type_param({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field })))));;
            { let new_val = { let __selector_holder = (*compute_interface_type_set(check.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ui.clone()).lock().unwrap().as_ref().unwrap()).terms.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *terms.lock().unwrap() = Some(new_val); };;
        } else if !is_valid(u.clone()) {
        continue
    } else {
        if (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) && !identical({ let __field = (*t.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, u.clone()) {
        *t.lock().unwrap() = None;
    }
        { let new_val = termlist(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(term::default())))])))); *terms.lock().unwrap() = Some(new_val); };
    }
    }
                // For now we don't permit type parameters as constraints.
                // There is no underlying type which is t.typ.
                // The corresponding type set is empty.
                // ∅ term
                // The type set of a union expression is the union
                // of the type sets of each term.
        { let new_val = (*allTerms.lock().unwrap().as_ref().unwrap()).union(terms.clone()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *allTerms.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ({ let __slice_holder = { let __named_slice = (*allTerms.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 100; __tmp_x > __tmp_y } {
        if { let __nil_result = (*check.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.errorf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some((*pos.lock().unwrap().as_ref().unwrap()).clone()))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_UNION as i32))))))), Arc::new(Mutex::new(Some("cannot handle more than %d union terms (implementation limitation)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(MAX_TERM_COUNT) as Box<dyn Any + Send + Sync>])))); __result };
    }
        { let __map_key = GoLocalPtrKey::new(utyp.clone()); let __map_value = invalidTypeSet.clone(); (*unionSets.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        return { let __map = { let __map_holder = unionSets.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(utyp.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
    }
    } }
        // For now we don't permit type parameters as constraints.
        // There is no underlying type which is t.typ.
        // The corresponding type set is empty.
        // ∅ term
        // The type set of a union expression is the union
        // of the type sets of each term.
    { let new_val = allTerms.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __map = { let __map_holder = unionSets.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(utyp.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap() = Some(new_val); };

    { let __map = { let __map_holder = unionSets.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(utyp.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for _TypeSet {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
