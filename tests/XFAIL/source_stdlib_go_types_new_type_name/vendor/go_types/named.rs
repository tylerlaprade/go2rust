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
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const UNRESOLVED: u32 = 0;
pub(crate) const RESOLVED: u32 = 1;
pub(crate) const COMPLETE: u32 = 2;


/// A Named represents a named (defined) type.
///
/// A declaration such as:
///
///	type S struct { ... }
///
/// creates a defined type whose underlying type is a struct,
/// and binds this type to the object S, a [TypeName].
/// Use [Named.Underlying] to access the underlying type.
/// Use [Named.Obj] to obtain the object S.
///
/// Before type aliases (Go 1.9), the spec called defined types "named types".
#[derive(Clone)]
pub struct Named {
    pub check: Arc<Mutex<Option<Checker>>>,
    pub obj: Arc<Mutex<Option<TypeName>>>,
    pub from_r_h_s: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub inst: Arc<Mutex<Option<instance>>>,
    pub mu: GoMutex,
    pub state_: Arc<Mutex<Option<u32>>>,
    pub underlying: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub tparams: Arc<Mutex<Option<TypeParamList>>>,
    pub methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>,
    pub loader: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Named>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) + Send + Sync>>>>,
}

impl Named {
    pub fn __go_value_clone(&self) -> Self {
        Self { check: self.check.clone(), obj: self.obj.clone(), from_r_h_s: self.from_r_h_s.clone(), inst: self.inst.clone(), mu: self.mu.clone(), state_: { let __guard = self.state_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, underlying: self.underlying.clone(), tparams: self.tparams.clone(), methods: self.methods.clone(), loader: self.loader.clone() }
    }
}


impl Default for Named {
    fn default() -> Self {
        Self { check: Arc::new(Mutex::new(None)), obj: Arc::new(Mutex::new(None)), from_r_h_s: Arc::new(Mutex::new(None)), inst: Arc::new(Mutex::new(None)), mu: GoMutex::new(), state_: Arc::new(Mutex::new(Some(0))), underlying: Arc::new(Mutex::new(None)), tparams: Arc::new(Mutex::new(None)), methods: Arc::new(Mutex::new(None)), loader: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Named {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Named {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// instance holds information that is only necessary for instantiated named
/// types.
#[derive(Clone)]
pub struct instance {
    pub orig: Arc<Mutex<Option<Named>>>,
    pub targs: Arc<Mutex<Option<TypeList>>>,
    pub expanded_methods: Arc<Mutex<Option<i32>>>,
    pub ctxt: Arc<Mutex<Option<Context>>>,
}

impl instance {
    pub fn __go_value_clone(&self) -> Self {
        Self { orig: self.orig.clone(), targs: self.targs.clone(), expanded_methods: { let __guard = self.expanded_methods.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ctxt: self.ctxt.clone() }
    }
}


impl Default for instance {
    fn default() -> Self {
        Self { orig: Arc::new(Mutex::new(None)), targs: Arc::new(Mutex::new(None)), expanded_methods: Arc::new(Mutex::new(Some(0))), ctxt: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for instance {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", { let __guard = self.orig.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.targs.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.expanded_methods.lock().unwrap().as_ref().unwrap()), { let __guard = self.ctxt.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for instance {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// namedState represents the possible states that a named type may assume.
#[derive(Debug, Clone, Default)]
pub struct namedState(pub Arc<Mutex<Option<u32>>>);

impl Display for namedState {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for namedState {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for namedState {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for namedState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for namedState {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<namedState> for u32 {
    fn eq(&self, other: &namedState) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<namedState> for u32 {
    fn partial_cmp(&self, other: &namedState) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for namedState {
    type Output = namedState;
    fn add(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for namedState {
    type Output = namedState;
    fn add(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<namedState> for u32 {
    type Output = namedState;
    fn add(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for namedState {
    type Output = namedState;
    fn sub(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for namedState {
    type Output = namedState;
    fn sub(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<namedState> for u32 {
    type Output = namedState;
    fn sub(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for namedState {
    type Output = namedState;
    fn mul(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for namedState {
    type Output = namedState;
    fn mul(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<namedState> for u32 {
    type Output = namedState;
    fn mul(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for namedState {
    type Output = namedState;
    fn div(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for namedState {
    type Output = namedState;
    fn div(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<namedState> for u32 {
    type Output = namedState;
    fn div(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for namedState {
    type Output = namedState;
    fn rem(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for namedState {
    type Output = namedState;
    fn rem(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<namedState> for u32 {
    type Output = namedState;
    fn rem(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for namedState {
    type Output = namedState;
    fn bitand(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for namedState {
    type Output = namedState;
    fn bitand(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<namedState> for u32 {
    type Output = namedState;
    fn bitand(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for namedState {
    type Output = namedState;
    fn bitor(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for namedState {
    type Output = namedState;
    fn bitor(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<namedState> for u32 {
    type Output = namedState;
    fn bitor(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for namedState {
    type Output = namedState;
    fn bitxor(self, other: Self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for namedState {
    type Output = namedState;
    fn bitxor(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<namedState> for u32 {
    type Output = namedState;
    fn bitxor(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for namedState {
    type Output = namedState;
    fn not(self) -> namedState {
        namedState(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for namedState {
    type Output = namedState;
    fn shl(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for namedState {
    type Output = namedState;
    fn shl(self, other: i32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for namedState {
    type Output = namedState;
    fn shl(self, other: i8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for namedState {
    type Output = namedState;
    fn shl(self, other: i16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for namedState {
    type Output = namedState;
    fn shl(self, other: i64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for namedState {
    type Output = namedState;
    fn shl(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for namedState {
    type Output = namedState;
    fn shl(self, other: u8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for namedState {
    type Output = namedState;
    fn shl(self, other: u16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for namedState {
    type Output = namedState;
    fn shl(self, other: u64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for namedState {
    type Output = namedState;
    fn shl(self, other: usize) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for namedState {
    type Output = namedState;
    fn shr(self, other: namedState) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for namedState {
    type Output = namedState;
    fn shr(self, other: i32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for namedState {
    type Output = namedState;
    fn shr(self, other: i8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for namedState {
    type Output = namedState;
    fn shr(self, other: i16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for namedState {
    type Output = namedState;
    fn shr(self, other: i64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for namedState {
    type Output = namedState;
    fn shr(self, other: u32) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for namedState {
    type Output = namedState;
    fn shr(self, other: u8) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for namedState {
    type Output = namedState;
    fn shr(self, other: u16) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for namedState {
    type Output = namedState;
    fn shr(self, other: u64) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for namedState {
    type Output = namedState;
    fn shr(self, other: usize) -> namedState {
        namedState(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for namedState {}

impl Ord for namedState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl Named {
    /// resolve resolves the type parameters, methods, and underlying type of n.
    /// This information may be loaded from a provided loader function, or computed
    /// from an origin type (in the case of instances).
    ///
    /// After resolution, the type parameters, methods, and underlying type of n are
    /// accessible; but if n is an instantiated type, its methods may still be
    /// unexpanded.
    pub fn resolve(&mut self) -> Arc<Mutex<Option<Named>>> {
        if { let __tmp_x = (*self.state().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = namedState(Arc::new(Mutex::new(Some(RESOLVED as u32)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // TODO(rfindley): if n.check is non-nil we can avoid locking here, since
                // type-checking is not concurrent. Evaluate if this is worth doing.
        let __mutex_guard_source_8513 = self.mu.clone(); let __mutex_guard_8513 = __mutex_guard_source_8513.guard();
        // mu.Unlock() handled by RAII guard
        if { let __tmp_x = (*self.state().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = namedState(Arc::new(Mutex::new(Some(RESOLVED as u32)))); __tmp_x >= __tmp_y } {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        if { let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = self.underlying.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.loader.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        let mut orig = (*self.inst.lock().unwrap().as_ref().unwrap()).orig.clone();
        { let __recv = orig.clone(); let __recv_ptr: *mut Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Named }; let __result = unsafe { &mut *__recv_ptr }.resolve(); __result };
        let mut underlying = self.expand_underlying();
        { let new_val = (*orig.lock().unwrap().as_ref().unwrap()).tparams.clone(); self.tparams = new_val; };
        { let __iface_handle = underlying.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.underlying.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = (*orig.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.from_r_h_s.lock().unwrap() = (*__iface_guard).clone(); };
        if { let __tmp_x = (({ let __len_target = { let __field = (*orig.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        self.set_state(Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some(COMPLETE as u32))))))));
        *(*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.lock().unwrap() = None;
    } else {
        self.set_state(Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some(RESOLVED as u32))))))));
    }
                // nothing further to do
        return Arc::new(Mutex::new(Some(self.clone())));
    }
                // n is an unresolved instance
                // instances are created by instantiation, in which case n.loader is nil
                // for cycle detection
                // nothing further to do
                // TODO(mdempsky): Since we're passing n to the loader anyway
                // (necessary because types2 expects the receiver type for methods
                // on defined interface types to be the Named rather than the
                // underlying Interface), maybe it should just handle calling
                // SetTypeParams, SetUnderlying, and AddMethod instead?  Those
                // methods would need to support reentrant calls though. It would
                // also make the API more future-proof towards further extensions.
        if { let __nil_target = self.loader.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = self.underlying.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() }))));
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __recv = self.type_args(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y }))));
        let (mut tparams, mut underlying, mut methods) = { let __f_holder = self.loader.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Named>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Named>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>, Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(self.clone())))) };
        { let new_val = bind_t_params(tparams.clone()).clone(); self.tparams = new_val; };
        { let __iface_handle = underlying.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.underlying.lock().unwrap() = (*__iface_guard).clone(); };
        { let __iface_handle = underlying.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.from_r_h_s.lock().unwrap() = (*__iface_guard).clone(); };
        { let new_val = methods.clone(); self.methods = new_val; };
        *self.loader.lock().unwrap() = None;
    }
                // instances are created by instantiation, in which case n.loader is nil
                // for cycle detection
        self.set_state(Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some(COMPLETE as u32))))))));
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// state atomically accesses the current state of the receiver.
    pub fn state(&self) -> Arc<Mutex<Option<namedState>>> {
        Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __target = self.state_.clone(); let __guard = __target.lock().unwrap(); *__guard.as_ref().unwrap() }))).lock().unwrap().as_ref().unwrap()) as u32)))))))
    }

    /// setState atomically stores the given state for n.
    /// Must only be called while holding n.mu.
    pub fn set_state(&self, state: Arc<Mutex<Option<namedState>>>) {
        { let __target = self.state_.clone(); let __stored = (*{ let __v = (*state.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as u32 as u32; let mut __guard = __target.lock().unwrap(); *__guard.as_mut().unwrap() = __stored; };
    }

    pub fn cleanup(&mut self) {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } || { let __nil_target = (*(*self.inst.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
                // Ensure that every defined type created in the course of type-checking has
                // either non-*Named underlying type, or is unexpanded.
                //
                // This guarantees that we don't leak any types whose underlying type is
                // *Named, because any unexpanded instances will lazily compute their
                // underlying type by substituting in the underlying type of their origin.
                // The origin must have either been imported or type-checked and expanded
                // here, and in either case its underlying type will be fully expanded.
        {
    let _ts_subject = self.underlying.clone();
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
        if { let __tmp_x = { let __recv = self.type_args(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        panic!("nil underlying");
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() {
        self.under();;
    }
    }
                // t.under may add entries to check.cleaners
        *self.check.lock().unwrap() = None;
    }

    /// Obj returns the type name for the declaration defining the named type t. For
    /// instantiated types, this is same as the type name of the origin type.
    pub fn obj(&self) -> Arc<Mutex<Option<crate::object::TypeName>>> {
        if { let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return self.obj.clone();
    }
        (*(*self.inst.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).obj.clone()
    }

    /// Origin returns the generic type from which the named type t is
    /// instantiated. If t is not an instantiated type, the result is t.
    pub fn origin(&self) -> Arc<Mutex<Option<Named>>> {
        if { let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some(self.clone())));
    }
        (*self.inst.lock().unwrap().as_ref().unwrap()).orig.clone()
    }

    /// TypeParams returns the type parameters of the named type t, or nil.
    /// The result is non-nil for an (originally) generic type even if it is instantiated.
    pub fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        (*self.resolve().lock().unwrap().as_ref().unwrap()).tparams.clone()
    }

    /// SetTypeParams sets the type parameters of the named type t.
    /// t must not have type arguments.
    pub fn set_type_params(&mut self, tparams: Arc<Mutex<Option<Vec<Arc<Mutex<Option<TypeParam>>>>>>>) {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        { let new_val = bind_t_params(tparams.clone()).clone(); (*self.resolve().lock().unwrap().as_mut().unwrap()).tparams = new_val; };
    }

    /// TypeArgs returns the type arguments used to instantiate the named type t.
    pub fn type_args(&self) -> Arc<Mutex<Option<crate::typelists::TypeList>>> {
        if { let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
        (*self.inst.lock().unwrap().as_ref().unwrap()).targs.clone()
    }

    /// NumMethods returns the number of explicit methods defined for t.
    pub fn num_methods(&self) -> i32 {
        ({ let __len_target = { let __field = (*{ let __recv = self.origin(); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).resolve(); __result }.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Method returns the i'th method of named type t for 0 <= i < t.NumMethods().
    ///
    /// For an ordinary or instantiated type t, the receiver base type of this
    /// method is the named type t. For an uninstantiated generic type t, each
    /// method receiver is instantiated with its receiver type parameters.
    ///
    /// Methods are numbered deterministically: given the same list of source files
    /// presented to the type checker, or the same sequence of NewMethod and AddMethod
    /// calls, the mapping from method index to corresponding method remains the same.
    /// But the specific ordering is not specified and must not be relied on as it may
    /// change in the future.
    pub fn method(&mut self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
        self.resolve();
        if { let __tmp_x = (*self.state().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = namedState(Arc::new(Mutex::new(Some(COMPLETE as u32)))); __tmp_x >= __tmp_y } {
        return { let __seq = { let __seq_holder = self.methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
    }
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }))));
        let mut orig = (*self.inst.lock().unwrap().as_ref().unwrap()).orig.clone();
        let __mutex_guard_source_14753 = self.mu.clone(); let __mutex_guard_14753 = __mutex_guard_source_14753.guard();
        // mu.Unlock() handled by RAII guard
        if { let __tmp_x = (({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = (*orig.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x != __tmp_y } {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y }))));
        { let new_val = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None)); (({ let __len_target = { let __field = (*orig.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize]))); self.methods = new_val; };
    }
        if (*{ let __seq = { let __seq_holder = self.methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap()).is_none() {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = (*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }))));
        (*self.methods.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = self.expand_method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __target = (*self.inst.lock().unwrap().as_ref().unwrap()).expanded_methods.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
                // Check if we've created all methods at this point. If we have, mark the
                // type as fully expanded.
        if { let __tmp_x = ((*(*self.inst.lock().unwrap().as_ref().unwrap()).expanded_methods.lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = (*orig.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x == __tmp_y } {
        self.set_state(Arc::new(Mutex::new(Some(namedState(Arc::new(Mutex::new(Some(COMPLETE as u32))))))));
        *(*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.lock().unwrap() = None;
    }
    }
                // we should still have a context remaining from the resolution phase
                // Check if we've created all methods at this point. If we have, mark the
                // type as fully expanded.
                // no need for a context anymore
        { let __seq = { let __seq_holder = self.methods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    /// expandMethod substitutes type arguments in the i'th method for an
    /// instantiated receiver.
    pub fn expand_method(&mut self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<crate::object::Func>>> {
                // t.orig.methods is not lazy. origm is the method instantiated with its
                // receiver type parameters (the "origin" method).
        let mut origm = (*(*self.inst.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_mut().unwrap()).method(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        assert(Arc::new(Mutex::new(Some((*origm.lock().unwrap()).is_some()))));
        let mut check = self.check.clone();
                // Ensure that the original method is type-checked.
        if (*check.lock().unwrap()).is_some() {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.obj_decl(Arc::new(Mutex::new(Some(Box::new(crate::object::FuncPtr(origm.clone())) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None))); __result };
    }
        let mut origSig = ({
        let val = (*(*origm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        let (mut rbase, _) = deref({ let __recv = { let __recv = origSig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone());
                // If rbase is t, then origm is already the instantiated method we're looking
                // for. In this case, we return origm to preserve the invariant that
                // traversing Method->Receiver Type->Method should get back to the same
                // method.
                //
                // This occurs if t is instantiated with the receiver type parameters, as in
                // the use of m in func (r T[_]) m() { r.m() }.
        if { let __left_holder = rbase.clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Type + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_wrapper = NamedPtr(Arc::new(Mutex::new(Some(self.clone())))); let __right_opt: Option<&(dyn Type + Send + Sync)> = Some(&__right_wrapper as &(dyn Type + Send + Sync)); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        return origm.clone();
    }
        let mut sig = origSig.clone();
                // We can only substitute if we have a correspondence between type arguments
                // and type parameters. This check is necessary in the presence of invalid
                // code.
        if { let __tmp_x = { let __recv = { let __recv = origSig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv_type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = (*(*self.inst.lock().unwrap().as_ref().unwrap()).targs.lock().unwrap().as_ref().unwrap()).len(); __tmp_x == __tmp_y } {
        let mut smap = make_subst_map({ let __recv = { let __recv = origSig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv_type_params(); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, (*(*self.inst.lock().unwrap().as_ref().unwrap()).targs.lock().unwrap().as_ref().unwrap()).list());
        let mut ctxt: Arc<Mutex<Option<Context>>> = Arc::new(Mutex::new(None));
        if (*check.lock().unwrap()).is_some() {
        { let new_val = { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.context(); __result }.clone(); ctxt = new_val; };
    }
        { let new_val = ({
        let val = { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.subst({ let __field = (*(*origm.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pos.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(origSig.clone())) as Box<dyn Type + Send + Sync>))), smap.clone(), Arc::new(Mutex::new(Some(self.clone()))), ctxt.clone()); __result }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::signature::SignaturePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(); sig = new_val; };
    }
        if { let __left = sig.clone(); let __right = origSig.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
                // No substitution occurred, but we still need to create a new signature to
                // hold the instantiated receiver.
        let mut copy = Arc::new(Mutex::new(Some({ let __v = (*origSig.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let new_val = copy.clone().clone(); sig = new_val; };
    }
                // No substitution occurred, but we still need to create a new signature to
                // hold the instantiated receiver.
        let mut rtyp: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
        if { let __recv = origm.clone(); let __recv_ptr: *const crate::object::Func = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::object::Func }; let __result = unsafe { &*__recv_ptr }.has_ptr_recv(); __result } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::pointer::PointerPtr(new_pointer(Arc::new(Mutex::new(Some(Box::new(NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))).clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *rtyp.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *rtyp.lock().unwrap() = (*__iface_guard).clone(); };
    }
        { let new_val = clone_var({ let __field = (*origSig.lock().unwrap().as_ref().unwrap()).recv.clone(); __field }, rtyp.clone()).clone(); (*sig.lock().unwrap().as_mut().unwrap()).recv = new_val; };
        return clone_func(origm.clone(), Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))));
    }

    /// SetUnderlying sets the underlying type and marks t as complete.
    /// t must not have type arguments.
    pub fn set_underlying(&mut self, underlying: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        if (*underlying.lock().unwrap()).is_none() {
        panic!("underlying type must not be nil");
    }
        if (*as_named(underlying.clone()).lock().unwrap()).is_some() {
        panic!("underlying type must not be *Named");
    }
        { let __iface_handle = underlying.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*self.resolve().lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = (*__iface_guard).clone(); };
        if { let __iface_handle = { let __field = self.from_r_h_s.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = underlying.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.from_r_h_s.lock().unwrap() = (*__iface_guard).clone(); };
    }
    }

    /// AddMethod adds method m unless it is already in the method list.
    /// The method must be in the same package as t, and t must not have
    /// type arguments.
    pub fn add_method(&mut self, m: Arc<Mutex<Option<Func>>>) {
        assert(Arc::new(Mutex::new(Some(same_pkg({ let __field = (*self.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); __field }, { let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field })))));
        assert(Arc::new(Mutex::new(Some({ let __nil_target = self.inst.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }))));
        self.resolve();
        if { let __tmp_x = self.method_index({ let __field = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); __field }, Arc::new(Mutex::new(Some(false)))); let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __append_target = self.methods.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(m.clone()); __append_target.clone() }; self.methods = new_val; };
    }
    }

    /// methodIndex returns the index of the method with the given name.
    /// If foldCase is set, capitalization in the name is ignored.
    /// The result is negative if no such method exists.
    pub fn method_index(&self, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> i32 {
        if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "_".to_string(); __tmp_x == __tmp_y } {
        return -(1);
    }
        if { let __v = (*foldCase.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __range_holder = self.methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, m) in __range_values.iter().enumerate() {
        if (*Arc::new(Mutex::new(Some({ let __a = (*(*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); let __b = (*name.lock().unwrap().as_ref().unwrap()).clone(); __a.to_lowercase() == __b.to_lowercase() }))).lock().unwrap().as_ref().unwrap()) {
        return i as i32;
    }
    } }
    } else {
        { let __range_holder = self.methods.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, m) in __range_values.iter().enumerate() {
        if { let __tmp_x = { let __selector_holder = (*(*m.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return i as i32;
    }
    } }
    }
        -(1)
    }

    /// Underlying returns the [underlying type] of the named type t, resolving all
    /// forwarding declarations. Underlying types are never Named, TypeParam, or
    /// Alias types.
    ///
    /// [underlying type]: https://go.dev/ref/spec#Underlying_types.
    pub fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
                // TODO(gri) Investigate if Unalias can be moved to where underlying is set.
        unalias((*self.resolve().lock().unwrap().as_ref().unwrap()).underlying.clone()).clone()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    /// under returns the expanded underlying type of n0; possibly by following
    /// forward chains of named types. If an underlying type is found, resolve
    /// the chain by setting the underlying type for each defined type in the
    /// chain before returning it. If no underlying type is found or a cycle
    /// is detected, the result is Typ[Invalid]. If a cycle is detected and
    /// n0.check != nil, the cycle is reported.
    ///
    /// This is necessary because the underlying type of named may be itself a
    /// named type that is incomplete:
    ///
    ///	type (
    ///		A B
    ///		B *C
    ///		C A
    ///	)
    ///
    /// The type of C is the (named) type of A which is incomplete,
    /// and which has as its underlying type the named type B.
    pub fn under(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut u = self.underlying();
                // If the underlying type of a defined type is not a defined
                // (incl. instance) type, then that is the desired underlying
                // type.
        let mut n1: Arc<Mutex<Option<Named>>> = Arc::new(Mutex::new(None));
        {
    let _ts_subject = u.clone();
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
        let u1 = u.clone();
        panic!("nil underlying");;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<NamedPtr>()).is_some() {
        let u1 = _ts_val.and_then(|__v| __v.downcast_ref::<NamedPtr>()).unwrap().0.clone();
        { let new_val = u1.clone(); n1 = new_val; };;
    } else {
        let u1 = u.clone();
        return u.clone();;
    }
    }
                // After expansion via Underlying(), we should never encounter a nil
                // underlying.
                // common case
                // handled below
        if { let __nil_target = self.check.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        panic!("Named.check == nil but type is incomplete");
    }
                // Invariant: after this point n0 as well as any named types in its
                // underlying chain should be set up when this function exits.
        let mut check = self.check.clone();
        let mut n = Arc::new(Mutex::new(Some(self.clone())));
        let mut seen = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Named>, Arc<Mutex<Option<i32>>>>::new())));
        let mut path: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        'r#loop: loop {
        { let __map_key = GoLocalPtrKey::new(n.clone()); let __map_value = Arc::new(Mutex::new(Some((*seen.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32))); (*seen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let new_val = { let __append_target = path.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr((*n.lock().unwrap().as_ref().unwrap()).obj.clone())) as Box<dyn Object + Send + Sync>)))); __append_target.clone() }; path = new_val; };
        { let new_val = n1.clone(); n = new_val; };
        {
        let (mut i, mut ok) = { let __map = { let __map_holder = seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(n.clone()))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.cycle_error(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() }))), Arc::new(Mutex::new(Some(first_in_src(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = path.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_vec() })))))))); __result };;
            { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };;
            break;
        }
    }
                // cycle
        { let __iface_handle = { let __recv = n.clone(); let __recv_ptr: *mut Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Named }; let __result = unsafe { &mut *__recv_ptr }.underlying(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };
        {
    let _ts_subject = u.clone();
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
        let u1 = u.clone();
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *u.lock().unwrap() = (*__iface_guard).clone(); };;
        break 'r#loop;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<NamedPtr>()).is_some() {
        let u1 = _ts_val.and_then(|__v| __v.downcast_ref::<NamedPtr>()).unwrap().0.clone();
        { let new_val = u1.clone(); n1 = new_val; };;
    } else {
        let u1 = u.clone();
        break 'r#loop;
    }
    }
    }
                // cycle
                // Continue collecting *Named types in the chain.
        for (__range_key, _) in { let __range_holder = seen.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        let n = __range_key.value();
                // We should never have to update the underlying type of an imported type;
                // those underlying types should have been resolved during the import.
                // Also, doing so would lead to a race condition (was go.dev/issue/31749).
                // Do this check always, not just in debug mode (it's cheap).
        if { let __left = (*(*n.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = (*check.lock().unwrap().as_ref().unwrap()).pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq } {
        panic!("imported type with unresolved underlying type");
    }
        { let __iface_handle = u.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *(*n.lock().unwrap().as_mut().unwrap()).underlying.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // We should never have to update the underlying type of an imported type;
                // those underlying types should have been resolved during the import.
                // Also, doing so would lead to a race condition (was go.dev/issue/31749).
                // Do this check always, not just in debug mode (it's cheap).
        return u.clone();
    }

    pub fn lookup_method(&mut self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> (i32, Arc<Mutex<Option<crate::object::Func>>>) {
        self.resolve();
        if same_pkg({ let __field = (*self.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); __field }, pkg.clone()) || is_exported(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __v = (*foldCase.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // If n is an instance, we may not have yet instantiated all of its methods.
                // Look up the method index in orig, and only instantiate method at the
                // matching index (if any).
        {
        let mut i = { let __recv = self.origin(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).method_index(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = foldCase.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result };;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            return (i, self.method(Arc::new(Mutex::new(Some(i)))));;
        }
    }
    }
                // If n is an instance, we may not have yet instantiated all of its methods.
                // Look up the method index in orig, and only instantiate method at the
                // matching index (if any).
                // For instances, m.Method(i) will be different from the orig method.
        (-(1), Arc::new(Mutex::new(None)))
    }

    /// expandUnderlying substitutes type arguments in the underlying type n.orig,
    /// returning the result. Returns Typ[Invalid] if there was an error.
    pub fn expand_underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

        let mut check = self.check.clone();
        if (*check.lock().unwrap()).is_some() && (*(*(*check.lock().unwrap().as_ref().unwrap()).conf.lock().unwrap().as_ref().unwrap()).__trace.lock().unwrap().as_ref().unwrap()) {
        { let __recv = check.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.trace({ let __field = (*self.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.clone(); __field }, Arc::new(Mutex::new(Some("-- Named.expandUnderlying %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(self.clone()) as Box<dyn Any + Send + Sync>])))); __result };
        { let __target = (*check.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let check_defer_captured = check.clone(); let mut n_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Box::new(move || {
        { let __target = (*check_defer_captured.lock().unwrap().as_ref().unwrap()).indent.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        { let __recv = check_defer_captured.clone(); let __recv_ptr: *const crate::check::Checker = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::check::Checker }; let __result = unsafe { &*__recv_ptr }.trace({ let __field = (*n_defer_captured.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.clone(); __field }, Arc::new(Mutex::new(Some("=> %s (tparams = %s, under = %s)".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(n_defer_captured.clone()) as Box<dyn Any + Send + Sync>, Box::new((*n_defer_captured.tparams.lock().unwrap().as_ref().unwrap()).list().clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = n_defer_captured.underlying.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };
    }) as Box<dyn FnMut() -> () + Send + Sync>))); let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
    }));
    }
        assert(Arc::new(Mutex::new(Some({ let __iface_handle = { let __field = (*(*self.inst.lock().unwrap().as_ref().unwrap()).orig.lock().unwrap().as_ref().unwrap()).underlying.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() }))));
        if { let __nil_target = (*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = new_context().clone(); (*self.inst.lock().unwrap().as_mut().unwrap()).ctxt = new_val; };
    }
        let mut orig = (*self.inst.lock().unwrap().as_ref().unwrap()).orig.clone();
        let mut targs = (*self.inst.lock().unwrap().as_ref().unwrap()).targs.clone();
        if (*as_named((*orig.lock().unwrap().as_ref().unwrap()).underlying.clone()).lock().unwrap()).is_some() {
                // We should only get a Named underlying type here during type checking
                // (for example, in recursive type declarations).
        assert(Arc::new(Mutex::new(Some((*check.lock().unwrap()).is_some()))));
    }
                // We should only get a Named underlying type here during type checking
                // (for example, in recursive type declarations).
        if { let __tmp_x = (*(*orig.lock().unwrap().as_ref().unwrap()).tparams.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = { let __recv = targs.clone(); let __recv_ptr: *const crate::typelists::TypeList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeList }; let __result = unsafe { &*__recv_ptr }.len(); __result }; __tmp_x != __tmp_y } {
                // Mismatching arg and tparam length may be checked elsewhere.
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>)));
    }
    }
                // Mismatching arg and tparam length may be checked elsewhere.
                // Ensure that an instance is recorded before substituting, so that we
                // resolve n for any recursive references.
        let mut h = (*(*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.lock().unwrap().as_mut().unwrap()).instance_hash(Arc::new(Mutex::new(Some(Box::new(NamedPtr(orig.clone())) as Box<dyn Type + Send + Sync>))), { let __recv = targs.clone(); let __recv_ptr: *const crate::typelists::TypeList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeList }; let __result = unsafe { &*__recv_ptr }.list(); __result });
        let mut n2 = (*(*self.inst.lock().unwrap().as_ref().unwrap()).ctxt.lock().unwrap().as_mut().unwrap()).update(Arc::new(Mutex::new(Some({ let __arg_holder = h.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(NamedPtr(orig.clone())) as Box<dyn Type + Send + Sync>))), { let __recv = self.type_args(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }, Arc::new(Mutex::new(Some(Box::new(NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))));
        assert(Arc::new(Mutex::new(Some({ let __left_wrapper = NamedPtr(Arc::new(Mutex::new(Some(self.clone())))); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = n2.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq }))));
        let mut smap = make_subst_map((*(*orig.lock().unwrap().as_ref().unwrap()).tparams.lock().unwrap().as_ref().unwrap()).list(), { let __recv = targs.clone(); let __recv_ptr: *const crate::typelists::TypeList = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::typelists::TypeList }; let __result = unsafe { &*__recv_ptr }.list(); __result });
        let mut ctxt: Arc<Mutex<Option<Context>>> = Arc::new(Mutex::new(None));
        if (*check.lock().unwrap()).is_some() {
        { let new_val = { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.context(); __result }.clone(); ctxt = new_val; };
    }
        let mut underlying = (*self.check.lock().unwrap().as_ref().unwrap()).subst({ let __field = (*self.obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pos.clone(); __field }, (*orig.lock().unwrap().as_ref().unwrap()).underlying.clone(), smap.clone(), Arc::new(Mutex::new(Some(self.clone()))), ctxt.clone());
                // If the underlying type of n is an interface, we need to set the receiver of
                // its methods accurately -- we set the receiver of interface methods on
                // the RHS of a type declaration to the defined type.
        {
        let (mut iface, _) = ({
        let val = underlying.clone();
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
        if (*iface.lock().unwrap()).is_some() {
            {
        let (mut methods, mut copied) = replace_recv_type({ let __field = (*iface.lock().unwrap().as_ref().unwrap()).methods.clone(); __field }, Arc::new(Mutex::new(Some(Box::new(NamedPtr(orig.clone())) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(NamedPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))));;
        if copied {
            if { let __left_wrapper = crate::interface::InterfacePtr(iface.clone()); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = (*orig.lock().unwrap().as_ref().unwrap()).underlying.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } {
        let mut old = iface.clone();
        { let new_val = { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.new_interface(); __result }.clone(); iface = new_val; };
        { let new_val = (*old.lock().unwrap().as_ref().unwrap()).embeddeds.clone(); (*iface.lock().unwrap().as_mut().unwrap()).embeddeds = new_val; };
        assert({ let __field = (*old.lock().unwrap().as_ref().unwrap()).complete.clone(); __field });
        { let new_val = { let __selector_holder = (*old.lock().unwrap().as_ref().unwrap()).complete.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*iface.lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*old.lock().unwrap().as_ref().unwrap()).implicit.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*iface.lock().unwrap().as_ref().unwrap()).implicit.lock().unwrap() = Some(new_val); };
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(iface.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *underlying.lock().unwrap() = (*__iface_guard).clone(); };
    };
            { let new_val = methods.clone(); (*iface.lock().unwrap().as_mut().unwrap()).methods = new_val; };;
            *(*iface.lock().unwrap().as_ref().unwrap()).tset.lock().unwrap() = None;;
            if (*check.lock().unwrap()).is_none() {
        { let __recv = iface.clone(); let __recv_ptr: *const crate::interface::Interface = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::interface::Interface }; let __result = unsafe { &*__recv_ptr }.type_set(); __result };
    };
        }
    };
        }
    }
                // If the underlying type doesn't actually use type parameters, it's
                // possible that it wasn't substituted. In this case we need to create
                // a new *Interface before modifying receivers.
                // otherwise we are copying incomplete data
                // should be false but be conservative
                // recompute type set with new methods
                // If check != nil, check.newInterface will have saved the interface for later completion.
                // golang/go#61561: all newly created interfaces must be fully evaluated
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return underlying.clone();
    }
    }
}

impl Type for Named {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Named::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Named::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Named>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct NamedPtr(pub Arc<Mutex<Option<Named>>>);

impl std::fmt::Display for NamedPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for NamedPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Named::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Named::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NamedPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl cleaner for Named {
    fn cleanup(&mut self) {
        Named::cleanup(self)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Named>() {
            false
        } else {
            false
        }
    }
}

impl cleaner for NamedPtr {
    fn cleanup(&mut self) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Named::cleanup(__recv)
    }
    fn __go_clone_box_cleaner(&self) -> Box<dyn cleaner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cleaner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_cleaner(&self, other: &(dyn cleaner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NamedPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl genericType for Named {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        Named::type_params(self)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Named>() {
            false
        } else {
            false
        }
    }
}

impl genericType for NamedPtr {
    fn type_params(&mut self) -> Arc<Mutex<Option<crate::typelists::TypeParamList>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Named::type_params(__recv)
    }
    fn __go_clone_box_generic_type(&self) -> Box<dyn genericType + Send + Sync> {
        Box::new(self.clone()) as Box<dyn genericType + Send + Sync>
    }
    fn __go_eq_generic_type(&self, other: &(dyn genericType + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NamedPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl crate::check::Checker {
    /// newNamed is like NewNamed but with a *Checker receiver.
    pub fn new_named(&mut self, obj: Arc<Mutex<Option<TypeName>>>, underlying: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) -> Arc<Mutex<Option<Named>>> {
        let mut typ = Arc::new(Mutex::new(Some(Named { check: Arc::new(Mutex::new(Some(self.clone()))), obj: obj.clone(), from_r_h_s: underlying.clone(), underlying: underlying.clone(), methods: methods.clone(), ..Default::default() })));
        if { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }
                // Ensure that typ is always sanity-checked.
        if true {
        self.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn cleaner + Send + Sync>))));
    }
        return typ.clone();
    }

    /// newNamedInstance creates a new named instance for the given origin and type
    /// arguments, recording pos as the position of its synthetic object (for error
    /// reporting).
    ///
    /// If set, expanding is the named type instance currently being expanded, that
    /// led to the creation of this instance.
    pub fn new_named_instance(&mut self, pos: Arc<Mutex<Option<token_Pos>>>, orig: Arc<Mutex<Option<Named>>>, targs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>>>>>, expanding: Arc<Mutex<Option<Named>>>) -> Arc<Mutex<Option<Named>>> {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = ((*targs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }))));
        let mut obj = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), { let __field = (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); __field }, { let __field = (*(*orig.lock().unwrap().as_ref().unwrap()).obj.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().name.clone(); __field }, Arc::new(Mutex::new(None)));
        let mut inst = Arc::new(Mutex::new(Some(instance { orig: orig.clone(), targs: new_type_list(targs.clone()).clone(), ..Default::default() })));
                // Only pass the expanding context to the new instance if their packages
                // match. Since type reference cycles are only possible within a single
                // package, this is sufficient for the purposes of short-circuiting cycles.
                // Avoiding passing the context in other cases prevents unnecessary coupling
                // of types across packages.
        if (*expanding.lock().unwrap()).is_some() && { let __left = (*{ let __recv = expanding.clone(); let __recv_ptr: *const Named = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Named }; let __result = unsafe { &*__recv_ptr }.obj(); __result }.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().pkg.clone(); let __right = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        { let new_val = (*(*expanding.lock().unwrap().as_ref().unwrap()).inst.lock().unwrap().as_ref().unwrap()).ctxt.clone(); (*inst.lock().unwrap().as_mut().unwrap()).ctxt = new_val; };
    }
        let mut typ = Arc::new(Mutex::new(Some(Named { check: Arc::new(Mutex::new(Some(self.clone()))), obj: obj.clone(), inst: inst.clone(), ..Default::default() })));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
                // Ensure that typ is always sanity-checked.
        if true {
        self.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn cleaner + Send + Sync>))));
    }
        return typ.clone();
    }

    /// context returns the type-checker context.
    pub fn context(&mut self) -> Arc<Mutex<Option<crate::context::Context>>> {
        if { let __nil_target = self.ctxt.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = new_context().clone(); self.ctxt = new_val; };
    }
        self.ctxt.clone()
    }
}

/// NewNamed returns a new named type for the given type name, underlying type, and associated methods.
/// If the given type name obj doesn't have a type yet, its type is set to the returned named type.
/// The underlying type must not be a *Named.
pub fn new_named(obj: Arc<Mutex<Option<TypeName>>>, underlying: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) -> Arc<Mutex<Option<Named>>> {
    if (*as_named(underlying.clone()).lock().unwrap()).is_some() {
        panic!("underlying type must not be *Named");
    }
    __go_nil_recv_crate__check___checker_new_named(Arc::new(Mutex::new(None::<Checker>)), obj.clone(), underlying.clone(), methods.clone())
}

/// safeUnderlying returns the underlying type of typ without expanding
/// instances, to avoid infinite recursion.
///
/// TODO(rfindley): eliminate this function or give it a better name.
pub fn safe_underlying(typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    {
        let mut t = as_named(typ.clone());;
        if (*t.lock().unwrap()).is_some() {
            return (*t.lock().unwrap().as_ref().unwrap()).underlying.clone();;
        }
    }
    (*typ.lock().unwrap().as_mut().unwrap()).underlying().clone()
}

pub fn __go_nil_recv_crate__check___checker_new_named(check: Arc<Mutex<Option<Checker>>>, obj: Arc<Mutex<Option<TypeName>>>, underlying: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, methods: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Func>>>>>>>) -> Arc<Mutex<Option<Named>>> {
    let mut typ = Arc::new(Mutex::new(Some(Named { check: check.clone(), obj: obj.clone(), from_r_h_s: underlying.clone(), underlying: underlying.clone(), methods: methods.clone(), ..Default::default() })));
    if { let __iface_handle = { let __field = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_none() } {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *(*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

        // Ensure that typ is always sanity-checked.
    if (*check.lock().unwrap()).is_some() {
        { let __recv = check.clone(); let __recv_ptr: *mut crate::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.needs_cleanup(Arc::new(Mutex::new(Some(Box::new(NamedPtr(typ.clone())) as Box<dyn cleaner + Send + Sync>)))); __result };
    }
    return typ.clone();
}

impl GoValueClone for Named {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for instance {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
