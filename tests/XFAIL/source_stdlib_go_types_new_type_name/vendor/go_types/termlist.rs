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

pub(crate) const TERM_SEP: &'static str = " | ";


/// A termlist represents the type set represented by the union
/// t1 ∪ y2 ∪ ... tn of the type sets of the terms t1 to tn.
/// A termlist is in normal form if all terms are disjoint.
/// termlist operations don't require the operands to be in
/// normal form.
#[derive(Clone, Default)]
pub struct termlist(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<term>>>>>>>);

impl Display for termlist {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
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


pub(crate) static allTermlist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<termlist>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *allTermlist.lock().unwrap() = Some(Default::default());
    *allTermlist.lock().unwrap() = Some(termlist(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(term::default())))])))));
}


pub(crate) fn __go_zero_globals() {
    *allTermlist.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_10() {
    *allTermlist.lock().unwrap() = Some(termlist(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(term::default())))])))));
}


impl termlist {
    /// String prints the termlist exactly (without normalization).
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("\u{2205}".to_string())));
    }
        let mut buf: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap()).push_str(TERM_SEP);
    }
        (*buf.lock().unwrap().as_mut().unwrap()).push_str(&(*{ let __recv = x.clone(); let __recv_ptr: *const term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const term }; let __result = unsafe { &*__recv_ptr }.string(); __result }.lock().unwrap().as_ref().unwrap()).clone());
    } }
        return Arc::new(Mutex::new(Some({ let __builder = buf.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }

    /// isEmpty reports whether the termlist xl represents the empty set of types.
    pub fn is_empty(&self) -> bool {
                // If there's a non-nil term, the entire list is not empty.
                // If the termlist is in normal form, this requires at most
                // one iteration.
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        if (*x.lock().unwrap()).is_some() {
        return false;
    }
    } }
        true
    }

    /// isAll reports whether the termlist xl represents the set of all types.
    pub fn is_all(&self) -> bool {
                // If there's a 𝓤 term, the entire list is 𝓤.
                // If the termlist is in normal form, this requires at most
                // one iteration.
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        if (*x.lock().unwrap()).is_some() && (*(*x.lock().unwrap().as_ref().unwrap()).typ.lock().unwrap()).is_none() {
        return true;
    }
    } }
        false
    }

    /// norm returns the normal form of xl.
    pub fn norm(&self) -> Arc<Mutex<Option<termlist>>> {
                // Quadratic algorithm, but good enough for now.
                // TODO(gri) fix asymptotic performance
        let mut used = Arc::new(Mutex::new(Some(vec![false; ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as usize])));
        let mut rl: Arc<Mutex<Option<termlist>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, mut xi) in __range_values.iter().cloned().enumerate() {
        if (*xi.lock().unwrap()).is_none() || { let __seq = { let __seq_holder = used.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() } {
        continue
    }
        let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32); __tmp_x < __tmp_y } {
        let mut xj = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone();
        if (*xj.lock().unwrap()).is_none() || { let __seq = { let __seq_holder = used.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        {
        let (mut u1, mut u2) = { let __recv = xi.clone(); let __recv_ptr: *const term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const term }; let __result = unsafe { &*__recv_ptr }.union(xj.clone()); __result };;
        if (*u2.lock().unwrap()).is_none() {
            if (*(*u1.lock().unwrap().as_ref().unwrap()).typ.lock().unwrap()).is_none() {
        return allTermlist.clone();
    };
            { let new_val = u1.clone(); xi = new_val; };;
            (*used.lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = true;;
        }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // If we encounter a 𝓤 term, the entire list is 𝓤.
                // Exit early.
                // (Note that this is not just an optimization;
                // if we continue, we may end up with a 𝓤 term
                // and other terms and the result would not be
                // in normal form.)
                // xj is now unioned into xi - ignore it in future iterations
        { let new_val = { let __base = { let __named_slice = (*rl.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(xi.clone()); Arc::new(Mutex::new(Some(termlist(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rl.lock().unwrap() = __moved_val; };
    } }
                // If we encounter a 𝓤 term, the entire list is 𝓤.
                // Exit early.
                // (Note that this is not just an optimization;
                // if we continue, we may end up with a 𝓤 term
                // and other terms and the result would not be
                // in normal form.)
                // xj is now unioned into xi - ignore it in future iterations
        return rl.clone();
    }

    /// union returns the union xl ∪ yl.
    pub fn union(&self, yl: Arc<Mutex<Option<termlist>>>) -> Arc<Mutex<Option<termlist>>> {
        { let __recv = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); let __src = { let __named_slice = (*yl.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __src_guard = __src.lock().unwrap(); if let Some(__src_values) = __src_guard.as_ref() { __values.extend(__src_values.iter().cloned()); }; Arc::new(Mutex::new(Some(termlist(Arc::new(Mutex::new(Some(__values))))))) }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).norm(); __result }
    }

    /// intersect returns the intersection xl ∩ yl.
    pub fn intersect(&self, yl: Arc<Mutex<Option<termlist>>>) -> Arc<Mutex<Option<termlist>>> {
        if self.is_empty() || (*yl.lock().unwrap().as_ref().unwrap()).is_empty() {
        return Arc::new(Mutex::new(None));
    }
                // Quadratic algorithm, but good enough for now.
                // TODO(gri) fix asymptotic performance
        let mut rl: Arc<Mutex<Option<termlist>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        { let __range_holder = { let __named_slice = (*yl.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for y in __range_values.iter() {
        {
        let mut r = { let __recv = x.clone(); let __recv_ptr: *const term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const term }; let __result = unsafe { &*__recv_ptr }.intersect((*y).clone()); __result };;
        if (*r.lock().unwrap()).is_some() {
            { let new_val = { let __base = { let __named_slice = (*rl.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); drop(__base_guard); __values.push(r.clone()); Arc::new(Mutex::new(Some(termlist(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rl.lock().unwrap() = __moved_val; };;
        }
    }
    } }
    } }
        return (*rl.lock().unwrap().as_ref().unwrap()).norm();
    }

    /// equal reports whether xl and yl represent the same type set.
    pub fn equal(&self, yl: Arc<Mutex<Option<termlist>>>) -> bool {
                // TODO(gri) this should be more efficient
        self.subset_of(yl.clone()) && (*yl.lock().unwrap().as_ref().unwrap()).subset_of(Arc::new(Mutex::new(Some(self.clone()))))
    }

    /// includes reports whether t ∈ xl.
    pub fn includes(&self, t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        if { let __recv = x.clone(); let __recv_ptr: *const term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const term }; let __result = unsafe { &*__recv_ptr }.includes(t.clone()); __result } {
        return true;
    }
    } }
        false
    }

    /// supersetOf reports whether y ⊆ xl.
    pub fn superset_of(&self, y: Arc<Mutex<Option<term>>>) -> bool {
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        if { let __recv = y.clone(); let __recv_ptr: *const term = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const term }; let __result = unsafe { &*__recv_ptr }.subset_of((*x).clone()); __result } {
        return true;
    }
    } }
        false
    }

    /// subsetOf reports whether xl ⊆ yl.
    pub fn subset_of(&self, yl: Arc<Mutex<Option<termlist>>>) -> bool {
        if (*yl.lock().unwrap().as_ref().unwrap()).is_empty() {
        return self.is_empty();
    }
                // each term x of xl must be a subset of yl
        { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        if !(*yl.lock().unwrap().as_ref().unwrap()).superset_of((*x).clone()) {
        return false;
    }
    } }
                // x is not a subset yl
        true
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
