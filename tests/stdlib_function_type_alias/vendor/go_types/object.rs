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
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const WHITE: u32 = 0;
pub(crate) const BLACK: u32 = 1;
pub(crate) const GREY: u32 = 2;


/// An Object is a named language entity.
/// An Object may be a constant ([Const]), type name ([TypeName]),
/// variable or struct field ([Var]), function or method ([Func]),
/// imported package ([PkgName]), label ([Label]),
/// built-in function ([Builtin]),
/// or the predeclared identifier 'nil' ([Nil]).
///
/// The environment, which is structured as a tree of Scopes,
/// maps each name to the unique Object that it denotes.
pub trait Object: std::fmt::Display + Any {
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool;
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>>;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>>;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>;
    fn exported(&self) -> bool;
    fn id(&self) -> Arc<Mutex<Option<String>>>;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
    fn order(&self) -> u32;
    fn color(&self) -> Arc<Mutex<Option<color>>>;
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>);
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>);
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>);
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<Scope>>>);
    fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool;
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>);
}

impl Clone for Box<dyn Object + Send + Sync> {
    fn clone(&self) -> Self {
        Object::__go_clone_box_object(self.as_ref())
    }
}

impl GoValueClone for Box<dyn Object + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        Object::__go_clone_box_object(self.as_ref())
    }
}

#[derive(Clone)]
pub struct GoObjectInterfaceKey(pub Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>);

impl GoObjectInterfaceKey {
    pub fn new(value: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>) -> Self { GoObjectInterfaceKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
    fn identity(&self) -> (u64, String) {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() {
            None => (0, String::new()),
            Some(__v) => {
                let mut __hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&__v.as_ref().__go_as_any().type_id(), &mut __hasher);
                (std::hash::Hasher::finish(&__hasher), format!("{}", __v))
            }
        }
    }
}
impl PartialEq for GoObjectInterfaceKey {
    fn eq(&self, other: &Self) -> bool {
        let __left_guard = self.0.lock().unwrap();
        let __right_guard = other.0.lock().unwrap();
        match (__left_guard.as_ref(), __right_guard.as_ref()) {
            (None, None) => true,
            (Some(__left), Some(__right)) => __left.as_ref().__go_eq_object(__right.as_ref()),
            _ => false,
        }
    }
}
impl Eq for GoObjectInterfaceKey {}
impl PartialOrd for GoObjectInterfaceKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl Ord for GoObjectInterfaceKey {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other { return std::cmp::Ordering::Equal; }
        match self.identity().cmp(&other.identity()) {
            std::cmp::Ordering::Equal => self.addr().cmp(&other.addr()),
            ordering => ordering,
        }
    }
}
impl std::fmt::Debug for GoObjectInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}
impl std::fmt::Display for GoObjectInterfaceKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.identity().1) }
}

impl positioner for Box<dyn Object + Send + Sync> {
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new((*self).clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        (**self).__go_as_any()
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        let _ = other;
        panic!("interface equality for structurally adapted Object as positioner")
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        (**self).pos()
    }
}

/// An object implements the common parts of an Object.
#[derive(Clone)]
pub struct object {
    pub parent: Arc<Mutex<Option<Scope>>>,
    pub pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub pkg: Arc<Mutex<Option<Package>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>,
    pub order_: Arc<Mutex<Option<u32>>>,
    pub color_: Arc<Mutex<Option<color>>>,
    pub scope_pos_: Arc<Mutex<Option<go_token::position::Pos>>>,
}

impl object {
    pub fn __go_value_clone(&self) -> Self {
        Self { parent: self.parent.clone(), pos: { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, pkg: self.pkg.clone(), name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: self.typ.clone(), order_: { let __guard = self.order_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, color_: { let __guard = self.color_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, scope_pos_: { let __guard = self.scope_pos_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for object {
    fn default() -> Self {
        Self { parent: Arc::new(Mutex::new(None)), pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), pkg: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(Some(String::new()))), typ: Arc::new(Mutex::new(None)), order_: Arc::new(Mutex::new(Some(0))), color_: Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(0))))))), scope_pos_: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for object {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// color encodes the color of an object (see Checker.objDecl for details).
#[derive(Debug, Clone, Default)]
pub struct color(pub Arc<Mutex<Option<u32>>>);

impl Display for color {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for color {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for color {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for color {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<color> for u32 {
    fn eq(&self, other: &color) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<color> for u32 {
    fn partial_cmp(&self, other: &color) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for color {
    type Output = color;
    fn add(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for color {
    type Output = color;
    fn add(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<color> for u32 {
    type Output = color;
    fn add(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for color {
    type Output = color;
    fn sub(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for color {
    type Output = color;
    fn sub(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<color> for u32 {
    type Output = color;
    fn sub(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for color {
    type Output = color;
    fn mul(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for color {
    type Output = color;
    fn mul(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<color> for u32 {
    type Output = color;
    fn mul(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for color {
    type Output = color;
    fn div(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for color {
    type Output = color;
    fn div(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<color> for u32 {
    type Output = color;
    fn div(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for color {
    type Output = color;
    fn rem(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for color {
    type Output = color;
    fn rem(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<color> for u32 {
    type Output = color;
    fn rem(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for color {
    type Output = color;
    fn bitand(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for color {
    type Output = color;
    fn bitand(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<color> for u32 {
    type Output = color;
    fn bitand(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for color {
    type Output = color;
    fn bitor(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for color {
    type Output = color;
    fn bitor(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<color> for u32 {
    type Output = color;
    fn bitor(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for color {
    type Output = color;
    fn bitxor(self, other: Self) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for color {
    type Output = color;
    fn bitxor(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<color> for u32 {
    type Output = color;
    fn bitxor(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for color {
    type Output = color;
    fn not(self) -> color {
        color(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for color {
    type Output = color;
    fn shl(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for color {
    type Output = color;
    fn shl(self, other: i32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for color {
    type Output = color;
    fn shl(self, other: i8) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for color {
    type Output = color;
    fn shl(self, other: i16) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for color {
    type Output = color;
    fn shl(self, other: i64) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for color {
    type Output = color;
    fn shl(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for color {
    type Output = color;
    fn shl(self, other: u8) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for color {
    type Output = color;
    fn shl(self, other: u16) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for color {
    type Output = color;
    fn shl(self, other: u64) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for color {
    type Output = color;
    fn shl(self, other: usize) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for color {
    type Output = color;
    fn shr(self, other: color) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for color {
    type Output = color;
    fn shr(self, other: i32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for color {
    type Output = color;
    fn shr(self, other: i8) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for color {
    type Output = color;
    fn shr(self, other: i16) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for color {
    type Output = color;
    fn shr(self, other: i64) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for color {
    type Output = color;
    fn shr(self, other: u32) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for color {
    type Output = color;
    fn shr(self, other: u8) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for color {
    type Output = color;
    fn shr(self, other: u16) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for color {
    type Output = color;
    fn shr(self, other: u64) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for color {
    type Output = color;
    fn shr(self, other: usize) -> color {
        color(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for color {}

impl Ord for color {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A PkgName represents an imported Go package.
/// PkgNames don't have a type.
#[derive(Clone)]
pub struct PkgName {
    pub object: Arc<Mutex<Option<object>>>,
    pub imported: Arc<Mutex<Option<Package>>>,
}

impl PkgName {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, imported: self.imported.clone() }
    }
}


impl Default for PkgName {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), imported: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for PkgName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for PkgName {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Const represents a declared constant.
#[derive(Clone)]
pub struct Const {
    pub object: Arc<Mutex<Option<object>>>,
    pub val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>,
}

impl Const {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, val: self.val.clone() }
    }
}


impl Default for Const {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), val: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Const {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A TypeName is an [Object] that represents a type with a name:
/// a defined type ([Named]),
/// an alias type ([Alias]),
/// a type parameter ([TypeParam]),
/// or a predeclared type such as int or error.
#[derive(Clone)]
pub struct TypeName {
    pub object: Arc<Mutex<Option<object>>>,
}

impl TypeName {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for TypeName {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))) }
    }
}

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for TypeName {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Variable represents a declared variable (including function parameters and results, and struct fields).
#[derive(Clone)]
pub struct Var {
    pub object: Arc<Mutex<Option<object>>>,
    pub origin: Arc<Mutex<Option<Var>>>,
    pub embedded: Arc<Mutex<Option<bool>>>,
    pub is_field: Arc<Mutex<Option<bool>>>,
    pub is_param: Arc<Mutex<Option<bool>>>,
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, origin: self.origin.clone(), embedded: { let __guard = self.embedded.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_field: { let __guard = self.is_field.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, is_param: { let __guard = self.is_param.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Var {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), origin: Arc::new(Mutex::new(None)), embedded: Arc::new(Mutex::new(Some(false))), is_field: Arc::new(Mutex::new(Some(false))), is_param: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Var {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for Var {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Func represents a declared function, concrete method, or abstract
/// (interface) method. Its Type() is always a *Signature.
/// An abstract method may belong to many interfaces due to embedding.
#[derive(Clone)]
pub struct Func {
    pub object: Arc<Mutex<Option<object>>>,
    pub has_ptr_recv_: Arc<Mutex<Option<bool>>>,
    pub origin: Arc<Mutex<Option<Func>>>,
}

impl Func {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, has_ptr_recv_: { let __guard = self.has_ptr_recv_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, origin: self.origin.clone() }
    }
}


impl Default for Func {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), has_ptr_recv_: Arc::new(Mutex::new(Some(false))), origin: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl GoComparable for Func {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}

impl GoJsonDecode for Func {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Label represents a declared label.
/// Labels don't have a type.
#[derive(Clone)]
pub struct Label {
    pub object: Arc<Mutex<Option<object>>>,
    pub used: Arc<Mutex<Option<bool>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, used: { let __guard = self.used.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Label {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), used: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Label {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A Builtin represents a built-in function.
/// Builtins don't have a valid type.
#[derive(Clone)]
pub struct Builtin {
    pub object: Arc<Mutex<Option<object>>>,
    pub id: Arc<Mutex<Option<builtinId>>>,
}

impl Builtin {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, id: { let __guard = self.id.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Builtin {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))), id: Arc::new(Mutex::new(Some(crate::universe::builtinId(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Builtin {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// Nil represents the predeclared value nil.
#[derive(Clone)]
pub struct Nil {
    pub object: Arc<Mutex<Option<object>>>,
}

impl Nil {
    pub fn __go_value_clone(&self) -> Self {
        Self { object: { let __guard = self.object.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Nil {
    fn default() -> Self {
        Self { object: Arc::new(Mutex::new(Some(object::default()))) }
    }
}

impl std::fmt::Display for Nil {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Nil {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl color {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        { let _switch_val = (*self.0.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (color(Arc::new(Mutex::new(Some(WHITE as u32))))) {
            return Arc::new(Mutex::new(Some("white".to_string())));
        } else if _switch_val == (color(Arc::new(Mutex::new(Some(BLACK as u32))))) {
            return Arc::new(Mutex::new(Some("black".to_string())));
        } else {
            return Arc::new(Mutex::new(Some("grey".to_string())));
        }
    }
    }
}

impl cmp::r#mod::Ordered for color {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<color>() {
            self == __other
        } else {
            false
        }
    }
}

impl object {
    /// Parent returns the scope in which the object is declared.
    /// The result is nil for methods and struct fields.
    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        self.parent.clone()
    }

    /// Pos returns the declaration position of the object's identifier.
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.pos.clone();
    }

    /// Pkg returns the package to which the object belongs.
    /// The result is nil for labels and objects in the Universe scope.
    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        self.pkg.clone()
    }

    /// Name returns the object's (package-local, unqualified) name.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    /// Type returns the object's type.
    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        return { let __field = self.typ.clone(); __field };
    }

    /// Exported reports whether the object is exported (starts with a capital letter).
    /// It doesn't take into account whether the object is in a local (function) scope
    /// or not.
    pub fn exported(&self) -> bool {
        is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    /// Id is a wrapper for Id(obj.Pkg(), obj.Name()).
    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        id({ let __field = self.pkg.clone(); __field }, Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        std::panic::panic_any(Box::new("abstract".to_string()) as Box<dyn Any + Send + Sync>);
    }

    pub fn order(&self) -> u32 {
        return (*self.order_.lock().unwrap().as_ref().unwrap());
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        return self.color_.clone();
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        return self.scope_pos_.clone();
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        { let new_val = parent.clone(); self.parent = new_val; };
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        { let __iface_handle = typ.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *self.typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*order.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u32; __tmp_x > __tmp_y }))));
        { let new_val = order.lock().unwrap().as_ref().unwrap().clone(); *self.order_.lock().unwrap() = Some(new_val); };
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        assert(Arc::new(Mutex::new(Some({ let __tmp_x = (*color_local.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = color(Arc::new(Mutex::new(Some(WHITE as u32)))); __tmp_x != __tmp_y }))));
        { let new_val = color_local.lock().unwrap().as_ref().unwrap().clone(); *self.color_.lock().unwrap() = Some(new_val); };
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        { let new_val = pos.lock().unwrap().as_ref().unwrap().clone(); *self.scope_pos_.lock().unwrap() = Some(new_val); };
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
                // If we don't care about capitalization, we also ignore packages.
        if { let __v = (*foldCase.lock().unwrap().as_ref().unwrap()).clone(); __v } && (*Arc::new(Mutex::new(Some({ let __a = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __b = (*name.lock().unwrap().as_ref().unwrap()).clone(); __a.to_lowercase() == __b.to_lowercase() }))).lock().unwrap().as_ref().unwrap()) {
        return true;
    }
                // spec:
                // "Two identifiers are different if they are spelled differently,
                // or if they appear in different packages and are not exported.
                // Otherwise, they are the same."
        if { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }
                // obj.Name == name
        if self.exported() {
        return true;
    }
                // not exported, so packages must be the same
        same_pkg({ let __field = self.pkg.clone(); __field }, pkg.clone())
    }

    /// cmp reports whether object a is ordered before object b.
    /// cmp returns:
    ///
    ///	-1 if a is before b
    ///	 0 if a is equivalent to b
    ///	+1 if a is behind b
    ///
    /// Objects are ordered nil before non-nil, exported before
    /// non-exported, then by name, and finally (for non-exported
    /// functions) by package path.
    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        if { let __peer = b.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); __eq } {
        return 0;
    }
                // Nil before non-nil.
        if false {
        return -(1);
    }
        if (*b.lock().unwrap()).is_none() {
        return 1;
    }
                // Exported functions before non-exported.
        let mut ea = is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut eb = is_exported(Arc::new(Mutex::new(Some({ let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = ea; let __tmp_y = eb; __tmp_x != __tmp_y } {
        if ea {
        return -(1);
    }
        return 1;
    }
                // Order by name and then (for non-exported names) by package.
        if { let __tmp_x = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*b.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
        return (*Arc::new(Mutex::new(Some({ let __a = (*self.name.lock().unwrap().as_ref().unwrap()).clone(); let __b = (*(*b.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).lock().unwrap().as_ref().unwrap());
    }
        if !ea {
        return (*Arc::new(Mutex::new(Some({ let __a = (*(*self.pkg.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).clone(); let __b = (*(*(*b.lock().unwrap().as_ref().unwrap()).pkg.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).lock().unwrap().as_ref().unwrap());
    }
        0
    }
}

impl Object for object {
    fn exported(&self) -> bool {
        object::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        object::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        object::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        object::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        object::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        object::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        object::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        object::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        object::color(self)
    }
    fn order(&self) -> u32 {
        object::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        object::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        object::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        object::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        object::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        object::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        object::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        object::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<object>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct objectPtr(pub Arc<Mutex<Option<object>>>);

impl std::fmt::Display for objectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for objectPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        object::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        object::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        object::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        object::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        object::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<objectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for object {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        object::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<object>() {
            false
        } else {
            false
        }
    }
}

impl positioner for objectPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        object::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<objectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl PkgName {
    /// Imported returns the package that was imported.
    /// It is distinct from Pkg(), which is the package containing the import statement.
    pub fn imported(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        self.imported.clone()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(PkgNamePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for PkgName {
    fn exported(&self) -> bool {
        PkgName::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        PkgName::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        PkgName::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        PkgName::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        PkgName::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        PkgName::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        PkgName::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        PkgName::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        PkgName::color(self)
    }
    fn order(&self) -> u32 {
        PkgName::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        PkgName::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        PkgName::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        PkgName::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        PkgName::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        PkgName::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        PkgName::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        PkgName::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PkgName>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct PkgNamePtr(pub Arc<Mutex<Option<PkgName>>>);

impl std::fmt::Display for PkgNamePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for PkgNamePtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        PkgName::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        PkgName::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        PkgName::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        PkgName::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        PkgName::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PkgNamePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for PkgName {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        PkgName::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PkgName>() {
            false
        } else {
            false
        }
    }
}

impl positioner for PkgNamePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        PkgName::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PkgNamePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Const {
    /// Val returns the constant's value.
    pub fn val(&self) -> Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>> {
        return { let __field = self.val.clone(); __field };
    }

    pub fn is_dependency(&self) {
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(ConstPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Const {
    fn exported(&self) -> bool {
        Const::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Const::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Const::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Const::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Const::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Const::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Const::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Const::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Const::color(self)
    }
    fn order(&self) -> u32 {
        Const::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Const::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Const::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Const::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Const::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Const::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Const::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Const::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Const>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ConstPtr(pub Arc<Mutex<Option<Const>>>);

impl std::fmt::Display for ConstPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for ConstPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Const::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Const::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Const::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Const::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Const::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ConstPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dependency for Const {
    fn is_dependency(&self) {
        Const::is_dependency(self)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Const>() {
            false
        } else {
            false
        }
    }
}

impl dependency for ConstPtr {
    fn is_dependency(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::is_dependency(__recv)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ConstPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Const {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Const::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Const>() {
            false
        } else {
            false
        }
    }
}

impl positioner for ConstPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Const::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ConstPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl TypeName {
    /// IsAlias reports whether obj is an alias name for a type.
    pub fn is_alias(&self) -> bool {
        {
    let _ts_subject = (*self.object.lock().unwrap().as_ref().unwrap()).typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_is_nil {
        let t = _ts_subject.clone();
        return false;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        if { let __left = (*self.object.lock().unwrap().as_ref().unwrap()).pkg.clone(); let __right = (*Unsafe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        return false;
    };
        return { let __nil_target = (*self.object.lock().unwrap().as_ref().unwrap()).pkg.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } || { let __tmp_x = { let __selector_holder = (*t.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*(*self.object.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || { let __left_wrapper = crate::basic::BasicPtr(t.clone()); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = universeByte.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq } || { let __left_wrapper = crate::basic::BasicPtr(t.clone()); let __left_opt: Option<&(dyn Type + Send + Sync)> = Some(&__left_wrapper as &(dyn Type + Send + Sync)); let __right_holder = universeRune.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Type + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (Some(__left), Some(__right)) => __left.__go_eq_type_(__right), _ => false }; __eq };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).unwrap().0.clone();
        return { let __peer = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::typeparam::TypeParamPtr>()).unwrap().0.clone();
        return { let __peer = (*t.lock().unwrap().as_ref().unwrap()).obj.clone(); let __peer_guard = __peer.lock().unwrap(); let __peer_ptr = __peer_guard.as_ref().map(|__v| __v as *const _ as usize); let __self_ptr = self as *const _ as usize; let __eq = __peer_ptr == Some(__self_ptr); !__eq };;
    } else {
        let t = _ts_subject.clone();
        return true;;
    }
    }
    unreachable!()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(TypeNamePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for TypeName {
    fn exported(&self) -> bool {
        TypeName::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        TypeName::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        TypeName::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        TypeName::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        TypeName::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeName::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        TypeName::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        TypeName::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        TypeName::color(self)
    }
    fn order(&self) -> u32 {
        TypeName::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        TypeName::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeName::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        TypeName::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        TypeName::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        TypeName::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        TypeName::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        TypeName::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeName>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TypeNamePtr(pub Arc<Mutex<Option<TypeName>>>);

impl std::fmt::Display for TypeNamePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for TypeNamePtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeName::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeName::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeName::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeName::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        TypeName::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeNamePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for TypeName {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        TypeName::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeName>() {
            false
        } else {
            false
        }
    }
}

impl positioner for TypeNamePtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        TypeName::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TypeNamePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Var {
    /// Anonymous reports whether the variable is an embedded field.
    /// Same as Embedded; only present for backward-compatibility.
    pub fn anonymous(&self) -> bool {
        return (*self.embedded.lock().unwrap().as_ref().unwrap());
    }

    /// Embedded reports whether the variable is an embedded field.
    pub fn embedded(&self) -> bool {
        return (*self.embedded.lock().unwrap().as_ref().unwrap());
    }

    /// IsField reports whether the variable is a struct field.
    pub fn is_field(&self) -> bool {
        return (*self.is_field.lock().unwrap().as_ref().unwrap());
    }

    /// Origin returns the canonical Var for its receiver, i.e. the Var object
    /// recorded in Info.Defs.
    ///
    /// For synthetic Vars created during instantiation (such as struct fields or
    /// function parameters that depend on type arguments), this will be the
    /// corresponding Var on the generic (uninstantiated) type. For all other Vars
    /// Origin returns the receiver.
    pub fn origin(&self) -> Arc<Mutex<Option<Var>>> {
        if { let __nil_target = self.origin.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return self.origin.clone();
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    pub fn is_dependency(&self) {
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(VarPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Var {
    fn exported(&self) -> bool {
        Var::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Var::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Var::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Var::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Var::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Var::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Var::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Var::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Var::color(self)
    }
    fn order(&self) -> u32 {
        Var::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Var::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Var::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Var::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Var::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Var::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Var::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Var::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Var>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct VarPtr(pub Arc<Mutex<Option<Var>>>);

impl std::fmt::Display for VarPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for VarPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Var::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Var::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Var::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Var::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Var::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<VarPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dependency for Var {
    fn is_dependency(&self) {
        Var::is_dependency(self)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Var>() {
            false
        } else {
            false
        }
    }
}

impl dependency for VarPtr {
    fn is_dependency(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::is_dependency(__recv)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<VarPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Var {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Var::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Var>() {
            false
        } else {
            false
        }
    }
}

impl positioner for VarPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<VarPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Func {
    /// Signature returns the signature (type) of the function or method.
    pub fn signature(&self) -> Arc<Mutex<Option<crate::signature::Signature>>> {
        if { let __iface_handle = { let __field = (*self.object.lock().unwrap().as_ref().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return ({
        let val = (*self.object.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    });
    }
                // normal case
                // No signature: Signature was called either:
                // - within go/types, before a FuncDecl's initially
                //   nil Func.Type was lazily populated, indicating
                //   a types bug; or
                // - by a client after NewFunc(..., nil),
                //   which is arguably a client bug, but we need a
                //   proposal to tighten NewFunc's precondition.
                // For now, return a trivial signature.
        Arc::new(Mutex::new(Some(Signature::default())))
    }

    /// FullName returns the package- or receiver-type-qualified name of
    /// function or method obj.
    pub fn full_name(&self) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
        write_func_name(buf.clone(), Arc::new(Mutex::new(Some(self.clone()))), Arc::new(Mutex::new(None)));
        return (*buf.lock().unwrap().as_mut().unwrap()).string();
    }

    /// Scope returns the scope of the function's body block.
    /// The result is nil for imported or instantiated functions and methods
    /// (but there is also no mechanism to get to an instantiated function).
    pub fn scope(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        (*({
        let val = (*self.object.lock().unwrap().as_ref().unwrap()).typ.clone();
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
    }).lock().unwrap().as_ref().unwrap()).scope.clone()
    }

    /// Origin returns the canonical Func for its receiver, i.e. the Func object
    /// recorded in Info.Defs.
    ///
    /// For synthetic functions created during instantiation (such as methods on an
    /// instantiated Named type or interface methods that depend on type arguments),
    /// this will be the corresponding Func on the generic (uninstantiated) type.
    /// For all other Funcs Origin returns the receiver.
    pub fn origin(&self) -> Arc<Mutex<Option<Func>>> {
        if { let __nil_target = self.origin.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return self.origin.clone();
    }
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// Pkg returns the package to which the function belongs.
    ///
    /// The result is nil for methods of types in the Universe scope,
    /// like method Error of the error built-in interface type.
    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        (*self.object.lock().unwrap().as_ref().unwrap()).pkg()
    }

    /// hasPtrRecv reports whether the receiver is of the form *T for the given method obj.
    pub fn has_ptr_recv(&self) -> bool {
                // If a method's receiver type is set, use that as the source of truth for the receiver.
                // Caution: Checker.funcDecl (decl.go) marks a function by setting its type to an empty
                // signature. We may reach here before the signature is fully set up: we must explicitly
                // check if the receiver is set (we cannot just look for non-nil obj.typ).
        {
        let (mut sig, _) = ({
        let val = (*self.object.lock().unwrap().as_ref().unwrap()).typ.clone();
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
        if (*sig.lock().unwrap()).is_some() && { let __nil_target = (*sig.lock().unwrap().as_ref().unwrap()).recv.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
            let (_, mut isPtr) = deref({ let __field = (*(*sig.lock().unwrap().as_ref().unwrap()).recv.lock().unwrap().as_ref().unwrap()).object.lock().unwrap().as_ref().unwrap().typ.clone(); __field });;
            return isPtr;;
        }
    }
                // If a method's type is not set it may be a method/function that is:
                // 1) client-supplied (via NewFunc with no signature), or
                // 2) internally created but not yet type-checked.
                // For case 1) we can't do anything; the client must know what they are doing.
                // For case 2) we can use the information gathered by the resolver.
        return (*self.has_ptr_recv_.lock().unwrap().as_ref().unwrap());
    }

    pub fn is_dependency(&self) {
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(FuncPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Func {
    fn exported(&self) -> bool {
        Func::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Func::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Func::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Func::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Func::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Func::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Func::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Func::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Func::color(self)
    }
    fn order(&self) -> u32 {
        Func::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Func::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Func::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Func::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Func::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Func::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Func::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Func::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Func>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct FuncPtr(pub Arc<Mutex<Option<Func>>>);

impl std::fmt::Display for FuncPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for FuncPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Func::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Func::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Func::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Func::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Func::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl dependency for Func {
    fn is_dependency(&self) {
        Func::is_dependency(self)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Func>() {
            false
        } else {
            false
        }
    }
}

impl dependency for FuncPtr {
    fn is_dependency(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::is_dependency(__recv)
    }
    fn __go_clone_box_dependency(&self) -> Box<dyn dependency + Send + Sync> {
        Box::new(self.clone()) as Box<dyn dependency + Send + Sync>
    }
    fn __go_eq_dependency(&self, other: &(dyn dependency + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Func {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Func::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Func>() {
            false
        } else {
            false
        }
    }
}

impl positioner for FuncPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Func::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FuncPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Label {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(LabelPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Label {
    fn exported(&self) -> bool {
        Label::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Label::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Label::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Label::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Label::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Label::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Label::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Label::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Label::color(self)
    }
    fn order(&self) -> u32 {
        Label::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Label::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Label::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Label::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Label::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Label::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Label::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Label::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Label>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct LabelPtr(pub Arc<Mutex<Option<Label>>>);

impl std::fmt::Display for LabelPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for LabelPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Label::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Label::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Label::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Label::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Label::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabelPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Label {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Label::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Label>() {
            false
        } else {
            false
        }
    }
}

impl positioner for LabelPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Label::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LabelPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Builtin {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(BuiltinPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Builtin {
    fn exported(&self) -> bool {
        Builtin::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Builtin::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Builtin::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Builtin::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Builtin::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Builtin::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Builtin::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Builtin::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Builtin::color(self)
    }
    fn order(&self) -> u32 {
        Builtin::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Builtin::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Builtin::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Builtin::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Builtin::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Builtin::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Builtin::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Builtin::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Builtin>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BuiltinPtr(pub Arc<Mutex<Option<Builtin>>>);

impl std::fmt::Display for BuiltinPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for BuiltinPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Builtin::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Builtin::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Builtin::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Builtin::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Builtin::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BuiltinPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Builtin {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Builtin::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Builtin>() {
            false
        } else {
            false
        }
    }
}

impl positioner for BuiltinPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Builtin::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BuiltinPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Nil {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        object_string(Arc::new(Mutex::new(Some(Box::new(NilPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn Object + Send + Sync>))), Arc::new(Mutex::new(None)))
    }

    pub fn exported(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported()
    }

    pub fn id(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.id()
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.name()
    }

    pub fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.parent()
    }

    pub fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pkg()
    }

    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pos()
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.r#type()
    }

    pub fn cmp(&self, b: Arc<Mutex<Option<object>>>) -> i32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cmp(b)
    }

    pub fn color(&self) -> Arc<Mutex<Option<color>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.color()
    }

    pub fn order(&self) -> u32 {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.order()
    }

    pub fn same_id(&self, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.same_id(pkg, name, foldCase)
    }

    pub fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.scope_pos()
    }

    pub fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_color(color_local)
    }

    pub fn set_order(&mut self, order: Arc<Mutex<Option<u32>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_order(order)
    }

    pub fn set_parent(&mut self, parent: Arc<Mutex<Option<Scope>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_parent(parent)
    }

    pub fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_scope_pos(pos)
    }

    pub fn set_type(&mut self, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        // Forward to embedded type's method
        let embedded = self.object.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.set_type(typ)
    }
}

impl Object for Nil {
    fn exported(&self) -> bool {
        Nil::exported(self)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        Nil::id(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        Nil::name(self)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        Nil::parent(self)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        Nil::pkg(self)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Nil::pos(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Nil::string(self)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        Nil::r#type(self)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        Nil::color(self)
    }
    fn order(&self) -> u32 {
        Nil::order(self)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        Nil::same_id(self, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Nil::scope_pos(self)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        Nil::set_color(self, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        Nil::set_order(self, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        Nil::set_parent(self, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        Nil::set_scope_pos(self, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        Nil::set_type(self, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Nil>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct NilPtr(pub Arc<Mutex<Option<Nil>>>);

impl std::fmt::Display for NilPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Object for NilPtr {
    fn exported(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::exported(__recv)
    }
    fn id(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::id(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::name(__recv)
    }
    fn parent(&self) -> Arc<Mutex<Option<crate::scope::Scope>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::parent(__recv)
    }
    fn pkg(&self) -> Arc<Mutex<Option<crate::package::Package>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::pkg(__recv)
    }
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::pos(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::string(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::r#type(__recv)
    }
    fn color(&self) -> Arc<Mutex<Option<color>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::color(__recv)
    }
    fn order(&self) -> u32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::order(__recv)
    }
    fn same_id(&self, pkg: Arc<Mutex<Option<crate::package::Package>>>, name: Arc<Mutex<Option<String>>>, foldCase: Arc<Mutex<Option<bool>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::same_id(__recv, pkg, name, foldCase)
    }
    fn scope_pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::scope_pos(__recv)
    }
    fn set_color(&mut self, color_local: Arc<Mutex<Option<color>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Nil::set_color(__recv, color_local)
    }
    fn set_order(&mut self, __arg0: Arc<Mutex<Option<u32>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Nil::set_order(__recv, __arg0)
    }
    fn set_parent(&mut self, __arg0: Arc<Mutex<Option<crate::scope::Scope>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Nil::set_parent(__recv, __arg0)
    }
    fn set_scope_pos(&mut self, pos: Arc<Mutex<Option<go_token::position::Pos>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Nil::set_scope_pos(__recv, pos)
    }
    fn set_type(&mut self, __arg0: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        Nil::set_type(__recv, __arg0)
    }
    fn __go_clone_box_object(&self) -> Box<dyn Object + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Object + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_object(&self, other: &(dyn Object + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NilPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl positioner for Nil {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        Nil::pos(self)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Nil>() {
            false
        } else {
            false
        }
    }
}

impl positioner for NilPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Nil::pos(__recv)
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NilPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn is_exported(name: Arc<Mutex<Option<String>>>) -> bool {
    let (mut ch, _) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    unicode::is_upper(Arc::new(Mutex::new(Some(ch))))
}

/// Id returns name if it is exported, otherwise it
/// returns the name qualified with the package path.
pub fn id(pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if is_exported(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return { let __owned = name.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

        // unexported names need the package path for differentiation
        // (if there's no package, make sure we don't start with '.'
        // as that may change the order of methods between a setup
        // inside a package and outside a package - which breaks some
        // tests)
    let mut path = Arc::new(Mutex::new(Some("_".to_string())));

        // pkg is nil for objects in Universe scope and possibly types
        // introduced via Eval (see also comment in object.sameId)
    if (*pkg.lock().unwrap()).is_some() && { let __tmp_x = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = (*pkg.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *path.lock().unwrap() = Some(new_val); };
    }
    return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", ".".to_string())); __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s })));
}

/// colorFor returns the (initial) color for an object depending on
/// whether its type t is known or not.
pub fn color_for(t: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<color>>> {
    if (*t.lock().unwrap()).is_some() {
        return Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(BLACK as u32)))))));
    }
    Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(WHITE as u32)))))))
}

/// NewPkgName returns a new PkgName object representing an imported package.
/// The remaining arguments set the attributes found with all Objects.
pub fn new_pkg_name(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, imported: Arc<Mutex<Option<Package>>>) -> Arc<Mutex<Option<PkgName>>> {
    { let __owner = Arc::new(Mutex::new(Some(PkgName { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(BLACK as u32))))))), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), imported: imported.clone(), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewConst returns a new constant with value val.
/// The remaining arguments set the attributes found with all Objects.
pub fn new_const(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, val: Arc<Mutex<Option<Box<dyn go_constant::value::Value + Send + Sync>>>>) -> Arc<Mutex<Option<Const>>> {
    { let __owner = Arc::new(Mutex::new(Some(Const { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), val: val.clone(), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewTypeName returns a new type name denoting the given typ.
/// The remaining arguments set the attributes found with all Objects.
///
/// The typ argument may be a defined (Named) type or an alias type.
/// It may also be nil such that the returned TypeName can be used as
/// argument for NewNamed, which will set the TypeName's type as a side-
/// effect.
pub fn new_type_name(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<TypeName>>> {
    { let __owner = Arc::new(Mutex::new(Some(TypeName { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewVar returns a new variable.
/// The arguments set the attributes found with all Objects.
pub fn new_var(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Var>>> {
    { let __owner = Arc::new(Mutex::new(Some(Var { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewParam returns a new variable representing a function parameter.
pub fn new_param(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> Arc<Mutex<Option<Var>>> {
    { let __owner = Arc::new(Mutex::new(Some(Var { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), is_param: Arc::new(Mutex::new(Some(true))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewField returns a new variable representing a struct field.
/// For embedded fields, the name is the unqualified type name
/// under which the field is accessible.
pub fn new_field(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>, embedded: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Var>>> {
    { let __owner = Arc::new(Mutex::new(Some(Var { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), embedded: Arc::new(Mutex::new(Some({ let __arg_holder = embedded.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), is_field: Arc::new(Mutex::new(Some(true))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

/// NewFunc returns a new function with the given signature, representing
/// the function's type.
pub fn new_func(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>, sig: Arc<Mutex<Option<Signature>>>) -> Arc<Mutex<Option<Func>>> {
    let mut typ: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> = Arc::new(Mutex::new(None));
    if (*sig.lock().unwrap()).is_some() {
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::signature::SignaturePtr(sig.clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    } else {
    }
        // Don't store a (typed) nil *Signature.
        // We can't simply replace it with new(Signature) either,
        // as this would violate object.{Type,color} invariants.
        // TODO(adonovan): propose to disallow NewFunc with nil *Signature.
    return { let __owner = Arc::new(Mutex::new(Some(Func { object: Arc::new(Mutex::new(Some(object { parent: Default::default(), pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: typ.clone(), order_: Arc::new(Mutex::new(Some(0 as u32))), color_: color_for(typ.clone()), scope_pos_: Arc::new(Mutex::new(Some({ let __arg_holder = nopos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))), has_ptr_recv_: Arc::new(Mutex::new(Some(false))), origin: Default::default(), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner };
}

/// NewLabel returns a new label.
pub fn new_label(pos: Arc<Mutex<Option<go_token::position::Pos>>>, pkg: Arc<Mutex<Option<Package>>>, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Label>>> {
    { let __owner = Arc::new(Mutex::new(Some(Label { object: Arc::new(Mutex::new(Some(object { pos: Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pkg: pkg.clone(), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), color_: Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(BLACK as u32))))))), ..Default::default() }))), used: Arc::new(Mutex::new(Some(false))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

pub fn new_builtin(id: Arc<Mutex<Option<builtinId>>>) -> Arc<Mutex<Option<Builtin>>> {
    { let __owner = Arc::new(Mutex::new(Some(Builtin { object: Arc::new(Mutex::new(Some(object { name: Arc::new(Mutex::new(Some({ let __selector_holder = { let __seq = { let __seq_holder = predeclaredFuncs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*{ let __v = (*id.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as usize].clone() }.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), typ: Arc::new(Mutex::new(Some(Box::new(crate::basic::BasicPtr({ let __seq = { let __seq_holder = Typ.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(INVALID as i32) as usize].clone() }.clone())) as Box<dyn Type + Send + Sync>))), color_: Arc::new(Mutex::new(Some(color(Arc::new(Mutex::new(Some(BLACK as u32))))))), ..Default::default() }))), id: Arc::new(Mutex::new(Some({ let __arg_holder = id.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().object.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner }
}

pub fn write_object(buf: Arc<Mutex<Option<bytes_Buffer>>>, mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, qf: crate::typestring::Qualifier) {
    let mut obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>> = Arc::new(Mutex::new(obj.lock().unwrap().as_ref().map(|__v| Object::__go_clone_box_object(__v.as_ref()))));
    let mut tname: Arc<Mutex<Option<TypeName>>> = Arc::new(Mutex::new(None));
    let mut typ = (*obj.lock().unwrap().as_ref().unwrap()).r#type();

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
    if _ts_val.and_then(|__v| __v.downcast_ref::<PkgNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<PkgNamePtr>()).unwrap().0.clone();
        { let __s = format!("package {}", (*{ let __recv = obj.clone(); let __recv_ptr: *const PkgName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const PkgName }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())); let __n = __s.len() as i32; (*buf.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };;
        {
        let mut path = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*obj.lock().unwrap().as_ref().unwrap()).imported.lock().unwrap().as_ref().unwrap()).path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));;
        if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } && { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = (*(*obj.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
            { let __s = format!(" ({:?})", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __n = __s.len() as i32; (*buf.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };;
        }
    };
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<ConstPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<ConstPtr>()).unwrap().0.clone();
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("const".to_string()); __result };;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<TypeNamePtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<TypeNamePtr>()).unwrap().0.clone();
        { let new_val = obj.clone(); tname = new_val; };;
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("type".to_string()); __result };;
        if is_type_param(typ.clone()) {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string(" parameter".to_string()); __result };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<VarPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<VarPtr>()).unwrap().0.clone();
        if (*{ let __field = (*obj.lock().unwrap().as_ref().unwrap()).is_field.clone(); __field }.lock().unwrap().as_ref().unwrap()) {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("field".to_string()); __result };
    } else {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("var".to_string()); __result };
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<FuncPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<FuncPtr>()).unwrap().0.clone();
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("func ".to_string()); __result };;
        write_func_name(buf.clone(), obj.clone(), qf.clone());;
        if (*typ.lock().unwrap()).is_some() {
        write_signature(buf.clone(), ({
        let val = typ.clone();
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
    }), qf.clone());
    };
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<LabelPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<LabelPtr>()).unwrap().0.clone();
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("label".to_string()); __result };;
        *typ.lock().unwrap() = None;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<BuiltinPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<BuiltinPtr>()).unwrap().0.clone();
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("builtin".to_string()); __result };;
        *typ.lock().unwrap() = None;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<NilPtr>()).is_some() {
        let obj = _ts_val.and_then(|__v| __v.downcast_ref::<NilPtr>()).unwrap().0.clone();
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("nil".to_string()); __result };;
        return;;
    } else {
        let obj = _ts_subject.clone();
        std::panic::panic_any(Box::new({ let __v = Arc::new(Mutex::new(Some(format!("writeObject({})", __go_type_name(obj.lock().unwrap().as_ref().unwrap()))))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>);;
    }
    }

    { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_byte((' ' as i32) as u8); __result };

        // For package-level objects, qualify the name.
    if (*(*obj.lock().unwrap().as_ref().unwrap()).pkg().lock().unwrap()).is_some() && { let __left_holder = (*(*(*obj.lock().unwrap().as_ref().unwrap()).pkg().lock().unwrap().as_ref().unwrap()).scope.lock().unwrap().as_ref().unwrap()).lookup((*obj.lock().unwrap().as_ref().unwrap()).name()).clone(); let __left_guard = __left_holder.lock().unwrap(); let __left_opt: Option<&(dyn Object + Send + Sync)> = __left_guard.as_ref().map(|__v| __v.as_ref()); let __right_holder = obj.clone(); let __right_guard = __right_holder.lock().unwrap(); let __right_opt: Option<&(dyn Object + Send + Sync)> = __right_guard.as_ref().map(|__v| __v.as_ref()); let __eq = match (__left_opt, __right_opt) { (None, None) => true, (Some(__left), Some(__right)) => __left.__go_eq_object(__right), _ => false }; __eq } {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string(package_prefix((*obj.lock().unwrap().as_ref().unwrap()).pkg(), qf.clone())); __result };
    }
    { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string((*obj.lock().unwrap().as_ref().unwrap()).name()); __result };

    if (*typ.lock().unwrap()).is_none() {
        return;
    }

    if (*tname.lock().unwrap()).is_some() {
        {
    let _ts_subject = typ.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn Type + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).is_some() {
        let t = _ts_val.and_then(|__v| __v.downcast_ref::<crate::basic::BasicPtr>()).unwrap().0.clone();
        return;;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::alias::AliasPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::named::NamedPtr>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<crate::signature::SignaturePtr>()).is_some() {
        let t: Arc<Mutex<Option<Box<dyn genericType + Send + Sync>>>> = unimplemented!("type info required: type switch on interface case with 3 concrete implementors needs a synthesized trait object");
        if { let __tmp_x = { let __recv = (*t.lock().unwrap().as_mut().unwrap()).type_params(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __recv = new_type_writer(buf.clone(), qf.clone()); let __result = (*__recv.lock().unwrap().as_mut().unwrap()).t_param_list({ let __recv = (*t.lock().unwrap().as_mut().unwrap()).type_params(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).list(); __result }); __result };
    };
    }
    }
                // Don't print anything more for basic types since there's
                // no more information.
        if { let __recv = tname.clone(); let __recv_ptr: *const TypeName = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const TypeName }; let __result = unsafe { &*__recv_ptr }.is_alias(); __result } {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string(" =".to_string()); __result };
        {
        let (mut alias, mut ok) = ({
        let val = typ.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Type + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::alias::AliasPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::alias::Alias>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::alias::Alias>)), false)
        }
    });;
        if ok {
            { let __iface_handle = { let __field = (*alias.lock().unwrap().as_ref().unwrap()).from_r_h_s.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    } else {
        let (mut t, _) = ({
        let val = typ.clone();
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
        if (*t.lock().unwrap()).is_some() {
            { let __iface_handle = { let __field = (*t.lock().unwrap().as_ref().unwrap()).bound.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        } else {
            { let __iface_handle = under(typ.clone()).clone(); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
    }

        // Don't print anything more for basic types since there's
        // no more information.
        // materialized? (gotypesalias=1)
        // TODO(gri) should this be fromRHS for *Named?
        // (See discussion in #66559.)
        // Special handling for any: because WriteType will format 'any' as 'any',
        // resulting in the object string `type any = any` rather than `type any =
        // interface{}`. To avoid this, swap in a different empty interface.
    if { let __tmp_x = (*(*obj.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "any".to_string(); __tmp_x == __tmp_y } && { let __left = (*obj.lock().unwrap().as_ref().unwrap()).parent(); let __right = (*Universe.lock().unwrap().as_ref().unwrap()).clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq } {
        assert(Arc::new(Mutex::new(Some(identical(typ.clone(), Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>))))))));
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(crate::interface::InterfacePtr(emptyInterface.clone().clone())) as Box<dyn Type + Send + Sync>))); let __iface_guard = __iface_handle.lock().unwrap(); *typ.lock().unwrap() = (*__iface_guard).clone(); };
    }

    { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_byte((' ' as i32) as u8); __result };
    write_type(buf.clone(), typ.clone(), qf.clone());
}

pub fn package_prefix(pkg: Arc<Mutex<Option<Package>>>, qf: crate::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
    if (*pkg.lock().unwrap()).is_none() {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let mut s: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    if (*qf.lock().unwrap()).is_some() {
        { let new_val = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = { let mut __f_guard = qf.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Package>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(pkg.clone()) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = { let __recv = pkg.clone(); let __recv_ptr: *const crate::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::package::Package }; let __result = unsafe { &*__recv_ptr }.path(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { (*s.lock().unwrap().as_mut().unwrap()).push_str(&".".to_string()); };
    }
    return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// ObjectString returns the string form of obj.
/// The Qualifier controls the printing of
/// package-level objects, and may be nil.
pub fn object_string(obj: Arc<Mutex<Option<Box<dyn Object + Send + Sync>>>>, qf: crate::typestring::Qualifier) -> Arc<Mutex<Option<String>>> {
    let mut buf: Arc<Mutex<Option<bytes_Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    write_object(buf.clone(), obj.clone(), qf.clone());
    return (*buf.lock().unwrap().as_mut().unwrap()).string();
}

pub fn write_func_name(buf: Arc<Mutex<Option<bytes_Buffer>>>, f: Arc<Mutex<Option<Func>>>, qf: crate::typestring::Qualifier) {
    if { let __iface_handle = { let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        let mut sig = ({
        let val = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).typ.clone();
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
        {
        let mut recv = { let __recv = sig.clone(); let __recv_ptr: *const crate::signature::Signature = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::signature::Signature }; let __result = unsafe { &*__recv_ptr }.recv(); __result };;
        if (*recv.lock().unwrap()).is_some() {
            { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_byte(('(' as i32) as u8); __result };;
            {
        let (_, mut ok) = ({
        let val = { let __recv = recv.clone(); let __recv_ptr: *const Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Var }; let __result = unsafe { &*__recv_ptr }.r#type(); __result }.clone();
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
            { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string("interface".to_string()); __result };;
        } else {
            write_type(buf.clone(), { let __recv = recv.clone(); let __recv_ptr: *const Var = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Var }; let __result = unsafe { &*__recv_ptr }.r#type(); __result }.clone(), qf.clone());;
        }
    };
            { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_byte((')' as i32) as u8); __result };;
            { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_byte(('.' as i32) as u8); __result };;
        } else if { let __nil_target = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string(package_prefix({ let __field = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).pkg.clone(); __field }, qf.clone())); __result };
    }
    }
    }
        // gcimporter creates abstract methods of
        // named interfaces using the interface type
        // (not the named type) as the receiver.
        // Don't print it in full.
    { let __recv = buf.clone(); let __recv_ptr: *mut bytes_Buffer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bytes_Buffer }; let __result = unsafe { &mut *__recv_ptr }.write_string({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap()).object.lock().unwrap().as_mut().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }); __result };
}

impl GoValueClone for object {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for PkgName {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Const {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for TypeName {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Var {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Func {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Label {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Builtin {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Nil {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
