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
use crate::universe::*;
use crate::util::*;
use crate::validtype::*;
use crate::version::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const SEND_RECV: i32 = 0;
pub const SEND_ONLY: i32 = 1;
pub const RECV_ONLY: i32 = 2;


/// A Chan represents a channel type.
#[derive(Clone)]
pub struct Chan {
    pub dir: Arc<Mutex<Option<ChanDir>>>,
    pub elem: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
}

impl Chan {
    pub fn __go_value_clone(&self) -> Self {
        Self { dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, elem: self.elem.clone() }
    }
}


impl Default for Chan {
    fn default() -> Self {
        Self { dir: Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some(0))))))), elem: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Chan {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Chan {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A ChanDir value indicates a channel direction.
#[derive(Debug, Clone, Default)]
pub struct ChanDir(pub Arc<Mutex<Option<i32>>>);

impl Display for ChanDir {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for ChanDir {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ChanDir {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ChanDir {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ChanDir {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ChanDir> for i32 {
    fn eq(&self, other: &ChanDir) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ChanDir> for i32 {
    fn partial_cmp(&self, other: &ChanDir) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ChanDir {
    type Output = ChanDir;
    fn add(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ChanDir {
    type Output = ChanDir;
    fn add(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ChanDir> for i32 {
    type Output = ChanDir;
    fn add(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ChanDir {
    type Output = ChanDir;
    fn sub(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ChanDir {
    type Output = ChanDir;
    fn sub(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ChanDir> for i32 {
    type Output = ChanDir;
    fn sub(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ChanDir {
    type Output = ChanDir;
    fn mul(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ChanDir {
    type Output = ChanDir;
    fn mul(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ChanDir> for i32 {
    type Output = ChanDir;
    fn mul(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ChanDir {
    type Output = ChanDir;
    fn div(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ChanDir {
    type Output = ChanDir;
    fn div(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ChanDir> for i32 {
    type Output = ChanDir;
    fn div(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ChanDir {
    type Output = ChanDir;
    fn neg(self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ChanDir {
    type Output = ChanDir;
    fn rem(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ChanDir {
    type Output = ChanDir;
    fn rem(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ChanDir> for i32 {
    type Output = ChanDir;
    fn rem(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ChanDir {
    type Output = ChanDir;
    fn bitand(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ChanDir {
    type Output = ChanDir;
    fn bitand(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitand(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ChanDir {
    type Output = ChanDir;
    fn bitor(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ChanDir {
    type Output = ChanDir;
    fn bitor(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitor(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ChanDir {
    type Output = ChanDir;
    fn bitxor(self, other: Self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ChanDir {
    type Output = ChanDir;
    fn bitxor(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ChanDir> for i32 {
    type Output = ChanDir;
    fn bitxor(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ChanDir {
    type Output = ChanDir;
    fn not(self) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: i64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: u64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ChanDir {
    type Output = ChanDir;
    fn shl(self, other: usize) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: ChanDir) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: i64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u32) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u8) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u16) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: u64) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ChanDir {
    type Output = ChanDir;
    fn shr(self, other: usize) -> ChanDir {
        ChanDir(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ChanDir {}

impl Ord for ChanDir {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


impl Chan {
    /// Dir returns the direction of channel c.
    pub fn dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        return self.dir.clone();
    }

    /// Elem returns the element type of channel c.
    pub fn elem(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return { let __field = self.elem.clone(); __field };
    }

    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(ChanPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        type_string(Arc::new(Mutex::new(Some(Box::new(ChanPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Type + Send + Sync>))), Arc::new(Mutex::new(None)))
    }
}

impl Type for Chan {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Chan::string(self)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Chan::underlying(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Chan>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ChanPtr(pub Arc<Mutex<Option<Chan>>>);

impl std::fmt::Display for ChanPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for ChanPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Chan::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Chan::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl ChanDir {
}

impl cmp::r#mod::Ordered for ChanDir {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ChanDir>() {
            self == __other
        } else {
            false
        }
    }
}

impl GoValueClone for Chan {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
