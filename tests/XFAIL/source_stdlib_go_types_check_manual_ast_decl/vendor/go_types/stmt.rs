use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, __go_type_name, format_any, format_any_slice, format_any_variadic, format_map, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values, go_lookup_embedded_owner, go_recover, go_register_embedded_owner, go_resume_unrecovered_panic, go_store_panic_payload, go_strconv_format_float, go_strconv_format_int};

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

pub(crate) const BREAK_OK: u64 = 1 << 0;
pub(crate) const CONTINUE_OK: u64 = 1 << 1;
pub(crate) const FALLTHROUGH_OK: u64 = 1 << 2;
pub(crate) const FINAL_SWITCH_CASE: u64 = 1 << 3;
pub(crate) const IN_TYPE_SWITCH: u64 = 1 << 4;


/// stmtContext is a bitset describing which
/// control-flow statements are permissible,
/// and provides additional context information
/// for better error messages.
#[derive(Debug, Clone, Default)]
pub struct stmtContext(pub Arc<Mutex<Option<u64>>>);

impl Display for stmtContext {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for stmtContext {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for stmtContext {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for stmtContext {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for stmtContext {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<stmtContext> for u64 {
    fn eq(&self, other: &stmtContext) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<stmtContext> for u64 {
    fn partial_cmp(&self, other: &stmtContext) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for stmtContext {
    type Output = stmtContext;
    fn add(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for stmtContext {
    type Output = stmtContext;
    fn add(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<stmtContext> for u64 {
    type Output = stmtContext;
    fn add(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for stmtContext {
    type Output = stmtContext;
    fn sub(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for stmtContext {
    type Output = stmtContext;
    fn sub(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<stmtContext> for u64 {
    type Output = stmtContext;
    fn sub(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for stmtContext {
    type Output = stmtContext;
    fn mul(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for stmtContext {
    type Output = stmtContext;
    fn mul(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<stmtContext> for u64 {
    type Output = stmtContext;
    fn mul(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for stmtContext {
    type Output = stmtContext;
    fn div(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for stmtContext {
    type Output = stmtContext;
    fn div(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<stmtContext> for u64 {
    type Output = stmtContext;
    fn div(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for stmtContext {
    type Output = stmtContext;
    fn rem(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for stmtContext {
    type Output = stmtContext;
    fn rem(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<stmtContext> for u64 {
    type Output = stmtContext;
    fn rem(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for stmtContext {
    type Output = stmtContext;
    fn bitand(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for stmtContext {
    type Output = stmtContext;
    fn bitand(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<stmtContext> for u64 {
    type Output = stmtContext;
    fn bitand(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for stmtContext {
    type Output = stmtContext;
    fn bitor(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for stmtContext {
    type Output = stmtContext;
    fn bitor(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<stmtContext> for u64 {
    type Output = stmtContext;
    fn bitor(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for stmtContext {
    type Output = stmtContext;
    fn bitxor(self, other: Self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for stmtContext {
    type Output = stmtContext;
    fn bitxor(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<stmtContext> for u64 {
    type Output = stmtContext;
    fn bitxor(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for stmtContext {
    type Output = stmtContext;
    fn not(self) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: i32) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: i8) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: i16) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: i64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: u32) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: u8) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: u16) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for stmtContext {
    type Output = stmtContext;
    fn shl(self, other: usize) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: stmtContext) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: i32) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: i8) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: i16) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: i64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: u32) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: u8) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: u16) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: u64) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for stmtContext {
    type Output = stmtContext;
    fn shr(self, other: usize) -> stmtContext {
        stmtContext(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for stmtContext {}

impl Ord for stmtContext {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A valueMap maps a case value (of a basic Go type) to a list of positions
/// where the same case value appeared, together with the corresponding case
/// types.
/// Since two case values may have the same "underlying" value but different
/// types we need to also check the value's types (e.g., byte(1) vs myByte(1))
/// when the switch expression is of interface type.
#[derive(Clone, Default)]
pub struct valueMap(pub Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn Any + Send + Sync>>, Arc<Mutex<Option<Vec<valueType>>>>>>>>);


/// A valueMap maps a case value (of a basic Go type) to a list of positions
/// where the same case value appeared, together with the corresponding case
/// types.
/// Since two case values may have the same "underlying" value but different
/// types we need to also check the value's types (e.g., byte(1) vs myByte(1))
/// when the switch expression is of interface type.
#[derive(Clone)]
pub struct valueType {
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl valueType {
    pub fn __go_value_clone(&self) -> Self {
        Self { pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone() }
    }
}


impl Default for valueType {
    fn default() -> Self {
        Self { pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), typ: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for valueType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.pos.lock().unwrap().as_ref().unwrap()), (*self.typ.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for valueType {
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


impl crate::check::Checker {
    /// decl may be nil
    pub fn func_body(&mut self, decl: Arc<Mutex<Option<declInfo>>>, name: Arc<Mutex<Option<String>>>, sig: Arc<Mutex<Option<Signature>>>, body: Arc<Mutex<Option<go_ast::r#mod::BlockStmt>>>, iota: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).ignore_func_bodies.lock().unwrap().as_ref().unwrap()) {
        std::panic::panic_any(Box::new("function body not ignored".to_string()) as Box<dyn Any + Send + Sync>);
    }
            if (*(*self.conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        self.trace({ let __recv = body.clone(); let __recv_ptr: *const go_ast::r#mod::BlockStmt = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BlockStmt }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, Arc::new(Mutex::new(Some("-- %s: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(sig.clone()) as Box<dyn Any + Send + Sync>]))));
    }
                        // save/restore current environment and set up function environment
                        // (and use 0 indentation at function start)
            let mut check_defer_captured = self.clone(); let __defer_arg_0 = Arc::new(Mutex::new(Some({ let __selector_holder = check_defer_captured.environment.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __defer_arg_1 = Arc::new(Mutex::new(Some({ let __selector_holder = check_defer_captured.indent.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); __defer_stack.push(Box::new(move || {
        (move |env: Arc<Mutex<Option<environment>>>, indent: Arc<Mutex<Option<i32>>>| {
        { let new_val = env.lock().unwrap().as_ref().unwrap().clone(); *check_defer_captured.environment.lock().unwrap() = Some(new_val); };;
        { let new_val = indent.lock().unwrap().as_ref().unwrap().clone(); *check_defer_captured.indent.lock().unwrap() = Some(new_val); };;
        })(__defer_arg_0, __defer_arg_1);
    }));
            { let new_val = environment { decl: decl.clone(), scope: { let __field = (*sig.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }, version: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.environment.lock().unwrap().as_ref().unwrap()).version.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), iota: iota.clone(), sig: sig.clone(), ..Default::default() }; *self.environment.lock().unwrap() = Some(new_val); };
                        // TODO(adonovan): would decl.version (if decl != nil) be better?
            { let new_val = 0; *self.indent.lock().unwrap() = Some(new_val); };
            self.stmt_list(Arc::new(Mutex::new(Some(stmtContext(Arc::new(Mutex::new(Some(0 as u64))))))), { let __field = (*body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });
            if (*(*self.environment.lock().unwrap().as_ref().unwrap()).has_label.clone().lock().unwrap().as_ref().unwrap()) {
        self.labels(body.clone());
    }
            if { let __tmp_x = (*(*sig.lock().unwrap().as_ref().unwrap()).results.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x > __tmp_y } && !self.is_terminating(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr(body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))), Arc::new(Mutex::new(Some("".to_string())))) {
        self.error(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*body.lock().unwrap().as_ref().unwrap()).rbrace.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISSING_RETURN as i32))))))), Arc::new(Mutex::new(Some("missing return".to_string()))));
    }
                        // spec: "Implementation restriction: A compiler may make it illegal to
                        // declare a variable inside a function body if the variable is never used."
            self.usage({ let __field = (*sig.lock().unwrap().as_ref().unwrap()).scope.clone(); __field });

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

    pub fn usage(&self, scope: Arc<Mutex<Option<Scope>>>) {
        let mut unused: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        for (name, mut elem) in { let __range_holder = (*scope.lock().unwrap().as_ref().unwrap()).elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        { let __iface_handle = resolve(Arc::new(Mutex::new(Some(name.clone()))), elem.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *elem.lock().unwrap() = (*__iface_guard).clone(); };
        {
        let (mut v, _) = ({
        let val = elem.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::VarPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Var>)), false)
        }
    });;
        if (*v.lock().unwrap()).is_some() && !(*{ let __field = (*v.lock().unwrap().as_ref().unwrap()).is_param.clone(); __field }.lock().unwrap().as_ref().unwrap()) && !{ let __map = { let __map_holder = self.used_vars.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(v.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
            { let new_val = { let __append_target = unused.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(v.clone()); __append_target.clone() }; unused = new_val; };;
        }
    }
    }
        slices::sort_func::<Vec<Arc<Mutex<Option<crate::object::Var>>>>, crate::object::Var>(unused.clone(), Arc::new(Mutex::new(Some(Box::new(move |a: Arc<Mutex<Option<Var>>>, b: Arc<Mutex<Option<Var>>>| -> i32 {
        cmp_pos(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*a.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*(*b.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))))
    }) as Box<dyn FnMut(Arc<Mutex<Option<Var>>>, Arc<Mutex<Option<Var>>>) -> i32 + Send + Sync>))));
        { let __range_holder = unused.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(v.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_VAR as i32))))))), Arc::new(Mutex::new(Some("declared and not used: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    } }
        { let __range_holder = (*scope.lock().unwrap().as_ref().unwrap()).children.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for scope in __range_values.iter() {
                // Don't go inside function literal scopes a second time;
                // they are handled explicitly by funcBody.
        if !(*{ let __field = (*scope.lock().unwrap().as_ref().unwrap()).is_func.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        self.usage((*scope).clone());
    }
    } }
    }

    pub fn simple_stmt(&mut self, s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) {
        if (*s.lock().unwrap()).is_some() {
        self.stmt(Arc::new(Mutex::new(Some(stmtContext(Arc::new(Mutex::new(Some(0 as u64))))))), s.clone());
    }
    }

    pub fn stmt_list(&mut self, ctxt: Arc<Mutex<Option<stmtContext>>>, mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) {
        let mut ok = Arc::new(Mutex::new(Some({ let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & FALLTHROUGH_OK as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y })));
        let mut inner = Arc::new(Mutex::new(Some(stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ! FALLTHROUGH_OK as u64))))))));
        { let new_val = trim_trailing_empty_stmts(list.clone()); list = new_val; };
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, s) in __range_values.iter().enumerate() {
        let mut inner = { let __owned = inner.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some(FALLTHROUGH_OK as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), s.clone());
    } }
    }

    pub fn multiple_defaults(&self, list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) {
        let mut first: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).is_some() {
        let c = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CaseClausePtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*c.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __iface_handle = s.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *d.lock().unwrap() = (*__iface_guard).clone(); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).is_some() {
        let c = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::CommClausePtr>()).unwrap().0.clone();
        if { let __iface_handle = { let __field = (*c.lock().unwrap().as_ref().unwrap()).comm.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = s.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *d.lock().unwrap() = (*__iface_guard).clone(); };
    };
    } else {
        let c = _ts_subject.clone();
        self.error(Arc::new(Mutex::new(Some(Box::new((*s.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("case/communication clause expected".to_string()))));;
    }
    }
        if (*d.lock().unwrap()).is_some() {
        if (*first.lock().unwrap()).is_some() {
        { let __method_arg0 = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))); let __method_arg1 = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_DEFAULT as i32))))))); let __method_arg2 = Arc::new(Mutex::new(Some("multiple defaults (first at %s)".to_string()))); self.errorf(__method_arg0, __method_arg1, __method_arg2, Arc::new(Mutex::new(Some(vec![Box::new({ let __v = (*self.fset.lock().unwrap().as_ref().unwrap()).position((*first.lock().unwrap().as_ref().unwrap()).pos()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>])))) };
    } else {
        { let __iface_handle = d.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *first.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
    } }
    }

    pub fn open_scope(&mut self, node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>, comment: Arc<Mutex<Option<String>>>) {
        let mut scope = new_scope({ let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }, (*node.lock().unwrap().as_ref().unwrap()).pos(), (*node.lock().unwrap().as_ref().unwrap()).end(), Arc::new(Mutex::new(Some({ let __arg_holder = comment.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        self.record_scope(node.clone(), scope.clone());
        { let new_val = scope.clone(); (*self.environment.lock().unwrap().as_mut().unwrap()).scope = new_val; };
    }

    pub fn close_scope(&mut self) {
        { let new_val = (*(*self.environment.lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).parent().clone(); (*self.environment.lock().unwrap().as_mut().unwrap()).scope = new_val; };
    }

    pub fn suspended_call(&mut self, keyword: Arc<Mutex<Option<String>>>, call: Arc<Mutex<Option<go_ast::r#mod::CallExpr>>>) {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let mut code: Arc<Mutex<Option<Code>>> = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0)))))));
        { let _switch_val = { let __v = self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false)))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (crate::expr::exprKind(Arc::new(Mutex::new(Some(CONVERSION as i32))))) {
            { let new_val = "requires function call, not conversion".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_DEFER as i32)))); *code.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = (*keyword.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "go".to_string(); __tmp_x == __tmp_y } {
        { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_GO as i32)))); *code.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))) {
            { let new_val = "discards result of".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_RESULTS as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))) {
            return;
        } else {
            std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);
        }
    }
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("%s %s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = keyword.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
    }

    pub fn case_values(&mut self, x: Arc<Mutex<Option<operand>>>, values: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, seen: Arc<Mutex<Option<valueMap>>>) {
        { let __range_holder = values.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); 'l: for e in __range_values.iter() {
        let mut v: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), v.clone(), e.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        continue 'l
    }
        self.convert_untyped(v.clone(), (*x.lock().unwrap().as_ref().unwrap()).typ.clone());
        if { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        continue 'l
    }
                // Order matters: By comparing v against x, error positions are at the case values.
        let mut res = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        self.comparison(res.clone(), x.clone(), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32))))))), Arc::new(Mutex::new(Some(true))));
        if { let __tmp_x = { let __selector_holder = (*res.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        continue 'l
    }
        if { let __tmp_x = { let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); __tmp_x != __tmp_y } {
        continue 'l
    }
                // we're done
                // look for duplicate values
        {
        let mut val = go_val((*v.lock().unwrap().as_ref().unwrap()).val.clone());;
        if (*val.lock().unwrap()).is_some() {
            { let __range_holder = { let __map = { let __map_holder = { let __named_map = (*seen.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(val.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for vt in __range_values.iter() {
        if identical((*v.lock().unwrap().as_ref().unwrap()).typ.clone(), vt.typ.clone()) {
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_CASE as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(v.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate case %s in expression switch".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(v.clone().clone()) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = vt.pos.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("previous case".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
        continue 'l
    }
    } };
            { let __map_key = GoLocalPtrKey::new(val.clone()); let __map_value = { let __slice = { let __map_holder = { let __named_map = (*seen.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }; let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&GoLocalPtrKey::new(val.clone())).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; (*__slice.lock().unwrap()).get_or_insert_with(Vec::new).push(valueType { pos: (*v.lock().unwrap().as_ref().unwrap()).pos(), typ: { let __field = (*v.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }, ..Default::default() }); __slice.clone() }; (*{ let __named_map = (*seen.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    } }
    }

    /// isNil reports whether the expression e denotes the predeclared value nil.
    pub fn is_nil(&self, e: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>) -> bool {
                // The only way to express the nil value is by literally writing nil (possibly in parentheses).
        {
        let (mut name, _) = ({
        let val = go_ast::unparen(e.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*name.lock().unwrap()).is_some() {
            let (_, mut ok) = ({
        let val = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::NilPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Nil>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Nil>)), false)
        }
    });;
            return ok;;
        }
    }
        false
    }

    /// caseTypes typechecks the type expressions of a type case, checks for duplicate types
    /// using the seen map, and verifies that each type is valid with respect to the type of
    /// the operand x corresponding to the type switch expression. If that expression is not
    /// valid, x must be nil.
    ///
    ///	switch <x>.(type) {
    ///	case <types>: ...
    ///	...
    ///	}
    ///
    /// caseTypes returns the case-specific type for a variable v introduced through a short
    /// variable declaration by the type switch:
    ///
    ///	switch v := <x>.(type) {
    ///	case <types>: // T is the type of <v> in this case
    ///	...
    ///	}
    ///
    /// If there is exactly one type expression, T is the type of that expression. If there
    /// are multiple type expressions, or if predeclared nil is among the types, the result
    /// is the type of x. If x is invalid (nil), the result is the invalid type.
    pub fn case_types(&mut self, x: Arc<Mutex<Option<operand>>>, types: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, seen: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut dummy: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = types.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); 'l: for e in __range_values.iter() {
                // The spec allows the value nil instead of a type.
        if self.is_nil(e.clone()) {
        *T.lock().unwrap() = None;
        self.expr(Arc::new(Mutex::new(None)), dummy.clone(), e.clone());
    } else {
        { let __iface_handle = self.var_type(e.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
        if !is_valid(T.clone()) {
        continue 'l
    }
    }
                // run e through expr so we get the usual Info recordings
                // look for duplicate types
                // (quadratic algorithm, but type switches tend to be reasonably small)
        for (__range_key, other) in { let __range_holder = seen.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let t = __range_key.value();
        if (*T.lock().unwrap()).is_none() && (*t.lock().unwrap()).is_none() || (*T.lock().unwrap()).is_some() && (*t.lock().unwrap()).is_some() && identical(T.clone(), t.clone()) {
                // talk about "case" rather than "type" because of nil case
        let mut Ts = Arc::new(Mutex::new(Some("nil".to_string())));
        if (*T.lock().unwrap()).is_some() {
        { let new_val = type_string(T.clone(), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *Ts.lock().unwrap() = __moved_val; };
    }
        let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_CASE as i32))))))));
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate case %s in type switch".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = Ts.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new((*other.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("previous case".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };
        { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };
        continue 'l
    }
    }
                // talk about "case" rather than "type" because of nil case
        { let __map_key = GoLocalPtrKey::new(T.clone()); let __map_value = (*e).clone(); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (*x.lock().unwrap()).is_some() && (*T.lock().unwrap()).is_some() {
        self.type_assertion(e.clone(), x.clone(), T.clone(), Arc::new(Mutex::new(Some(true))));
    }
    } }
                // The spec allows the value nil instead of a type.
                // run e through expr so we get the usual Info recordings
                // look for duplicate types
                // (quadratic algorithm, but type switches tend to be reasonably small)
                // talk about "case" rather than "type" because of nil case
                // spec: "In clauses with a case listing exactly one type, the variable has that type;
                // otherwise, the variable has the type of the expression in the TypeSwitchGuard.
        if { let __tmp_x = ((*types.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } || (*T.lock().unwrap()).is_none() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
        if (*x.lock().unwrap()).is_some() {
        { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
        assert(Arc::new(Mutex::new(Some((*T.lock().unwrap()).is_some()))));
        return T.clone();
    }

    /// TODO(gri) Once we are certain that typeHash is correct in all situations, use this version of caseTypes instead.
    /// (Currently it may be possible that different types have identical names and import paths due to ImporterFrom.)
    pub fn case_types_currently_unused(&mut self, x: Arc<Mutex<Option<operand>>>, xtyp: Arc<Mutex<Option<Interface>>>, types: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>, seen: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut T: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        let mut dummy: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let __range_holder = types.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); 'l: for e in __range_values.iter() {
                // The spec allows the value nil instead of a type.
        let mut hash: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if self.is_nil(e.clone()) {
        self.expr(Arc::new(Mutex::new(None)), dummy.clone(), e.clone());
        *T.lock().unwrap() = None;
        { let new_val = "<nil>".to_string(); *hash.lock().unwrap() = Some(new_val); };
    } else {
        { let __iface_handle = self.var_type(e.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
        if !is_valid(T.clone()) {
        continue 'l
    }
        std::panic::panic_any(Box::new("enable typeHash(T, nil)".to_string()) as Box<dyn Any + Send + Sync>);
    }
                // run e through expr so we get the usual Info recordings
                // avoid collision with a type named nil
                // hash = typeHash(T, nil)
                // look for duplicate types
        {
        let mut other = { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*hash.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*other.lock().unwrap()).is_some() {
            let mut Ts = Arc::new(Mutex::new(Some("nil".to_string())));;
            if (*T.lock().unwrap()).is_some() {
        { let new_val = type_string(T.clone(), Arc::new(Mutex::new(Some({ let mut __recv = self.clone(); Box::new(move |__arg0: Arc<Mutex<Option<crate::package::Package>>>| -> Arc<Mutex<Option<String>>> { __recv.qualifier(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<crate::package::Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *Ts.lock().unwrap() = __moved_val; };
    };
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(DUPLICATE_CASE as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new((*e.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("duplicate case %s in type switch".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = Ts.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = other.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("previous case".to_string()))), Arc::new(Mutex::new(Some(vec![])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
            continue 'l;
        }
    }
                // talk about "case" rather than "type" because of nil case
        { let __map_key = (*hash.lock().unwrap().as_ref().unwrap()).clone(); let __map_value = (*e).clone(); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if (*T.lock().unwrap()).is_some() {
        self.type_assertion(e.clone(), x.clone(), T.clone(), Arc::new(Mutex::new(Some(true))));
    }
    } }
                // The spec allows the value nil instead of a type.
                // run e through expr so we get the usual Info recordings
                // avoid collision with a type named nil
                // hash = typeHash(T, nil)
                // look for duplicate types
                // talk about "case" rather than "type" because of nil case
                // spec: "In clauses with a case listing exactly one type, the variable has that type;
                // otherwise, the variable has the type of the expression in the TypeSwitchGuard.
        if { let __tmp_x = ((*types.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } || (*T.lock().unwrap()).is_none() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
        if (*x.lock().unwrap()).is_some() {
        { let __iface_handle = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *T.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }
        assert(Arc::new(Mutex::new(Some((*T.lock().unwrap()).is_some()))));
        return T.clone();
    }

    /// stmt typechecks statement s.
    pub fn stmt(&mut self, ctxt: Arc<Mutex<Option<stmtContext>>>, mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut s: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(s.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Stmt::__go_clone_box_stmt(__v.as_ref()))));
        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // statements must end with the same top scope as they started with
            if DEBUG {
        let mut check_defer_captured = self.clone(); let __defer_arg_0 = (*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __defer_stack.push(Box::new(move || {
        (move |scope: Arc<Mutex<Option<Scope>>>| {
        {
        let mut p = go_recover();;
        if (*p.lock().unwrap()).is_some() {
            std::panic::panic_any({ let __any_holder = p.clone(); let __any_guard = __any_holder.lock().unwrap(); go_any_clone(__any_guard.as_ref().expect("nil interface in variadic any argument").as_ref()) });;
        }
    };
        assert(Arc::new(Mutex::new(Some({ let __left = scope.clone(); let __right = (*check_defer_captured.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq }))));;
        })(__defer_arg_0);
    }));
    }
                        // don't check if code is panicking
                        // process collected function literals before scope changes
            let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __method_arg0 = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = check_defer_captured.delayed.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32))); check_defer_captured.process_delayed(__method_arg0) };
    }));
                        // reset context for statements of inner blocks
            let mut inner = Arc::new(Mutex::new(Some(stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ! (((FALLTHROUGH_OK as u64 | FINAL_SWITCH_CASE as u64) | IN_TYPE_SWITCH as u64))))))))));
            {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::EmptyStmtPtr>()).is_some() {
        let s = _ts_subject.clone();
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeclStmtPtr>()).unwrap().0.clone();
        self.decl_stmt((*s.lock().unwrap().as_ref().unwrap()).decl.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::LabeledStmtPtr>()).unwrap().0.clone();
        { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_label.lock().unwrap() = Some(new_val); };;
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = ctxt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*s.lock().unwrap().as_ref().unwrap()).stmt.clone());;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        let mut kind = self.raw_expr(Arc::new(Mutex::new(None)), x.clone(), (*s.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(false))));;
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));;
        let mut code: Arc<Mutex<Option<Code>>> = Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(0)))))));;
        { let _switch_val = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(BUILTIN as u8))))) {
            { let new_val = "must be called".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNCALLED_BUILTIN as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (crate::operand::operandMode(Arc::new(Mutex::new(Some(TYPEXPR as u8))))) {
            { let new_val = "is not an expression".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NOT_AN_EXPR as i32)))); *code.lock().unwrap() = Some(new_val); };
        } else {
            if { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            { let new_val = "is not used".to_string(); *msg.lock().unwrap() = Some(new_val); };
            { let new_val = internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_EXPR as i32)))); *code.lock().unwrap() = Some(new_val); };
        }
    };
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = code.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("%s %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SendStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SendStmtPtr>()).unwrap().0.clone();
        let mut ch: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));let mut val: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.expr(Arc::new(Mutex::new(None)), ch.clone(), (*s.lock().unwrap().as_ref().unwrap()).chan.clone());;
        self.expr(Arc::new(Mutex::new(None)), val.clone(), (*s.lock().unwrap().as_ref().unwrap()).value.clone());;
        if { let __tmp_x = { let __selector_holder = (*ch.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*val.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        let mut u = core_type((*ch.lock().unwrap().as_ref().unwrap()).typ.clone());;
        if (*u.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SendStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).arrow.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SEND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot send to %s: no core type".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(ch.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        let (mut uch, _) = ({
        let val = u.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::chan::ChanPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::chan::Chan>)), false)
        }
    });;
        if (*uch.lock().unwrap()).is_none() {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SendStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).arrow.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SEND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot send to non-channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(ch.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        if { let __tmp_x = { let __selector_holder = (*uch.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(RECV_ONLY as i32)))); __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SendStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).arrow.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SEND as i32))))))), Arc::new(Mutex::new(Some("invalid operation: cannot send to receive-only channel %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(ch.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        self.assignment(val.clone(), (*uch.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some("send".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IncDecStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IncDecStmtPtr>()).unwrap().0.clone();
        let mut op: Arc<Mutex<Option<go_token::r#mod::Token>>> = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(0)))))));;
        { let _switch_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_C as i32))))) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32)))); *op.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_C as i32))))) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32)))); *op.lock().unwrap() = Some(new_val); };
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IncDecStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown inc/dec operation %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
            {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
        }
    };
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*s.lock().unwrap().as_ref().unwrap()).x.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        if !all_numeric((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*(*s.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NON_NUMERIC_INC_DEC as i32))))))), Arc::new(Mutex::new(Some("invalid operation: %s%s (non-numeric type %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        let mut Y = Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { value_pos: (*(*s.lock().unwrap().as_ref().unwrap()).x.lock().unwrap().as_ref().unwrap()).pos(), kind: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32))))))), value: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() })));;
        self.binary(x.clone(), Arc::new(Mutex::new(None)), (*s.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(Y.clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        self.assign_var((*s.lock().unwrap().as_ref().unwrap()).x.clone(), Arc::new(Mutex::new(None)), x.clone(), Arc::new(Mutex::new(Some("assignment".to_string()))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        { let _switch_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32))))) {
            if { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("missing lhs in assignment".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            if { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
        self.short_var_decl(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), { let __field = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }, { let __field = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field });
    } else {
        self.assign_vars({ let __field = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }, { let __field = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field });
    }
        } else {
            if { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } || { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new((*in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MULTI_VAL_ASSIGN_OP as i32))))))), Arc::new(Mutex::new(Some("assignment operation %s requires single-valued expressions".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            let mut op = assign_op(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).tok.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))));
            if { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_L_L_E_G_A_L as i32)))); __tmp_x == __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::errors::atPos(Arc::new(Mutex::new(Some({ let __named_value_holder = (*s.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); let __named_value_guard = __named_value_holder.lock().unwrap(); let __cloned = (*__named_value_guard.as_ref().unwrap()).clone(); drop(__named_value_guard); __cloned }))))) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("unknown assignment operation %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
            self.binary(x.clone(), Arc::new(Mutex::new(None)), { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = op.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x == __tmp_y } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
            self.assign_var({ let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(), Arc::new(Mutex::new(None)), x.clone(), Arc::new(Mutex::new(Some("assignment".to_string()))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GoStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GoStmtPtr>()).unwrap().0.clone();
        self.suspended_call(Arc::new(Mutex::new(Some("go".to_string()))), { let __field = (*s.lock().unwrap().as_ref().unwrap()).call.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeferStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::DeferStmtPtr>()).unwrap().0.clone();
        self.suspended_call(Arc::new(Mutex::new(Some("defer".to_string()))), { let __field = (*s.lock().unwrap().as_ref().unwrap()).call.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ReturnStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ReturnStmtPtr>()).unwrap().0.clone();
        let mut res = (*(*self.environment.lock().unwrap().as_ref().unwrap()).sig.lock().unwrap().as_ref().unwrap()).results.clone();;
        if { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).results.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = { let __recv = res.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __selector_holder = (*{ let __seq = { let __seq_holder = (*res.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let __range_holder = (*res.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        {
        let mut alt = { let __promoted_recv = self.environment.clone(); let __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_ref().unwrap(); let __result = __promoted_ref.lookup(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); __result };;
        if (*alt.lock().unwrap()).is_some() && { let __left_holder = alt.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::object::VarPtr(obj.clone()); let __right_opt: Option<&(dyn Object + Send + Sync)> = Some(&__right_wrapper as &(dyn Object + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; !__eq } {
            let mut err = self.new_error(Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(OUT_OF_SCOPE_RESULT as i32))))))));;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ReturnStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("result parameter %s not in scope at return".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.addf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = alt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some("inner declaration of %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(obj.clone()) as Box<dyn Any + Send + Sync>])))); __result };;
            { let __recv = err.clone(); let __recv_ptr: *mut crate::errors::error_ = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::errors::error_ }; let __result = unsafe { &mut *__recv_ptr }.report(); __result };;
        }
    }
    } }
    } else {
        let mut lhs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = { let __recv = res.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = (*res.lock().unwrap().as_ref().unwrap()).vars.clone(); lhs = new_val; };
    }
        self.init_vars(lhs.clone(), { let __field = (*s.lock().unwrap().as_ref().unwrap()).results.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ReturnStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))));
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BranchStmtPtr>()).unwrap().0.clone();
        if { let __nil_target = (*s.lock().unwrap().as_ref().unwrap()).label.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = true; *(*self.environment.lock().unwrap().as_ref().unwrap()).has_label.lock().unwrap() = Some(new_val); };
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        { let _switch_val = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32))))) {
            if { let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & BREAK_OK as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_BREAK as i32))))))), Arc::new(Mutex::new(Some("break not in for, switch, or select statement".to_string()))));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32))))) {
            if { let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & CONTINUE_OK as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_CONTINUE as i32))))))), Arc::new(Mutex::new(Some("continue not in for statement".to_string()))));
    }
        } else if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32))))) {
            if { let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & FALLTHROUGH_OK as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        let mut msg: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        if { let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & FINAL_SWITCH_CASE as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
            { let new_val = "cannot fallthrough final case in switch".to_string(); *msg.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = stmtContext(Arc::new(Mutex::new(Some(((*{ let __v = (*ctxt.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & IN_TYPE_SWITCH as u64))))); let __tmp_y = stmtContext(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
            { let new_val = "cannot fallthrough in type switch".to_string(); *msg.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = "fallthrough statement out of place".to_string(); *msg.lock().unwrap() = Some(new_val); };
        }
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(MISPLACED_FALLTHROUGH as i32))))))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BranchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("branch statement: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).unwrap().0.clone();
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("block".to_string()))));;
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));;
        self.stmt_list(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*s.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).unwrap().0.clone();
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IfStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("if".to_string()))));;
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));;
        self.simple_stmt((*s.lock().unwrap().as_ref().unwrap()).init.clone());;
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*s.lock().unwrap().as_ref().unwrap()).cond.clone());;
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && !all_boolean((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*s.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COND as i32))))))), Arc::new(Mutex::new(Some("non-boolean condition in if statement".to_string()))));
    };
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))));;
        {
    let _ts_subject = (*s.lock().unwrap().as_ref().unwrap()).r#else.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BadStmtPtr>()).is_some() {
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::IfStmtPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::BlockStmtPtr>()).is_some() {
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*s.lock().unwrap().as_ref().unwrap()).r#else.clone());;
    } else {
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*s.lock().unwrap().as_ref().unwrap()).r#else.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("invalid else branch in if statement".to_string()))));;
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SwitchStmtPtr>()).unwrap().0.clone();
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some(BREAK_OK as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };;
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SwitchStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("switch".to_string()))));;
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));;
        self.simple_stmt((*s.lock().unwrap().as_ref().unwrap()).init.clone());;
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));;
        if { let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).tag.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*s.lock().unwrap().as_ref().unwrap()).tag.clone());
        self.assignment(x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("switch expression".to_string()))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && !comparable((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) && !has_nil((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_EXPR_SWITCH as i32))))))), Arc::new(Mutex::new(Some("cannot switch on %s (%s is not comparable)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(CONSTANT_ as u8)))); *(*x.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(BOOL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = go_constant::make_bool(Arc::new(Mutex::new(Some(true)))).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).val.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { name_pos: Arc::new(Mutex::new(Some({ let __selector_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).lbrace.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), name: Arc::new(Mutex::new(Some("true".to_string()))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*x.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
    };
        self.multiple_defaults({ let __field = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
        let mut seen = Arc::new(Mutex::new(Some(valueMap(Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn Any + Send + Sync>>, Arc<Mutex<Option<Vec<valueType>>>>>::new())))))));;
        { let __range_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, c) in __range_values.iter().enumerate() {
        let (mut clause, _) = ({
        let val = c.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CaseClausePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CaseClause>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CaseClause>)), false)
        }
    });
        if (*clause.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new((*c.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect expression switch case".to_string()))));
        continue
    }
        self.case_values(x.clone(), { let __field = (*clause.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, seen.clone());
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CaseClausePtr(clause.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("case".to_string()))));
        let mut inner = { let __owned = inner.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = ({ let __tmp_x = i as i32; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = (({ let __len_target = { let __field = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some(FALLTHROUGH_OK as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    } else {
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some(FINAL_SWITCH_CASE as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
        self.stmt_list(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*clause.lock().unwrap().as_ref().unwrap()).body.clone(); __field });
        self.close_scope();
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::TypeSwitchStmtPtr>()).unwrap().0.clone();
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some((BREAK_OK as u64 | IN_TYPE_SWITCH as u64) as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };;
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("type switch".to_string()))));;
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));;
        self.simple_stmt((*s.lock().unwrap().as_ref().unwrap()).init.clone());;
        let mut lhs: Arc<Mutex<Option<go_ast::r#mod::Ident>>> = Arc::new(Mutex::new(None));;
        let mut rhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));;
        {
    let _ts_subject = (*s.lock().unwrap().as_ref().unwrap()).assign.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).is_some() {
        let guard = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        { let __iface_handle = (*guard.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *rhs.lock().unwrap() = (*__iface_guard).clone(); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let guard = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*guard.lock().unwrap().as_ref().unwrap()).lhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } || { let __tmp_x = { let __selector_holder = (*guard.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x != __tmp_y } || { let __tmp_x = (({ let __len_target = { let __field = (*guard.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x != __tmp_y } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect form of type switch guard".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        { let (__tmp_0, __tmp_1) = ({
        let val = { let __seq = { let __seq_holder = (*guard.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    }); lhs = __tmp_0.clone(); };;
        if (*lhs.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect form of type switch guard".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        if { let __tmp_x = { let __selector_holder = (*lhs.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(lhs.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NO_NEW_VAR as i32))))))), Arc::new(Mutex::new(Some("no new variable on left side of :=".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        *lhs.lock().unwrap() = None;
    } else {
        self.record_def(lhs.clone(), Arc::new(Mutex::new(None)));
    };
        { let __iface_handle = { let __seq = { let __seq_holder = (*guard.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *rhs.lock().unwrap() = (*__iface_guard).clone(); };;
    } else {
        let guard = _ts_subject.clone();
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect form of type switch guard".to_string()))));;
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    };
    }
    };
        let (mut expr, _) = ({
        let val = rhs.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::TypeAssertExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::TypeAssertExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::TypeAssertExpr>)), false)
        }
    });;
        if (*expr.lock().unwrap()).is_none() || { let __iface_handle = { let __field = (*expr.lock().unwrap().as_ref().unwrap()).r#type.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.error(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::TypeSwitchStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect form of type switch guard".to_string()))));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    };
        let mut sx: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(None));;
        {
    let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
    self.expr(Arc::new(Mutex::new(None)), x.clone(), (*expr.lock().unwrap().as_ref().unwrap()).x.clone());
    if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
        if is_type_param((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_SWITCH as i32))))))), Arc::new(Mutex::new(Some("cannot use type switch on type parameter value %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
    } else if is_interface((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        { let new_val = x.clone().clone(); sx = new_val; };
    } else {
        self.errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_TYPE_SWITCH as i32))))))), Arc::new(Mutex::new(Some("%s is not an interface".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
    }
    }
};
        self.multiple_defaults({ let __field = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
        let mut lhsVars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));;
        let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Box<dyn Type + Send + Sync>>, Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>::new())));;
        { let __range_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for s in __range_values.iter() {
        let (mut clause, _) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CaseClausePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CaseClause>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CaseClause>)), false)
        }
    });
        if (*clause.lock().unwrap()).is_none() {
        self.error(Arc::new(Mutex::new(Some(Box::new((*s.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("incorrect type switch case".to_string()))));
        continue
    }
        let mut T = self.case_types(sx.clone(), { let __field = (*clause.lock().unwrap().as_ref().unwrap()).list.clone(); __field }, seen.clone());
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CaseClausePtr(clause.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("case".to_string()))));
        if (*lhs.lock().unwrap()).is_some() {
        let mut obj = new_var({ let __recv = lhs.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = (*lhs.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), T.clone());
        { let __method_arg0 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __selector_holder = (*clause.lock().unwrap().as_ref().unwrap()).colon.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
        self.record_implicit(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CaseClausePtr(clause.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
        { let new_val = { let __append_target = lhsVars.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(obj.clone()); __append_target.clone() }; lhsVars = new_val; };
    }
        self.stmt_list(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*clause.lock().unwrap().as_ref().unwrap()).body.clone(); __field });
        self.close_scope();
    } };
        if (*lhs.lock().unwrap()).is_some() {
        let mut used: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let __range_holder = lhsVars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for v in __range_values.iter() {
        if { let __map = { let __map_holder = self.used_vars.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(v.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| false) } {
        { let new_val = true; *used.lock().unwrap() = Some(new_val); };
    }
        { let __map_key = GoLocalPtrKey::new(v.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
        if !{ let __v = (*used.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(lhs.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(UNUSED_VAR as i32))))))), Arc::new(Mutex::new(Some("%s declared and not used".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*lhs.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).is_some() {
        let mut s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SelectStmtPtr>()).unwrap().0.clone();
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some(BREAK_OK as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };;
        self.multiple_defaults({ let __field = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); __field });;
        { let __range_holder = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).list.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut s in __range_values.iter().cloned() {
        let (mut clause, _) = ({
        let val = s.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::CommClausePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::CommClause>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::CommClause>)), false)
        }
    });
        if (*clause.lock().unwrap()).is_none() {
        continue
    }
        let mut valid = Arc::new(Mutex::new(Some(false)));
        let mut rhs: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = (*clause.lock().unwrap().as_ref().unwrap()).comm.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil || _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::SendStmtPtr>()).is_some() {
        let s = _ts_subject.clone();
        { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::AssignStmtPtr>()).unwrap().0.clone();
        if { let __tmp_x = (({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = (*s.lock().unwrap().as_ref().unwrap()).rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *rhs.lock().unwrap() = (*__iface_guard).clone(); };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        { let __iface_handle = (*s.lock().unwrap().as_ref().unwrap()).x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *rhs.lock().unwrap() = (*__iface_guard).clone(); };;
    }
    }
        if (*rhs.lock().unwrap()).is_some() {
        {
        let (mut x, _) = ({
        let val = go_ast::unparen(rhs.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::UnaryExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::UnaryExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::UnaryExpr>)), false)
        }
    });;
        if (*x.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))); __tmp_x == __tmp_y } {
            { let new_val = true; *valid.lock().unwrap() = Some(new_val); };;
        }
    }
    }
        if !{ let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*clause.lock().unwrap().as_ref().unwrap()).comm.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SELECT_CASE as i32))))))), Arc::new(Mutex::new(Some("select case must be send or receive (possibly with assignment)".to_string()))));
        continue
    }
        self.open_scope(Arc::new(Mutex::new(Some(Box::new((*s.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("case".to_string()))));
        if { let __iface_handle = { let __field = (*clause.lock().unwrap().as_ref().unwrap()).comm.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), (*clause.lock().unwrap().as_ref().unwrap()).comm.clone());
    }
        self.stmt_list(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*clause.lock().unwrap().as_ref().unwrap()).body.clone(); __field });
        self.close_scope();
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).is_some() {
        let mut s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ForStmtPtr>()).unwrap().0.clone();
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some((BREAK_OK as u64 | CONTINUE_OK as u64) as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };;
        self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ForStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("for".to_string()))));;
        let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));;
        self.simple_stmt((*s.lock().unwrap().as_ref().unwrap()).init.clone());;
        if { let __iface_handle = { let __field = (*s.lock().unwrap().as_ref().unwrap()).cond.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        self.expr(Arc::new(Mutex::new(None)), x.clone(), (*s.lock().unwrap().as_ref().unwrap()).cond.clone());
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && !all_boolean((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.error(Arc::new(Mutex::new(Some(Box::new((*(*s.lock().unwrap().as_ref().unwrap()).cond.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_COND as i32))))))), Arc::new(Mutex::new(Some("non-boolean condition in for statement".to_string()))));
    }
    };
        self.simple_stmt((*s.lock().unwrap().as_ref().unwrap()).post.clone());;
        {
        let (mut s, _) = ({
        let val = (*s.lock().unwrap().as_ref().unwrap()).post.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::AssignStmtPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::AssignStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::AssignStmt>)), false)
        }
    });;
        if (*s.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y } {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::AssignStmtPtr(s.clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_POST_DECL as i32))))))), Arc::new(Mutex::new(Some("cannot declare in post statement".to_string()))), Arc::new(Mutex::new(Some(vec![]))));;
            self.r#use((*s.lock().unwrap().as_ref().unwrap()).lhs.clone());;
        }
    };
        self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::RangeStmtPtr>()).unwrap().0.clone();
        { let __rhs = stmtContext(Arc::new(Mutex::new(Some((BREAK_OK as u64 | CONTINUE_OK as u64) as u64)))); let mut guard = inner.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };;
        self.range_stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), s.clone());;
    } else {
        let s = _ts_subject.clone();
        self.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("invalid statement".to_string()))));;
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

    pub fn range_stmt(&mut self, inner: Arc<Mutex<Option<stmtContext>>>, s: Arc<Mutex<Option<go_ast::r#mod::RangeStmt>>>) {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let __go_previous_panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        // Convert go/ast form to local variables.
            type Expr = Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>;
            type identType = Arc<Mutex<Option<go_ast::r#mod::Ident>>>;
            let mut identName = Arc::new(Mutex::new(Some(Box::new(move |n: Arc<Mutex<Option<go_ast::r#mod::Ident>>>| -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    }) as Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
            let (mut sKey, mut sValue) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).value.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
            let mut sExtra: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(None));
            let mut isDef = Arc::new(Mutex::new(Some({ let __tmp_x = { let __selector_holder = (*s.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))); __tmp_x == __tmp_y })));
            let mut rangeVar = (*s.lock().unwrap().as_ref().unwrap()).x.clone();
            let mut noNewVarPos = in_node(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::RangeStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some((*(*(*s.lock().unwrap().as_ref().unwrap()).tok_pos.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap())))))))));
                        // Everything from here on is shared between cmd/compile/internal/types2 and go/types.
                        // check expression to iterate over
            let mut x: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
            self.expr(Arc::new(Mutex::new(None)), x.clone(), rangeVar.clone());
                        // determine key/value types
            let mut key: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));let mut val: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
            if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } {
                // Ranging over a type parameter is permitted if it has a core type.
        let mut check_closure_clone = (*self).clone(); let (mut k, mut v, mut cause, mut ok) = range_key_val((*x.lock().unwrap().as_ref().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(Box::new(move |v: Arc<Mutex<Option<goVersion>>>| -> bool {
        check_closure_clone.allow_version(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }) as Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync>))));
        if !ok && { let __tmp_x = (*cause.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RANGE_EXPR as i32))))))), Arc::new(Mutex::new(Some("cannot range over %s: %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = cause.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
        } else if !ok {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new(crate::operand::operandPtr(x.clone().clone())) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RANGE_EXPR as i32))))))), Arc::new(Mutex::new(Some("cannot range over %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        } else if (*k.lock().unwrap()).is_none() && (*sKey.lock().unwrap()).is_some() {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = sKey.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ITER_VAR as i32))))))), Arc::new(Mutex::new(Some("range over %s permits no iteration variables".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        } else if (*v.lock().unwrap()).is_none() && (*sValue.lock().unwrap()).is_some() {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = sValue.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ITER_VAR as i32))))))), Arc::new(Mutex::new(Some("range over %s permits only one iteration variable".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone().clone()) as Box<dyn Any + Send + Sync>]))));
        } else if (*sExtra.lock().unwrap()).is_some() {
            self.soft_errorf(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = sExtra.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_ITER_VAR as i32))))))), Arc::new(Mutex::new(Some("range clause permits at most two iteration variables".to_string()))), Arc::new(Mutex::new(Some(vec![]))));
        }
        { let __tmp_0 = k.clone(); let __tmp_1 = v.clone(); { let __iface_handle = __tmp_0; let __iface_guard = __iface_handle.lock().unwrap(); *key.lock().unwrap() = (*__iface_guard).clone(); } { let __iface_handle = __tmp_1; let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); } };
    }
                        // Ranging over a type parameter is permitted if it has a core type.
                        // Open the for-statement block scope now, after the range clause.
                        // Iteration variables declared with := need to go in this scope (was go.dev/issue/51437).
            self.open_scope(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::RangeStmtPtr(s.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some("range".to_string()))));
            let mut check_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        check_defer_captured.close_scope();
    }));
                        // check assignment to/declaration of iteration variables
                        // (irregular assignment, cannot easily map to existing assignment checks)
                        // lhs expressions and initialization value (rhs) types
            let mut lhs = Arc::new(Mutex::new(Some([sKey.clone(), sValue.clone()])));
            let mut rhs = Arc::new(Mutex::new(Some([key.clone(), val.clone()])));
            let mut rangeOverInt = is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone());
            if { let __v = (*isDef.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // short variable declaration
        let mut vars: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Var>>>>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        if (*lhs.lock().unwrap()).is_none() {
        continue
    }
                // determine lhs variable
        let mut obj: Arc<Mutex<Option<Var>>> = Arc::new(Mutex::new(None));
        {
        let (mut ident, _) = ({
        let val = lhs.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if (*ident.lock().unwrap()).is_some() {
            let mut name = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = identName.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_ast::r#mod::Ident>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(ident.clone()) };;
            { let new_val = new_var({ let __recv = ident.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }, { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None))).clone(); obj = new_val; };;
            self.record_def(ident.clone(), Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))));;
            if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = vars.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(obj.clone()); __append_target.clone() }; vars = new_val; };
    };
        } else {
            self.errorf(Arc::new(Mutex::new(Some(Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_SYNTAX_TREE as i32))))))), Arc::new(Mutex::new(Some("cannot declare %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(lhs.clone()) as Box<dyn Any + Send + Sync>]))));;
            { let new_val = new_var((*lhs.lock().unwrap().as_ref().unwrap()).pos(), { let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some("_".to_string()))), Arc::new(Mutex::new(None))).clone(); obj = new_val; };;
        }
    }
                // declare new variable
                // _ variables don't count as new variables
                // dummy variable
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
                // initialize lhs iteration variable, if any
        let mut typ = { let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
        if (*typ.lock().unwrap()).is_none() || { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
                // typ == Typ[Invalid] can happen if allowVersion fails.
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        { let __map_key = GoLocalPtrKey::new(obj.clone()); let __map_value = Arc::new(Mutex::new(Some(true))); (*self.used_vars.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        continue
    }
                // typ == Typ[Invalid] can happen if allowVersion fails.
                // don't complain about unused variable
        if rangeOverInt {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y }))));
        self.init_var(obj.clone(), x.clone(), Arc::new(Mutex::new(Some("range clause".to_string()))));
    } else {
        let mut y: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*y.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = lhs.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*y.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*y.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.init_var(obj.clone(), y.clone(), Arc::new(Mutex::new(Some("assignment".to_string()))));
    }
                // at most one iteration variable (rhs[1] == nil or Typ[Invalid] for rangeOverInt)
                // we don't have a better rhs expression to use here
                // error is on variable, use "assignment" not "range clause"
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
    } }
                // determine lhs variable
                // declare new variable
                // _ variables don't count as new variables
                // dummy variable
                // initialize lhs iteration variable, if any
                // typ == Typ[Invalid] can happen if allowVersion fails.
                // don't complain about unused variable
                // at most one iteration variable (rhs[1] == nil or Typ[Invalid] for rangeOverInt)
                // we don't have a better rhs expression to use here
                // error is on variable, use "assignment" not "range clause"
                // declare variables
        if { let __tmp_x = ((*vars.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut scopePos = (*(*s.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).pos();
        { let __range_holder = vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for obj in __range_values.iter() {
        { let __method_arg0 = { let __field = (*self.environment.lock().unwrap().as_ref().unwrap()).scope.clone(); __field }; let __method_arg1 = Arc::new(Mutex::new(None)); let __method_arg2 = Arc::new(Mutex::new(Some(Box::new(crate::object::VarPtr(obj.clone())) as Box<dyn Object + Send + Sync>))); let __method_arg3 = Arc::new(Mutex::new(Some({ let __arg_holder = scopePos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.declare(__method_arg0, __method_arg1, __method_arg2, __method_arg3) };
    } }
    } else {
        self.error(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = noNewVarPos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(NO_NEW_VAR as i32))))))), Arc::new(Mutex::new(Some("no new variables on left side of :=".to_string()))));
    }
    } else if (*sKey.lock().unwrap()).is_some() {
        { let __range_holder = lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, lhs) in __range_values.iter().enumerate() {
        if (*lhs.lock().unwrap()).is_none() {
        continue
    }
        let mut typ = { let __seq = { let __seq_holder = rhs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() };
        if (*typ.lock().unwrap()).is_none() || { let __left_holder = typ.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone()); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        continue
    }
        if rangeOverInt {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x == __tmp_y }))));
        self.assign_var(lhs.clone(), Arc::new(Mutex::new(None)), x.clone(), Arc::new(Mutex::new(Some("range clause".to_string()))));
        if { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).mode.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::operand::operandMode(Arc::new(Mutex::new(Some(INVALID_1 as u8)))); __tmp_x != __tmp_y } && !is_integer((*x.lock().unwrap().as_ref().unwrap()).typ.clone()) {
        self.soft_errorf(Arc::new(Mutex::new(Some(Box::new((*lhs.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn positioner + Send + Sync>))), Arc::new(Mutex::new(Some(internal_types_errors::codes::Code(Arc::new(Mutex::new(Some(INVALID_RANGE_EXPR as i32))))))), Arc::new(Mutex::new(Some("cannot use iteration variable of type %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>]))));
    }
    } else {
        let mut y: Arc<Mutex<Option<operand>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = crate::operand::operandMode(Arc::new(Mutex::new(Some(VALUE as u8)))); *(*y.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
        { let __iface_handle = lhs.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*y.lock().unwrap().as_mut().unwrap()).expr.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*y.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
        self.assign_var(lhs.clone(), Arc::new(Mutex::new(None)), y.clone(), Arc::new(Mutex::new(Some("assignment".to_string()))));
    }
    } }
    } else if rangeOverInt {
        self.assignment(x.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("range clause".to_string()))));
    }
                        // short variable declaration
                        // determine lhs variable
                        // declare new variable
                        // _ variables don't count as new variables
                        // dummy variable
                        // initialize lhs iteration variable, if any
                        // typ == Typ[Invalid] can happen if allowVersion fails.
                        // don't complain about unused variable
                        // at most one iteration variable (rhs[1] == nil or Typ[Invalid] for rangeOverInt)
                        // we don't have a better rhs expression to use here
                        // error is on variable, use "assignment" not "range clause"
                        // declare variables
                        /* recordDef already called */
                        /* lhs[0] != nil */
                        // ordinary assignment
                        // assign to lhs iteration variable, if any
                        // at most one iteration variable (rhs[1] == nil or Typ[Invalid] for rangeOverInt)
                        // If the assignment succeeded, if x was untyped before, it now
                        // has a type inferred via the assignment. It must be an integer.
                        // (go.dev/issues/67027)
                        // we don't have a better rhs expression to use here
                        // error is on variable, use "assignment" not "range clause"
                        // If we don't have any iteration variables, we still need to
                        // check that a (possibly untyped) integer range expression x
                        // is valid.
                        // We do this by checking the assignment _ = x. This ensures
                        // that an untyped x can be converted to a value of its default
                        // type (rune or int).
            self.stmt(Arc::new(Mutex::new(Some({ let __arg_holder = inner.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BlockStmtPtr((*s.lock().unwrap().as_ref().unwrap()).body.clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>))));

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
}

impl stmtContext {
}

impl cmp::r#mod::Ordered for stmtContext {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<stmtContext>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn trim_trailing_empty_stmts(list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>> {
    let mut i = Arc::new(Mutex::new(Some((*list.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        {
        let (_, mut ok) = ({
        let val = { let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::EmptyStmtPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::EmptyStmt>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::EmptyStmt>)), false)
        }
    });;
        if !ok {
            return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })));;
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return Arc::new(Mutex::new(None));
}

pub fn assign_op(op: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::r#mod::Token>>> {
        // token_test.go verifies the token ordering this function relies on
    if { let __tmp_x = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D__A_S_S_I_G_N as i32)))); let __tmp_y = (*op.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*op.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T__A_S_S_I_G_N as i32)))); __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(((*{ let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) + ((go_token::A_D_D as i32 - go_token::A_D_D__A_S_S_I_G_N as i32))))))))));
    }
    Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_L_L_E_G_A_L as i32)))))))
}

/// goVal returns the Go value for val, or nil.
pub fn go_val(val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        // val should exist, but be conservative and check
    if (*val.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(None));
    }

        // Match implementation restriction of other compilers.
        // gc only checks duplicates for integer, floating-point
        // and string values, so only create Go values for these
        // types.
    { let _switch_val = { let __v = (*val.lock().unwrap().as_ref().unwrap()).kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::INT as i32))))) {
            {
        let (mut x, mut ok) = go_constant::int64_val(val.clone());;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(x) as Box<dyn Any + Send + Sync>)));;
        }
    }
            {
        let (mut x, mut ok) = go_constant::uint64_val(val.clone());;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(x) as Box<dyn Any + Send + Sync>)));;
        }
    }
        } else if _switch_val == (go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::FLOAT as i32))))) {
            {
        let (mut x, mut ok) = go_constant::float64_val(val.clone());;
        if ok {
            return Arc::new(Mutex::new(Some(Box::new(x) as Box<dyn Any + Send + Sync>)));;
        }
    }
        } else if _switch_val == (go_constant::value::Kind(Arc::new(Mutex::new(Some(go_constant::STRING as i32))))) {
            return Arc::new(Mutex::new(Some(Box::new({ let __v = go_constant::string_val(val.clone()); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>)));
        }
    }
    return Arc::new(Mutex::new(None));
}

/// rangeKeyVal returns the key and value type produced by a range clause
/// over an expression of type typ.
/// If allowVersion != nil, it is used to check the required language version.
/// If the range clause is not permitted, rangeKeyVal returns ok = false.
/// When ok = false, rangeKeyVal may also return a reason in cause.
pub fn range_key_val(mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, allowVersion: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync>>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) {
    let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(typ.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
    let mut key: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut val: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut cause: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut bad = Arc::new(Mutex::new(Some(Box::new(move |cause: Arc<Mutex<Option<String>>>| -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) {
        (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), { let __owned = cause.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, false)
    }) as Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync>)));

    let mut orig = typ.clone();
    {
    let _ts_subject = array_ptr_deref(core_type(typ.clone()).clone()).clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let typ = _ts_subject.clone();
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("no core type".to_string())))) };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if is_string(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), universeRune.clone(), Arc::new(Mutex::new(Some("".to_string()))), true);
    };
        if is_integer(Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(typ.clone())) as Box<dyn Type + Send + Sync>)))) {
        if (*allowVersion.lock().unwrap()).is_some() && !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync> = { let mut __f_guard = allowVersion.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = go1_22.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) } {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("requires go1.22 or later".to_string())))) };
    }
        return (orig.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), true);
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some("".to_string()))), true);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        return (Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INT as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some("".to_string()))), true);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        return ((*typ.lock().unwrap().as_ref().unwrap()).key.clone(), (*typ.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some("".to_string()))), true);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __selector_holder = (*typ.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(SEND_ONLY as i32)))); __tmp_x == __tmp_y } {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("receive from send-only channel".to_string())))) };
    };
        return ((*typ.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), true);;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let typ = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        if !(*(*(*internal_buildcfg::Experiment.lock().unwrap().as_ref().unwrap()).flags.lock().unwrap().as_ref().unwrap()).range_func.lock().unwrap().as_ref().unwrap()) && (*allowVersion.lock().unwrap()).is_some() && !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync> = { let mut __f_guard = allowVersion.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<goVersion>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let __arg_holder = go1_23.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) } {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("requires go1.23 or later".to_string())))) };
    };
        if { let __tmp_x = { let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 1; __tmp_x != __tmp_y } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): wrong argument count".to_string())))) };
        } else if { let __tmp_x = { let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x != __tmp_y } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): unexpected results".to_string())))) };
        };
        assert(Arc::new(Mutex::new(Some((*{ let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv(); __result }.lock().unwrap()).is_none()))));;
        let (mut cb, _) = ({
        let val = core_type({ let __recv = { let __recv = { let __recv = typ.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(0)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::signature::Signature>)), false)
        }
    });;
        if (*cb.lock().unwrap()).is_none() {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): argument is not func".to_string())))) };
        } else if { let __tmp_x = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 2; __tmp_x > __tmp_y } {
            return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): yield func has too many parameters".to_string())))) };
        } else if { let __tmp_x = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 1; __tmp_x != __tmp_y } || !identical({ let __recv = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(0)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(), universeBool.clone()) {
            if { let __tmp_x = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 1; __tmp_x == __tmp_y } && is_boolean({ let __recv = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.results(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(0)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone()) {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): yield func returns user-defined boolean, not bool".to_string())))) };
    } else {
        return { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> = { let mut __f_guard = bad.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<String>>>, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some("func must be func(yield func(...) bool): yield func does not return bool".to_string())))) };
    }
        };
        assert(Arc::new(Mutex::new(Some((*{ let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv(); __result }.lock().unwrap()).is_none()))));;
        if { let __tmp_x = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 1; __tmp_x >= __tmp_y } {
        { let __iface_handle = { let __recv = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(0)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *key.lock().unwrap() = (*__iface_guard).clone(); };
    };
        if { let __tmp_x = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 2; __tmp_x >= __tmp_y } {
        { let __iface_handle = { let __recv = { let __recv = { let __recv = cb.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).at(Arc::new(Mutex::new(Some(1)))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *val.lock().unwrap() = (*__iface_guard).clone(); };
    };
        return (key.clone(), val.clone(), Arc::new(Mutex::new(Some("".to_string()))), true);;
    }
    }
        // use 'rune' name
        // check iterator arity
        // check iterator argument type
        // see go.dev/issues/71131, go.dev/issues/71164
        // determine key and value types, if any
    return (key.clone(), val.clone(), cause.clone(), (*ok.lock().unwrap().as_ref().unwrap()));
}

impl GoValueClone for valueType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
