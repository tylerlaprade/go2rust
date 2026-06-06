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
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

impl crate::check::Checker {
    /// lookupError returns a case-specific error when a lookup of selector sel in the
    /// given type fails but an object with alternative spelling (case folding) is found.
    /// If structLit is set, the error message is specifically for struct literal fields.
    pub fn lookup_error(&self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, sel: Arc<Mutex<Option<String>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, structLit: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
                // Provide more detail if there is an unexported object, or one with different capitalization.
                // If selector and object are in the same package (==), export doesn't matter, otherwise (!=) it does.
                // Messages depend on whether it's a general lookup or a field lookup in a struct literal.
                //
                // case           sel     pkg   have   message (examples for general lookup)
                // ---------------------------------------------------------------------------------------------------------
                // ok             x.Foo   ==    Foo
                // misspelled     x.Foo   ==    FoO    type X has no field or method Foo, but does have field FoO
                // misspelled     x.Foo   ==    foo    type X has no field or method Foo, but does have field foo
                // misspelled     x.Foo   ==    foO    type X has no field or method Foo, but does have field foO
                //
                // misspelled     x.foo   ==    Foo    type X has no field or method foo, but does have field Foo
                // misspelled     x.foo   ==    FoO    type X has no field or method foo, but does have field FoO
                // ok             x.foo   ==    foo
                // misspelled     x.foo   ==    foO    type X has no field or method foo, but does have field foO
                //
                // ok             x.Foo   !=    Foo
                // misspelled     x.Foo   !=    FoO    type X has no field or method Foo, but does have field FoO
                // unexported     x.Foo   !=    foo    type X has no field or method Foo, but does have unexported field foo
                // missing        x.Foo   !=    foO    type X has no field or method Foo
                //
                // misspelled     x.foo   !=    Foo    type X has no field or method foo, but does have field Foo
                // missing        x.foo   !=    FoO    type X has no field or method foo
                // inaccessible   x.foo   !=    foo    cannot refer to unexported field foo
                // missing        x.foo   !=    foO    type X has no field or method foo
        const ok: i32 = 0;
const missing: i32 = 1;
const misspelled: i32 = 2;
const unexported: i32 = 3;
const inaccessible: i32 = 4;

                // no object found
                // found object with different spelling
                // found object with name differing only in first letter
                // found object with matching name but inaccessible from the current package
                // determine case
        let mut e = Arc::new(Mutex::new(Some(missing)));
        let mut alt: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __nil_result = (*obj.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *alt.lock().unwrap() = __moved_val; };
        if { let __left = (*obj.lock().unwrap().as_ref().unwrap()).pkg(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*alt.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*sel.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y }))));
        { let new_val = 2; *e.lock().unwrap() = Some(new_val); };
    } else if is_exported(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if is_exported(Arc::new(Mutex::new(Some({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = 2; *e.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*tail(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*tail(Arc::new(Mutex::new(Some({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        { let new_val = 3; *e.lock().unwrap() = Some(new_val); };
    }
    } else if is_exported(Arc::new(Mutex::new(Some({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if { let __tmp_x = (*tail(Arc::new(Mutex::new(Some({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*tail(Arc::new(Mutex::new(Some({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        { let new_val = 2; *e.lock().unwrap() = Some(new_val); };
    }
    } else if { let __tmp_x = (*sel.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*alt.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        { let new_val = 4; *e.lock().unwrap() = Some(new_val); };
    }
    }
                // otherwise there is no lookup error
        if { let __v = (*structLit.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let _switch_val = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            return self.sprintf(Arc::new(Mutex::new(Some("unknown field %s in struct literal of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (2) {
            return self.sprintf(Arc::new(Mutex::new(Some("unknown field %s in struct literal of type %s, but does have %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (3) {
            return self.sprintf(Arc::new(Mutex::new(Some("unknown field %s in struct literal of type %s, but does have unexported %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (4) {
            return self.sprintf(Arc::new(Mutex::new(Some("cannot refer to unexported field %s in struct literal of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        }
    }
    } else {
        let mut what = Arc::new(Mutex::new(Some("object".to_string())));
        {
    let _ts_subject = obj.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Object + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        { let new_val = "field".to_string(); *what.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        { let new_val = "method".to_string(); *what.lock().unwrap() = Some(new_val); };;
    }
    }
        { let _switch_val = { let __v = (*e.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (1) {
            return self.sprintf(Arc::new(Mutex::new(Some("type %s has no field or method %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (2) {
            return self.sprintf(Arc::new(Mutex::new(Some("type %s has no field or method %s, but does have %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (3) {
            return self.sprintf(Arc::new(Mutex::new(Some("type %s has no field or method %s, but does have unexported %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = sel.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if _switch_val == (4) {
            return self.sprintf(Arc::new(Mutex::new(Some("cannot refer to unexported %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = what.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        }
    }
    }
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

/// tail returns the string s without its first (UTF-8) character.
/// If len(s) == 0, the result is s.
pub fn tail(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    for (i, _) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() })));
    }
    }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}