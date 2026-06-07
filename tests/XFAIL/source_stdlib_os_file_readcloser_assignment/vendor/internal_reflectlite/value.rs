use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any};

use crate::swapper::*;
use crate::r#type::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const FLAG_KIND_WIDTH: i32 = 5;
pub(crate) const FLAG_KIND_MASK: usize = (((1 as usize) << (FLAG_KIND_WIDTH as usize)) - (1 as usize));
pub(crate) const FLAG_STICKY_R_O: usize = ((1 as usize) << (5 as usize));
pub(crate) const FLAG_EMBED_R_O: usize = ((1 as usize) << (6 as usize));
pub(crate) const FLAG_INDIR: usize = ((1 as usize) << (7 as usize));
pub(crate) const FLAG_ADDR: usize = ((1 as usize) << (8 as usize));
pub(crate) const FLAG_METHOD: usize = ((1 as usize) << (9 as usize));
pub(crate) const FLAG_METHOD_SHIFT: i32 = 10;
pub(crate) const FLAG_R_O: usize = ((FLAG_STICKY_R_O as usize) | (FLAG_EMBED_R_O as usize));


#[derive(Debug, Clone, Default)]
pub struct flag(pub Arc<Mutex<Option<usize>>>);

impl Display for flag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for flag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<usize> for flag {
    fn eq(&self, other: &usize) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for flag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<usize> for flag {
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<flag> for usize {
    fn eq(&self, other: &flag) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<flag> for usize {
    fn partial_cmp(&self, other: &flag) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for flag {
    type Output = flag;
    fn add(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<usize> for flag {
    type Output = flag;
    fn add(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<flag> for usize {
    type Output = flag;
    fn add(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for flag {
    type Output = flag;
    fn sub(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<usize> for flag {
    type Output = flag;
    fn sub(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<flag> for usize {
    type Output = flag;
    fn sub(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for flag {
    type Output = flag;
    fn mul(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<usize> for flag {
    type Output = flag;
    fn mul(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<flag> for usize {
    type Output = flag;
    fn mul(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for flag {
    type Output = flag;
    fn div(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<usize> for flag {
    type Output = flag;
    fn div(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<flag> for usize {
    type Output = flag;
    fn div(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for flag {
    type Output = flag;
    fn rem(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<usize> for flag {
    type Output = flag;
    fn rem(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<flag> for usize {
    type Output = flag;
    fn rem(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for flag {
    type Output = flag;
    fn bitand(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<usize> for flag {
    type Output = flag;
    fn bitand(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<flag> for usize {
    type Output = flag;
    fn bitand(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for flag {
    type Output = flag;
    fn bitor(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<usize> for flag {
    type Output = flag;
    fn bitor(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<flag> for usize {
    type Output = flag;
    fn bitor(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for flag {
    type Output = flag;
    fn bitxor(self, other: Self) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<usize> for flag {
    type Output = flag;
    fn bitxor(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<flag> for usize {
    type Output = flag;
    fn bitxor(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for flag {
    type Output = flag;
    fn not(self) -> flag {
        flag(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for flag {
    type Output = flag;
    fn shl(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for flag {
    type Output = flag;
    fn shl(self, other: i32) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for flag {
    type Output = flag;
    fn shl(self, other: i8) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for flag {
    type Output = flag;
    fn shl(self, other: i16) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for flag {
    type Output = flag;
    fn shl(self, other: i64) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for flag {
    type Output = flag;
    fn shl(self, other: u32) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for flag {
    type Output = flag;
    fn shl(self, other: u8) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for flag {
    type Output = flag;
    fn shl(self, other: u16) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for flag {
    type Output = flag;
    fn shl(self, other: u64) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for flag {
    type Output = flag;
    fn shl(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for flag {
    type Output = flag;
    fn shr(self, other: flag) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for flag {
    type Output = flag;
    fn shr(self, other: i32) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for flag {
    type Output = flag;
    fn shr(self, other: i8) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for flag {
    type Output = flag;
    fn shr(self, other: i16) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for flag {
    type Output = flag;
    fn shr(self, other: i64) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for flag {
    type Output = flag;
    fn shr(self, other: u32) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for flag {
    type Output = flag;
    fn shr(self, other: u8) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for flag {
    type Output = flag;
    fn shr(self, other: u16) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for flag {
    type Output = flag;
    fn shr(self, other: u64) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for flag {
    type Output = flag;
    fn shr(self, other: usize) -> flag {
        flag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for flag {}

impl Ord for flag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A ValueError occurs when a Value method is invoked on
/// a Value that does not support it. Such cases are documented
/// in the description of each method.
#[derive(Clone)]
pub struct ValueError {
    pub method: Arc<Mutex<Option<String>>>,
    pub kind: Kind,
}

impl ValueError {
    pub fn __go_value_clone(&self) -> Self {
        Self { method: { let __guard = self.method.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, kind: { let __guard = self.kind.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for ValueError {
    fn default() -> Self {
        Self { method: Arc::new(Mutex::new(Some(String::new()))), kind: Arc::new(Mutex::new(Some(internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(0))))))) }
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for ValueError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Method") {
            out.method = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub(crate) static dummy: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct1>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *dummy.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *dummy.lock().unwrap() = Some(Default::default());
}


impl flag {
    pub fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        Arc::new(Mutex::new(Some(internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & FLAG_KIND_MASK as usize) as u8)))))))
    }

    pub fn ro(&self) -> Arc<Mutex<Option<flag>>> {
        if { let __tmp_x = flag(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & FLAG_R_O as usize))))); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some(flag(Arc::new(Mutex::new(Some(FLAG_STICKY_R_O as usize)))))));
    }
        Arc::new(Mutex::new(Some(flag(Arc::new(Mutex::new(Some(0 as usize)))))))
    }

    /// mustBeExported panics if f records that the value was obtained using
    /// an unexported field.
    pub fn must_be_exported(&self) {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(ValueError { method: method_name(), kind: Arc::new(Mutex::new(Some(internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(0 as u8))))))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>);
    }
        if { let __tmp_x = flag(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & FLAG_R_O as usize))))); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let mut __s = String::new(); __s.push_str(&format!("{}", "reflect: ".to_string())); __s.push_str(&format!("{}", (*method_name().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " using value obtained using unexported field".to_string())); __s }) as Box<dyn Any + Send + Sync>);
    }
    }

    /// mustBeAssignable panics if f records that the value is not assignable,
    /// which is to say that either it was obtained using an unexported field
    /// or it is not addressable.
    pub fn must_be_assignable(&self) {
        if { let __tmp_x = (*self.0.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new(Arc::new(Mutex::new(Some(ValueError { method: method_name(), kind: Arc::new(Mutex::new(Some(internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::INVALID as u8))))))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>);
    }
                // Assignable if addressable and not read-only.
        if { let __tmp_x = flag(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & FLAG_R_O as usize))))); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new({ let mut __s = String::new(); __s.push_str(&format!("{}", "reflect: ".to_string())); __s.push_str(&format!("{}", (*method_name().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " using value obtained using unexported field".to_string())); __s }) as Box<dyn Any + Send + Sync>);
    }
        if { let __tmp_x = flag(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & FLAG_ADDR as usize))))); let __tmp_y = flag(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x == __tmp_y } {
        std::panic::panic_any(Box::new({ let mut __s = String::new(); __s.push_str(&format!("{}", "reflect: ".to_string())); __s.push_str(&format!("{}", (*method_name().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " using unaddressable value".to_string())); __s }) as Box<dyn Any + Send + Sync>);
    }
    }
}

impl ValueError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = (*self.kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "reflect: call of ".to_string())); __s.push_str(&format!("{}", (*self.method.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " on zero Value".to_string())); __s })));
    }
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "reflect: call of ".to_string())); __s.push_str(&format!("{}", (*self.method.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " on ".to_string())); __s.push_str(&format!("{}", (*internal_abi::r#type::Kind::string(&(*self.kind.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", " Value".to_string())); __s })));
    }
}

impl StdError for ValueError {}


/// methodName returns the name of the calling method,
/// assumed to be two stack frames above.
pub fn method_name() -> Arc<Mutex<Option<String>>> {
    let (mut pc, _, _, _) = runtime::caller(Arc::new(Mutex::new(Some(2))));
    let mut f: GoPtr<runtime::symtab::Func> = { let __go_ptr = runtime::func_for_p_c(Arc::new(Mutex::new(Some(pc)))).clone(); match __go_ptr { runtime::GoPtr::Nil => GoPtr::nil(), runtime::GoPtr::Local(__value) => GoPtr::local(__value.clone()), runtime::GoPtr::Raw(__addr) => GoPtr::raw(__addr), runtime::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), runtime::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
    if f.is_nil() {
        return Arc::new(Mutex::new(Some("unknown method".to_string())));
    }
    { let __recv_value = f.borrow(); let __result = (*__recv_value.as_ref().unwrap()).name(); __result }
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub b: Arc<Mutex<Option<bool>>>,
    pub x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { b: { let __guard = self.b.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, x: self.x.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { b: Arc::new(Mutex::new(Some(false))), x: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.b.lock().unwrap().as_ref().unwrap()), format_any(self.x.lock().unwrap().as_ref().unwrap().as_ref()))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type dummy = AnonymousStruct1;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for ValueError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
