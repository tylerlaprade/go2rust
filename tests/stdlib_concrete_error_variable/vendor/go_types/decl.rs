use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoMutex, GoOnce, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

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
use crate::validtype::*;
use crate::version::*;

use internal_types_errors::*;

use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub trait decl: std::fmt::Display + Any {
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool;
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>;
}

impl Clone for Box<dyn decl + Send + Sync> {
    fn clone(&self) -> Self {
        decl::__go_clone_box_decl(self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct importDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::ImportSpec>>>,
}

impl importDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { spec: self.spec.clone() }
    }
}

impl std::fmt::Display for importDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for importDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct constDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>,
    pub iota: Arc<Mutex<Option<i32>>>,
    pub typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>,
    pub init: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>,
    pub inherited: Arc<Mutex<Option<bool>>>,
}

impl constDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { spec: self.spec.clone(), iota: { let __guard = self.iota.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone(), init: self.init.clone(), inherited: { let __guard = self.inherited.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for constDecl {
    fn default() -> Self {
        Self { spec: Arc::new(Mutex::new(None)), iota: Arc::new(Mutex::new(Some(0))), typ: Arc::new(Mutex::new(None)), init: Arc::new(Mutex::new(None)), inherited: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for constDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.iota.lock().unwrap().as_ref().unwrap()), (*self.typ.lock().unwrap().as_ref().unwrap()), format_slice_wrapped_stringer(&self.init), (*self.inherited.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for constDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct varDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>,
}

impl varDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { spec: self.spec.clone() }
    }
}

impl std::fmt::Display for varDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for varDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct typeDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::TypeSpec>>>,
}

impl typeDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { spec: self.spec.clone() }
    }
}

impl std::fmt::Display for typeDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for typeDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct funcDecl {
    pub decl: Arc<Mutex<Option<go_ast::r#mod::FuncDecl>>>,
}

impl funcDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { decl: self.decl.clone() }
    }
}

impl std::fmt::Display for funcDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.decl.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for funcDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    pub fn declare(&self, scope: Arc<Mutex<Option<Scope>>>, id: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
                // spec: "The blank identifier, represented by the underscore
                // character _, may be used in a declaration like any other
                // identifier but the declaration does not introduce a new
                // binding."
        if { let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        {
        let mut alt = { let __recv = scope.clone(); let __recv_ptr: *mut crate::scope::Scope = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::scope::Scope }; let __result = unsafe { &mut *__recv_ptr }.insert(obj.clone()); __result };;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DECL as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s redeclared in this block".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(alt.clone()); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
            return;;
        }
    }
        (*obj.lock().unwrap().as_mut().unwrap()).set_scope_pos(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        if { let __nil_result = (*id.lock().unwrap()).is_some(); __nil_result } {
        self.record_def(id.clone(), obj.clone());
    }
    }

    /// objDecl type-checks the declaration of obj in its respective (file) environment.
    /// For the meaning of def, see Checker.definedType, in typexpr.go.
    pub fn obj_decl(&mut self, mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, def: Arc<Mutex<Option<TypeName>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(obj.lock().unwrap().as_ref().map(|__v| Object::__go_clone_box_object(__v.as_ref()))));
        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) && { let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap()).is_none(); __nil_result } {
        if { let __tmp_x = (*self.indent.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        println!();
    }
                // empty line between top-level objects for readability
        { let __method_arg0 = (*obj.lock().unwrap().as_ref().unwrap()).pos(); let __method_arg1 = Arc::new(Mutex::new(Some("-- checking %s (%s, objPath = %s)".to_string()))); self.trace(__method_arg0, __method_arg1, Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).color(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = path_string({ let __field = self.obj_path.clone(); __field }); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))) };
        { let __target = self.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let mut check_defer_captured = self.clone(); let obj_defer_captured = obj.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = check_defer_captured.indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        check_defer_captured.trace((*obj_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> %s (%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = obj_defer_captured.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*obj_defer_captured.lock().unwrap().as_ref().unwrap()).color(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
                        // empty line between top-level objects for readability
                        // Checking the declaration of obj means inferring its type
                        // (and possibly its value, for constants).
                        // An object's type (and thus the object) may be in one of
                        // three states which are expressed by colors:
                        //
                        // - an object whose type is not yet known is painted white (initial color)
                        // - an object whose type is in the process of being inferred is painted grey
                        // - an object whose type is fully inferred is painted black
                        //
                        // During type inference, an object's color changes from white to grey
                        // to black (pre-declared objects are painted black from the start).
                        // A black object (i.e., its type) can only depend on (refer to) other black
                        // ones. White and grey objects may depend on white and black objects.
                        // A dependency on a grey object indicates a cycle which may or may not be
                        // valid.
                        //
                        // When objects turn grey, they are pushed on the object path (a stack);
                        // they are popped again when they turn black. Thus, if a grey object (a
                        // cycle) is encountered, it is on the object path, and all the objects
                        // it depends on are the remaining objects on that path. Color encoding
                        // is such that the color value of a grey object indicates the index of
                        // that object in the object path.
                        // During type-checking, white objects may be assigned a type without
                        // traversing through objDecl; e.g., when initializing constants and
                        // variables. Update the colors of those objects here (rather than
                        // everywhere where we set the type) to satisfy the color invariants.
            if { let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).color().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::object::color(Arc::new(Mutex::new(Some(WHITE as u32)))); __tmp_x == __tmp_y } && { let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap()).is_some(); __nil_result } {
        (*obj.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32))))))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            {
        let _switch_val = { let __v = (*obj.lock().unwrap().as_ref().unwrap()).color(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == crate::object::color(Arc::new(Mutex::new(Some(WHITE as u32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            assert(Arc::new(Mutex::new(Some({ let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap()).is_none(); __nil_result }))));
                        // All color values other than white and black are considered grey.
                        // Because black and white are < grey, all values >= grey are grey.
                        // Use those values to encode the object's index into the object path.
            (*obj.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some({ let __tmp_x = crate::object::color(Arc::new(Mutex::new(Some(GREY as u32)))); let __tmp_y = crate::object::color(Arc::new(Mutex::new(Some(self.push(obj.clone()) as u32)))); __tmp_x + __tmp_y }))));
            let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __recv = check_defer_captured.pop(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
        }
        if !_matched && (_switch_val == crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            assert(Arc::new(Mutex::new(Some({ let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap()).is_some(); __nil_result }))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && (_switch_val == crate::object::color(Arc::new(Mutex::new(Some(GREY as u32))))) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // We have a (possibly invalid) cycle.
                        // In the existing code, this is marked by a non-nil type
                        // for the object except for constants and variables whose
                        // type may be non-nil (known), or nil if it depends on the
                        // not-yet known initialization value.
                        // In the former case, set the type to Typ[Invalid] because
                        // we have an initialization cycle. The cycle error will be
                        // reported later, when determining initialization order.
                        // TODO(gri) Report cycle here and simplify initialization
                        // order code.
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).unwrap().0.clone();
        if !self.valid_cycle(Arc::new(Mutex::new(Some(Box::new(crate::object::ConstPtr(obj.clone())) as Box<dyn Object + Send + Sync>)))) || { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).unwrap().0.clone();
        if !self.valid_cycle(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>)))) || { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        if !self.valid_cycle(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>)))) {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
        if !self.valid_cycle(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj.clone())) as Box<dyn Object + Send + Sync>)))) {
    };
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
                        // break cycle
                        // (without this, calling underlying()
                        // below may lead to an endless loop
                        // if we have a cycle for a defined
                        // (*Named) type)
                        // Don't set obj.typ to Typ[Invalid] here
                        // because plenty of code type-asserts that
                        // functions have a *Signature type. Grey
                        // functions have their type set to an empty
                        // signature which makes it impossible to
                        // initialize a variable with the function.
            assert(Arc::new(Mutex::new(Some({ let __nil_result = (*(*obj.lock().unwrap().as_ref().unwrap()).r#type().lock().unwrap()).is_some(); __nil_result }))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
        }
    }
                        // All color values other than white and black are considered grey.
                        // Because black and white are < grey, all values >= grey are grey.
                        // Use those values to encode the object's index into the object path.
                        // Color values other than white or black are considered grey.
                        // We have a (possibly invalid) cycle.
                        // In the existing code, this is marked by a non-nil type
                        // for the object except for constants and variables whose
                        // type may be non-nil (known), or nil if it depends on the
                        // not-yet known initialization value.
                        // In the former case, set the type to Typ[Invalid] because
                        // we have an initialization cycle. The cycle error will be
                        // reported later, when determining initialization order.
                        // TODO(gri) Report cycle here and simplify initialization
                        // order code.
                        // break cycle
                        // (without this, calling underlying()
                        // below may lead to an endless loop
                        // if we have a cycle for a defined
                        // (*Named) type)
                        // Don't set obj.typ to Typ[Invalid] here
                        // because plenty of code type-asserts that
                        // functions have a *Signature type. Grey
                        // functions have their type set to an empty
                        // signature which makes it impossible to
                        // initialize a variable with the function.
            let mut d = { let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(obj.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
            if { let __nil_result = (*d.lock().unwrap()).is_none(); __nil_result } {
        self.dump(Arc::new(Mutex::new(Some("%v: %s should have been declared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }
                        // save/restore current environment and set up object environment
            let mut check_defer_captured = self.clone(); let __defer_arg_0 = Arc::new(Mutex::new(Some({ let __selector_holder = check_defer_captured.environment.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __defer_stack.push(Box::new(move || {
        (move |env: Arc<Mutex<Option<environment>>>| {
        { let new_val = env.lock().unwrap().as_ref().unwrap().clone(); *check_defer_captured.environment.lock().unwrap() = Some(new_val); };;
        })(__defer_arg_0);
    }));
            { let new_val = environment { scope: { let __field = (*d.lock().unwrap().as_ref().unwrap()).file.clone(); __field }, version: Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() }; *self.environment.lock().unwrap() = Some(new_val); };
                        // Const and var declarations must not have initialization
                        // cycles. We track them by remembering the current declaration
                        // in check.decl. Initialization expressions depending on other
                        // consts, vars, or functions, add dependencies to the current
                        // check.decl.
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).unwrap().0.clone();
        { let new_val = d.clone(); (*self.environment.lock().unwrap().as_mut().unwrap()).decl = new_val; };;
        self.const_decl(obj.clone(), { let __field = (*d.lock().unwrap().as_ref().unwrap()).vtyp.clone(); __field }, { let __field = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).inherited.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).unwrap().0.clone();
        { let new_val = d.clone(); (*self.environment.lock().unwrap().as_mut().unwrap()).decl = new_val; };;
        self.var_decl(obj.clone(), { let __field = (*d.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }, { let __field = (*d.lock().unwrap().as_ref().unwrap()).vtyp.clone(); __field }, { let __field = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        self.type_decl(obj.clone(), { let __field = (*d.lock().unwrap().as_ref().unwrap()).tdecl.clone(); __field }, def.clone());;
        self.collect_methods(obj.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
        self.func_decl(obj.clone(), d.clone());;
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
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
                ()
            }
        }
    }

    /// validCycle checks if the cycle starting with obj is valid and
    /// reports an error if it is not.
    pub fn valid_cycle(&self, mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut valid: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(obj.lock().unwrap().as_ref().map(|__v| Object::__go_clone_box_object(__v.as_ref()))));
        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // The object map contains the package scope objects and the non-interface methods.
            if DEBUG {
        let mut info = { let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(obj.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        let mut inObjMap = Arc::new(Mutex::new(Some({ let __nil_result = (*info.lock().unwrap()).is_some(); __nil_result } && ({ let __nil_target = (*info.lock().unwrap().as_ref().unwrap()).fdecl.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || { let __nil_target = (*(*info.lock().unwrap().as_ref().unwrap()).fdecl.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        let mut isPkgObj = Arc::new(Mutex::new(Some({ let __left = (*obj.lock().unwrap().as_ref().unwrap()).parent(); let __right = (*self.pkg.lock().unwrap().as_ref().unwrap()).scope.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq })));
        if { let __tmp_x = { let __v = (*isPkgObj.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*inObjMap.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        self.dump(Arc::new(Mutex::new(Some("%v: inconsistent object map for %s (isPkgObj = %v, inObjMap = %v)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = isPkgObj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = inObjMap.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
                        // exclude methods
                        // Count cycle objects.
            assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).color().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::object::color(Arc::new(Mutex::new(Some(GREY as u32)))); __tmp_x >= __tmp_y }))));
            let mut start = Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(((*(*(*obj.lock().unwrap().as_ref().unwrap()).color().lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) - GREY as u32))))))));
            let mut cycle = Arc::new(Mutex::new(Some({ let __seq_holder = self.obj_path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*{ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
            let mut tparCycle = Arc::new(Mutex::new(Some(false)));
            let mut nval = Arc::new(Mutex::new(Some(0)));
            let mut ndef = Arc::new(Mutex::new(Some(0)));
            { let __range_holder = cycle.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); 'r#loop: for mut obj in __range_values.iter().cloned() {
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::ConstPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::VarPtr>()).is_some() {
        let obj = _ts_subject.clone();
        { let mut guard = nval.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        if (*(*self.environment.lock().unwrap().as_ref().unwrap()).in_t_param_list.clone().lock().unwrap().as_ref().unwrap()) && is_generic({ let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }) {
        { let new_val = true; *tparCycle.lock().unwrap() = Some(new_val); };
        break 'r#loop
    };
        let mut alias: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));;
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) {
        { let new_val = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.is_alias(); __result }; *alias.lock().unwrap() = Some(new_val); };
    } else {
        {
        let mut d = { let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if { let __nil_result = (*d.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = go_token::position::Pos::is_valid(&(*(*(*d.lock().unwrap().as_ref().unwrap()).tdecl.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap())); *alias.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.is_alias(); __result }; *alias.lock().unwrap() = Some(new_val); };;
        }
    }
    };
        if !{ let __v = (*alias.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let mut guard = ndef.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::FuncPtr>()).unwrap().0.clone();
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    } }
                        // If we reach a generic type that is part of a cycle
                        // and we are in a type parameter list, we have a cycle
                        // through a type parameter list, which is invalid.
                        // Determine if the type name is an alias or not. For
                        // package-level objects, use the object map which
                        // provides syntactic information (which doesn't rely
                        // on the order in which the objects are set up). For
                        // local objects, we can rely on the order, so use
                        // the object's predicate.
                        // TODO(gri) It would be less fragile to always access
                        // the syntactic information. We should consider storing
                        // this information explicitly in the object.
                        // package-level object
                        // function local object
                        // ignored for now
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace((*obj.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("## cycle detected: objPath = %s->%s (len = %d)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = path_string(cycle.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as Box<dyn Any + Send + Sync>]))));
        if { let __v = (*tparCycle.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.trace((*obj.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("## cycle contains: generic type in a type parameter list".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    } else {
        self.trace((*obj.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("## cycle contains: %d values, %d type definitions".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = nval.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = ndef.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        let mut check_defer_captured = self.clone(); let obj_defer_captured = obj.clone(); let valid_defer_captured = valid.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        if { let __v = (*valid_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        check_defer_captured.trace((*obj_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> cycle is valid".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    } else {
        check_defer_captured.trace((*obj_defer_captured.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some("=> error: cycle is invalid".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
            if !{ let __v = (*tparCycle.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // A cycle involving only constants and variables is invalid but we
                // ignore them here because they are reported via the initialization
                // cycle check.
        if { let __tmp_x = ({ let __v = (*nval.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        {
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*valid.lock().unwrap().as_ref().unwrap());
    }
    }
                // A cycle involving only types (and possibly functions) must have at least
                // one type definition to be permitted: If there is no type definition, we
                // have a sequence of alias type names which will expand ad infinitum.
        if { let __tmp_x = { let __v = (*nval.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*ndef.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        {
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*valid.lock().unwrap().as_ref().unwrap());
    }
    }
    }
                        // A cycle involving only constants and variables is invalid but we
                        // ignore them here because they are reported via the initialization
                        // cycle check.
                        // A cycle involving only types (and possibly functions) must have at least
                        // one type definition to be permitted: If there is no type definition, we
                        // have a sequence of alias type names which will expand ad infinitum.
            self.cycle_error(cycle.clone(), Arc::new(Mutex::new(Some(first_in_src(cycle.clone())))));
            {
        { let new_val = false; *valid.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*valid.lock().unwrap().as_ref().unwrap());
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
                (*valid.lock().unwrap().as_ref().unwrap())
            }
        }
    }

    /// cycleError reports a declaration cycle starting with the object at cycle[start].
    pub fn cycle_error(&self, cycle: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>, start: Arc<Mutex<Option<i32>>>) {
                // name returns the (possibly qualified) object name.
                // This is needed because with generic types, cycles
                // may refer to imported types. See go.dev/issue/50788.
                // TODO(gri) This functionality is used elsewhere. Factor it out.
        let mut check_closure_clone = (*self).clone(); let mut name = Arc::new(Mutex::new(Some(Box::new(move |obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*package_prefix((*obj.lock().unwrap().as_ref().unwrap()).pkg(), Arc::new(Mutex::new(Some({ let mut __recv = check_closure_clone.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> })))).lock().unwrap().as_ref().unwrap()), (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap())))));
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
                // If obj is a type alias, mark it as valid (not broken) in order to avoid follow-on errors.
        let mut obj = { let __seq = { let __seq_holder = cycle.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
        let (mut tname, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::TypeName>)), false)
        }
    });
        if { let __nil_result = (*tname.lock().unwrap()).is_some(); __nil_result } && { let __recv = tname.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.is_alias(); __result } {
                // If we use Alias nodes, it is initialized with Typ[Invalid].
                // TODO(gri) Adjust this code if we initialize with nil.
        if !(*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) {
        self.valid_alias(tname.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
    }
    }
                // If we use Alias nodes, it is initialized with Typ[Invalid].
                // TODO(gri) Adjust this code if we initialize with nil.
                // report a more concise error for self references
        if { let __tmp_x = ((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        if { let __nil_result = (*tname.lock().unwrap()).is_some(); __nil_result } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DECL_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid recursive type: %s refers to itself".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(obj.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DECL_CYCLE as i32))))))), Arc::new(Mutex::new(Some("invalid cycle in declaration: %s refers to itself".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(obj.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        return;
    }
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DECL_CYCLE as i32))))))));
        if { let __nil_result = (*tname.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("invalid recursive type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(obj.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
    } else {
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("invalid cycle in declaration of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(obj.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
                // "cycle[i] refers to cycle[j]" for (i,j) = (s,s+1), (s+1,s+2), ..., (n-1,0), (0,1), ..., (s-1,s) for len(cycle) = n, s = start.
        for i in 0..(({ let __range_holder = cycle.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut next = { let __seq = { let __seq_holder = cycle.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = (({ let __tmp_x = { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i as i32; __tmp_x + __tmp_y }; let __tmp_y = 1; __tmp_x + __tmp_y }) as i32); let __tmp_y = ((*cycle.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x % __tmp_y }) as usize].clone() };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("%s refers to %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(obj.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = name.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(next.clone()) }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __iface_handle = next.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *obj.lock().unwrap() = __iface_value; };
    }
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }

    pub fn walk_decls(&self, decls: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync>>>>) {
        { let __range_holder = decls.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d in __range_values.iter() {
        self.walk_decl(d.clone(), f.clone());
    } }
    }

    pub fn walk_decl(&self, mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync>>>>) {
        let mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>> = Arc::new(Mutex::new(d.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Decl::__go_clone_box_decl(__v.as_ref()))));
        {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadDeclPtr>()).unwrap().0.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).unwrap().0.clone();
        let mut last: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>> = Arc::new(Mutex::new(None));;
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (iota, mut s) in __range_values.iter().cloned().enumerate() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Spec + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ImportSpecPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ImportSpecPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(importDecl { spec: s.clone(), ..Default::default() }) as Box<dyn decl + Send + Sync>)))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).unwrap().0.clone();
        { let _switch_val = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_S_T as i32))))) {
            let mut inherited = Arc::new(Mutex::new(Some(true)));
            if { let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } || { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
            { let new_val = s.clone(); last = new_val; };
            { let new_val = false; *inherited.lock().unwrap() = Some(new_val); };
        } else if { let __nil_result = (*last.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = Arc::new(Mutex::new(Some(go_ast::r#mod::ValueSpec::default()))).clone(); last = new_val; };
            { let new_val = false; *inherited.lock().unwrap() = Some(new_val); };
        }
            self.arity_match(s.clone(), last.clone());
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(constDecl { spec: s.clone(), iota: Arc::new(Mutex::new(Some(iota as i32))), typ: { let __field = (*last.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, init: { let __field = (*last.lock().unwrap().as_ref().unwrap()).values.clone(); __field }, inherited: Arc::new(Mutex::new(Some({ let __arg_holder = inherited.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Box<dyn decl + Send + Sync>)))) };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))) {
            self.arity_match(s.clone(), Arc::new(Mutex::new(None)));
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(varDecl { spec: s.clone(), ..Default::default() }) as Box<dyn decl + Send + Sync>)))) };
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("invalid token %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSpecPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSpecPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(typeDecl { spec: s.clone(), ..Default::default() }) as Box<dyn decl + Send + Sync>)))) };;
    } else {
        let s = _ts_subject.clone();
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown ast.Spec node %T".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
    }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).unwrap().0.clone();
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(funcDecl { decl: d.clone(), ..Default::default() }) as Box<dyn decl + Send + Sync>)))) };;
    } else {
        let d = _ts_subject.clone();
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown ast.Decl node %T".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
    }
    }
    }

    pub fn const_decl(&mut self, obj: Arc<Mutex<Option<Const>>>, typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, inherited: Arc<Mutex<Option<bool>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
                        // use the correct value of iota
            let mut check_defer_captured = self.clone(); let __defer_arg_0 = (*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).iota.clone(); let __defer_arg_1 = (*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).errpos.clone(); __defer_stack.push(Box::new(move || {
        (move |iota: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>, errpos: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>| {
        { let __iface_handle = iota.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*check_defer_captured.environment.lock().unwrap().as_mut().unwrap()).iota.lock().unwrap() = __iface_value; };;
        { let __iface_handle = errpos.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*check_defer_captured.environment.lock().unwrap().as_mut().unwrap()).errpos.lock().unwrap() = __iface_value; };;
        })(__defer_arg_0, __defer_arg_1);
    }));
            { let __iface_handle = { let __field = (*obj.lock().unwrap().as_ref().unwrap()).val.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*self.environment.lock().unwrap().as_mut().unwrap()).iota.lock().unwrap() = __iface_value; };
            *(*self.environment.lock().unwrap().as_ref().unwrap()).errpos.lock().unwrap() = None;
                        // provide valid constant value under all circumstances
            { let __iface_handle = go_constant::make_unknown().clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*obj.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = __iface_value; };
                        // determine type, if any
            if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
        let mut t = self.typ(typ.clone());
        if !is_const_type(t.clone()) {
                // don't report an error if the type is an invalid C (defined) type
                // (go.dev/issue/22090)
        if is_valid(under(t.clone()).clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_CONST_TYPE as i32))))))), Arc::new(Mutex::new(Some("invalid constant type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                // don't report an error if the type is an invalid C (defined) type
                // (go.dev/issue/22090)
        { let __iface_handle = t.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
                        // don't report an error if the type is an invalid C (defined) type
                        // (go.dev/issue/22090)
                        // check initialization
            let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
            if { let __nil_result = (*init.lock().unwrap()).is_some(); __nil_result } {
        if { let __v = (*inherited.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The initialization expression is inherited from a previous
                // constant declaration, and (error) positions refer to that
                // expression and not the current constant declaration. Use
                // the constant identifier position for any errors during
                // init expression evaluation since that is all we have
                // (see issues go.dev/issue/42991, go.dev/issue/42992).
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*self.environment.lock().unwrap().as_mut().unwrap()).errpos.lock().unwrap() = __iface_value; };
    }
                // The initialization expression is inherited from a previous
                // constant declaration, and (error) positions refer to that
                // expression and not the current constant declaration. Use
                // the constant identifier position for any errors during
                // init expression evaluation since that is all we have
                // (see issues go.dev/issue/42991, go.dev/issue/42992).
        self.expr(Arc::new(Mutex::new(None)), x.clone(), init.clone());
    }
                        // The initialization expression is inherited from a previous
                        // constant declaration, and (error) positions refer to that
                        // expression and not the current constant declaration. Use
                        // the constant identifier position for any errors during
                        // init expression evaluation since that is all we have
                        // (see issues go.dev/issue/42991, go.dev/issue/42992).
            self.init_const(obj.clone(), x.clone());

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
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
                ()
            }
        }
    }

    pub fn var_decl(&mut self, obj: Arc<Mutex<Option<Var>>>, lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>>, typ: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>, init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
                // determine type, if any
        if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
        { let __iface_handle = self.var_type(typ.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
                // We cannot spread the type to all lhs variables if there
                // are more than one since that would mark them as checked
                // (see Checker.objDecl) and the assignment of init exprs,
                // if any, would not be checked.
                //
                // TODO(gri) If we have no init expr, we should distribute
                // a given type otherwise we need to re-evaluate the type
                // expr for each lhs variable, leading to duplicate work.
                // check initialization
        if { let __nil_result = (*init.lock().unwrap()).is_none(); __nil_result } {
        if { let __nil_result = (*typ.lock().unwrap()).is_none(); __nil_result } {
                // error reported before by arityMatch
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    }
                // error reported before by arityMatch
        return;
    }
                // error reported before by arityMatch
        if { let __nil_result = (*lhs.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = ((*lhs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*lhs.lock().unwrap()).is_none(); __nil_result } || { let __left = { let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __right = obj.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(new_target({ let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))), x.clone(), init.clone());
        self.init_var(obj.clone(), x.clone(), Arc::new(Mutex::new(Some("variable declaration".to_string()))));
        return;
    }
        if DEBUG {
                // obj must be one of lhs
        if !slices::contains::<Vec<Arc<Mutex<Option<crate::object::Var>>>>, crate::object::Var>(lhs.clone(), obj.clone()) {
        std::panic::panic_any(Box::new("inconsistent lhs".to_string()) as Box<dyn Any + Send + Sync>);
    }
    }
                // obj must be one of lhs
                // We have multiple variables on the lhs and one init expr.
                // Make sure all variables have been given the same type if
                // one was specified, otherwise they assume the type of the
                // init expression values (was go.dev/issue/15755).
        if { let __nil_result = (*typ.lock().unwrap()).is_some(); __nil_result } {
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for lhs in __range_values.iter() {
        { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*lhs.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
    } }
    }
        self.init_vars(lhs.clone(), Arc::new(Mutex::new(Some(vec![init.clone()]))), Arc::new(Mutex::new(None)));
    }

    /// isImportedConstraint reports whether typ is an imported type constraint.
    pub fn is_imported_constraint(&self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        let mut named = as_named(typ.clone());
        if { let __nil_result = (*named.lock().unwrap()).is_none(); __nil_result } || { let __left = (*(*named.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = self.pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } || { let __nil_target = (*(*named.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
        let (mut u, _) = ({
        let val = { let __recv = named.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone();
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
    });
        return { let __nil_result = (*u.lock().unwrap()).is_some(); __nil_result } && !{ let __recv = u.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.is_method_set(); __result };
    }

    pub fn type_decl(&mut self, obj: Arc<Mutex<Option<TypeName>>>, tdecl: Arc<Mutex<Option<go_ast::r#mod::TypeSpec>>>, def: Arc<Mutex<Option<TypeName>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
                        // Only report a version error if we have not reported one already.
            let mut versionErr = Arc::new(Mutex::new(Some(false)));
            let mut rhs: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
            let mut check_closure_clone = (*self).clone(); let obj_closure_clone = obj.clone(); let rhs_closure_clone = rhs.clone(); let tdecl_closure_clone = tdecl.clone(); let versionErr_closure_clone = versionErr.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let obj_closure_clone_closure_clone = obj_closure_clone.clone(); Box::new(move || {
        {
        let mut t = as_named({ let __field = (*(*obj_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });;
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            check_closure_clone_closure_clone.valid_type(t.clone());;
        }
    }
        let _ = !{ let __v = (*versionErr_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } && check_closure_clone_closure_clone.is_imported_constraint(rhs_closure_clone.clone()) && check_closure_clone_closure_clone.verify_versionf(Arc::new(Mutex::new(Some(Box::new((*(*tdecl_closure_clone.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("using type constraint %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = rhs_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("validType(%s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = obj_closure_clone.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))); __result };
                        // type may be invalid
                        // If typ is local, an error was already reported where typ is specified/defined.
                        // First type parameter, or nil.
            let mut tparam0: Arc<Mutex<Option<go_ast::r#mod::Field>>> = Arc::new(Mutex::new(None));
            if { let __tmp_x = (*(*tdecl.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = (*(*tdecl.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); tparam0 = new_val; };
    }
                        // alias declaration
            if go_token::position::Pos::is_valid(&(*(*tdecl.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap())) {
                // Report highest version requirement first so that fixing a version issue
                // avoids possibly two -lang changes (first to Go 1.9 and then to Go 1.23).
        if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*tparam0.lock().unwrap()).is_some(); __nil_result } && !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr(tparam0.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_23.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("generic type alias".to_string()))), Arc::new(Mutex::new(Some(vec![])))) {
        { let new_val = true; *versionErr.lock().unwrap() = Some(new_val); };
    }
        if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*tdecl.lock().unwrap().as_ref().unwrap()).assign.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_9.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type alias".to_string()))), Arc::new(Mutex::new(Some(vec![])))) {
        { let new_val = true; *versionErr.lock().unwrap() = Some(new_val); };
    }
        if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__enable_alias.lock().unwrap().as_ref().unwrap()) {
                // TODO(gri) Should be able to use nil instead of Typ[Invalid] to mark
                //           the alias as incomplete. Currently this causes problems
                //           with certain cycles. Investigate.
                //
                // NOTE(adonovan): to avoid the Invalid being prematurely observed
                // by (e.g.) a var whose type is an unfinished cycle,
                // Unalias does not memoize if Invalid. Perhaps we should use a
                // special sentinel distinct from Invalid.
        let mut alias = self.new_alias(obj.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
        set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(alias.clone())) as Box<dyn Type + Send + Sync>))));
                // handle type parameters even if not allowed (Alias type is supported)
        if { let __nil_result = (*tparam0.lock().unwrap()).is_some(); __nil_result } {
        if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && !(*(*(*internal_buildcfg::Experiment.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).alias_type_params.lock().unwrap().as_ref().unwrap()) {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(tdecl.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNSUPPORTED_FEATURE as i32))))))), Arc::new(Mutex::new(Some("generic type alias requires GOEXPERIMENT=aliastypeparams".to_string()))));
        { let new_val = true; *versionErr.lock().unwrap() = Some(new_val); };
    }
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(tdecl.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("type parameters".to_string()))));
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));
        self.collect_type_params(Arc::new(Mutex::new(Some((*alias.lock().unwrap().as_ref().unwrap()).tparams.clone()))), { let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).type_params.clone(); __field });
    }
        { let __iface_handle = self.defined_type({ let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, obj.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rhs.lock().unwrap() = __iface_value; };
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*rhs.lock().unwrap()).is_some(); __nil_result }))));
        { let __iface_handle = rhs.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*alias.lock().unwrap().as_mut().unwrap()).from_r_h_s.lock().unwrap() = __iface_value; };
        unalias(Arc::new(Mutex::new(Some(Box::new(crate::alias::AliasPtr(alias.clone())) as Box<dyn Type + Send + Sync>))));
    } else {
                // With Go1.23, the default behavior is to use Alias nodes,
                // reflected by check.enableAlias. Signal non-default behavior.
                //
                // TODO(gri) Testing runs tests in both modes. Do we need to exclude
                //           tracking of non-default behavior for tests?
        { let __recv_holder = (*gotypesalias.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.inc_non_default(); __result };
        if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*tparam0.lock().unwrap()).is_some(); __nil_result } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(tdecl.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNSUPPORTED_FEATURE as i32))))))), Arc::new(Mutex::new(Some("generic type alias requires GODEBUG=gotypesalias=1 or unset".to_string()))));
        { let new_val = true; *versionErr.lock().unwrap() = Some(new_val); };
    }
        self.broken_alias(obj.clone());
        { let __iface_handle = self.typ({ let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rhs.lock().unwrap() = __iface_value; };
        self.valid_alias(obj.clone(), rhs.clone());
    }
                // TODO(gri) Should be able to use nil instead of Typ[Invalid] to mark
                //           the alias as incomplete. Currently this causes problems
                //           with certain cycles. Investigate.
                //
                // NOTE(adonovan): to avoid the Invalid being prematurely observed
                // by (e.g.) a var whose type is an unfinished cycle,
                // Unalias does not memoize if Invalid. Perhaps we should use a
                // special sentinel distinct from Invalid.
                // handle type parameters even if not allowed (Alias type is supported)
                // resolve alias.actual
                // With Go1.23, the default behavior is to use Alias nodes,
                // reflected by check.enableAlias. Signal non-default behavior.
                //
                // TODO(gri) Testing runs tests in both modes. Do we need to exclude
                //           tracking of non-default behavior for tests?
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
                        // Report highest version requirement first so that fixing a version issue
                        // avoids possibly two -lang changes (first to Go 1.9 and then to Go 1.23).
                        // TODO(gri) Should be able to use nil instead of Typ[Invalid] to mark
                        //           the alias as incomplete. Currently this causes problems
                        //           with certain cycles. Investigate.
                        //
                        // NOTE(adonovan): to avoid the Invalid being prematurely observed
                        // by (e.g.) a var whose type is an unfinished cycle,
                        // Unalias does not memoize if Invalid. Perhaps we should use a
                        // special sentinel distinct from Invalid.
                        // handle type parameters even if not allowed (Alias type is supported)
                        // resolve alias.actual
                        // With Go1.23, the default behavior is to use Alias nodes,
                        // reflected by check.enableAlias. Signal non-default behavior.
                        //
                        // TODO(gri) Testing runs tests in both modes. Do we need to exclude
                        //           tracking of non-default behavior for tests?
                        // type definition or generic type declaration
            if !{ let __v = (*versionErr.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __nil_result = (*tparam0.lock().unwrap()).is_some(); __nil_result } && !self.verify_versionf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FieldPtr(tparam0.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = go1_18.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("type parameter".to_string()))), Arc::new(Mutex::new(Some(vec![])))) {
        { let new_val = true; *versionErr.lock().unwrap() = Some(new_val); };
    }
            let mut named = self.new_named(obj.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));
            set_def_type(def.clone(), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(named.clone())) as Box<dyn Type + Send + Sync>))));
            if { let __nil_target = (*tdecl.lock().unwrap().as_ref().unwrap()).type_params.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(tdecl.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("type parameters".to_string()))));
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));
        self.collect_type_params(Arc::new(Mutex::new(Some((*named.lock().unwrap().as_ref().unwrap()).tparams.clone()))), { let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).type_params.clone(); __field });
    }
                        // determine underlying type of named
            { let __iface_handle = self.defined_type({ let __field = (*tdecl.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, obj.clone()).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *rhs.lock().unwrap() = __iface_value; };
            assert(Arc::new(Mutex::new(Some({ let __nil_result = (*rhs.lock().unwrap()).is_some(); __nil_result }))));
            { let __iface_handle = rhs.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*named.lock().unwrap().as_mut().unwrap()).from_r_h_s.lock().unwrap() = __iface_value; };
                        // If the underlying type was not set while type-checking the right-hand
                        // side, it is invalid and an error should have been reported elsewhere.
            if { let __iface_handle = { let __field = (*named.lock().unwrap().as_ref().unwrap()).underlying.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*named.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = __iface_value; };
    }
                        // Disallow a lone type parameter as the RHS of a type declaration (go.dev/issue/45639).
                        // We don't need this restriction anymore if we make the underlying type of a type
                        // parameter its constraint interface: if the RHS is a lone type parameter, we will
                        // use its underlying type (like we do for any RHS in a type declaration), and its
                        // underlying type is an interface and the type declaration is well defined.
            if is_type_param(rhs.clone()) {
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*tdecl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_TYPE_PARAM as i32))))))), Arc::new(Mutex::new(Some("cannot use a type parameter as RHS in type declaration".to_string()))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*named.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = __iface_value; };
    }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
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
                ()
            }
        }
    }

    pub fn collect_type_params(&mut self, dst: Arc<Mutex<Option<Arc<Mutex<Option<TypeParamList>>>>>>, list: Arc<Mutex<Option<go_ast::r#mod::FieldList>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>> = Arc::new(Mutex::new(None));
                        // Declare type parameters up-front, with empty interface as type bound.
                        // The scope of type parameters starts at the beginning of the type parameter
                        // list (so we can have mutually recursive parameterized interfaces).
            let mut scopePos = { let __recv = list.clone(); let __recv_ptr: *const go_ast::r#mod::FieldList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FieldList }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
            { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        { let __range_holder = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for name in __range_values.iter() {
        { let new_val = { let __append_target = tparams.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(self.declare_type_param((*name).clone(), Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))); __append_target.clone() }; tparams = new_val; };
    } }
    } }
                        // Set the type parameters before collecting the type constraints because
                        // the parameterized type may be used by the constraints (go.dev/issue/47887).
                        // Example: type T[P T[P]] interface{}
            { let new_val = bind_t_params(tparams.clone()).clone(); let __dst = dst.clone(); let __dst_guard = __dst.lock().unwrap(); *__dst_guard.as_ref().unwrap().lock().unwrap() = (*new_val.lock().unwrap()).clone(); };
                        // Signal to cycle detection that we are in a type parameter list.
                        // We can only be inside one type parameter list at any given time:
                        // function closures may appear inside a type parameter list but they
                        // cannot be generic, and their bodies are processed in delayed and
                        // sequential fashion. Note that with each new declaration, we save
                        // the existing environment and restore it when done; thus inTPList is
                        // true exactly only when we are in a specific type parameter list.
            assert(Arc::new(Mutex::new(Some(!((*(*self.environment.lock().unwrap().as_ref().unwrap()).in_t_param_list.clone().lock().unwrap().as_ref().unwrap()))))));
            { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).in_t_param_list.lock().unwrap() = Some(new_val); };
            let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let new_val = false; *(*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).in_t_param_list.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
            let mut index = Arc::new(Mutex::new(Some(0)));
            { let __range_holder = (*list.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for f in __range_values.iter() {
        let mut bound: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
                // NOTE: we may be able to assert that f.Type != nil here, but this is not
                // an invariant of the AST, so we are cautious.
        if { let __iface_handle = { let __field = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        { let __iface_handle = self.bound({ let __field = (*f.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }).clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *bound.lock().unwrap() = __iface_value; };
        if is_type_param(bound.clone()) {
                // We may be able to allow this since it is now well-defined what
                // the underlying type and thus type set of a type parameter is.
                // But we may need some additional form of cycle detection within
                // type parameter lists.
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*f.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_TYPE_PARAM as i32))))))), Arc::new(Mutex::new(Some("cannot use a type parameter as constraint".to_string()))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *bound.lock().unwrap() = __iface_value; };
    }
    } else {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *bound.lock().unwrap() = __iface_value; };
    }
                // We may be able to allow this since it is now well-defined what
                // the underlying type and thus type set of a type parameter is.
                // But we may need some additional form of cycle detection within
                // type parameter lists.
        for i in 0..(({ let __range_holder = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __iface_handle = bound.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*{ let __seq = { let __seq_holder = tparams.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i as i32; __tmp_x + __tmp_y }) as usize].clone() }.lock().unwrap().as_mut().unwrap()).bound.lock().unwrap() = __iface_value; };
    }
        { let __rhs = ({ let __len_target = { let __field = (*f.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; let mut guard = index.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }

            // Execute deferred functions
            while let Some(f) = __defer_stack.pop() {
                f();
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
                ()
            }
        }
    }

    pub fn bound(&mut self, mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut x: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Expr::__go_clone_box_expr(__v.as_ref()))));
                // A type set literal of the form ~T and A|B may only appear as constraint;
                // embed it in an implicit interface so that only interface type-checking
                // needs to take care of such type expressions.
        let mut wrap = Arc::new(Mutex::new(Some(false)));
        {
    let _ts_subject = x.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExprPtr>()).is_some() {
        let op = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::UnaryExprPtr>()).unwrap().0.clone();
        { let new_val = { let __tmp_x = { let __selector_holder = (*op.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); __tmp_x == __tmp_y }; *wrap.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).is_some() {
        let op = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BinaryExprPtr>()).unwrap().0.clone();
        { let new_val = { let __tmp_x = { let __selector_holder = (*op.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32)))); __tmp_x == __tmp_y }; *wrap.lock().unwrap() = Some(new_val); };;
    }
    }
        if { let __v = (*wrap.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::InterfaceTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::InterfaceType { methods: Arc::new(Mutex::new(Some(go_ast::r#mod::FieldList { list: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: x.clone(), ..Default::default() })))]))), ..Default::default() }))).clone(), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *x.lock().unwrap() = __iface_value; };
        let mut t = self.typ(x.clone());
                // mark t as implicit interface if all went well
        {
        let (mut t, _) = ({
        let val = t.clone();
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
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            { let new_val = true; *(*t.lock().unwrap().as_ref().unwrap()).implicit.lock().unwrap() = Some(new_val); };;
        }
    }
        return t.clone();
    }
                // mark t as implicit interface if all went well
        return self.typ(x.clone()).clone();
    }

    pub fn declare_type_param(&mut self, name: Arc<Mutex<Option<go_ast::r#mod::Ident>>>, scopePos: Arc<Mutex<Option<go_token::position::Pos>>>) -> Arc<Mutex<Option<crate::typeparam::TypeParam>>> {
                // Use Typ[Invalid] for the type constraint to ensure that a type
                // is present even if the actual constraint has not been assigned
                // yet.
                // TODO(gri) Need to systematically review all uses of type parameter
                //           constraints to make sure we don't rely on them if they
                //           are not properly set yet.
        let mut tname = new_type_name({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));
        let mut tpar = self.new_type_param(tname.clone(), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
        { let __method_arg0 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = name.clone(); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(tname.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
        return tpar.clone();
    }

    pub fn collect_methods(&mut self, obj: Arc<Mutex<Option<TypeName>>>) {
                // get associated methods
                // (Checker.collectObjects only collects methods with non-blank names;
                // Checker.resolveBaseTypeName ensures that obj is not an alias name
                // if it has attached methods.)
        let mut methods = { let __map = { let __map_holder = self.methods.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(obj.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        if { let __nil_result = (*methods.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
        { let __map_handle = self.methods.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&GoLocalPtrKey::new(obj.clone())); };
        assert(Arc::new(Mutex::new(Some(!go_token::position::Pos::is_valid(&(*(*(*{ let __map = { let __map_holder = self.obj_map.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoObjectInterfaceKey::new(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>)))))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).tdecl.lock().unwrap().as_ref().unwrap()).assign.lock().unwrap().as_ref().unwrap()))))));
                // use an objset to check for name conflicts
        let mut mset: Arc<Mutex<Option<objset>>> = Arc::new(Mutex::new(Some(crate::objset::objset(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new())))))));
                // spec: "If the base type is a struct type, the non-blank method
                // and field names must be distinct."
        let mut base = as_named({ let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field });
        if { let __nil_result = (*base.lock().unwrap()).is_some(); __nil_result } {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = { let __recv = base.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y }))));
                // See go.dev/issue/52529: we must delay the expansion of underlying here, as
                // base may not be fully set-up.
        let base_closure_clone = base.clone(); let mut check_closure_clone = (*self).clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let base_closure_clone_closure_clone = base_closure_clone.clone(); let mut check_closure_clone_closure_clone = check_closure_clone.clone(); Box::new(move || {
        check_closure_clone_closure_clone.check_field_uniqueness(base_closure_clone_closure_clone.clone());
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("verifying field uniqueness for %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(base_closure_clone.clone()) as Box<dyn Any + Send + Sync>])))); __result };
                // Checker.Files may be called multiple times; additional package files
                // may add methods to already type-checked types. Add pre-existing methods
                // so that we can detect redeclarations.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = base.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.num_methods(); __result }; __tmp_x < __tmp_y } {
        let mut m = { let __recv = base.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y }))));
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*(*mset.lock().unwrap().as_mut().unwrap()).insert(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>)))).lock().unwrap()).is_none(); __nil_result }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // collectMethods should not be called on an instantiated type
                // See go.dev/issue/52529: we must delay the expansion of underlying here, as
                // base may not be fully set-up.
                // Checker.Files may be called multiple times; additional package files
                // may add methods to already type-checked types. Add pre-existing methods
                // so that we can detect redeclarations.
                // add valid methods
        { let __range_holder = methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for m in __range_values.iter() {
                // spec: "For a base type, the non-blank names of methods bound
                // to it must be unique."
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y }))));
        {
        let mut alt = (*mset.lock().unwrap().as_mut().unwrap()).insert(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>))));;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            if go_token::position::Pos::is_valid(&(*(*alt.lock().unwrap().as_ref().unwrap()).pos().lock().unwrap().as_ref().unwrap())) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_METHOD as i32))))))), Arc::new(Mutex::new(Some("method %s.%s already declared at %v".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = (*alt.lock().unwrap().as_ref().unwrap()).pos(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_METHOD as i32))))))), Arc::new(Mutex::new(Some("method %s.%s already declared".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = { let __recv = obj.clone(); let __recv_ptr: *const crate::object::TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::TypeName }; let __result = unsafe { &*__recv_ptr }.name(); __result }; let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    };
            continue;
        }
    }
        if { let __nil_result = (*base.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = base.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.add_method((*m).clone()); __result };
    }
    } }
    }

    pub fn check_field_uniqueness(&self, base: Arc<Mutex<Option<Named>>>) {
        {
        let (mut t, _) = ({
        let val = { let __recv = base.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone();
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
        if { let __nil_result = (*t.lock().unwrap()).is_some(); __nil_result } {
            let mut mset: Arc<Mutex<Option<objset>>> = Arc::new(Mutex::new(Some(crate::objset::objset(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>::new())))))));;
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv = base.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.num_methods(); __result }; __tmp_x < __tmp_y } {
        let mut m = { let __recv = base.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y }))));
        assert(Arc::new(Mutex::new(Some({ let __nil_result = (*(*mset.lock().unwrap().as_mut().unwrap()).insert(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(m.clone())) as Box<dyn Object + Send + Sync>)))).lock().unwrap()).is_none(); __nil_result }))));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
            { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for fld in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        {
        let mut alt = (*mset.lock().unwrap().as_mut().unwrap()).insert(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(fld.clone())) as Box<dyn Object + Send + Sync>))));;
        if { let __nil_result = (*alt.lock().unwrap()).is_some(); __nil_result } {
            let _ = ({
        let val = alt.clone();
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
    });;
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_FIELD_AND_METHOD as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("field and method with the same name %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*fld.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.add_alt_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(fld.clone())) as Box<dyn Object + Send + Sync>)))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
        }
    }
    }
    } };
        }
    }
    }

    pub fn func_decl(&mut self, obj: Arc<Mutex<Option<Func>>>, decl: Arc<Mutex<Option<declInfo>>>) {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
                // func declarations cannot use iota
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).iota.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
        let mut sig = Arc::new(Mutex::new(Some(Signature::default())));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = __iface_value; };
                // Avoid cycle error when referring to method while type-checking the signature.
                // This avoids a nuisance in the best case (non-parameterized receiver type) and
                // since the method is not a type, we get an error. If we have a parameterized
                // receiver type, instantiating the receiver type leads to the instantiation of
                // its methods, and we don't want a cycle error in that case.
                // TODO(gri) review if this is correct and/or whether we still need this?
        let mut saved = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).color_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).color_.lock().unwrap() = Some(new_val); };
        let mut fdecl = (*decl.lock().unwrap().as_ref().unwrap()).fdecl.clone();
        self.func_type(sig.clone(), { let __field = (*fdecl.lock().unwrap().as_ref().unwrap()).recv.clone(); __field }, { let __field = (*fdecl.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field });
        { let new_val = saved.lock().unwrap().as_ref().unwrap().clone(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).color_.lock().unwrap() = Some(new_val); };
                // Set the scope's extent to the complete "func (...) { ... }"
                // so that Scope.Innermost works correctly.
        { let new_val = { let __recv = fdecl.clone(); let __recv_ptr: *const go_ast::r#mod::FuncDecl = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncDecl }; let __result = unsafe { &*__recv_ptr }.pos(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*sig.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap() = __moved_val; };
        { let new_val = { let __recv = fdecl.clone(); let __recv_ptr: *const go_ast::r#mod::FuncDecl = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::FuncDecl }; let __result = unsafe { &*__recv_ptr }.end(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*(*sig.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).end.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (*(*(*fdecl.lock().unwrap().as_ref().unwrap()).r#type.lock().unwrap().as_ref().unwrap()).type_params.lock().unwrap().as_ref().unwrap()).num_fields(); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __nil_target = (*fdecl.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr((*fdecl.lock().unwrap().as_ref().unwrap()).name.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(BAD_DECL as i32))))))), Arc::new(Mutex::new(Some("generic function is missing function body".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
    }
                // function body must be type-checked after global declarations
                // (functions implemented elsewhere have no body)
        if !(*(*self.conf.lock().unwrap().as_ref().unwrap()).ignore_func_bodies.lock().unwrap().as_ref().unwrap()) && { let __nil_target = (*fdecl.lock().unwrap().as_ref().unwrap()).body.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        let mut check_closure_clone = (*self).clone(); let decl_closure_clone = decl.clone(); let fdecl_closure_clone = fdecl.clone(); let obj_closure_clone = obj.clone(); let sig_closure_clone = sig.clone(); { let __recv = { let mut __recv = check_closure_clone.clone(); let __method_arg0 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); let obj_closure_clone_closure_clone = obj_closure_clone.clone(); Box::new(move || {
        check_closure_clone_closure_clone.func_body(decl_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), sig_closure_clone.clone(), { let __field = (*fdecl_closure_clone.lock().unwrap().as_ref().unwrap()).body.clone(); __field }, Arc::new(Mutex::new(None)));
    }) as Box<dyn FnMut() -> () + Send + Sync> }))); __recv.later(__method_arg0) }; let __result = (*__recv.as_ref().unwrap().borrow_mut().as_mut().unwrap()).describef(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(obj_closure_clone.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("func %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*obj_closure_clone.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
    }

    pub fn decl_stmt(&mut self, d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>) {
        let mut pkg = self.pkg.clone();
        let mut check_closure_clone = (*self).clone(); let pkg_closure_clone = pkg.clone(); { let mut __recv = check_closure_clone.clone(); let __method_arg0 = d.clone(); let __method_arg1 = Arc::new(Mutex::new(Some({ let mut check_closure_clone_closure_clone = check_closure_clone.clone(); Box::new(move |mut d: Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>| {
        {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<constDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<constDecl>()).unwrap().clone())));
        let mut top = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = check_closure_clone_closure_clone.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        let mut lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Const>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));;
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        let mut obj = new_const({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)), go_constant::make_int64(Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).iota.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64)))).clone());
        (*lhs.lock().unwrap().as_mut().unwrap())[(i) as usize] = obj.clone();
        let mut init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = (*d.lock().unwrap().as_ref().unwrap()).init.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
    }
        check_closure_clone_closure_clone.const_decl(obj.clone(), { let __field = (*d.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, init.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).inherited.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    } };
        check_closure_clone_closure_clone.process_delayed(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        let mut scopePos = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).end();;
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        { let __method_arg0 = { let __field = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = (*name).clone(); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::ConstPtr({ let __seq = { let __seq_holder = lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); check_closure_clone_closure_clone.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<varDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<varDecl>()).unwrap().clone())));
        let mut top = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = check_closure_clone_closure_clone.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));;
        let mut lhs0: Arc<Mutex<Option<Vec<Arc<Mutex<Option<crate::object::Var>>>>>>> = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])));;
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        (*lhs0.lock().unwrap().as_mut().unwrap())[(i) as usize] = new_var({ let __recv = name.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));
    } };
        { let __range_holder = lhs0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, obj) in __range_values.iter().enumerate() {
        let mut lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        let mut init: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let _switch_val = ({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) });
    if _switch_val == (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
            { let __iface_handle = { let __seq = { let __seq_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
        } else if _switch_val == (1) {
            { let new_val = lhs0.clone(); lhs = new_val; };
            { let __iface_handle = { let __seq = { let __seq_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
        } else {
            if { let __tmp_x = (i as i32); let __tmp_y = (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *init.lock().unwrap() = __iface_value; };
    }
        }
    }
        check_closure_clone_closure_clone.var_decl((*obj).clone(), lhs.clone(), { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }, init.clone());
        if { let __tmp_x = (({ let __len_target = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).values.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        if DEBUG {
        { let __range_holder = lhs0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
    } }
    }
        break
    }
    } };
        check_closure_clone_closure_clone.process_delayed(Arc::new(Mutex::new(Some({ let __arg_holder = top.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        let mut scopePos = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).end();;
        { let __range_holder = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, name) in __range_values.iter().enumerate() {
        { let __method_arg0 = { let __field = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = (*name).clone(); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr({ let __seq = { let __seq_holder = lhs0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); check_closure_clone_closure_clone.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<typeDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<typeDecl>()).unwrap().clone())));
        let mut obj = new_type_name((*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos(), pkg_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(None)));;
        let mut scopePos = (*(*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos();;
        { let __method_arg0 = { let __field = (*check_closure_clone_closure_clone.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = { let __field = (*(*d.lock().unwrap().as_ref().unwrap()).spec.lock().unwrap().as_ref().unwrap()).name.clone(); __field }; let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); check_closure_clone_closure_clone.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };;
        { let __recv = obj.clone(); let __recv_ptr: *mut crate::object::TypeName = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::object::TypeName }; let __result = unsafe { &mut *__recv_ptr }.set_color(Arc::new(Mutex::new(Some({ let __tmp_x = crate::object::color(Arc::new(Mutex::new(Some(GREY as u32)))); let __tmp_y = crate::object::color(Arc::new(Mutex::new(Some(check_closure_clone_closure_clone.push(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>)))) as u32)))); __tmp_x + __tmp_y })))); __result };;
        check_closure_clone_closure_clone.type_decl(obj.clone(), { let __field = (*d.lock().unwrap().as_ref().unwrap()).spec.clone(); __field }, Arc::new(Mutex::new(None)));;
        { let __recv = check_closure_clone_closure_clone.pop(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };;
    } else {
        let d = _ts_subject.clone();
        check_closure_clone_closure_clone.errorf(Arc::new(Mutex::new(Some(Box::new((*(*d.lock().unwrap().as_ref().unwrap()).node().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown ast.Decl node %T".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*d.lock().unwrap().as_ref().unwrap()).node(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));;
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn decl + Send + Sync>>>>) -> () + Send + Sync> }))); __recv.walk_decl(__method_arg0, __method_arg1) };
    }
}

impl importDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ImportSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl decl for importDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        importDecl::node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<importDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct importDeclPtr(pub Arc<Mutex<Option<importDecl>>>);

impl std::fmt::Display for importDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl decl for importDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        importDecl::node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<importDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl constDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl decl for constDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        constDecl::node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<constDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct constDeclPtr(pub Arc<Mutex<Option<constDecl>>>);

impl std::fmt::Display for constDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl decl for constDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        constDecl::node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<constDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl varDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl decl for varDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        varDecl::node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<varDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct varDeclPtr(pub Arc<Mutex<Option<varDecl>>>);

impl std::fmt::Display for varDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl decl for varDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        varDecl::node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<varDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl typeDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl decl for typeDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        typeDecl::node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<typeDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct typeDeclPtr(pub Arc<Mutex<Option<typeDecl>>>);

impl std::fmt::Display for typeDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl decl for typeDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        typeDecl::node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<typeDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl funcDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncDeclPtr(self.decl.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl decl for funcDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        funcDecl::node(self)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<funcDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct funcDeclPtr(pub Arc<Mutex<Option<funcDecl>>>);

impl std::fmt::Display for funcDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl decl for funcDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        funcDecl::node(__recv)
    }
    fn __go_clone_box_decl(&self) -> Box<dyn decl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn decl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_decl(&self, other: &(dyn decl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<funcDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// pathString returns a string of the form a->b-> ... ->g for a path [a, b, ... g].
pub fn path_string(path: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<String>>> {
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    { let __range_holder = path.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, p) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&"->".to_string()); };
    }
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&{ let __s = (*p.lock().unwrap().as_ref().unwrap()).name(); let __value = (*__s.lock().unwrap().as_ref().unwrap()).clone(); __value }); };
    } }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// firstInSrc reports the index of the object with the "smallest"
/// source position in path. path must not be empty.
pub fn first_in_src(path: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>>) -> i32 {
    let (mut fst, mut pos) = (Arc::new(Mutex::new(Some(0))), { let __recv = { let __seq = { let __seq_holder = path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result });
    for (i, t) in { let __seq_holder = path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }.iter().enumerate() {
        if { let __tmp_x = cmp_pos((*t.lock().unwrap().as_ref().unwrap()).pos(), Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let __tmp_0 = { let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_1 = (*t.lock().unwrap().as_ref().unwrap()).pos(); *fst.lock().unwrap() = Some(__tmp_0); *pos.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
    }
    return { let __v = (*fst.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

impl GoValueClone for importDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for constDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for varDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for typeDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for funcDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
