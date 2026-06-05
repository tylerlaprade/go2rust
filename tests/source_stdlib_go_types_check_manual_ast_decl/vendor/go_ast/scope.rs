use go2rust_stdlib_stubs::*;

use crate::{format_any, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::r#mod::*;
use crate::commentmap::*;
use crate::filter::*;
use crate::import::*;
use crate::print::*;
use crate::resolve::*;
use crate::walk::*;

use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const BAD: i32 = 0;
pub const PKG: i32 = 1;
pub const CON: i32 = 2;
pub const TYP: i32 = 3;
pub const VAR: i32 = 4;
pub const FUN: i32 = 5;
pub const LBL: i32 = 6;


/// A Scope maintains the set of named language entities declared
/// in the scope and a link to the immediately surrounding (outer)
/// scope.
///
/// Deprecated: use the type checker [go/types] instead; see [Object].
#[derive(Clone, Default)]
pub struct Scope {
    pub outer: Arc<Mutex<Option<Scope>>>,
    pub objects: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Object>>>>>>>,
}

impl Scope {
    pub fn __go_value_clone(&self) -> Self {
        Self { outer: self.outer.clone(), objects: self.objects.clone() }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Scope {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// An Object describes a named language entity such as a package,
/// constant, type, variable, function (incl. methods), or label.
///
/// The Data fields contains object-specific data:
///
///	Kind    Data type         Data value
///	Pkg     *Scope            package scope
///	Con     int               iota for the respective declaration
///
/// Deprecated: The relationship between Idents and Objects cannot be
/// correctly computed without type information. For example, the
/// expression T{K: 0} may denote a struct, map, slice, or array
/// literal, depending on the type of T. If T is a struct, then K
/// refers to a field of T, whereas for the other types it refers to a
/// value in the environment.
///
/// New programs should set the [parser.SkipObjectResolution] parser
/// flag to disable syntactic object resolution (which also saves CPU
/// and memory), and instead use the type checker [go/types] if object
/// resolution is desired. See the Defs, Uses, and Implicits fields of
/// the [types.Info] struct for details.
#[derive(Clone)]
pub struct Object {
    pub kind: Arc<Mutex<Option<ObjKind>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub decl: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub data: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
    pub r#type: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl Object {
    pub fn __go_value_clone(&self) -> Self {
        Self { kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, decl: self.decl.clone(), data: self.data.clone(), r#type: self.r#type.clone() }
    }
}


impl Default for Object {
    fn default() -> Self {
        Self { kind: Arc::new(Mutex::new(Some(ObjKind(Arc::new(Mutex::new(Some(0))))))), name: Arc::new(Mutex::new(Some(String::new()))), decl: Arc::new(Mutex::new(None)), data: Arc::new(Mutex::new(None)), r#type: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.kind.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()), format_any(self.decl.lock().unwrap().as_ref().unwrap().as_ref()), format_any(self.data.lock().unwrap().as_ref().unwrap().as_ref()), format_any(self.r#type.lock().unwrap().as_ref().unwrap().as_ref()))
    }
}

impl GoJsonDecode for Object {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Name") {
            out.name = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// ObjKind describes what an [Object] represents.
#[derive(Debug, Clone, Default)]
pub struct ObjKind(pub Arc<Mutex<Option<i32>>>);

impl Display for ObjKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for ObjKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for ObjKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for ObjKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for ObjKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<ObjKind> for i32 {
    fn eq(&self, other: &ObjKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<ObjKind> for i32 {
    fn partial_cmp(&self, other: &ObjKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for ObjKind {
    type Output = ObjKind;
    fn add(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for ObjKind {
    type Output = ObjKind;
    fn add(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<ObjKind> for i32 {
    type Output = ObjKind;
    fn add(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for ObjKind {
    type Output = ObjKind;
    fn sub(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<ObjKind> for i32 {
    type Output = ObjKind;
    fn sub(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for ObjKind {
    type Output = ObjKind;
    fn mul(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for ObjKind {
    type Output = ObjKind;
    fn mul(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<ObjKind> for i32 {
    type Output = ObjKind;
    fn mul(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for ObjKind {
    type Output = ObjKind;
    fn div(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for ObjKind {
    type Output = ObjKind;
    fn div(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<ObjKind> for i32 {
    type Output = ObjKind;
    fn div(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for ObjKind {
    type Output = ObjKind;
    fn neg(self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for ObjKind {
    type Output = ObjKind;
    fn rem(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for ObjKind {
    type Output = ObjKind;
    fn rem(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<ObjKind> for i32 {
    type Output = ObjKind;
    fn rem(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for ObjKind {
    type Output = ObjKind;
    fn bitand(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitand(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for ObjKind {
    type Output = ObjKind;
    fn bitor(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitor(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: Self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for ObjKind {
    type Output = ObjKind;
    fn bitxor(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<ObjKind> for i32 {
    type Output = ObjKind;
    fn bitxor(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for ObjKind {
    type Output = ObjKind;
    fn not(self) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i8) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i16) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: i64) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u8) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u16) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: u64) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for ObjKind {
    type Output = ObjKind;
    fn shl(self, other: usize) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: ObjKind) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i8) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i16) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: i64) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u32) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u8) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u16) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: u64) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for ObjKind {
    type Output = ObjKind;
    fn shr(self, other: usize) -> ObjKind {
        ObjKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for ObjKind {}

impl Ord for ObjKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static objKindStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 7]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *objKindStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *objKindStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["bad".to_string(), "package".to_string(), "const".to_string(), "type".to_string(), "var".to_string(), "func".to_string(), "label".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *objKindStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
}


pub(crate) fn __go_init_order_2() {
    *objKindStrings.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(["bad".to_string(), "package".to_string(), "const".to_string(), "type".to_string(), "var".to_string(), "func".to_string(), "label".to_string()]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl Scope {
    /// Lookup returns the object with the given name if it is
    /// found in scope s, otherwise it returns nil. Outer scopes
    /// are ignored.
    pub fn lookup(&self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Object>>> {
        { let __map = { let __map_holder = self.objects.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&(*name.lock().unwrap().as_ref().unwrap()).clone())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }
    }

    /// Insert attempts to insert a named object obj into the scope s.
    /// If the scope already contains an object alt with the same name,
    /// Insert leaves the scope unchanged and returns alt. Otherwise
    /// it inserts obj and returns nil.
    pub fn insert(&mut self, obj: Arc<Mutex<Option<Object>>>) -> Arc<Mutex<Option<Object>>> {
    let mut alt: Arc<Mutex<Option<Object>>> = Arc::new(Mutex::new(None));

        {
        { let new_val = { let __map = { let __map_holder = self.objects.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = (*obj.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); alt = new_val; };;
        if (*alt.lock().unwrap()).is_none() {
            { let __map_key = { let __selector_holder = (*obj.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __map_value = obj.clone(); (*self.objects.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
        alt.clone()
    }

    /// Debugging support
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("scope {:p} {{", self)))));
        if true && { let __tmp_x = (({ let __len_target = { let __field = self.objects.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("\n")))));
        for (_, obj) in { let __range_holder = self.objects.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); __range_map } {
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("\t{} {}\n", (*{ let __field = (*obj.lock().unwrap().as_ref().unwrap()).kind.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone(), (*{ let __field = (*obj.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())))));
    }
    }
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("}}\n")))));
        return (*buf.lock().unwrap().as_ref().unwrap()).string();
    }
}

impl Object {
    /// Pos computes the source position of the declaration of an object name.
    /// The result may be an invalid position if it cannot be computed
    /// (obj.Decl may be nil or not correct).
    pub fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        let mut name = Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        {
    let _ts_subject = self.decl.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| __v.as_ref() as &dyn Any);
    if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::Field>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::Field>()).unwrap().clone())));
        drop(_ts_guard);
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __recv = n.clone(); let __recv_ptr: *const crate::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ImportSpec>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ImportSpec>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __nil_target = (*d.lock().unwrap().as_ref().unwrap()).name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = { let __selector_holder = (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos();
    };
        return (*(*d.lock().unwrap().as_ref().unwrap()).path.lock().unwrap().as_ref().unwrap()).pos();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ValueSpec>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::ValueSpec>()).unwrap().clone())));
        drop(_ts_guard);
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __recv = n.clone(); let __recv_ptr: *const crate::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result };
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSpec>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::TypeSpec>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos();
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncDecl>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::FuncDecl>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (*(*d.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).pos();
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::LabeledStmt>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::LabeledStmt>()).unwrap().clone())));
        drop(_ts_guard);
        if { let __tmp_x = { let __selector_holder = (*(*d.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (*(*d.lock().unwrap().as_ref().unwrap()).label.lock().unwrap().as_ref().unwrap()).pos();
    };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::AssignStmt>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<crate::r#mod::AssignStmt>()).unwrap().clone())));
        drop(_ts_guard);
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).lhs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for x in __range_values.iter() {
        {
        let (mut ident, mut isIdent) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<crate::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<crate::r#mod::Ident>)), false)
        }
    });;
        if isIdent && { let __tmp_x = { let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = (*name.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
            return { let __recv = ident.clone(); let __recv_ptr: *const crate::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const crate::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result };;
        }
    }
    } };
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Scope>()).is_some() {
        let d = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<Scope>()).unwrap().clone())));
        drop(_ts_guard);
    }
    }
                // predeclared object - nothing to do for now
        Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))))))
    }
}

impl ObjKind {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = objKindStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })))
    }
}

impl cmp::r#mod::Ordered for ObjKind {
    fn __go_clone_box_ordered(&self) -> Box<dyn cmp::r#mod::Ordered + Send + Sync> {
        Box::new(self.clone()) as Box<dyn cmp::r#mod::Ordered + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_ordered(&self, other: &(dyn cmp::r#mod::Ordered + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ObjKind>() {
            self == __other
        } else {
            false
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Scope {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Object {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
