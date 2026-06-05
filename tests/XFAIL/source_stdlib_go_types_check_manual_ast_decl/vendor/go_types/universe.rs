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
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __APPEND: i32 = 0;
pub(crate) const __CAP: i32 = 1;
pub(crate) const __CLEAR: i32 = 2;
pub(crate) const __CLOSE: i32 = 3;
pub(crate) const __COMPLEX: i32 = 4;
pub(crate) const __COPY: i32 = 5;
pub(crate) const __DELETE: i32 = 6;
pub(crate) const __IMAG: i32 = 7;
pub(crate) const __LEN: i32 = 8;
pub(crate) const __MAKE: i32 = 9;
pub(crate) const __MAX: i32 = 10;
pub(crate) const __MIN: i32 = 11;
pub(crate) const __NEW: i32 = 12;
pub(crate) const __PANIC: i32 = 13;
pub(crate) const __PRINT: i32 = 14;
pub(crate) const __PRINTLN: i32 = 15;
pub(crate) const __REAL: i32 = 16;
pub(crate) const __RECOVER: i32 = 17;
pub(crate) const __ADD: i32 = 18;
pub(crate) const __ALIGNOF: i32 = 19;
pub(crate) const __OFFSETOF: i32 = 20;
pub(crate) const __SIZEOF: i32 = 21;
pub(crate) const __SLICE: i32 = 22;
pub(crate) const __SLICE_DATA: i32 = 23;
pub(crate) const __STRING: i32 = 24;
pub(crate) const __STRING_DATA: i32 = 25;
pub(crate) const __ASSERT: i32 = 26;
pub(crate) const __TRACE: i32 = 27;


/// A builtinId is the id of a builtin function.
#[derive(Debug, Clone, Default)]
pub struct builtinId(pub Arc<Mutex<Option<i32>>>);

impl Display for builtinId {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for builtinId {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for builtinId {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for builtinId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for builtinId {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<builtinId> for i32 {
    fn eq(&self, other: &builtinId) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<builtinId> for i32 {
    fn partial_cmp(&self, other: &builtinId) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for builtinId {
    type Output = builtinId;
    fn add(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for builtinId {
    type Output = builtinId;
    fn add(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<builtinId> for i32 {
    type Output = builtinId;
    fn add(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for builtinId {
    type Output = builtinId;
    fn sub(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for builtinId {
    type Output = builtinId;
    fn sub(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<builtinId> for i32 {
    type Output = builtinId;
    fn sub(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for builtinId {
    type Output = builtinId;
    fn mul(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for builtinId {
    type Output = builtinId;
    fn mul(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<builtinId> for i32 {
    type Output = builtinId;
    fn mul(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for builtinId {
    type Output = builtinId;
    fn div(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for builtinId {
    type Output = builtinId;
    fn div(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<builtinId> for i32 {
    type Output = builtinId;
    fn div(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for builtinId {
    type Output = builtinId;
    fn neg(self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for builtinId {
    type Output = builtinId;
    fn rem(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for builtinId {
    type Output = builtinId;
    fn rem(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<builtinId> for i32 {
    type Output = builtinId;
    fn rem(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for builtinId {
    type Output = builtinId;
    fn bitand(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for builtinId {
    type Output = builtinId;
    fn bitand(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<builtinId> for i32 {
    type Output = builtinId;
    fn bitand(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for builtinId {
    type Output = builtinId;
    fn bitor(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for builtinId {
    type Output = builtinId;
    fn bitor(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<builtinId> for i32 {
    type Output = builtinId;
    fn bitor(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for builtinId {
    type Output = builtinId;
    fn bitxor(self, other: Self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for builtinId {
    type Output = builtinId;
    fn bitxor(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<builtinId> for i32 {
    type Output = builtinId;
    fn bitxor(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for builtinId {
    type Output = builtinId;
    fn not(self) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for builtinId {
    type Output = builtinId;
    fn shl(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for builtinId {
    type Output = builtinId;
    fn shl(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for builtinId {
    type Output = builtinId;
    fn shl(self, other: i8) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for builtinId {
    type Output = builtinId;
    fn shl(self, other: i16) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for builtinId {
    type Output = builtinId;
    fn shl(self, other: i64) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for builtinId {
    type Output = builtinId;
    fn shl(self, other: u32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for builtinId {
    type Output = builtinId;
    fn shl(self, other: u8) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for builtinId {
    type Output = builtinId;
    fn shl(self, other: u16) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for builtinId {
    type Output = builtinId;
    fn shl(self, other: u64) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for builtinId {
    type Output = builtinId;
    fn shl(self, other: usize) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for builtinId {
    type Output = builtinId;
    fn shr(self, other: builtinId) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for builtinId {
    type Output = builtinId;
    fn shr(self, other: i32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for builtinId {
    type Output = builtinId;
    fn shr(self, other: i8) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for builtinId {
    type Output = builtinId;
    fn shr(self, other: i16) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for builtinId {
    type Output = builtinId;
    fn shr(self, other: i64) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for builtinId {
    type Output = builtinId;
    fn shr(self, other: u32) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for builtinId {
    type Output = builtinId;
    fn shr(self, other: u8) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for builtinId {
    type Output = builtinId;
    fn shr(self, other: u16) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for builtinId {
    type Output = builtinId;
    fn shr(self, other: u64) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for builtinId {
    type Output = builtinId;
    fn shr(self, other: usize) -> builtinId {
        builtinId(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for builtinId {}

impl Ord for builtinId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
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


pub static Universe: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::scope::Scope>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Unsafe: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::package::Package>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeIota: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Object + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeBool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Type + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeByte: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Type + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeRune: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Type + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeAnyNoAlias: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::object::TypeName>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeAnyAlias: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<crate::object::TypeName>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeError: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Type + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static universeComparable: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Object + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Typ: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<crate::basic::Basic>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static basicAliases: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[Arc<Mutex<Option<crate::basic::Basic>>>; 2]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static predeclaredConsts: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[AnonymousStruct2; 3]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static predeclaredFuncs: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[AnonymousStruct3; 28]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Universe.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Unsafe.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeIota.lock().unwrap() = None;
    *universeBool.lock().unwrap() = None;
    *universeByte.lock().unwrap() = None;
    *universeRune.lock().unwrap() = None;
    *universeAnyNoAlias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeAnyAlias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeError.lock().unwrap() = None;
    *universeComparable.lock().unwrap() = None;
    *Typ.lock().unwrap() = Some(vec![]);
    *basicAliases.lock().unwrap() = Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))));
    *predeclaredConsts.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *predeclaredFuncs.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *Typ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INVALID as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32))))))), name: Arc::new(Mutex::new(Some("invalid type".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(BOOL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_BOOLEAN as i32))))))), name: Arc::new(Mutex::new(Some("bool".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT8 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int8".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT16 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int16".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT8 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint8".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT16 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint16".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINTPTR as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uintptr".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_FLOAT as i32))))))), name: Arc::new(Mutex::new(Some("float32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_FLOAT as i32))))))), name: Arc::new(Mutex::new(Some("float64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_COMPLEX as i32))))))), name: Arc::new(Mutex::new(Some("complex64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX128 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_COMPLEX as i32))))))), name: Arc::new(Mutex::new(Some("complex128".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(STRING as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_STRING as i32))))))), name: Arc::new(Mutex::new(Some("string".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32))))))), name: Arc::new(Mutex::new(Some("Pointer".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_BOOLEAN as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped bool".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped int".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_RUNE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped rune".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_FLOAT as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped float".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_COMPLEX as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped complex".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_STRING as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_STRING as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped string".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_NIL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_UNTYPED as i32))))))), name: Arc::new(Mutex::new(Some("untyped nil".to_string()))), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
    *basicAliases.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(BYTE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("byte".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(RUNE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("rune".to_string()))), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
    *predeclaredConsts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([AnonymousStruct2 { name: Arc::new(Mutex::new(Some("true".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), val: go_constant::make_bool(Arc::new(Mutex::new(Some(true)))).clone(), ..Default::default() }, AnonymousStruct2 { name: Arc::new(Mutex::new(Some("false".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), val: go_constant::make_bool(Arc::new(Mutex::new(Some(false)))).clone(), ..Default::default() }, AnonymousStruct2 { name: Arc::new(Mutex::new(Some("iota".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))))), val: go_constant::make_int64(Arc::new(Mutex::new(Some(0 as i64)))).clone(), ..Default::default() }]))).lock().unwrap().as_ref().unwrap()).clone());
    *predeclaredFuncs.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([AnonymousStruct3 { name: Arc::new(Mutex::new(Some("append".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("cap".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("clear".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("close".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("complex".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("copy".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("delete".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("imag".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("len".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("make".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("max".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("min".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("new".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("panic".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("print".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("println".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("real".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("recover".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Add".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Alignof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Offsetof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Sizeof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Slice".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("SliceData".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("String".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("StringData".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("assert".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("trace".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *Universe.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Unsafe.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeIota.lock().unwrap() = None;
    *universeBool.lock().unwrap() = None;
    *universeByte.lock().unwrap() = None;
    *universeRune.lock().unwrap() = None;
    *universeAnyNoAlias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeAnyAlias.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *universeError.lock().unwrap() = None;
    *universeComparable.lock().unwrap() = None;
    *Typ.lock().unwrap() = Some(vec![]);
    *basicAliases.lock().unwrap() = Some(std::array::from_fn(|_| Arc::new(Mutex::new(None))));
    *predeclaredConsts.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
    *predeclaredFuncs.lock().unwrap() = Some(std::array::from_fn(|_| Default::default()));
}


pub(crate) fn __go_init_order_13() {
    *Typ.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INVALID as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32))))))), name: Arc::new(Mutex::new(Some("invalid type".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(BOOL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_BOOLEAN as i32))))))), name: Arc::new(Mutex::new(Some("bool".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT8 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int8".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT16 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int16".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(INT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("int64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT8 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint8".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT16 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint16".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uint64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UINTPTR as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("uintptr".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT32 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_FLOAT as i32))))))), name: Arc::new(Mutex::new(Some("float32".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(FLOAT64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_FLOAT as i32))))))), name: Arc::new(Mutex::new(Some("float64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX64 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_COMPLEX as i32))))))), name: Arc::new(Mutex::new(Some("complex64".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(COMPLEX128 as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_COMPLEX as i32))))))), name: Arc::new(Mutex::new(Some("complex128".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(STRING as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_STRING as i32))))))), name: Arc::new(Mutex::new(Some("string".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNSAFE_POINTER as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(0 as i32))))))), name: Arc::new(Mutex::new(Some("Pointer".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_BOOLEAN as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped bool".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped int".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_RUNE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped rune".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_FLOAT as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_FLOAT as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped float".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_COMPLEX as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_COMPLEX as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped complex".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_STRING as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_STRING as i32 | IS_UNTYPED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("untyped string".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_NIL as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_UNTYPED as i32))))))), name: Arc::new(Mutex::new(Some("untyped nil".to_string()))), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_14() {
    *basicAliases.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(BYTE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some((IS_INTEGER as i32 | IS_UNSIGNED as i32) as i32))))))), name: Arc::new(Mutex::new(Some("byte".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(crate::basic::Basic { kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(RUNE as i32))))))), info: Arc::new(Mutex::new(Some(crate::basic::BasicInfo(Arc::new(Mutex::new(Some(IS_INTEGER as i32))))))), name: Arc::new(Mutex::new(Some("rune".to_string()))), ..Default::default() })))]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_15() {
    *predeclaredConsts.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([AnonymousStruct2 { name: Arc::new(Mutex::new(Some("true".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), val: go_constant::make_bool(Arc::new(Mutex::new(Some(true)))).clone(), ..Default::default() }, AnonymousStruct2 { name: Arc::new(Mutex::new(Some("false".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_BOOL as i32))))))), val: go_constant::make_bool(Arc::new(Mutex::new(Some(false)))).clone(), ..Default::default() }, AnonymousStruct2 { name: Arc::new(Mutex::new(Some("iota".to_string()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(UNTYPED_INT as i32))))))), val: go_constant::make_int64(Arc::new(Mutex::new(Some(0 as i64)))).clone(), ..Default::default() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_init_order_16() {
    *predeclaredFuncs.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([AnonymousStruct3 { name: Arc::new(Mutex::new(Some("append".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("cap".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("clear".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("close".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("complex".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("copy".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("delete".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("imag".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("len".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("make".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("max".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("min".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("new".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("panic".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("print".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("println".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("real".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("recover".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Add".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Alignof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Offsetof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Sizeof".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("Slice".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("SliceData".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("String".to_string()))), nargs: Arc::new(Mutex::new(Some(2))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("StringData".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(EXPRESSION as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("assert".to_string()))), nargs: Arc::new(Mutex::new(Some(1))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }, AnonymousStruct3 { name: Arc::new(Mutex::new(Some("trace".to_string()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(true))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(STATEMENT as i32))))))), ..Default::default() }]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl builtinId {
}

impl cmp::r#mod::Ordered for builtinId {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<builtinId>() {
            self == __other
        } else {
            false
        }
    }
}

pub fn def_predeclared_types() {
    { let __range_holder = Typ.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))).clone())) as Box<dyn Object + Send + Sync>))));
    } }
    { let __range_holder = basicAliases.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for t in __range_values.iter() {
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr(t.clone())) as Box<dyn Type + Send + Sync>)))).clone())) as Box<dyn Object + Send + Sync>))));
    } }

        // type any = interface{}
        //
        // Implement two representations of any: one for the legacy gotypesalias=0,
        // and one for gotypesalias=1. This is necessary for consistent
        // representation of interface aliases during type checking, and is
        // implemented via hijacking [Scope.Lookup] for the [Universe] scope.
        //
        // Both representations use the same distinguished pointer for their RHS
        // interface type, allowing us to detect any (even with the legacy
        // representation), and format it as "any" rather than interface{}, which
        // clarifies user-facing error messages significantly.
        //
        // TODO(rfindley): once the gotypesalias GODEBUG variable is obsolete (and we
        // consistently use the Alias node), we should be able to clarify user facing
        // error messages without using a distinguished pointer for the any
        // interface.
    {
        { let new_val = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("any".to_string()))), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(Arc::new(Mutex::new(Some(Interface { complete: Arc::new(Mutex::new(Some(true))), tset: topTypeSet.clone().clone(), ..Default::default() }))).clone())) as Box<dyn Type + Send + Sync>)))).clone(); *universeAnyNoAlias.lock().unwrap() = Some(new_val); };
        { let __recv_holder = (*universeAnyNoAlias.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };

                // ensure that the any TypeName reports a consistent Parent, after
                // hijacking Universe.Lookup with gotypesalias=0.
        { let __recv_holder = (*universeAnyNoAlias.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).set_parent({ let __arg_holder = Universe.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); __result };

                // It shouldn't matter which representation of any is actually inserted
                // into the Universe, but we lean toward the future and insert the Alias
                // representation.
        { let new_val = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("any".to_string()))), Arc::new(Mutex::new(None))).clone(); *universeAnyAlias.lock().unwrap() = Some(new_val); };
        { let __recv_holder = (*universeAnyAlias.lock().unwrap().as_ref().unwrap()).clone(); let __result = (*__recv_holder.lock().unwrap().as_mut().unwrap()).set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };
        let _ = new_alias({ let __arg_holder = universeAnyAlias.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }, { let __recv = { let __recv_holder = (*universeAnyNoAlias.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.r#type(); __result }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).underlying(); __result }.clone());
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr({ let __arg_holder = universeAnyAlias.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Object + Send + Sync>))));
    }

        // ensure that the any TypeName reports a consistent Parent, after
        // hijacking Universe.Lookup with gotypesalias=0.
        // It shouldn't matter which representation of any is actually inserted
        // into the Universe, but we lean toward the future and insert the Alias
        // representation.
        // Link TypeName and Alias
        // type error interface{ Error() string }
    {
        let mut obj = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("error".to_string()))), Arc::new(Mutex::new(None)));
        { let __recv = obj.clone(); let __recv_ptr: *mut crate::object::TypeName = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::object::TypeName }; let __result = unsafe { &mut *__recv_ptr }.set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };
        let mut typ = new_named(obj.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));

                // error.Error() string
        let mut recv = new_var(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(crate::named::NamedPtr(typ.clone())) as Box<dyn Type + Send + Sync>))));
        let mut res = new_var(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(STRING as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))));
        let mut sig = new_signature_type(recv.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), new_tuple(Arc::new(Mutex::new(Some(vec![res.clone()])))), Arc::new(Mutex::new(Some(false))));
        let mut err = new_func(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("Error".to_string()))), sig.clone());

                // interface{ Error() string }
        let mut ityp = Arc::new(Mutex::new(Some(Interface { methods: Arc::new(Mutex::new(Some(vec![err.clone()]))), complete: Arc::new(Mutex::new(Some(true))), ..Default::default() })));
        compute_interface_type_set(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ityp.clone());

        { let __recv = typ.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.set_underlying(Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(ityp.clone())) as Box<dyn Type + Send + Sync>)))); __result };
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
    }

        // error.Error() string
        // interface{ Error() string }
        // prevent races due to lazy computation of tset
        // type comparable interface{} // marked as comparable
    {
        let mut obj = new_type_name(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("comparable".to_string()))), Arc::new(Mutex::new(None)));
        { let __recv = obj.clone(); let __recv_ptr: *mut crate::object::TypeName = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::object::TypeName }; let __result = unsafe { &mut *__recv_ptr }.set_color(Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))))))); __result };
        let mut typ = new_named(obj.clone(), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));

                // interface{} // marked as comparable
        let mut ityp = Arc::new(Mutex::new(Some(Interface { complete: Arc::new(Mutex::new(Some(true))), tset: Arc::new(Mutex::new(Some(_TypeSet { methods: Arc::new(Mutex::new(None)), terms: allTermlist.clone(), comparable: Arc::new(Mutex::new(Some(true))), ..Default::default() }))).clone(), ..Default::default() })));

        { let __recv = typ.clone(); let __recv_ptr: *mut crate::named::Named = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::named::Named }; let __result = unsafe { &mut *__recv_ptr }.set_underlying(Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(ityp.clone())) as Box<dyn Type + Send + Sync>)))); __result };
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::TypeNamePtr(obj.clone())) as Box<dyn Object + Send + Sync>))));
    }
}

pub fn def_predeclared_consts() {
    { let __range_holder = predeclaredConsts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter() {
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::ConstPtr(new_const(Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __selector_holder = c.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*(*c.kind.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), { let __field = c.val.clone(); __field }).clone())) as Box<dyn Object + Send + Sync>))));
    } }
}

pub fn def_predeclared_nil() {
    def(Arc::new(Mutex::new(Some(Box::new(crate::object::NilPtr({ let __owner = Arc::new(Mutex::new(Some(crate::object::Nil { object: Arc::new(Mutex::new(Some(object { name: Arc::new(Mutex::new(Some("nil".to_string()))), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(UNTYPED_NIL as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), color_: Arc::new(Mutex::new(Some(crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32))))))), ..Default::default() }))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }.clone())) as Box<dyn Object + Send + Sync>))));
}

pub fn def_predeclared_funcs() {
    for i in 0..(({ let __range_holder = predeclaredFuncs.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut id = Arc::new(Mutex::new(Some(builtinId(Arc::new(Mutex::new(Some(i as i32)))))));
        if { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = builtinId(Arc::new(Mutex::new(Some(__ASSERT as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = builtinId(Arc::new(Mutex::new(Some(__TRACE as i32)))); __tmp_x == __tmp_y } {
        continue
    }
                // only define these in testing environment
        def(Arc::new(Mutex::new(Some(Box::new(crate::object::BuiltinPtr(new_builtin(Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone())) as Box<dyn Object + Send + Sync>))));
    }
}

fn __go_init_0() {
    { let new_val = new_scope(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("universe".to_string())))).clone(); *Universe.lock().unwrap() = Some(new_val); };
    { let new_val = new_package(Arc::new(Mutex::new(Some("unsafe".to_string()))), Arc::new(Mutex::new(Some("unsafe".to_string())))).clone(); *Unsafe.lock().unwrap() = Some(new_val); };
    { let new_val = true; *(*(*Unsafe.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).complete.lock().unwrap() = Some(new_val); };

    def_predeclared_types();
    def_predeclared_consts();
    def_predeclared_nil();
    def_predeclared_funcs();

    { let __iface_handle = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("iota".to_string())))); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeIota.lock().unwrap() = (*__iface_guard).clone(); };
    { let __iface_handle = { let __recv = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("bool".to_string())))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeBool.lock().unwrap() = (*__iface_guard).clone(); };
    { let __iface_handle = { let __recv = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("byte".to_string())))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeByte.lock().unwrap() = (*__iface_guard).clone(); };
    { let __iface_handle = { let __recv = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("rune".to_string())))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeRune.lock().unwrap() = (*__iface_guard).clone(); };
    { let __iface_handle = { let __recv = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("error".to_string())))); __result }; let __result = (*__recv.lock().unwrap().as_ref().unwrap()).r#type(); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeError.lock().unwrap() = (*__iface_guard).clone(); };
    { let __iface_handle = { let __recv_holder = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __recv_value = (*__recv_holder.lock().unwrap().as_ref().unwrap()).clone(); let __result = __recv_value.lookup(Arc::new(Mutex::new(Some("comparable".to_string())))); __result }.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *universeComparable.lock().unwrap() = (*__iface_guard).clone(); };
}

/// Objects with names containing blanks are internal and not entered into
/// a scope. Objects with exported names are inserted in the unsafe package
/// scope; other objects are inserted in the universe scope.
pub fn def(mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(obj.lock().unwrap().as_ref().map(|__v| Object::__go_clone_box_object(__v.as_ref()))));
    assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).color().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::object::color(Arc::new(Mutex::new(Some(BLACK as u32)))); __tmp_x == __tmp_y }))));
    let mut name = (*obj.lock().unwrap().as_ref().unwrap()).name();
    if strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(" ".to_string())))) {
        return;
    }

        // nothing to do
        // fix Obj link for named types
    {
        let mut typ = as_named((*obj.lock().unwrap().as_ref().unwrap()).r#type().clone());;
        if (*typ.lock().unwrap()).is_some() {
            { let new_val = ({
        let val = obj.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Object + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::object::TypeNamePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone(); (*typ.lock().unwrap().as_mut().unwrap()).obj = new_val; };;
        }
    }

        // exported identifiers go into package unsafe
    let mut scope = (*Universe.lock().unwrap().as_ref().unwrap()).clone();
    if (*obj.lock().unwrap().as_ref().unwrap()).exported() {
        { let new_val = (*(*Unsafe.lock().unwrap().as_ref().unwrap()).lock().unwrap().as_ref().unwrap()).scope.clone(); scope = new_val; };
                // set Pkg field
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::TypeNamePtr>()).unwrap().0.clone();
        { let new_val = (*Unsafe.lock().unwrap().as_ref().unwrap()).clone(); (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg = new_val; };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<crate::object::BuiltinPtr>()).unwrap().0.clone();
        { let new_val = (*Unsafe.lock().unwrap().as_ref().unwrap()).clone(); (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg = new_val; };;
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new("unreachable".to_string()) as Box<dyn Any + Send + Sync>);;
    }
    }
    }
        // set Pkg field
    if (*{ let __recv = scope.clone(); let __recv_ptr: *mut crate::scope::Scope = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut crate::scope::Scope }; let __result = unsafe { &mut *__recv_ptr }.insert(obj.clone()); __result }.lock().unwrap()).is_some() {
        std::panic::panic_any(Box::new("double declaration of predeclared identifier".to_string()) as Box<dyn Any + Send + Sync>);
    }
}

#[derive(Clone)]
pub struct AnonymousStruct2 {
    pub name: Arc<Mutex<Option<String>>>,
    pub kind: Arc<Mutex<Option<BasicKind>>>,
    pub val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: self.val.clone() }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(crate::basic::BasicKind(Arc::new(Mutex::new(Some(0))))))), val: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.val.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct2 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub name: Arc<Mutex<Option<String>>>,
    pub nargs: Arc<Mutex<Option<i32>>>,
    pub variadic: Arc<Mutex<Option<bool>>>,
    pub kind: Arc<Mutex<Option<exprKind>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nargs: { let __guard = self.nargs.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, variadic: { let __guard = self.variadic.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))), nargs: Arc::new(Mutex::new(Some(0))), variadic: Arc::new(Mutex::new(Some(false))), kind: Arc::new(Mutex::new(Some(crate::expr::exprKind(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {}}}", (*self.name.lock().unwrap().as_ref().unwrap()), (*self.nargs.lock().unwrap().as_ref().unwrap()), (*self.variadic.lock().unwrap().as_ref().unwrap()), (*self.kind.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct3 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) fn __go_init_functions() {
    self::__go_init_0();
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
    self::__go_init_0();
}
