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
use crate::under::*;
use crate::union::*;
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const UNIFICATION_DEPTH_LIMIT: i32 = 50;
pub(crate) const PANIC_AT_UNIFICATION_DEPTH_LIMIT: bool = true;
pub(crate) const ENABLE_CORE_TYPE_UNIFICATION: bool = true;
pub(crate) const TRACE_INFERENCE: bool = false;


pub(crate) const ASSIGN: u64 = 1 << 0;
pub(crate) const EXACT: u64 = 1 << 1;


/// A unifier maintains a list of type parameters and
/// corresponding types inferred for each type parameter.
/// A unifier is created by calling newUnifier.
#[derive(Clone)]
pub struct unifier {
    pub handles: Arc<Mutex<Option<BTreeMap<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>,
    pub depth: Arc<Mutex<Option<i32>>>,
    pub enable_interface_inference: Arc<Mutex<Option<bool>>>,
}

impl unifier {
    pub fn __go_value_clone(&self) -> Self {
        Self { handles: self.handles.clone(), depth: { let __guard = self.depth.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, enable_interface_inference: { let __guard = self.enable_interface_inference.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for unifier {
    fn default() -> Self {
        Self { handles: Arc::new(Mutex::new(None)), depth: Arc::new(Mutex::new(Some(0))), enable_interface_inference: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for unifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for unifier {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// unifyMode controls the behavior of the unifier.
#[derive(Debug, Clone, Default)]
pub struct unifyMode(pub Arc<Mutex<Option<u64>>>);

impl Display for unifyMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for unifyMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for unifyMode {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for unifyMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for unifyMode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<unifyMode> for u64 {
    fn eq(&self, other: &unifyMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<unifyMode> for u64 {
    fn partial_cmp(&self, other: &unifyMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for unifyMode {
    type Output = unifyMode;
    fn add(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for unifyMode {
    type Output = unifyMode;
    fn add(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<unifyMode> for u64 {
    type Output = unifyMode;
    fn add(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for unifyMode {
    type Output = unifyMode;
    fn sub(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for unifyMode {
    type Output = unifyMode;
    fn sub(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<unifyMode> for u64 {
    type Output = unifyMode;
    fn sub(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for unifyMode {
    type Output = unifyMode;
    fn mul(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for unifyMode {
    type Output = unifyMode;
    fn mul(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<unifyMode> for u64 {
    type Output = unifyMode;
    fn mul(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for unifyMode {
    type Output = unifyMode;
    fn div(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for unifyMode {
    type Output = unifyMode;
    fn div(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<unifyMode> for u64 {
    type Output = unifyMode;
    fn div(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for unifyMode {
    type Output = unifyMode;
    fn rem(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for unifyMode {
    type Output = unifyMode;
    fn rem(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<unifyMode> for u64 {
    type Output = unifyMode;
    fn rem(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for unifyMode {
    type Output = unifyMode;
    fn bitand(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for unifyMode {
    type Output = unifyMode;
    fn bitand(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<unifyMode> for u64 {
    type Output = unifyMode;
    fn bitand(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for unifyMode {
    type Output = unifyMode;
    fn bitor(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for unifyMode {
    type Output = unifyMode;
    fn bitor(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<unifyMode> for u64 {
    type Output = unifyMode;
    fn bitor(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for unifyMode {
    type Output = unifyMode;
    fn bitxor(self, other: Self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for unifyMode {
    type Output = unifyMode;
    fn bitxor(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<unifyMode> for u64 {
    type Output = unifyMode;
    fn bitxor(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for unifyMode {
    type Output = unifyMode;
    fn not(self) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: i32) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: i8) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: i16) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: i64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: u32) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: u8) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: u16) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for unifyMode {
    type Output = unifyMode;
    fn shl(self, other: usize) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: unifyMode) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: i32) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: i8) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: i16) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: i64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: u32) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: u8) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: u16) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: u64) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for unifyMode {
    type Output = unifyMode;
    fn shr(self, other: usize) -> unifyMode {
        unifyMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for unifyMode {}

impl Ord for unifyMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone, Default)]
pub struct typeParamsById(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>);

impl Display for typeParamsById {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice_wrapped(&self.0))
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


impl unifyMode {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        { let _switch_val = (*self.0.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (unifyMode(Arc::new(Mutex::new(Some(0 as u64))))) {
            return Arc::new(Mutex::new(Some("inexact".to_string())));
        } else if _switch_val == (unifyMode(Arc::new(Mutex::new(Some(ASSIGN as u64))))) {
            return Arc::new(Mutex::new(Some("assign".to_string())));
        } else if _switch_val == (unifyMode(Arc::new(Mutex::new(Some(EXACT as u64))))) {
            return Arc::new(Mutex::new(Some("exact".to_string())));
        } else if _switch_val == (unifyMode(Arc::new(Mutex::new(Some((ASSIGN as u64 | EXACT as u64) as u64))))) {
            return Arc::new(Mutex::new(Some("assign, exact".to_string())));
        }
    }
        Arc::new(Mutex::new(Some(format!("mode {}", (*self.0.lock().unwrap().as_ref().unwrap()).clone()))))
    }
}

impl cmp::r#mod::Ordered for unifyMode {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<unifyMode>() {
            self == __other
        } else {
            false
        }
    }
}

impl unifier {
    /// unify attempts to unify x and y and reports whether it succeeded.
    /// As a side-effect, types may be inferred for type parameters.
    /// The mode parameter controls how types are compared.
    pub fn unify(&mut self, x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mode: Arc<Mutex<Option<unifyMode>>>) -> bool {
        self.nify(x.clone(), y.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)))
    }

    pub fn tracef(&self, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        println!("{}", format!("{}", format!("{}{}", (*Arc::new(Mutex::new(Some({ let __s = ".  ".to_string(); let __count = { let __selector_holder = self.depth.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __s.repeat(__count as usize) }))).lock().unwrap().as_ref().unwrap()), (*sprintf(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __arg_holder = format.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), args.clone()).lock().unwrap().as_ref().unwrap()))));
    }

    /// String returns a string representation of the current mapping
    /// from type parameters to types.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
                // sort type parameters for reproducible strings
        let mut tparams = Arc::new(Mutex::new(Some(typeParamsById(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = self.handles.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize])))))));
        let mut i = Arc::new(Mutex::new(Some(0)));
        for (__range_key, _) in { let __range_holder = self.handles.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let tpar = __range_key.value();
        (*{ let __named_slice = (*tparams.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = tpar.clone();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut __sort_data = (*tparams.lock().unwrap().as_ref().unwrap()).clone(); let __sort_len = __sort_data.len(); for __sort_i in 1..(__sort_len as usize) { let mut __sort_j = __sort_i as i32; while __sort_j > 0 { if !__sort_data.less(Arc::new(Mutex::new(Some(__sort_j))), Arc::new(Mutex::new(Some(__sort_j - 1)))) { break; } __sort_data.swap(Arc::new(Mutex::new(Some(__sort_j))), Arc::new(Mutex::new(Some(__sort_j - 1)))); __sort_j -= 1; } } };
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut w = new_type_writer(buf.clone(), Arc::new(Mutex::new(None)));
        { let __recv = w.clone(); let __recv_ptr: *const crate::typestring::typeWriter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typestring::typeWriter }; let __result = unsafe { &*__recv_ptr }.byte(Arc::new(Mutex::new(Some(('[' as i32) as u8)))); __result };
        { let __range_holder = { let __named_slice = (*tparams.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        if { let __tmp_x = i as i32; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __recv = w.clone(); let __recv_ptr: *const crate::typestring::typeWriter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typestring::typeWriter }; let __result = unsafe { &*__recv_ptr }.string(Arc::new(Mutex::new(Some(", ".to_string())))); __result };
    }
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(x.clone())) as Box<dyn Type + Send + Sync>)))); __result };
        { let __recv = w.clone(); let __recv_ptr: *const crate::typestring::typeWriter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typestring::typeWriter }; let __result = unsafe { &*__recv_ptr }.string(Arc::new(Mutex::new(Some(": ".to_string())))); __result };
        { let __recv = w.clone(); let __recv_ptr: *mut crate::typestring::typeWriter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::typestring::typeWriter }; let __result = unsafe { &mut *__recv_ptr }.typ(self.at((*x).clone()).clone()); __result };
    } }
        { let __recv = w.clone(); let __recv_ptr: *const crate::typestring::typeWriter = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typestring::typeWriter }; let __result = unsafe { &*__recv_ptr }.byte(Arc::new(Mutex::new(Some((']' as i32) as u8)))); __result };
        return (*buf.lock().unwrap().as_mut().unwrap()).string();
    }

    /// join unifies the given type parameters x and y.
    /// If both type parameters already have a type associated with them
    /// and they are not joined, join fails and returns false.
    pub fn join(&mut self, x: Arc<Mutex<Option<TypeParam>>>, y: Arc<Mutex<Option<TypeParam>>>) -> bool {
        if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{21c4} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new(y.clone()) as Box<dyn Any + Send + Sync>]))));
    }
        let (mut hx, mut hy) = ({ let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(), { let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(y.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone());
    if { let __left = hx.clone(); let __right = hy.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        } else if { let __iface_handle = hx.clone(); let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } && { let __iface_handle = hy.clone(); let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
                        // Both type parameters have (possibly different) inferred types. Cannot join.
            return false;
        } else if { let __iface_handle = hx.clone(); let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
                        // Only type parameter x has an inferred type. Use handle of x.
            self.set_handle(y.clone(), hx.clone());
        } else {
                        // Neither type parameter has an inferred type. Use handle of y.
            self.set_handle(x.clone(), hy.clone());
        }
                // Both type parameters already share the same handle. Nothing to do.
                // Both type parameters have (possibly different) inferred types. Cannot join.
                // Only type parameter x has an inferred type. Use handle of x.
                // This case is treated like the default case.
                // case *hy != nil:
                // 	// Only type parameter y has an inferred type. Use handle of y.
                //	u.setHandle(x, hy)
                // Neither type parameter has an inferred type. Use handle of y.
        true
    }

    /// asBoundTypeParam returns x.(*TypeParam) if x is a type parameter recorded with u.
    /// Otherwise, the result is nil.
    pub fn as_bound_type_param(&self, mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<crate::typeparam::TypeParam>>> {
        let mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        {
        let (mut x, _) = ({
        let val = unalias(x.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });;
        if (*x.lock().unwrap()).is_some() {
            {
        let (_, mut found) = { let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if found {
            return x.clone();;
        }
    };
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// setHandle sets the handle for type parameter x
    /// (and all its joined type parameters) to h.
    pub fn set_handle(&mut self, x: Arc<Mutex<Option<TypeParam>>>, h: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut hx = { let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        assert(Arc::new(Mutex::new(Some((*hx.lock().unwrap()).is_some()))));
        for (__range_key, hy) in { let __range_holder = self.handles.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let y = __range_key.value();
        if { let __left = hy.clone(); let __right = hx.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let __map_key = GoLocalPtrKey::new(y.clone()); let __map_value = h.clone(); (*self.handles.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }
    }
    }

    /// at returns the (possibly nil) type for type parameter x.
    pub fn at(&self, x: Arc<Mutex<Option<TypeParam>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        { let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone()
    }

    /// set sets the type t for type parameter x;
    /// t must not be nil.
    pub fn set(&mut self, x: Arc<Mutex<Option<TypeParam>>>, t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some((*t.lock().unwrap()).is_some()))));
        if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{279e} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(x.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let new_val = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; *{ let __map = { let __map_holder = self.handles.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(x.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap() = Some(new_val); };
    }

    /// unknowns returns the number of type parameters for which no type has been set yet.
    pub fn unknowns(&self) -> i32 {
        let mut n = Arc::new(Mutex::new(Some(0)));
        for (_, h) in { let __range_holder = self.handles.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        if { let __iface_handle = h.clone(); let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }

    /// inferred returns the list of inferred types for the given type parameter list.
    /// The result is never nil and has the same length as tparams; result types that
    /// could not be inferred are nil. Corresponding type parameters and result types
    /// have identical indices.
    pub fn inferred(&self, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> {
        let mut list: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>> = Arc::new(Mutex::new(Some(vec![Default::default(); ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
        { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        (*list.lock().unwrap().as_mut().unwrap())[(i) as usize] = self.at((*x).clone()).clone();
    } }
        return list.clone();
    }

    /// nify implements the core unification algorithm which is an
    /// adapted version of Checker.identical. For changes to that
    /// code the corresponding changes should be made here.
    /// Must not be called directly from outside the unifier.
    pub fn nify(&mut self, mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, mode: Arc<Mutex<Option<unifyMode>>>, mut p: Arc<Mutex<Option<ifacePair>>>) -> bool {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut result: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        let mut x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(x.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        let mut y: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(y.lock().unwrap().as_ref().map(|__v| Type::__go_clone_box_type_(__v.as_ref()))));
        { let __target = self.depth.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{2261} %s\t// %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new((*mode.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>]))));
    }
        let result_defer_captured = result.clone(); let mut u_defer_captured = self.clone(); let x_defer_captured = x.clone(); let y_defer_captured = y.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        if TRACE_INFERENCE && !{ let __v = (*result_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        u_defer_captured.tracef(Arc::new(Mutex::new(Some("%s \u{2262} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = x_defer_captured.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = y_defer_captured.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let __target = u_defer_captured.depth.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
                // nothing to do if x == y
        if { let __left_holder = x.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = y.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } || { let __left_holder = unalias(x.clone()).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = unalias(y.clone()).clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // Stop gap for cases where unification fails.
        if { let __tmp_x = (*self.depth.lock().unwrap().as_ref().unwrap()); let __tmp_y = 50; __tmp_x > __tmp_y } {
        if TRACE_INFERENCE {
        { let __method_arg0 = Arc::new(Mutex::new(Some("depth %d >= %d".to_string()))); self.tracef(__method_arg0, Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = self.depth.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(UNIFICATION_DEPTH_LIMIT) as Box<dyn Any + Send + Sync>])))) };
    }
        if PANIC_AT_UNIFICATION_DEPTH_LIMIT {
        panic!("unification reached recursion depth limit");
    }
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // Unification is symmetric, so we can swap the operands.
                // Ensure that if we have at least one
                // - defined type, make sure one is in y
                // - type parameter recorded with u, make sure one is in x
        if (*as_named(x.clone()).lock().unwrap()).is_some() || (*self.as_bound_type_param(y.clone()).lock().unwrap()).is_some() {
        if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{2261} %s\t// swap".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); { let __iface_handle = __tmp_0; let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); } { let __iface_handle = __tmp_1; let __iface_guard = __iface_handle.lock().unwrap(); *y.lock().unwrap() = (*__iface_guard).clone(); } };
    }
                // Unification will fail if we match a defined type against a type literal.
                // If we are matching types in an assignment, at the top-level, types with
                // the same type structure are permitted as long as at least one of them
                // is not a defined type. To accommodate for that possibility, we continue
                // unification with the underlying type of a defined type if the other type
                // is a type literal. This is controlled by the exact unification mode.
                // We also continue if the other type is a basic type because basic types
                // are valid underlying types and may appear as core types of type constraints.
                // If we exclude them, inferred defined types for type parameters may not
                // match against the core types of their constraints (even though they might
                // correctly match against some of the types in the constraint's type set).
                // Finally, if unification (incorrectly) succeeds by matching the underlying
                // type of a defined type against a basic type (because we include basic types
                // as type literals here), and if that leads to an incorrectly inferred type,
                // we will fail at function instantiation or argument assignment time.
                //
                // If we have at least one defined type, there is one in y.
        {
        let mut ny = as_named(y.clone());;
        if { let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & EXACT as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } && (*ny.lock().unwrap()).is_some() && is_type_lit(x.clone()) && !((*self.enable_interface_inference.clone().lock().unwrap().as_ref().unwrap()) && is_interface(x.clone())) {
            if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{2261} under %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(ny.clone()) as Box<dyn Any + Send + Sync>]))));
    };
            { let __iface_handle = { let __recv = ny.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.under(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *y.lock().unwrap() = (*__iface_guard).clone(); };;
            assert(Arc::new(Mutex::new(Some(!is_type_param(y.clone())))));;
            if { let __left_holder = x.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = y.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } || { let __left_holder = unalias(x.clone()).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = unalias(y.clone()).clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        }
    }
                // Per the spec, a defined type cannot have an underlying type
                // that is a type parameter.
                // x and y may be identical now
                // Cases where at least one of x or y is a type parameter recorded with u.
                // If we have at least one type parameter, there is one in x.
                // If we have exactly one type parameter, because it is in x,
                // isTypeLit(x) is false and y was not changed above. In other
                // words, if y was a defined type, it is still a defined type
                // (relevant for the logic below).
        let (mut px, mut py) = (self.as_bound_type_param(x.clone()), self.as_bound_type_param(y.clone()));
    if (*px.lock().unwrap()).is_some() && (*py.lock().unwrap()).is_some() {
                        // both x and y are type parameters
            if self.join(px.clone(), py.clone()) {
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                        // both x and y have an inferred type - they must match
            {
        { let new_val = { let __method_arg0 = self.at(px.clone()).clone(); let __method_arg1 = self.at(py.clone()).clone(); let __method_arg2 = Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); let __method_arg3 = p.clone(); self.nify(__method_arg0, __method_arg1, __method_arg2, __method_arg3) }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
        } else if (*px.lock().unwrap()).is_some() {
                        // x is a type parameter, y is not
            {
        let mut x = self.at(px.clone());;
        if (*x.lock().unwrap()).is_some() {
            if self.nify(x.clone(), y.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) {
        let mut xi = as_interface(x.clone());
        let mut yi = as_interface(y.clone());
        let mut xn = Arc::new(Mutex::new(Some((*as_named(x.clone()).lock().unwrap()).is_some())));
        let mut yn = Arc::new(Mutex::new(Some((*as_named(y.clone()).lock().unwrap()).is_some())));
        if (*xi.lock().unwrap()).is_some() && (*yi.lock().unwrap()).is_some() {
        if { let __v = (*xn.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __v = (*yn.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        { let new_val = identical(x.clone(), y.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
        if { let __tmp_x = (({ let __len_target = { let __field = (*{ let __recv = xi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = (*{ let __recv = yi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x != __tmp_y } {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    } else if (*xi.lock().unwrap()).is_some() || (*yi.lock().unwrap()).is_some() {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
        if { let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & EXACT as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        if { let __v = (*xn.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        } else if { let __v = (*yn.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            self.set(px.clone(), y.clone());
        } else {
            {
        let (mut yc, _) = ({
        let val = under(y.clone()).clone();
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
        if (*yc.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*yc.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = crate::chan::ChanDir(Arc::new(Mutex::new(Some(SEND_RECV as i32)))); __tmp_x != __tmp_y } {
            self.set(px.clone(), y.clone());;
        }
    }
        }
    }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
            {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    }
                        // x has an inferred type which must match y
                        // We have a match, possibly through underlying types.
                        // If we have two interfaces, what to do depends on
                        // whether they are named and their method sets.
                        // Both types are interfaces.
                        // If both types are defined types, they must be identical
                        // because unification doesn't know which type has the "right" name.
                        // In all other cases, the method sets must match.
                        // The types unified so we know that corresponding methods
                        // match and we can simply compare the number of methods.
                        // TODO(gri) We may be able to relax this rule and select
                        // the more general interface. But if one of them is a defined
                        // type, it's not clear how to choose and whether we introduce
                        // an order dependency or not. Requiring the same method set
                        // is conservative.
                        // One but not both of them are interfaces.
                        // In this case, either x or y could be viable matches for the corresponding
                        // type parameter, which means choosing either introduces an order dependence.
                        // Therefore, we must fail unification (go.dev/issue/60933).
                        // If we have inexact unification and one of x or y is a defined type, select the
                        // defined type. This ensures that in a series of types, all matching against the
                        // same type parameter, we infer a defined type if there is one, independent of
                        // order. Type inference or assignment may fail, which is ok.
                        // Selecting a defined type, if any, ensures that we don't lose the type name;
                        // and since we have inexact unification, a value of equally named or matching
                        // undefined type remains assignable (go.dev/issue/43056).
                        //
                        // Similarly, if we have inexact unification and there are no defined types but
                        // channel types, select a directed channel, if any. This ensures that in a series
                        // of unnamed types, all matching against the same type parameter, we infer the
                        // directed channel if there is one, independent of order.
                        // Selecting a directional channel, if any, ensures that a value of another
                        // inexactly unifying channel type remains assignable (go.dev/issue/62157).
                        //
                        // If we have multiple defined channel types, they are either identical or we
                        // have assignment conflicts, so we can ignore directionality in this case.
                        //
                        // If we have defined and literal channel types, a defined type wins to avoid
                        // order dependencies.
                        // x is a defined type: nothing to do.
                        // x is not a defined type and y is a defined type: select y.
                        // Neither x nor y are defined types.
                        // y is a directed channel type: select y.
                        // otherwise, infer type from y
            self.set(px.clone(), y.clone());
            {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
        }
                // both x and y are type parameters
                // both x and y have an inferred type - they must match
                // x is a type parameter, y is not
                // x has an inferred type which must match y
                // We have a match, possibly through underlying types.
                // If we have two interfaces, what to do depends on
                // whether they are named and their method sets.
                // Both types are interfaces.
                // If both types are defined types, they must be identical
                // because unification doesn't know which type has the "right" name.
                // In all other cases, the method sets must match.
                // The types unified so we know that corresponding methods
                // match and we can simply compare the number of methods.
                // TODO(gri) We may be able to relax this rule and select
                // the more general interface. But if one of them is a defined
                // type, it's not clear how to choose and whether we introduce
                // an order dependency or not. Requiring the same method set
                // is conservative.
                // One but not both of them are interfaces.
                // In this case, either x or y could be viable matches for the corresponding
                // type parameter, which means choosing either introduces an order dependence.
                // Therefore, we must fail unification (go.dev/issue/60933).
                // If we have inexact unification and one of x or y is a defined type, select the
                // defined type. This ensures that in a series of types, all matching against the
                // same type parameter, we infer a defined type if there is one, independent of
                // order. Type inference or assignment may fail, which is ok.
                // Selecting a defined type, if any, ensures that we don't lose the type name;
                // and since we have inexact unification, a value of equally named or matching
                // undefined type remains assignable (go.dev/issue/43056).
                //
                // Similarly, if we have inexact unification and there are no defined types but
                // channel types, select a directed channel, if any. This ensures that in a series
                // of unnamed types, all matching against the same type parameter, we infer the
                // directed channel if there is one, independent of order.
                // Selecting a directional channel, if any, ensures that a value of another
                // inexactly unifying channel type remains assignable (go.dev/issue/62157).
                //
                // If we have multiple defined channel types, they are either identical or we
                // have assignment conflicts, so we can ignore directionality in this case.
                //
                // If we have defined and literal channel types, a defined type wins to avoid
                // order dependencies.
                // x is a defined type: nothing to do.
                // x is not a defined type and y is a defined type: select y.
                // Neither x nor y are defined types.
                // y is a directed channel type: select y.
                // otherwise, infer type from y
                // x != y if we get here
        assert(Arc::new(Mutex::new(Some({ let __left_holder = x.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = y.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq } && { let __left_holder = unalias(x.clone()).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = unalias(y.clone()).clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; !__eq }))));
                // If u.EnableInterfaceInference is set and we don't require exact unification,
                // if both types are interfaces, one interface must have a subset of the
                // methods of the other and corresponding method signatures must unify.
                // If only one type is an interface, all its methods must be present in the
                // other type and corresponding method signatures must unify.
        if (*self.enable_interface_inference.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & EXACT as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // One or both interfaces may be defined types.
                // Look under the name, but not under type parameters (go.dev/issue/60564).
        let mut xi = as_interface(x.clone());
        let mut yi = as_interface(y.clone());
                // If we have two interfaces, check the type terms for equivalence,
                // and unify common methods if possible.
        if (*xi.lock().unwrap()).is_some() && (*yi.lock().unwrap()).is_some() {
        let mut xset = { let __recv = xi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };
        let mut yset = { let __recv = yi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };
        if { let __tmp_x = (*{ let __field = (*xset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*yset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // For now we require terms to be equal.
                // We should be able to relax this as well, eventually.
        if !(*(*xset.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).equal({ let __field = (*yset.lock().unwrap().as_ref().unwrap()).terms.clone(); __field }) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // Interface types are the only types where cycles can occur
                // that are not "terminated" via named types; and such cycles
                // can only be created via method parameter types that are
                // anonymous interfaces (directly or indirectly) embedding
                // the current interface. Example:
                //
                //    type T interface {
                //        m() interface{T}
                //    }
                //
                // If two such (differently named) interfaces are compared,
                // endless recursion occurs if the cycle is not detected.
                //
                // If x and y were compared before, they must be equal
                // (if they were not, the recursion would have stopped);
                // search the ifacePair stack for the same pair.
                //
                // This is a quadratic algorithm, but in practice these stacks
                // are extremely short (bounded by the nesting depth of interface
                // type declarations that recur via parameter types, an extremely
                // rare occurrence). An alternative implementation might use a
                // "visited" map, but that is probably less efficient overall.
        let mut q = Arc::new(Mutex::new(Some(ifacePair { x: xi.clone(), y: yi.clone(), prev: p.clone(), ..Default::default() })));
        while (*p.lock().unwrap()).is_some() {
        if { let __recv = p.clone(); let __recv_ptr: *const crate::predicates::ifacePair = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::predicates::ifacePair }; let __result = unsafe { &*__recv_ptr }.identical(q.clone()); __result } {
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // same pair was compared before
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).prev.clone(); p = new_val; };
    }
                // same pair was compared before
                // The method set of x must be a subset of the method set
                // of y or vice versa, and the common methods must unify.
        let mut xmethods = (*xset.lock().unwrap().as_ref().unwrap()).methods.clone();
        let mut ymethods = (*yset.lock().unwrap().as_ref().unwrap()).methods.clone();
                // The smaller method set must be the subset, if it exists.
        if { let __tmp_x = ((*xmethods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*ymethods.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x > __tmp_y } {
        { let __tmp_0 = ymethods.clone(); let __tmp_1 = xmethods.clone(); *xmethods.lock().unwrap() = __tmp_0.lock().unwrap().take(); *ymethods.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
                // len(xmethods) <= len(ymethods)
                // Collect the ymethods in a map for quick lookup.
        let mut ymap = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<crate::object::Func>>>>::new())));
        { let __range_holder = ymethods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for ym in __range_values.iter() {
        { let __map_key = { let __map_key_holder = { let __recv = ym.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned }; let __map_value = ym.clone(); (*ymap.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
                // All xmethods must exist in ymethods and corresponding signatures must unify.
        { let __range_holder = xmethods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for xm in __range_values.iter() {
        {
        let mut ym = { let __map = { let __map_holder = ymap.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __map_key_holder = { let __recv = xm.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); drop(__map_key_guard); __cloned })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };;
        if (*ym.lock().unwrap()).is_none() || !self.nify((*(*xm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*ym.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(unifyMode(Arc::new(Mutex::new(Some(EXACT as u64))))))), p.clone()) {
            {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    }
    } }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
                // For now we require terms to be equal.
                // We should be able to relax this as well, eventually.
                // Interface types are the only types where cycles can occur
                // that are not "terminated" via named types; and such cycles
                // can only be created via method parameter types that are
                // anonymous interfaces (directly or indirectly) embedding
                // the current interface. Example:
                //
                //    type T interface {
                //        m() interface{T}
                //    }
                //
                // If two such (differently named) interfaces are compared,
                // endless recursion occurs if the cycle is not detected.
                //
                // If x and y were compared before, they must be equal
                // (if they were not, the recursion would have stopped);
                // search the ifacePair stack for the same pair.
                //
                // This is a quadratic algorithm, but in practice these stacks
                // are extremely short (bounded by the nesting depth of interface
                // type declarations that recur via parameter types, an extremely
                // rare occurrence). An alternative implementation might use a
                // "visited" map, but that is probably less efficient overall.
                // same pair was compared before
                // The method set of x must be a subset of the method set
                // of y or vice versa, and the common methods must unify.
                // The smaller method set must be the subset, if it exists.
                // len(xmethods) <= len(ymethods)
                // Collect the ymethods in a map for quick lookup.
                // All xmethods must exist in ymethods and corresponding signatures must unify.
                // We don't have two interfaces. If we have one, make sure it's in xi.
        if (*yi.lock().unwrap()).is_some() {
        { let new_val = yi.clone(); xi = new_val; };
        { let __iface_handle = x.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *y.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // If we have one interface, at a minimum each of the interface methods
                // must be implemented and thus unify with a corresponding method from
                // the non-interface type, otherwise unification fails.
        if (*xi.lock().unwrap()).is_some() {
                // All xi methods must exist in y and corresponding signatures must unify.
        let mut xmethods = (*{ let __recv = xi.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone();
        { let __range_holder = xmethods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for xm in __range_values.iter() {
        let (mut obj, _, _) = lookup_field_or_method(y.clone(), Arc::new(Mutex::new(Some(false))), { let __field = (*(*xm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, { let __field = (*(*xm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); __field });
        {
        let (mut ym, _) = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::FuncPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::object::Func>)), false)
        }
    });;
        if (*ym.lock().unwrap()).is_none() || !self.nify((*(*xm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*ym.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(unifyMode(Arc::new(Mutex::new(Some(EXACT as u64))))))), p.clone()) {
            {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    }
    } }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    }
                // One or both interfaces may be defined types.
                // Look under the name, but not under type parameters (go.dev/issue/60564).
                // If we have two interfaces, check the type terms for equivalence,
                // and unify common methods if possible.
                // For now we require terms to be equal.
                // We should be able to relax this as well, eventually.
                // Interface types are the only types where cycles can occur
                // that are not "terminated" via named types; and such cycles
                // can only be created via method parameter types that are
                // anonymous interfaces (directly or indirectly) embedding
                // the current interface. Example:
                //
                //    type T interface {
                //        m() interface{T}
                //    }
                //
                // If two such (differently named) interfaces are compared,
                // endless recursion occurs if the cycle is not detected.
                //
                // If x and y were compared before, they must be equal
                // (if they were not, the recursion would have stopped);
                // search the ifacePair stack for the same pair.
                //
                // This is a quadratic algorithm, but in practice these stacks
                // are extremely short (bounded by the nesting depth of interface
                // type declarations that recur via parameter types, an extremely
                // rare occurrence). An alternative implementation might use a
                // "visited" map, but that is probably less efficient overall.
                // same pair was compared before
                // The method set of x must be a subset of the method set
                // of y or vice versa, and the common methods must unify.
                // The smaller method set must be the subset, if it exists.
                // len(xmethods) <= len(ymethods)
                // Collect the ymethods in a map for quick lookup.
                // All xmethods must exist in ymethods and corresponding signatures must unify.
                // We don't have two interfaces. If we have one, make sure it's in xi.
                // If we have one interface, at a minimum each of the interface methods
                // must be implemented and thus unify with a corresponding method from
                // the non-interface type, otherwise unification fails.
                // All xi methods must exist in y and corresponding signatures must unify.
                // Unless we have exact unification, neither x nor y are interfaces now.
                // Except for unbound type parameters (see below), x and y must be structurally
                // equivalent to unify.
                // If we get here and x or y is a type parameter, they are unbound
                // (not recorded with the unifier).
                // Ensure that if we have at least one type parameter, it is in x
                // (the earlier swap checks for _recorded_ type parameters only).
                // This ensures that the switch switches on the type parameter.
                //
                // TODO(gri) Factor out type parameter handling from the switch.
        if is_type_param(y.clone()) {
        if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("%s \u{2261} %s\t// swap".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = y.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    }
        { let __tmp_0 = y.clone(); let __tmp_1 = x.clone(); { let __iface_handle = __tmp_0; let __iface_guard = __iface_handle.lock().unwrap(); *x.lock().unwrap() = (*__iface_guard).clone(); } { let __iface_handle = __tmp_1; let __iface_guard = __iface_handle.lock().unwrap(); *y.lock().unwrap() = (*__iface_guard).clone(); } };
    }
                // Type elements (array, slice, etc. elements) use emode for unification.
                // Element types must match exactly if the types are used in an assignment.
        let mut emode = { let __owned = mode.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ASSIGN as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y } {
        { let __rhs = unifyMode(Arc::new(Mutex::new(Some(EXACT as u64)))); let mut guard = emode.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
                // Continue with unaliased types but don't lose original alias names, if any (go.dev/issue/67628).
        let (mut xorig, __tmp_1) = (Arc::new(Mutex::new(Some((*x.lock().unwrap().as_ref().unwrap()).clone()))), unalias(x.clone())); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *x.lock().unwrap() = __moved_tmp_1;;
        let (mut yorig, __tmp_1) = (Arc::new(Mutex::new(Some((*y.lock().unwrap().as_ref().unwrap()).clone()))), unalias(y.clone())); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *y.lock().unwrap() = __moved_tmp_1;;
        {
    let _ts_subject = x.clone();
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::basic::BasicPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::basic::Basic>)), false)
        }
    });;
        if ok {
            {
        { let new_val = { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).kind.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y }; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::array::ArrayPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::array::ArrayPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::array::Array>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::array::Array>)), false)
        }
    });;
        if ok {
            {
        { let new_val = ({ let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).len.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y }) && self.nify((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::slice::SlicePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::slice::SlicePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::slice::Slice>)), false)
        }
    });;
        if ok {
            {
        { let new_val = self.nify((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#struct::StructPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
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
        if ok {
            if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; let __tmp_y = { let __recv = y.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.num_fields(); __result }; __tmp_x == __tmp_y } {
        { let __range_holder = (*x.lock().unwrap().as_ref().unwrap()).fields.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        let mut g = { let __seq = { let __seq_holder = (*y.lock().unwrap().as_ref().unwrap()).fields.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if { let __tmp_x = (*{ let __field = (*f.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*g.lock().unwrap().as_ref().unwrap()).embedded.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __recv = x.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.tag(Arc::new(Mutex::new(Some(i as i32)))); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = y.clone(); let __recv_ptr: *const crate::r#struct::Struct = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#struct::Struct }; let __result = unsafe { &*__recv_ptr }.tag(Arc::new(Mutex::new(Some(i as i32)))); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || !{ let __recv = f.clone(); let __recv_ptr: *const crate::object::Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Var }; let __result = unsafe { &*__recv_ptr }.same_id({ let __field = (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, { let __field = (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(false)))); __result } || !self.nify((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    } }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::pointer::PointerPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::pointer::PointerPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::pointer::Pointer>)), false)
        }
    });;
        if ok {
            {
        { let new_val = self.nify((*x.lock().unwrap().as_ref().unwrap()).base.clone(), (*y.lock().unwrap().as_ref().unwrap()).base.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::tuple::TuplePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::tuple::TuplePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::tuple::Tuple>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::tuple::Tuple>)), false)
        }
    });;
        if ok {
            if { let __tmp_x = { let __recv = x.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = { let __recv = y.clone(); let __recv_ptr: *const crate::tuple::Tuple = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::tuple::Tuple }; let __result = unsafe { &*__recv_ptr }.len(); __result }; __tmp_x == __tmp_y } {
        if (*x.lock().unwrap()).is_some() {
        { let __range_holder = (*x.lock().unwrap().as_ref().unwrap()).vars.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, v) in __range_values.iter().enumerate() {
        let mut w = { let __seq = { let __seq_holder = (*y.lock().unwrap().as_ref().unwrap()).vars.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if !self.nify((*(*v.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*w.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    } }
    }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
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
        if ok {
            {
        { let new_val = { let __tmp_x = (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*y.lock().unwrap().as_ref().unwrap()).variadic.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && self.nify(Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*x.lock().unwrap().as_ref().unwrap()).params.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*y.lock().unwrap().as_ref().unwrap()).params.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) && self.nify(Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*x.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(crate::tuple::TuplePtr((*y.lock().unwrap().as_ref().unwrap()).results.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::interface::InterfacePtr>()).unwrap().0.clone();
        assert(Arc::new(Mutex::new(Some(!((*self.enable_interface_inference.clone().lock().unwrap().as_ref().unwrap())) || { let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & EXACT as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x != __tmp_y }))));;
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
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
        if ok {
            let mut xset = { let __recv = x.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            let mut yset = { let __recv = y.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };;
            if { let __tmp_x = (*{ let __field = (*xset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*yset.lock().unwrap().as_ref().unwrap()).comparable.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
            if !(*(*xset.lock().unwrap().as_ref().unwrap()).terms.lock().unwrap().as_ref().unwrap()).equal({ let __field = (*yset.lock().unwrap().as_ref().unwrap()).terms.clone(); __field }) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
            let mut a = (*xset.lock().unwrap().as_ref().unwrap()).methods.clone();;
            let mut b = (*yset.lock().unwrap().as_ref().unwrap()).methods.clone();;
            if { let __tmp_x = ((*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
        let mut q = Arc::new(Mutex::new(Some(ifacePair { x: x.clone(), y: y.clone(), prev: p.clone(), ..Default::default() })));
        while (*p.lock().unwrap()).is_some() {
        if { let __recv = p.clone(); let __recv_ptr: *const crate::predicates::ifacePair = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::predicates::ifacePair }; let __result = unsafe { &*__recv_ptr }.identical(q.clone()); __result } {
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).prev.clone(); p = new_val; };
    }
        if DEBUG {
        assert_sorted_methods(a.clone());
        assert_sorted_methods(b.clone());
    }
        { let __range_holder = a.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        let mut g = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone();
        if { let __tmp_x = (*{ let __recv = f.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = g.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.id(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || !self.nify((*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), (*(*g.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(), Arc::new(Mutex::new(Some(unifyMode(Arc::new(Mutex::new(Some(EXACT as u64))))))), q.clone()) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    } }
        {
        { let new_val = true; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::map::MapPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::map::MapPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::map::Map>)), false)
        }
    });;
        if ok {
            {
        { let new_val = self.nify((*x.lock().unwrap().as_ref().unwrap()).key.clone(), (*y.lock().unwrap().as_ref().unwrap()).key.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) && self.nify((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::chan::ChanPtr>()).unwrap().0.clone();
        {
        let (mut y, mut ok) = ({
        let val = y.clone();
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
        if ok {
            {
        { let new_val = ({ let __tmp_x = unifyMode(Arc::new(Mutex::new(Some(((*{ let __v = (*mode.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & EXACT as u64))))); let __tmp_y = unifyMode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } || { let __tmp_x = { let __selector_holder = (*x.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = { let __selector_holder = (*y.lock().unwrap().as_ref().unwrap()).dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x == __tmp_y }) && self.nify((*x.lock().unwrap().as_ref().unwrap()).elem.clone(), (*y.lock().unwrap().as_ref().unwrap()).elem.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = emode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        {
        let mut y = as_named(y.clone());;
        if (*y.lock().unwrap()).is_some() {
            let mut xargs = { let __recv = { let __recv = x.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };;
            let mut yargs = { let __recv = { let __recv = y.clone(); let __recv_ptr: *const crate::named::Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::named::Named }; let __result = unsafe { &*__recv_ptr }.type_args(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result };;
            if { let __tmp_x = ((*xargs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*yargs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    };
            { let __range_holder = xargs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, xarg) in __range_values.iter().enumerate() {
        if !self.nify(xarg.clone(), { let __seq = { let __seq_holder = yargs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), p.clone()) {
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
    } };
            {
        { let new_val = identical_origin(x.clone(), y.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        if DEBUG {
        assert(Arc::new(Mutex::new(Some((*self.as_bound_type_param(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(x.clone())) as Box<dyn Type + Send + Sync>)))).lock().unwrap()).is_none()))));
    };
        if ENABLE_CORE_TYPE_UNIFICATION {
        {
        let mut cx = core_type(Arc::new(Mutex::new(Some(Box::new(crate::typeparam::TypeParamPtr(x.clone())) as Box<dyn Type + Send + Sync>))));;
        if (*cx.lock().unwrap()).is_some() {
            if TRACE_INFERENCE {
        self.tracef(Arc::new(Mutex::new(Some("core %s \u{2261} %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = xorig.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = yorig.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>]))));
    };
            {
        { let new_val = self.nify(cx.clone(), yorig.clone(), Arc::new(Mutex::new(Some(unifyMode(Arc::new(Mutex::new(Some(ASSIGN as u64))))))), p.clone()); *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    };
        }
    }
    };
    } else if _ts_is_nil {
        let x = x.clone();
    } else {
        let x = x.clone();
        panic!("{}", (*sprintf(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some("u.nify(%s, %s, %d)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = xorig.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new({ let __arg_holder = yorig.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new((*mode.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Any + Send + Sync>])))).lock().unwrap().as_ref().unwrap()));;
    }
    }
                // Basic types are singletons except for the rune and byte
                // aliases, thus we cannot solely rely on the x == y check
                // above. See also comment in TypeName.IsAlias.
                // Two array types unify if they have the same array length
                // and their element types unify.
                // If one or both array lengths are unknown (< 0) due to some error,
                // assume they are the same to avoid spurious follow-on errors.
                // Two slice types unify if their element types unify.
                // Two struct types unify if they have the same sequence of fields,
                // and if corresponding fields have the same names, their (field) types unify,
                // and they have identical tags. Two embedded fields are considered to have the same
                // name. Lower-case field names from different packages are always different.
                // Two pointer types unify if their base types unify.
                // Two tuples types unify if they have the same number of elements
                // and the types of corresponding elements unify.
                // Two function types unify if they have the same number of parameters
                // and result values, corresponding parameter and result types unify,
                // and either both functions are variadic or neither is.
                // Parameter and result names are not required to match.
                // TODO(gri) handle type parameters or document why we can ignore them.
                // handled before this switch
                // Two interface types unify if they have the same set of methods with
                // the same names, and corresponding function types unify.
                // Lower-case method names from different packages are always different.
                // The order of the methods is irrelevant.
                // Interface types are the only types where cycles can occur
                // that are not "terminated" via named types; and such cycles
                // can only be created via method parameter types that are
                // anonymous interfaces (directly or indirectly) embedding
                // the current interface. Example:
                //
                //    type T interface {
                //        m() interface{T}
                //    }
                //
                // If two such (differently named) interfaces are compared,
                // endless recursion occurs if the cycle is not detected.
                //
                // If x and y were compared before, they must be equal
                // (if they were not, the recursion would have stopped);
                // search the ifacePair stack for the same pair.
                //
                // This is a quadratic algorithm, but in practice these stacks
                // are extremely short (bounded by the nesting depth of interface
                // type declarations that recur via parameter types, an extremely
                // rare occurrence). An alternative implementation might use a
                // "visited" map, but that is probably less efficient overall.
                // same pair was compared before
                // Two map types unify if their key and value types unify.
                // Two channel types unify if their value types unify
                // and if they have the same direction.
                // The channel direction is ignored for inexact unification.
                // Two named types unify if their type names originate in the same type declaration.
                // If they are instantiated, their type argument lists must unify.
                // Check type arguments before origins so they unify
                // even if the origins don't match; for better error
                // messages (see go.dev/issue/53692).
                // x must be an unbound type parameter (see comment above).
                // By definition, a valid type argument must be in the type set of
                // the respective type constraint. Therefore, the type argument's
                // underlying type must be in the set of underlying types of that
                // constraint. If there is a single such underlying type, it's the
                // constraint's core type. It must match the type argument's under-
                // lying type, irrespective of whether the actual type argument,
                // which may be a defined type, is actually in the type set (that
                // will be determined at instantiation time).
                // Thus, if we have the core type of an unbound type parameter,
                // we know the structure of the possible types satisfying such
                // parameters. Use that core type for further unification
                // (see go.dev/issue/50755 for a test case).
                // Because the core type is always an underlying type,
                // unification will take care of matching against a
                // defined or literal type automatically.
                // If y is also an unbound type parameter, we will end
                // up here again with x and y swapped, so we don't
                // need to take care of that case separately.
                // If y is a defined type, it may not match against cx which
                // is an underlying type (incl. int, string, etc.). Use assign
                // mode here so that the unifier automatically takes under(y)
                // if necessary.
                // x != y and there's nothing to do
                // avoid a crash in case of nil type
        {
        { let new_val = false; *result.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (*result.lock().unwrap().as_ref().unwrap());
    }
    }
}

impl typeParamsById {
    pub fn len(&self) -> i32 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i32;
    }

    pub fn less(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) -> bool {
        return { let __tmp_x = (*(*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*(*{ let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y };
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i32>>>, j: Arc<Mutex<Option<i32>>>) {
        { let __tmp_0 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_1 = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_0; (*self.0.clone().lock().unwrap().as_mut().unwrap())[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = __tmp_1; };
    }
}

/// newUnifier returns a new unifier initialized with the given type parameter
/// and corresponding type argument lists. The type argument list may be shorter
/// than the type parameter list, and it may contain nil types. Matching type
/// parameters and arguments must have the same index.
pub fn new_unifier(tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, enableInterfaceInference: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<unifier>>> {
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*tparams.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y }))));
    let mut handles = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<crate::typeparam::TypeParam>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>::new())));

        // Allocate all handles up-front: in a correct program, all type parameters
        // must be resolved and thus eventually will get a handle.
        // Also, sharing of handles caused by unified type parameters is rare and
        // so it's ok to not optimize for that case (and delay handle allocation).
    { let __range_holder = tparams.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, x) in __range_values.iter().enumerate() {
        let mut t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __tmp_x = (i as i32); let __tmp_y = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let __iface_handle = { let __seq = { let __seq_holder = targs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *t.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let __map_key = GoLocalPtrKey::new(x.clone()); let __map_value = t.clone(); (*handles.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    return Arc::new(Mutex::new(Some(unifier { handles: handles.clone(), depth: Arc::new(Mutex::new(Some(0))), enable_interface_inference: Arc::new(Mutex::new(Some({ let __arg_holder = enableInterfaceInference.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
}

/// asInterface returns the underlying type of x as an interface if
/// it is a non-type parameter interface. Otherwise it returns nil.
pub fn as_interface(x: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<crate::interface::Interface>>> {
    let mut i: Arc<Mutex<Option<Interface>>> = Arc::new(Mutex::new(None));

    {
        let (_, mut ok) = ({
        let val = unalias(x.clone()).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::typeparam::TypeParamPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::typeparam::TypeParam>)), false)
        }
    });;
        if !ok {
            { let (__tmp_0, __tmp_1) = ({
        let val = under(x.clone()).clone();
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
    }); i = __tmp_0.clone(); };;
        }
    }
    i.clone()
}

impl GoValueClone for unifier {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
