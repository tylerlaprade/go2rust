use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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
use crate::typexpr::*;
use crate::under::*;
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A term describes elementary type sets:
///
///	 ∅:  (*term)(nil)     == ∅                      // set of no types (empty set)
///	 𝓤:  &term{}          == 𝓤                      // set of all types (𝓤niverse)
///	 T:  &term{false, T}  == {T}                    // set of type T
///	~t:  &term{true, t}   == {t' | under(t') == t}  // set of types with underlying type t
#[derive(Clone)]
pub struct term {
    pub tilde: Arc<Mutex<Option<bool>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl term {
    pub fn __go_value_clone(&self) -> Self {
        Self { tilde: { let __guard = self.tilde.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for term {
    fn default() -> Self {
        Self { tilde: Arc::new(Mutex::new(Some(false))), typ: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for term {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for term {
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


impl term {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if false {
            return Arc::new(Mutex::new(Some("\u{2205}".to_string())));
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return Arc::new(Mutex::new(Some("\u{1d4e4}".to_string())));
        } else if (*self.tilde.clone().lock().unwrap().as_ref().unwrap()) {
            return Arc::new(Mutex::new(Some(format!("{}{}", "~".to_string(), (*(*self.typ.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())))));
        } else {
            return (*self.typ.lock().unwrap().as_ref().unwrap()).string();
        }
    }

    /// equal reports whether x and y represent the same type set.
    pub fn equal(&self, y: Arc<Mutex<Option<term>>>) -> bool {
                // easy cases
        if false || { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
            return { let __peer = y.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq };
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } || { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return { let __left_holder = self.typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq };
        }
                // ∅ ⊂ x, y ⊂ 𝓤
        return { let __tmp_x = (*self.tilde.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && identical({ let __field = self.typ.clone(); __field }, { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field });
    }

    /// union returns the union x ∪ y: zero, one, or two non-nil terms.
    pub fn union(&self, y: Arc<Mutex<Option<term>>>) -> (Arc<Mutex<Option<term>>>, Arc<Mutex<Option<term>>>) {
    let _: Arc<Mutex<Option<term>>> = Arc::new(Mutex::new(None));
    let _: Arc<Mutex<Option<term>>> = Arc::new(Mutex::new(None));

                // easy cases
        if false && { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
        } else if false {
            return (y.clone(), Arc::new(Mutex::new(None)));
        } else if { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
            return (Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)));
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return (Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)));
        } else if { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return (y.clone(), Arc::new(Mutex::new(None)));
        }
                // ∅ ∪ ∅ == ∅
                // ∅ ∪ y == y
                // x ∪ ∅ == x
                // 𝓤 ∪ y == 𝓤
                // x ∪ 𝓤 == 𝓤
                // ∅ ⊂ x, y ⊂ 𝓤
        if self.disjoint(y.clone()) {
        return (Arc::new(Mutex::new(Some(self.clone()))), y.clone());
    }
                // x ∪ y == (x, y) if x ∩ y == ∅
                // x.typ == y.typ
                // ~t ∪ ~t == ~t
                // ~t ∪  T == ~t
                //  T ∪ ~t == ~t
                //  T ∪  T ==  T
        if (*self.tilde.clone().lock().unwrap().as_ref().unwrap()) || !(*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)));
    }
        (y.clone(), Arc::new(Mutex::new(None)))
    }

    /// intersect returns the intersection x ∩ y.
    pub fn intersect(&self, y: Arc<Mutex<Option<term>>>) -> Arc<Mutex<Option<term>>> {
                // easy cases
        if false || { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
            return Arc::new(Mutex::new(None));
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return y.clone();
        } else if { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return Arc::new(Mutex::new(Some(self.clone())));
        }
                // ∅ ∩ y == ∅ and ∩ ∅ == ∅
                // 𝓤 ∩ y == y
                // x ∩ 𝓤 == x
                // ∅ ⊂ x, y ⊂ 𝓤
        if self.disjoint(y.clone()) {
        return Arc::new(Mutex::new(None));
    }
                // x ∩ y == ∅ if x ∩ y == ∅
                // x.typ == y.typ
                // ~t ∩ ~t == ~t
                // ~t ∩  T ==  T
                //  T ∩ ~t ==  T
                //  T ∩  T ==  T
        if !(*self.tilde.clone().lock().unwrap().as_ref().unwrap()) || (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        y.clone()
    }

    /// includes reports whether t ∈ x.
    pub fn includes(&self, t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
                // easy cases
        if false {
            return false;
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return true;
        }
                // t ∈ ∅ == false
                // t ∈ 𝓤 == true
                // ∅ ⊂ x ⊂ 𝓤
        let mut u = t.clone();
        if (*self.tilde.clone().lock().unwrap().as_ref().unwrap()) {
        { let __iface_handle = under(u.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *u.lock().unwrap() = __iface_value; };
    }
        return identical({ let __field = self.typ.clone(); __field }, u.clone());
    }

    /// subsetOf reports whether x ⊆ y.
    pub fn subset_of(&self, y: Arc<Mutex<Option<term>>>) -> bool {
                // easy cases
        if false {
            return true;
        } else if { let __nil_result = (*y.lock().unwrap()).is_none(); __nil_result } {
            return false;
        } else if { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return true;
        } else if { let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
            return false;
        }
                // ∅ ⊆ y == true
                // x ⊆ ∅ == false since x != ∅
                // x ⊆ 𝓤 == true
                // 𝓤 ⊆ y == false since y != 𝓤
                // ∅ ⊂ x, y ⊂ 𝓤
        if self.disjoint(y.clone()) {
        return false;
    }
                // x ⊆ y == false if x ∩ y == ∅
                // x.typ == y.typ
                // ~t ⊆ ~t == true
                // ~t ⊆ T == false
                //  T ⊆ ~t == true
                //  T ⊆  T == true
        !((*self.tilde.clone().lock().unwrap().as_ref().unwrap())) || (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap())
    }

    /// disjoint reports whether x ∩ y == ∅.
    /// x.typ and y.typ must not be nil.
    pub fn disjoint(&self, y: Arc<Mutex<Option<term>>>) -> bool {
        if DEBUG && ({ let __iface_handle = { let __field = self.typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } || { let __iface_handle = { let __field = (*y.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }) {
        std::panic::panic_any(Box::new("invalid argument(s)".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut ux = self.typ.clone();
        if (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).tilde.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __iface_handle = under(ux.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *ux.lock().unwrap() = __iface_value; };
    }
        let mut uy = (*y.lock().unwrap().as_ref().unwrap()).typ.clone();
        if (*self.tilde.clone().lock().unwrap().as_ref().unwrap()) {
        { let __iface_handle = under(uy.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *uy.lock().unwrap() = __iface_value; };
    }
        return !identical(ux.clone(), uy.clone());
    }
}

impl GoValueClone for term {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
