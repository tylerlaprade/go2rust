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

use std::sync::{Arc, Mutex};

impl crate::interface::Interface {
    /// Methods returns a go1.23 iterator over all the methods of an
    /// interface, ordered by Id.
    ///
    /// Example: for m := range t.Methods() { ... }
    pub fn methods(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut t_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(t_closure_clone.num_methods()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t_closure_clone.method(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }

    /// ExplicitMethods returns a go1.23 iterator over the explicit methods of
    /// an interface, ordered by Id.
    ///
    /// Example: for m := range t.ExplicitMethods() { ... }
    pub fn explicit_methods(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut t_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(t_closure_clone.num_explicit_methods()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t_closure_clone.explicit_method(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }

    /// EmbeddedTypes returns a go1.23 iterator over the types embedded within an interface.
    ///
    /// Example: for e := range t.EmbeddedTypes() { ... }
    pub fn embedded_types(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut t_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(t_closure_clone.num_embeddeds()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t_closure_clone.embedded_type(Arc::new(Mutex::new(Some(i)))).clone()) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::named::Named {
    /// Methods returns a go1.23 iterator over the declared methods of a named type.
    ///
    /// Example: for m := range t.Methods() { ... }
    pub fn methods(&mut self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::object::Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut t_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(t_closure_clone.num_methods()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t_closure_clone.method(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Func>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::scope::Scope {
    /// Children returns a go1.23 iterator over the child scopes nested within scope s.
    ///
    /// Example: for child := range scope.Children() { ... }
    pub fn children(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::scope::Scope>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut s_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Scope>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(s_closure_clone.num_children()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Scope>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Scope>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(s_closure_clone.child(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Scope>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::r#struct::Struct {
    /// Fields returns a go1.23 iterator over the fields of a struct type.
    ///
    /// Example: for field := range s.Fields() { ... }
    pub fn fields(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::object::Var>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut s_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(s_closure_clone.num_fields()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(s_closure_clone.field(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::tuple::Tuple {
    /// Variables returns a go1.23 iterator over the variables of a tuple type.
    ///
    /// Example: for v := range tuple.Variables() { ... }
    pub fn variables(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::object::Var>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut t_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(t_closure_clone.len()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(t_closure_clone.at(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Var>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::methodset::MethodSet {
    /// Methods returns a go1.23 iterator over the methods of a method set.
    ///
    /// Example: for method := range s.Methods() { ... }
    pub fn methods(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::selection::Selection>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut s_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Selection>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(s_closure_clone.len()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Selection>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Selection>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(s_closure_clone.at(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Selection>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::union::Union {
    /// Terms returns a go1.23 iterator over the terms of a union.
    ///
    /// Example: for term := range union.Terms() { ... }
    pub fn terms(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::union::Term>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut u_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Term>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(u_closure_clone.len()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Term>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Term>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(u_closure_clone.term(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Term>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::typelists::TypeParamList {
    /// TypeParams returns a go1.23 iterator over a list of type parameters.
    ///
    /// Example: for tparam := range l.TypeParams() { ... }
    pub fn type_params(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<crate::typeparam::TypeParam>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut l_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<TypeParam>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(l_closure_clone.len()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<TypeParam>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<TypeParam>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(l_closure_clone.at(Arc::new(Mutex::new(Some(i))))) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<TypeParam>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}

impl crate::typelists::TypeList {
    /// Types returns a go1.23 iterator over the elements of a list of types.
    ///
    /// Example: for t := range l.Types() { ... }
    pub fn types(&self) -> Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
        let mut l_closure_clone = (*self).clone(); return Arc::new(Mutex::new(Some(Box::new(move |r#yield: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>| {
        for i in 0..(l_closure_clone.len()) {
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> = { let mut __f_guard = r#yield.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(l_closure_clone.at(Arc::new(Mutex::new(Some(i)))).clone()) } {
        break
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
    }
}