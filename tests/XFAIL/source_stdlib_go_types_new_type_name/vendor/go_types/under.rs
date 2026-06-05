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
use crate::unify::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

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


/// under returns the true expanded underlying type.
/// If it doesn't exist, the result is Typ[Invalid].
/// under must only be called when a type is known
/// to be fully set up.
pub fn under(mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = t.clone();
    {
        let mut t = as_named(t.clone());;
        if (*t.lock().unwrap()).is_some() {
            return { let __recv = t.clone(); let __recv_ptr: *mut Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone();;
        }
    }
    return (*t.lock().unwrap().as_mut().unwrap()).underlying().clone();
}

/// If typ is a type parameter, underIs returns the result of typ.underIs(f).
/// Otherwise, underIs returns the result of f(under(typ)).
pub fn under_is(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) -> bool {
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let f_closure_clone = f.clone(); let mut ok_closure_clone = ok.clone(); typeset(typ.clone(), Arc::new(Mutex::new(Some(Box::new(move |_: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = f_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(u.clone()) }; *ok_closure_clone.lock().unwrap() = Some(new_val); };
        return { let __v = (*ok_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))));
    return { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// typeset is an iterator over the (type/underlying type) pairs of the
/// specific type terms of the type set implied by t.
/// If t is a type parameter, the implied type set is the type set of t's constraint.
/// In that case, if there are no specific terms, typeset calls yield with (nil, nil).
/// If t is not a type parameter, the implied type set consists of just t.
/// In any case, typeset is guaranteed to call yield at least once.
pub fn typeset(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) {
    {
        let (mut p, _) = ({
        let val = unalias(t.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<TypeParam>)), false)
        }
    });;
        if (*p.lock().unwrap()).is_some() {
            { let __recv = p.clone(); let __recv_ptr: *mut TypeParam = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut TypeParam }; let __result = unsafe { &mut *__recv_ptr }.typeset(r#yield.clone()); __result };;
            return;;
        }
    }
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t.clone(), under(t.clone()).clone()) };
}

/// If t is not a type parameter, coreType returns the underlying type.
/// If t is a type parameter, coreType returns the single underlying
/// type of all types in its type set if it exists, or nil otherwise. If the
/// type set contains only unrestricted and restricted channel types (with
/// identical element types), the single underlying type is the restricted
/// channel type if the restrictions are always the same, or nil otherwise.
pub fn core_type(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut su: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut su_closure_clone = su.clone(); typeset(t.clone(), Arc::new(Mutex::new(Some(Box::new(move |_: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        if (*u.lock().unwrap()).is_none() {
        return false;
    }
        if (*su_closure_clone.lock().unwrap()).is_some() {
        { let __iface_handle = r#match(su_closure_clone.clone(), u.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
        if (*u.lock().unwrap()).is_none() {
        *su_closure_clone.lock().unwrap() = None;
        return false;
    }
    }
        { let __iface_handle = u.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *su_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))));
        // su == nil || match(su, u) != nil
    return su.clone();
}

/// coreString is like coreType but also considers []byte
/// and strings as identical. In this case, if successful and we saw
/// a string, the result is of type (possibly untyped) string.
pub fn core_string(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // This explicit case is needed because otherwise the
        // result would be string if t is an untyped string.
    if !is_type_param(t.clone()) {
        return under(t.clone()).clone();
    }

        // untyped string remains untyped
    let mut su: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut hasString = Arc::new(Mutex::new(Some(false)));
    let mut hasString_closure_clone = hasString.clone(); let mut su_closure_clone = su.clone(); typeset(t.clone(), Arc::new(Mutex::new(Some(Box::new(move |_: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>| -> bool {
        if (*u.lock().unwrap()).is_none() {
        return false;
    }
        if is_string(u.clone()) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::slice::SlicePtr(new_slice(universeByte.clone()).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = true; *hasString_closure_clone.lock().unwrap() = Some(new_val); };
    }
        if (*su_closure_clone.lock().unwrap()).is_some() {
        { let __iface_handle = r#match(su_closure_clone.clone(), u.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
        if (*u.lock().unwrap()).is_none() {
        *su_closure_clone.lock().unwrap() = None;
        { let new_val = false; *hasString_closure_clone.lock().unwrap() = Some(new_val); };
        return false;
    }
    }
        { let __iface_handle = u.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *su_closure_clone.lock().unwrap() = (*__iface_guard).clone(); };
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>))));
        // su == nil || match(su, u) != nil
    if { let __v = (*hasString.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    }
    return su.clone();
}

/// If x and y are identical, match returns x.
/// If x and y are identical channels but for their direction
/// and one of them is unrestricted, match returns the channel
/// with the restricted direction.
/// In all other cases, match returns nil.
pub fn r#match(mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    let mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = x.clone();
    let mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = y.clone();
        // Common case: we don't have channels.
    if identical(x.clone(), y.clone()) {
        return x.clone();
    }

        // We may have channels that differ in direction only.
    {
        let (mut x, _) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Chan>)), false)
        }
    });;
        if (*x.lock().unwrap()).is_some() {
            {
        let (mut y, _) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Chan>)), false)
        }
    });;
        if (*y.lock().unwrap()).is_some() && identical((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone()) {
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32)))); __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(Box::new(crate::chan::ChanPtr(y.clone())) as Box<dyn Type + Send + Sync>)));
        } else if { let __tmp_x = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32)))); __tmp_x == __tmp_y } {
            return Arc::new(Mutex::new(Some(Box::new(crate::chan::ChanPtr(x.clone())) as Box<dyn Type + Send + Sync>)));
        };
        }
    };
        }
    }

        // We have channels that differ in direction only.
        // If there's an unrestricted channel, select the restricted one.
        // types are different
    return Arc::new(Mutex::new(None));
}