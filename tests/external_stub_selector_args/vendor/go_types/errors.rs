use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_any_clone, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload};

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
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const INVALID_ARG: &'static str = "invalid argument: ";
pub(crate) const INVALID_OP: &'static str = "invalid operation: ";


/// An errorDesc describes part of a type-checking error.
#[derive(Clone)]
pub struct errorDesc {
    pub posn: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>,
    pub msg: Arc<Mutex<Option<String>>>,
}

impl errorDesc {
    pub fn __go_value_clone(&self) -> Self {
        Self { posn: self.posn.clone(), msg: { let __guard = self.msg.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for errorDesc {
    fn default() -> Self {
        Self { posn: Arc::new(Mutex::new(None)), msg: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for errorDesc {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.posn.lock().unwrap().as_ref().unwrap()), (*self.msg.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for errorDesc {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An error_ represents a type-checking error.
/// A new error_ is created with Checker.newError.
/// To report an error_, call error_.report.
#[derive(Clone)]
pub struct error_ {
    pub check: Arc<Mutex<Option<Checker>>>,
    pub desc: Arc<Mutex<Option<Vec<errorDesc>>>>,
    pub code: Arc<Mutex<Option<Code>>>,
    pub soft: Arc<Mutex<Option<bool>>>,
}

impl error_ {
    pub fn __go_value_clone(&self) -> Self {
        Self { check: self.check.clone(), desc: self.desc.clone(), code: { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, soft: { let __guard = self.soft.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for error_ {
    fn default() -> Self {
        Self { check: Arc::new(Mutex::new(None)), desc: Arc::new(Mutex::new(None)), code: Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0))))))), soft: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for error_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { let __guard = self.check.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, format_slice(&self.desc), (*self.code.lock().unwrap().as_ref().unwrap()), (*self.soft.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for error_ {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// The positioner interface is used to extract the position of type-checker errors.
pub trait positioner: std::fmt::Display + Any {
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
}

impl Clone for Box<dyn positioner + Send + Sync> {
    fn clone(&self) -> Self {
        positioner::__go_clone_box_positioner(self.as_ref())
    }
}

/// atPos wraps a token.Pos to implement the positioner interface.
#[derive(Debug, Clone, Default)]
pub struct atPos(pub Arc<Mutex<Option<go_token::position::Pos>>>);

impl Display for atPos {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}


/// posSpan holds a position range along with a highlighted position within that
/// range. This is used for positioning errors, with pos by convention being the
/// first position in the source where the error is known to exist, and start
/// and end defining the full span of syntax being considered when the error was
/// detected. Invariant: start <= pos < end || start == pos == end.
#[derive(Clone)]
pub struct posSpan {
    pub start: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub end: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl posSpan {
    pub fn __go_value_clone(&self) -> Self {
        Self { start: { let __guard = self.start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for posSpan {
    fn default() -> Self {
        Self { start: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), end: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for posSpan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.start.lock().unwrap().as_ref().unwrap()), (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for posSpan {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::check::Checker {
    /// newError returns a new error_ with the given error code.
    pub fn new_error(&self, code: Arc<Mutex<Option<Code>>>) -> Arc<Mutex<Option<error_>>> {
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new("error code must not be 0".to_string()) as Box<dyn Any + Send + Sync>);
    }
        Arc::new(Mutex::new(Some(error_ { check: Arc::new(Mutex::new(Some(self.clone()))), code: Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
    }

    /// handleError should only be called by error_.report.
    pub fn handle_error(&mut self, index: Arc<Mutex<Option<i32>>>, mut posn: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, code: Arc<Mutex<Option<Code>>>, mut msg: Arc<Mutex<Option<String>>>, soft: Arc<Mutex<Option<bool>>>) {
        let mut posn: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> = Arc::new(Mutex::new(posn.lock().unwrap().as_ref().map(|__v| positioner::__go_clone_box_positioner(__v.as_ref()))));
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0 as i32)))); __tmp_x != __tmp_y }))));
        if { let __tmp_x = { let __v = (*index.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                // If we are encountering an error while evaluating an inherited
                // constant initialization expression, pos is the position of
                // the original expression, and not of the currently declared
                // constant identifier. Use the provided errpos instead.
                // TODO(gri) We may also want to augment the error message and
                // refer to the position (pos) in the original expression.
        if { let __iface_handle = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).errpos.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } && go_token::position::Pos::is_valid(&(*(*(*self.environment.lock().unwrap().as_ref().unwrap()).errpos.lock().unwrap().as_ref().unwrap()).pos().lock().unwrap().as_ref().unwrap())) {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).iota.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        { let __iface_handle = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).errpos.clone(); __field }; let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *posn.lock().unwrap() = __iface_value; };
    }
                // Report invalid syntax trees explicitly.
        if { let __tmp_x = (*code.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32)))); __tmp_x == __tmp_y } {
        { let new_val = format!("{}{}", "invalid syntax tree: ".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }); *msg.lock().unwrap() = Some(new_val); };
    }
                // If we have a URL for error codes, add a link to the first line.
        if { let __tmp_x = { let __selector_holder = (*self.conf.lock().unwrap().as_ref().unwrap()).__error_u_r_l.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        let mut url = Arc::new(Mutex::new(Some(format!("{}", (*(*self.conf.lock().unwrap().as_ref().unwrap()).__error_u_r_l.lock().unwrap().as_ref().unwrap()).clone()))));
        {
        let mut i = Arc::new(Mutex::new(Some({ let __s = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "\n".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) })));;
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            { let new_val = { let mut __s = String::new(); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*msg.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", { let __v = (*url.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some({ let __s = &((*msg.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()))); __s }; *msg.lock().unwrap() = Some(new_val); };;
        } else {
            { (*msg.lock().unwrap().as_mut().unwrap()).push_str(&{ let __v = (*url.lock().unwrap().as_ref().unwrap()).clone(); __v }); };;
        }
    }
    }
    } else {
                // Indent sub-error.
                // Position information is passed explicitly to Error, below.
        { let new_val = format!("{}{}", "\t".to_string(), { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v }); *msg.lock().unwrap() = Some(new_val); };
    }
                // If we are encountering an error while evaluating an inherited
                // constant initialization expression, pos is the position of
                // the original expression, and not of the currently declared
                // constant identifier. Use the provided errpos instead.
                // TODO(gri) We may also want to augment the error message and
                // refer to the position (pos) in the original expression.
                // Report invalid syntax trees explicitly.
                // If we have a URL for error codes, add a link to the first line.
                // Indent sub-error.
                // Position information is passed explicitly to Error, below.
        let mut span = span_of(posn.clone());
        let mut e = Arc::new(Mutex::new(Some(Error { fset: { let __field = self.fset.clone(); __field }, pos: Arc::new(Mutex::new(Some({ let __selector_holder = (*span.lock().unwrap().as_ref().unwrap()).pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), msg: strip_annotations(Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), soft: Arc::new(Mutex::new(Some({ let __arg_holder = soft.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), go116code: Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), go116start: Arc::new(Mutex::new(Some({ let __selector_holder = (*span.lock().unwrap().as_ref().unwrap()).start.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), go116end: Arc::new(Mutex::new(Some({ let __selector_holder = (*span.lock().unwrap().as_ref().unwrap()).end.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), ..Default::default() })));
        if { let __iface_handle = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).errpos.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
                // If we have an internal error and the errpos override is set, use it to
                // augment our error positioning.
                // TODO(rFindley) we may also want to augment the error message and refer
                // to the position (pos) in the original expression.
        let mut span = span_of({ let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).errpos.clone(); __field });
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*span.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*e.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*span.lock().unwrap().as_ref().unwrap()).start.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*e.lock().unwrap().as_ref().unwrap()).go116start.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*span.lock().unwrap().as_ref().unwrap()).end.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*e.lock().unwrap().as_ref().unwrap()).go116end.lock().unwrap() = Some(new_val); };
    }
                // If we have an internal error and the errpos override is set, use it to
                // augment our error positioning.
                // TODO(rFindley) we may also want to augment the error message and refer
                // to the position (pos) in the original expression.
        if { let __nil_target = self.first_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>; *self.first_err.lock().unwrap() = Some(new_val); };
    }
        let mut f = (*self.conf.lock().unwrap().as_ref().unwrap()).error.clone();
        if { let __nil_result = (*f.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new(bailout {  }) as Box<dyn Any + Send + Sync>);
    }
                // record first error and exit
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>)))) };
    }

    pub fn error(&self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, code: Arc<Mutex<Option<Code>>>, msg: Arc<Mutex<Option<String>>>) {
        let mut err = self.new_error(Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some("%s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }

    pub fn errorf(&self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, code: Arc<Mutex<Option<Code>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        let mut err = self.new_error(Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }

    pub fn soft_errorf(&self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, code: Arc<Mutex<Option<Code>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        let mut err = self.new_error(Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()); __result };
        { let new_val = true; *(*err.lock().unwrap().as_ref().unwrap()).soft.lock().unwrap() = Some(new_val); };
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }

    pub fn version_errorf(&self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, v: Arc<Mutex<Option<goVersion>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        let mut msg = self.sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone());
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNSUPPORTED_FEATURE as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(at.clone(), Arc::new(Mutex::new(Some("%s requires %s or later".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new((*v.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
    }
}

impl error_ {
    /// addf adds formatted error information to err.
    /// It may be called multiple times to provide additional information.
    /// The position of the first call to addf determines the position of the reported Error.
    /// Subsequent calls to addf provide additional information in the form of additional lines
    /// in the error message (types2) or continuation errors identified by a tab-indented error
    /// message (go/types).
    pub fn addf(&mut self, at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        { let new_val = { let __append_target = self.desc.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(errorDesc { posn: at.clone(), msg: (*self.check.lock().unwrap().as_ref().unwrap()).sprintf(Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()), ..Default::default() }); __append_target.clone() }; self.desc = new_val; };
    }

    /// addAltDecl is a specialized form of addf reporting another declaration of obj.
    pub fn add_alt_decl(&mut self, obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
        {
        let mut pos = (*obj.lock().unwrap().as_ref().unwrap()).pos();;
        if go_token::position::Pos::is_valid(&(*pos.lock().unwrap().as_ref().unwrap())) {
            self.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = obj.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("other declaration of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*obj.lock().unwrap().as_ref().unwrap()).name(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));;
        }
    }
    }

    pub fn empty(&self) -> bool {
        return { let __nil_target = self.desc.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result };
    }

    pub fn posn(&self) -> Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>> {
        if self.empty() {
        return Arc::new(Mutex::new(Some(Box::new((*noposn.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>)));
    }
        return { let __field = { let __seq = { let __seq_holder = self.desc.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.posn.clone(); __field };
    }

    /// msg returns the formatted error message without the primary error position pos().
    pub fn msg(&self) -> Arc<Mutex<Option<String>>> {
        if self.empty() {
        return Arc::new(Mutex::new(Some("no error".to_string())));
    }
        let mut buf: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(Default::default())));
        for i in 0..(({ let __range_holder = self.desc.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut p: Option<GoSliceElemPtr<errorDesc>> = Some(GoSliceElemPtr::new(self.desc.clone(), (i) as usize));
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        fmt::fprint(buf.clone(), ("\n\t".to_string(),));
        if go_token::position::Pos::is_valid(&(*(*(*p.as_ref().unwrap().borrow().as_ref().unwrap()).posn.lock().unwrap().as_ref().unwrap()).pos().lock().unwrap().as_ref().unwrap())) {
        (*buf.clone().lock().unwrap().as_mut().unwrap()).push_str(&format!("{}: ", (*(*(*self.check.lock().unwrap().as_ref().unwrap()).fset.lock().unwrap().as_ref().unwrap()).position((*(*p.as_ref().unwrap().borrow().as_ref().unwrap()).posn.lock().unwrap().as_ref().unwrap()).pos()).lock().unwrap().as_ref().unwrap())));
    }
    }
        (*buf.lock().unwrap().as_mut().unwrap()).push_str(&(*(*p.as_ref().unwrap().borrow().as_ref().unwrap()).msg.lock().unwrap().as_ref().unwrap()).clone());
    }
        return Arc::new(Mutex::new(Some({ let __builder = buf.clone(); let __guard = __builder.lock().unwrap(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    }

    /// report reports the error err, setting check.firstError if necessary.
    pub fn report(&mut self) {
        if self.empty() {
        std::panic::panic_any(Box::new("no error".to_string()) as Box<dyn Any + Send + Sync>);
    }
                // Cheap trick: Don't report errors with messages containing
                // "invalid operand" or "invalid type" as those tend to be
                // follow-on errors which don't add useful information. Only
                // exclude them if these strings are not at the beginning,
                // and only if we have at least one error already reported.
        let mut check = self.check.clone();
        if { let __nil_target = (*check.lock().unwrap().as_ref().unwrap()).first_err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // It is sufficient to look at the first sub-error only.
        let mut msg = Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = self.desc.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "invalid operand".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = (*msg.lock().unwrap().as_ref().unwrap()).clone(); let __substr = "invalid type".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return;
    }
    }
                // It is sufficient to look at the first sub-error only.
        if (*(*(*check.lock().unwrap().as_ref().unwrap()).conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.trace({ let __recv = self.posn(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).pos(); __result }, Arc::new(Mutex::new(Some("ERROR: %s (code = %d)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = { let __seq = { let __seq_holder = self.desc.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }
                // In go/types, if there is a sub-error with a valid position,
                // call the typechecker error handler for each sub-error.
                // Otherwise, call it once, with a single combined message.
        let mut multiError = Arc::new(Mutex::new(Some(false)));
        if !IS_TYPES2 {
        let mut i = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = self.desc.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        if go_token::position::Pos::is_valid(&(*(*{ let __seq = { let __seq_holder = self.desc.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.posn.lock().unwrap().as_ref().unwrap()).pos().lock().unwrap().as_ref().unwrap())) {
        { let new_val = true; *multiError.lock().unwrap() = Some(new_val); };
        break
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        if { let __v = (*multiError.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        for i in 0..(({ let __range_holder = self.desc.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut p: Option<GoSliceElemPtr<errorDesc>> = Some(GoSliceElemPtr::new(self.desc.clone(), (i) as usize));
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.handle_error(Arc::new(Mutex::new(Some(i as i32))), { let __field = (*p.as_ref().unwrap().borrow().as_ref().unwrap()).posn.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*p.as_ref().unwrap().borrow().as_ref().unwrap()).msg.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = self.soft.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
    }
    } else {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.handle_error(Arc::new(Mutex::new(Some(0))), self.posn().clone(), Arc::new(Mutex::new(Some({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), self.msg(), Arc::new(Mutex::new(Some({ let __selector_holder = self.soft.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };
    }
                // make sure the error is not reported twice
        *self.desc.lock().unwrap() = None;
    }
}

impl atPos {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*self.0.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as i32)))))))
    }
}

impl positioner for atPos {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        atPos::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<atPos>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct atPosPtr(pub Arc<Mutex<Option<atPos>>>);

impl std::fmt::Display for atPosPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl positioner for atPosPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        atPos::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<atPosPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cmp::r#mod::Ordered for atPos {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<atPos>() {
            false
        } else {
            false
        }
    }
}

impl posSpan {
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.pos.clone();
    }
}

impl positioner for posSpan {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        posSpan::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<posSpan>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct posSpanPtr(pub Arc<Mutex<Option<posSpan>>>);

impl std::fmt::Display for posSpanPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl positioner for posSpanPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        posSpan::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<posSpanPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Box<dyn go_ast::r#mod::Decl + Send + Sync> {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<Box<dyn go_ast::r#mod::Decl + Send + Sync>>() {
            false
        } else {
            false
        }
    }
}

impl positioner for Box<dyn go_ast::r#mod::Expr + Send + Sync> {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<Box<dyn go_ast::r#mod::Expr + Send + Sync>>() {
            false
        } else {
            false
        }
    }
}

impl positioner for Box<dyn go_ast::r#mod::Node + Send + Sync> {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<Box<dyn go_ast::r#mod::Node + Send + Sync>>() {
            false
        } else {
            false
        }
    }
}

impl positioner for Box<dyn go_ast::r#mod::Spec + Send + Sync> {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<Box<dyn go_ast::r#mod::Spec + Send + Sync>>() {
            false
        } else {
            false
        }
    }
}

impl positioner for Box<dyn go_ast::r#mod::Stmt + Send + Sync> {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::AssignStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::AssignStmt::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::BasicLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::BasicLit::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BasicLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::BranchStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::BranchStmt::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BranchStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::CallExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::CallExpr::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CallExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::ChanTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::ChanType::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ChanTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::CompositeLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::CompositeLit::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::CompositeLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::EllipsisPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ellipsis::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::EllipsisPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::FieldListPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::FieldList::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FieldListPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::FieldPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Field::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FieldPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::FilePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::File::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FilePtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::FuncLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::FuncLit::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::FuncLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::IdentPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ident::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::ImportSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::ImportSpec::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ImportSpecPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::InterfaceTypePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::InterfaceType::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::InterfaceTypePtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::KeyValueExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::KeyValueExpr::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::ReturnStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::ReturnStmt::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ReturnStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::SelectorExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::SelectorExpr::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::SelectorExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::TypeAssertExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::TypeAssertExpr::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeAssertExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::TypeSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::TypeSpec::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeSpecPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::TypeSwitchStmtPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::TypeSwitchStmt::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::UnaryExprPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::UnaryExpr::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::ValueSpecPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::ValueSpec::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::ValueSpecPtr>() {
            false
        } else {
            false
        }
    }
}

pub fn assert(p: Arc<Mutex<Option<bool>>>) {
    if !{ let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut msg = Arc::new(Mutex::new(Some("assertion failed".to_string())));
                // Include information about the assertion location. Due to panic recovery,
                // this location is otherwise buried in the middle of the panicking stack.
        {
        let (_, mut file, mut line, mut ok) = runtime::caller(1);;
        if ok {
            { let new_val = Arc::new(Mutex::new(Some(format!("{}:{}: {}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v }, line, { let __v = (*msg.lock().unwrap().as_ref().unwrap()).clone(); __v })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *msg.lock().unwrap() = __moved_val; };;
        }
    }
        std::panic::panic_any(Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>);
    }
}

/// inNode creates a posSpan for the given node.
/// Invariant: node.Pos() <= pos < node.End() (node.End() is the position of the
/// first byte after node within the source).
pub fn in_node(node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>, pos: Arc<Mutex<Option<go_token::position::Pos>>>) -> Arc<Mutex<Option<posSpan>>> {
    let (mut start, mut end) = ((*node.lock().unwrap().as_ref().unwrap()).pos(), (*node.lock().unwrap().as_ref().unwrap()).end());
    if DEBUG {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*start.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*pos.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*end.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x < __tmp_y }))));
    }
    return Arc::new(Mutex::new(Some(posSpan { start: Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some({ let __arg_holder = end.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
}

/// spanOf extracts an error span from the given positioner. By default this is
/// the trivial span starting and ending at pos, but this span is expanded when
/// the argument naturally corresponds to a span of source code.
pub fn span_of(at: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>) -> Arc<Mutex<Option<posSpan>>> {
    {
    let _ts_subject = at.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn positioner + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let x = _ts_subject.clone();
        std::panic::panic_any(Box::new("nil positioner".to_string()) as Box<dyn Any + Send + Sync>);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<posSpan>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<posSpan>()).unwrap().clone())));
        return { let __owned = x.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Box<dyn go_ast::r#mod::Node + Send + Sync>>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Box<dyn go_ast::r#mod::Node + Send + Sync>>()).unwrap().clone())));
        let mut pos = (*x.lock().unwrap().as_ref().unwrap()).pos();;
        return Arc::new(Mutex::new(Some(posSpan { start: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: (*x.lock().unwrap().as_ref().unwrap()).end(), ..Default::default() })));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operandPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::operand::operandPtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*x.lock().unwrap().as_ref().unwrap()).expr.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        let mut pos = { let __recv = x.clone(); let __recv_ptr: *const crate::operand::operand = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::operand::operand }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
        return Arc::new(Mutex::new(Some(posSpan { start: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: (*(*x.lock().unwrap().as_ref().unwrap()).expr.lock().unwrap().as_ref().unwrap()).end(), ..Default::default() })));
    };
        return Arc::new(Mutex::new(Some(posSpan { start: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));;
    } else {
        let x = _ts_subject.clone();
        let mut pos = (*at.lock().unwrap().as_ref().unwrap()).pos();;
        return Arc::new(Mutex::new(Some(posSpan { start: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));;
    }
    }
    unreachable!()
}

impl GoValueClone for errorDesc {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for error_ {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for posSpan {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
