use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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


impl crate::scope::Scope {
    /// LookupParent follows the parent chain of scopes starting with s until
    /// it finds a scope where Lookup(name) returns a non-nil object, and then
    /// returns that scope and object. If a valid position pos is provided,
    /// only objects that were declared at or before pos are considered.
    /// If no such scope and object exists, the result is (nil, nil).
    /// The results are guaranteed to be valid only if the type-checked
    /// AST has complete position information.
    ///
    /// Note that obj.Parent() may be different from the returned scope if the
    /// object was inserted into the scope and already had a parent at that
    /// time (see Insert). This can only happen for dot-imported objects
    /// whose parent is the scope of the package that exported them.
    pub fn lookup_parent(&mut self, name: Arc<Mutex<Option<String>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>) -> (Arc<Mutex<Option<crate::scope::Scope>>>, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        let mut __self = Arc::new(Mutex::new(Some(self.clone())));
        while { let __self_guard = __self.lock().unwrap(); __self_guard.is_some() } {
        {
        let mut obj = (*__self.lock().unwrap().as_ref().unwrap()).lookup(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if (*obj.lock().unwrap()).is_some() && (!go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) || { let __tmp_x = cmp_pos((*obj.lock().unwrap().as_ref().unwrap()).scope_pos(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x <= __tmp_y }) {
            return (__self.clone(), obj.clone());;
        }
    }
        { let new_val = (*__self.lock().unwrap().as_ref().unwrap()).parent.clone(); __self = new_val; };
    }
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
    }

    /// Pos and End describe the scope's source code extent [pos, end).
    /// The results are guaranteed to be valid only if the type-checked
    /// AST has complete position information. The extent is undefined
    /// for Universe and package scopes.
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.pos.clone();
    }

    pub fn end(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.end.clone();
    }

    /// Contains reports whether pos is within the scope's extent.
    /// The result is guaranteed to be valid only if the type-checked
    /// AST has complete position information.
    pub fn contains(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) -> bool {
        return { let __tmp_x = cmp_pos(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x <= __tmp_y } && { let __tmp_x = cmp_pos(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.end.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))); let __tmp_y = 0; __tmp_x < __tmp_y };
    }

    /// Innermost returns the innermost (child) scope containing
    /// pos. If pos is not within any scope, the result is nil.
    /// The result is also nil for the Universe scope.
    /// The result is guaranteed to be valid only if the type-checked
    /// AST has complete position information.
    pub fn innermost(&self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) -> Arc<Mutex<Option<crate::scope::Scope>>> {
                // Package scopes do not have extents since they may be
                // discontiguous, so iterate over the package's files.
        if { let __left = self.parent.clone(); let __right = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let __range_holder = self.children.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        {
        let mut inner = { let __recv = s.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.innermost(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if (*inner.lock().unwrap()).is_some() {
            return inner.clone();;
        }
    }
    } }
    }
        if self.contains(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let __range_holder = self.children.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        if { let __recv = s.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.contains(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result } {
        return { let __recv = s.clone(); let __recv_ptr: *const crate::scope::Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::scope::Scope }; let __result = unsafe { &*__recv_ptr }.innermost(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
    }
    } }
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        return Arc::new(Mutex::new(None));
    }
}