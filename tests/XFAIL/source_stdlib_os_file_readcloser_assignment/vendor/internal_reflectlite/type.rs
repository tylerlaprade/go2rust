use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any};

use crate::swapper::*;
use crate::value::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const PTR: u8 = internal_abi::POINTER;


pub const INTERFACE: u8 = internal_abi::INTERFACE;
pub const SLICE: u8 = internal_abi::SLICE;
pub const STRING: u8 = internal_abi::STRING;
pub const STRUCT: u8 = internal_abi::STRUCT;


/// Type is the representation of a Go type.
///
/// Not all methods apply to all kinds of types. Restrictions,
/// if any, are noted in the documentation for each method.
/// Use the Kind method to find out the kind of type before
/// calling kind-specific methods. Calling a method
/// inappropriate to the kind of type causes a run-time panic.
///
/// Type values are comparable, such as with the == operator,
/// so they can be used as map keys.
/// Two Type values are equal if they represent identical types.
pub trait Type: std::fmt::Display + Any {
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn pkg_path(&self) -> Arc<Mutex<Option<String>>>;
    fn size(&self) -> usize;
    fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>>;
    fn implements(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool;
    fn assignable_to(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool;
    fn comparable(&self) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
    fn elem(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>;
    fn common(&self) -> GoPtr<internal_abi::r#type::Type>;
    fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>>;
}

impl Clone for Box<dyn Type + Send + Sync> {
    fn clone(&self) -> Self {
        Type::__go_clone_box_type_(self.as_ref())
    }
}

/// A Kind represents the specific kind of type that a Type represents.
/// The zero Kind is not a valid kind.
pub type Kind = Arc<Mutex<Option<internal_abi::r#type::Kind>>>;


pub type nameOff = Arc<Mutex<Option<internal_abi::r#type::NameOff>>>;


pub type typeOff = Arc<Mutex<Option<internal_abi::r#type::TypeOff>>>;


#[derive(Clone, Default)]
pub struct rtype {
    pub r#type: GoPtr<internal_abi::r#type::Type>,
}

impl rtype {
    pub fn __go_value_clone(&self) -> Self {
        Self { r#type: self.r#type.clone() }
    }
}

impl std::fmt::Display for rtype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for rtype {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// uncommonType is present only for defined types or types with methods
/// (if T is a defined type, the uncommonTypes for T and *T have methods).
/// Using a pointer to this struct reduces the overall size required
/// to describe a non-defined type with no methods.
pub type uncommonType = Arc<Mutex<Option<internal_abi::r#type::UncommonType>>>;


pub type funcType = Arc<Mutex<Option<internal_abi::r#type::FuncType>>>;


pub type interfaceType = Arc<Mutex<Option<internal_abi::r#type::InterfaceType>>>;


/// structType represents a struct type.
pub type structType = Arc<Mutex<Option<internal_abi::r#type::StructType>>>;


/// name is an encoded type name with optional extra data.
///
/// The first byte is a bit field containing:
///
///	1<<0 the name is exported
///	1<<1 tag data follows the name
///	1<<2 pkgPath nameOff follows the name and tag
///
/// The next two bytes are the data length:
///
///	l := uint16(data[1])<<8 | uint16(data[2])
///
/// Bytes [3:3+l] are the string data.
///
/// If tag data follows then bytes 3+l and 3+l+1 are the tag length,
/// with the data following.
///
/// If the import path follows, then 4 bytes at the end of
/// the data form a nameOff. The import path is only set for concrete
/// methods that are defined in a different package than their type.
///
/// If a name starts with "*", then the exported bit represents
/// whether the pointed to type is exported.
#[derive(Debug, Clone, Default)]
pub struct name {
    pub bytes: Arc<Mutex<Option<u8>>>,
}

impl name {
    pub fn __go_value_clone(&self) -> Self {
        Self { bytes: self.bytes.clone() }
    }
}

impl std::fmt::Display for name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.bytes.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for name {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl name {
    pub fn data(&self, off: Arc<Mutex<Option<i32>>>, whySafe: Arc<Mutex<Option<String>>>) -> GoPtr<u8> {
        GoPtr::raw({ let __ptr = add(Arc::new(Mutex::new(Some(Arc::as_ptr(&self.bytes.clone()) as usize))), Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = whySafe.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    pub fn is_exported(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __v = (*self.bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ((1 as u8) << (0 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    pub fn has_tag(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __v = (*self.bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ((1 as u8) << (1 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    pub fn embedded(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __v = (*self.bytes.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = ((1 as u8) << (3 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// readVarint parses a varint as encoded by encoding/binary.
    /// It returns the number of encoded bytes and the encoded value.
    pub fn read_varint(&self, off: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
        let mut v = Arc::new(Mutex::new(Some(0)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut x = Arc::new(Mutex::new(Some({ let __ptr_handle = self.data(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some("read varint".to_string())))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7f as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 7; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }, { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if { let __nil_target = self.bytes.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let (mut i, mut l) = self.read_varint(Arc::new(Mutex::new(Some(1))));
        { let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }
    }

    pub fn tag(&self) -> Arc<Mutex<Option<String>>> {
        if !self.has_tag() {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let (mut i, mut l) = self.read_varint(Arc::new(Mutex::new(Some(1))));
        let (mut i2, mut l2) = self.read_varint(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = l; __tmp_x + __tmp_y }))));
        { let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }
    }
}

impl rtype {
    pub fn name_off(&self, off: nameOff) -> Arc<Mutex<Option<internal_abi::r#type::Name>>> {
        Arc::new(Mutex::new(Some(internal_abi::r#type::Name { bytes: internal_abi::GoPtr::local(Arc::new(Mutex::new({ let __ptr = resolve_name_off(Arc::new(Mutex::new(Some(self.r#type.addr()))), Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone()), ..Default::default() })))
    }

    pub fn type_off(&self, off: typeOff) -> GoPtr<internal_abi::r#type::Type> {
        GoPtr::raw({ let __ptr = resolve_type_off(Arc::new(Mutex::new(Some(self.r#type.addr()))), Arc::new(Mutex::new(Some((*{ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) as i32)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>> {
        { let __promoted_recv = self.r#type.clone(); let __result = __promoted_recv.with_mut(|__promoted_ref| { __promoted_ref.uncommon() }); __result }
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut s = { let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.str.clone(); __field }); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result };
        if { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_EXTRA_STAR as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() })));
    }
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn common(&self) -> GoPtr<internal_abi::r#type::Type> {
        self.r#type.clone()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<internal_abi::r#type::Method>>>> {
        let mut ut = self.uncommon();
        if { let __nil_result = (*ut.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
        return { let __recv = ut.clone(); let __recv_ptr: *const internal_abi::r#type::UncommonType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::UncommonType }; let __result = unsafe { &*__recv_ptr }.exported_methods(); __result };
    }

    pub fn num_method(&self) -> i32 {
        let mut tt: GoPtr<internal_abi::r#type::InterfaceType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).interface_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if !tt.is_nil() {
        return { let __recv_value = tt.borrow(); let __result = (*__recv_value.as_ref().unwrap()).num_method(); __result };
    }
        (*self.exported_methods().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }

    pub fn pkg_path(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = { let __tmp_x = (*{ let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.t_flag.clone(); __field }); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(internal_abi::T_FLAG_NAMED as u8)))); __tmp_x & __tmp_y }; let __tmp_y = internal_abi::r#type::TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut ut = self.uncommon();
        if { let __nil_result = (*ut.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        return { let __recv = self.name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*ut.lock().unwrap().as_ref().unwrap()).pkg_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result };
    }

    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if !{ let __promoted_recv = self.r#type.clone(); let __result = __promoted_recv.with_mut(|__promoted_ref| { __promoted_ref.has_name() }); __result } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let mut s = self.string();
        let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut sqBrackets = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && ({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } || { let __tmp_x = { let __v = (*sqBrackets.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x != __tmp_y }) {
        { let _switch_val = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
    if _switch_val == ((']' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if _switch_val == (('[' as i32) as u8) {
            { let mut guard = sqBrackets.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })));
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        to_type(GoPtr::local(elem(self.common()))).clone()
    }

    pub fn r#in(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut tt: GoPtr<internal_abi::r#type::FuncType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).func_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: In of non-func type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        to_type(GoPtr::local({ let __seq = { let __seq_holder = { let __result = tt.with_mut(|__recv_value| __recv_value.in_slice()); __result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).clone()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut tt: GoPtr<internal_abi::map_swiss::SwissMapType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).map_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: Key of non-map type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        to_type(GoPtr::local({ let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.key.clone()); __ptr_value }.clone())).clone()
    }

    pub fn len(&self) -> i32 {
        let mut tt: GoPtr<internal_abi::r#type::ArrayType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).array_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: Len of non-array type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.len.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn num_field(&self) -> i32 {
        let mut tt: GoPtr<internal_abi::r#type::StructType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).struct_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: NumField of non-struct type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        ({ let __len_target = { let __field = { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    pub fn num_in(&self) -> i32 {
        let mut tt: GoPtr<internal_abi::r#type::FuncType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).func_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: NumIn of non-func type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        (*Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.in_count.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn num_out(&self) -> i32 {
        let mut tt: GoPtr<internal_abi::r#type::FuncType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).func_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: NumOut of non-func type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __recv_value = tt.borrow(); let __result = (*__recv_value.as_ref().unwrap()).num_out(); __result }
    }

    pub fn out(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let mut tt: GoPtr<internal_abi::r#type::FuncType> = { let __go_ptr = { let __recv_field = self.r#type.clone(); let __recv_value = __recv_field.borrow(); let __result = (*__recv_value.as_ref().unwrap()).func_type(); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if tt.is_nil() {
        std::panic::panic_any(Box::new("reflect: Out of non-func type".to_string()) as Box<dyn Any + Send + Sync>);
    }
        to_type(GoPtr::local({ let __seq = { let __seq_holder = { let __result = tt.with_mut(|__recv_value| __recv_value.out_slice()); __result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).clone()
    }

    pub fn implements(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        if { let __nil_result = (*u.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("reflect: nil type passed to Type.Implements".to_string()) as Box<dyn Any + Send + Sync>);
    }
        if { let __tmp_x = (*(*u.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(INTERFACE as u8)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("reflect: non-interface type passed to Type.Implements".to_string()) as Box<dyn Any + Send + Sync>);
    }
        implements((*u.lock().unwrap().as_ref().unwrap()).common(), self.common())
    }

    pub fn assignable_to(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        if { let __nil_result = (*u.lock().unwrap()).is_none(); __nil_result } {
        std::panic::panic_any(Box::new("reflect: nil type passed to Type.AssignableTo".to_string()) as Box<dyn Any + Send + Sync>);
    }
        let mut uu: GoPtr<internal_abi::r#type::Type> = (*u.lock().unwrap().as_ref().unwrap()).common();
        let mut tt: GoPtr<internal_abi::r#type::Type> = self.common();
        directly_assignable(uu.clone(), tt.clone()) || implements(uu.clone(), tt.clone())
    }

    pub fn comparable(&self) -> bool {
        return { let __nil_target = { let __ptr_value = self.r#type.with_mut(|__ptr_value| { let __field = __ptr_value.equal.clone(); __field }); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
    }

    pub fn align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.align() })
    }

    pub fn array_type(&self) -> GoPtr<internal_abi::r#type::ArrayType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.array_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<internal_abi::r#type::ChanDir>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.chan_dir() })
    }

    pub fn field_align(&self) -> i32 {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.field_align() })
    }

    pub fn func_type(&self) -> GoPtr<internal_abi::r#type::FuncType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.func_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn gc_slice(&self, _arg0: Arc<Mutex<Option<usize>>>, _arg1: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.gc_slice(_arg0, _arg1) })
    }

    pub fn has_name(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.has_name() })
    }

    pub fn iface_indir(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.iface_indir() })
    }

    pub fn interface_type(&self) -> GoPtr<internal_abi::r#type::InterfaceType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.interface_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn is_direct_iface(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.is_direct_iface() })
    }

    pub fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.kind() })
    }

    pub fn map_type(&self) -> GoPtr<internal_abi::map_swiss::SwissMapType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.map_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }

    pub fn pointers(&self) -> bool {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.pointers() })
    }

    pub fn size(&self) -> usize {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { embedded_ref.size() })
    }

    pub fn struct_type(&self) -> GoPtr<internal_abi::r#type::StructType> {
        let embedded = self.r#type.clone();
        embedded.with_mut(|embedded_ref| { { let __go_ptr = embedded_ref.struct_type().clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } } })
    }
}

impl Type for rtype {
    fn assignable_to(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        rtype::assignable_to(self, u)
    }
    fn comparable(&self) -> bool {
        rtype::comparable(self)
    }
    fn elem(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        rtype::elem(self)
    }
    fn implements(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        rtype::implements(self, u)
    }
    fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        rtype::kind(self)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        rtype::name(self)
    }
    fn pkg_path(&self) -> Arc<Mutex<Option<String>>> {
        rtype::pkg_path(self)
    }
    fn size(&self) -> usize {
        rtype::size(self)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        rtype::string(self)
    }
    fn common(&self) -> GoPtr<internal_abi::r#type::Type> {
        rtype::common(self)
    }
    fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>> {
        rtype::uncommon(self)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rtype>() {
            panic!("interface comparison with uncomparable dynamic type")
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct rtypePtr(pub Arc<Mutex<Option<rtype>>>);

impl std::fmt::Display for rtypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Type for rtypePtr {
    fn assignable_to(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::assignable_to(__recv, u)
    }
    fn comparable(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::comparable(__recv)
    }
    fn elem(&self) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::elem(__recv)
    }
    fn implements(&self, u: Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::implements(__recv, u)
    }
    fn kind(&self) -> Arc<Mutex<Option<internal_abi::r#type::Kind>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::kind(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::name(__recv)
    }
    fn pkg_path(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::pkg_path(__recv)
    }
    fn size(&self) -> usize {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::size(__recv)
    }
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::string(__recv)
    }
    fn common(&self) -> GoPtr<internal_abi::r#type::Type> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::common(__recv)
    }
    fn uncommon(&self) -> Arc<Mutex<Option<internal_abi::r#type::UncommonType>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rtype::uncommon(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rtypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn pkg_path(n: Arc<Mutex<Option<internal_abi::r#type::Name>>>) -> Arc<Mutex<Option<String>>> {
    if { let __ptr_field = (*n.lock().unwrap().as_ref().unwrap()).bytes.clone(); __ptr_field.is_nil() } || { let __tmp_x = { let __tmp_x = { let __ptr_handle = (*n.lock().unwrap().as_ref().unwrap()).data_checked(Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some("name flag field".to_string())))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ((1 as u8) << (2 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    let (mut i, mut l) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some(1))));
    let mut off = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = l; __tmp_x + __tmp_y })));
    if (*n.lock().unwrap().as_ref().unwrap()).has_tag() {
        let (mut i2, mut l2) = (*n.lock().unwrap().as_ref().unwrap()).read_varint(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __rhs = { let __tmp_x = i2; let __tmp_y = l2; __tmp_x + __tmp_y }; let mut guard = off.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    let mut nameOff: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        // Note that this field may not be aligned in memory,
        // so we cannot use a direct int32 assignment here.
    { let _dst_start = 0; let _dst_len = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_ref().unwrap()).len() - _dst_start; let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).data_checked(Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("name offset field".to_string())))).addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = __seq.len(); let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min(_dst_len, _src.len()); for _i in 0.._n { (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&nameOff.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 4]>(unimplemented!("unsafe.Pointer conversion to [u8; 4]")) } })).lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    let mut pkgPathName = Arc::new(Mutex::new(Some(name { bytes: Arc::new(Mutex::new({ let __ptr = resolve_type_off(Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).bytes.addr()))), Arc::new(Mutex::new(Some({ let __arg_holder = nameOff.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).clone(), ..Default::default() })));
    return (*pkgPathName.lock().unwrap().as_ref().unwrap()).name();
}

/// resolveNameOff resolves a name offset from a base pointer.
/// The (*rtype).nameOff method is a convenience wrapper for this function.
/// Implemented in the runtime package.
///
///go:noescape
pub fn resolve_name_off(ptrInModule: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
    unimplemented!("Go function declaration has no body");
}


/// resolveTypeOff resolves an *rtype offset from a base type.
/// The (*rtype).typeOff method is a convenience wrapper for this function.
/// Implemented in the runtime package.
///
///go:noescape
pub fn resolve_type_off(rtype: Arc<Mutex<Option<usize>>>, off: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<usize>>> {
    unimplemented!("Go function declaration has no body");
}


pub fn to_r_type(t: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<rtype>>> {
    Arc::new(Mutex::new(Some(rtype { r#type: t.clone(), ..Default::default() })))
}

pub fn elem(t: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<internal_abi::r#type::Type>>> {
    let mut et = { let __result = t.with_mut(|__recv_value| __recv_value.elem()); __result };
    if { let __nil_result = (*et.lock().unwrap()).is_some(); __nil_result } {
        return et.clone();
    }
    std::panic::panic_any(Box::new(format!("{}{}", "reflect: Elem of invalid type ".to_string(), (*{ let __recv = to_r_type(t.clone()); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).string(); __result }.lock().unwrap().as_ref().unwrap()))) as Box<dyn Any + Send + Sync>);
}

/// add returns p+x.
///
/// The whySafe string is ignored, so that the function still inlines
/// as efficiently as p+x, but all call sites should use the string to
/// record why the addition is safe, which is to say why the addition
/// does not cause x to advance to the very end of p's allocation
/// and therefore point incorrectly at the next block in memory.
pub fn add(p: Arc<Mutex<Option<usize>>>, x: Arc<Mutex<Option<usize>>>, whySafe: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<usize>>> {
    Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
}

/// TypeOf returns the reflection Type that represents the dynamic type of i.
/// If i is a nil interface value, TypeOf returns nil.
pub fn type_of(i: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    to_type({ let __go_ptr = internal_abi::type_of(i.clone()).clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }).clone()
}

/// implements reports whether the type V implements the interface type T.
pub fn implements(T: GoPtr<internal_abi::r#type::Type>, V: GoPtr<internal_abi::r#type::Type>) -> bool {
    let mut t: GoPtr<internal_abi::r#type::InterfaceType> = { let __go_ptr = { let __result = T.with_mut(|__recv_value| __recv_value.interface_type()); __result }.clone(); match __go_ptr { internal_abi::GoPtr::Nil => GoPtr::nil(), internal_abi::GoPtr::Local(__value) => GoPtr::local(__value.clone()), internal_abi::GoPtr::Raw(__addr) => GoPtr::raw(__addr), internal_abi::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), internal_abi::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
    if t.is_nil() {
        return false;
    }
    if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return true;
    }
    let mut rT = to_r_type(T.clone());
    let mut rV = to_r_type(V.clone());

        // The same algorithm applies in both cases, but the
        // method tables for an interface type and a concrete type
        // are different, so the code is duplicated.
        // In both cases the algorithm is a linear scan over the two
        // lists - T's methods and V's methods - simultaneously.
        // Since method tables are stored in a unique sorted order
        // (alphabetical, with no duplicate method names), the scan
        // through V's methods must hit a match for each of T's
        // methods along the way, or else V does not implement T.
        // This lets us run the scan in overall linear time instead of
        // the quadratic time  a naive search would require.
        // See also ../runtime/iface.go.
    if { let __tmp_x = (*{ let __recv_value = V.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(INTERFACE as u8)))); __tmp_x == __tmp_y } {
        let mut v: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(V.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
        let mut i = Arc::new(Mutex::new(Some(0)));
        let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = { let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        let mut tm: Option<GoSliceElemPtr<internal_abi::r#type::Imethod>> = Some(GoSliceElemPtr::new({ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut tmName = (*rT.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*tm.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut vm: Option<GoSliceElemPtr<internal_abi::r#type::Imethod>> = Some(GoSliceElemPtr::new({ let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(), ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut vmName = (*rV.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*vm.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = (*(*vmName.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*tmName.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __left_addr = (*rV.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*vm.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).addr(); let __right_addr = (*rT.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*tm.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).addr(); let __eq = __left_addr == __right_addr; __eq } {
        if !(*tmName.lock().unwrap().as_ref().unwrap()).is_exported() {
        let mut tmPkgPath = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = tmName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*tmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tmPkgPath.lock().unwrap() = __moved_val; };
    }
        let mut vmPkgPath = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = vmName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*vmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *vmPkgPath.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (*tmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*vmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
    }
        {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
            return true;;
        }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return false;
    }

    let mut v = { let __result = V.with_mut(|__recv_value| __recv_value.uncommon()); __result };
    if { let __nil_result = (*v.lock().unwrap()).is_none(); __nil_result } {
        return false;
    }
    let mut i = Arc::new(Mutex::new(Some(0)));
    let mut vmethods = { let __recv = v.clone(); let __recv_ptr: *const internal_abi::r#type::UncommonType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::UncommonType }; let __result = unsafe { &*__recv_ptr }.methods(); __result };
    let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).mcount.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        let mut tm: Option<GoSliceElemPtr<internal_abi::r#type::Imethod>> = Some(GoSliceElemPtr::new({ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(), ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize));
        let mut tmName = (*rT.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*tm.as_ref().unwrap().borrow().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        let mut vm = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = vmethods.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        let mut vmName = (*rV.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*vm.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if { let __tmp_x = (*(*vmName.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*tmName.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && { let __left_addr = (*rV.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*vm.lock().unwrap().as_ref().unwrap()).mtyp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).addr(); let __right_addr = (*rT.lock().unwrap().as_ref().unwrap()).type_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*tm.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).addr(); let __eq = __left_addr == __right_addr; __eq } {
        if !(*tmName.lock().unwrap().as_ref().unwrap()).is_exported() {
        let mut tmPkgPath = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = tmName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*tmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = (*{ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tmPkgPath.lock().unwrap() = __moved_val; };
    }
        let mut vmPkgPath = pkg_path(Arc::new(Mutex::new(Some({ let __arg_holder = vmName.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = (*vmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        { let new_val = { let __recv = (*rV.lock().unwrap().as_ref().unwrap()).name_off(Arc::new(Mutex::new(Some({ let __selector_holder = (*v.lock().unwrap().as_ref().unwrap()).pkg_path.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *vmPkgPath.lock().unwrap() = __moved_val; };
    }
        if { let __tmp_x = (*tmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*vmPkgPath.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
    }
        {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
        if { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __len_target = { let __field = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x >= __tmp_y } {
            return true;;
        }
    }
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    false
}

/// directlyAssignable reports whether a value x of type V can be directly
/// assigned (using memmove) to a value of type T.
/// https://golang.org/doc/go_spec.html#Assignability
/// Ignoring the interface rules (implemented elsewhere)
/// and the ideal constant rules (no ideal constants at run time).
pub fn directly_assignable(T: GoPtr<internal_abi::r#type::Type>, V: GoPtr<internal_abi::r#type::Type>) -> bool {
        // x's type V is identical to T?
    if { let __left_addr = T.addr(); let __right_addr = V.addr(); let __eq = __left_addr == __right_addr; __eq } {
        return true;
    }

        // Otherwise at least one of T and V must not be defined
        // and they must have the same kind.
    if { let __recv_value = T.borrow(); let __result = (*__recv_value.as_ref().unwrap()).has_name(); __result } && { let __recv_value = V.borrow(); let __result = (*__recv_value.as_ref().unwrap()).has_name(); __result } || { let __tmp_x = (*{ let __recv_value = T.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv_value = V.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }

        // x's type T and V must  have identical underlying types.
    have_identical_underlying_type(T.clone(), V.clone(), Arc::new(Mutex::new(Some(true))))
}

pub fn have_identical_type(T: Arc<Mutex<Option<internal_abi::r#type::Type>>>, V: Arc<Mutex<Option<internal_abi::r#type::Type>>>, cmpTags: Arc<Mutex<Option<bool>>>) -> bool {
    if { let __v = (*cmpTags.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return { let __left = T.clone(); let __right = V.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
    }

    if { let __tmp_x = (*{ let __recv = to_r_type(GoPtr::local(T.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = to_r_type(GoPtr::local(V.clone())); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __recv = T.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv = V.clone(); let __recv_ptr: *const internal_abi::r#type::Type = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const internal_abi::r#type::Type }; let __result = unsafe { &*__recv_ptr }.kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }

    have_identical_underlying_type(GoPtr::local(T.clone()), GoPtr::local(V.clone()), Arc::new(Mutex::new(Some(false))))
}

pub fn have_identical_underlying_type(T: GoPtr<internal_abi::r#type::Type>, V: GoPtr<internal_abi::r#type::Type>, cmpTags: Arc<Mutex<Option<bool>>>) -> bool {
    if { let __left_addr = T.addr(); let __right_addr = V.addr(); let __eq = __left_addr == __right_addr; __eq } {
        return true;
    }

    let mut kind = { let __recv_value = T.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result };
    if { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __recv_value = V.borrow(); let __result = (*__recv_value.as_ref().unwrap()).kind(); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }

        // Non-composite types of equal kind have same underlying type
        // (the predefined instance of the type).
    if { let __tmp_x = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::BOOL as u8)))); let __tmp_y = (*kind.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } && { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::COMPLEX128 as u8)))); __tmp_x <= __tmp_y } || { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRING as u8)))); __tmp_x == __tmp_y } || { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::UNSAFE_POINTER as u8)))); __tmp_x == __tmp_y } {
        return true;
    }

        // Composite types.
    { let _switch_val = (*kind.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::ARRAY as u8))))) {
            return { let __tmp_x = { let __result = T.with_mut(|__recv_value| __recv_value.len()); __result }; let __tmp_y = { let __result = V.with_mut(|__recv_value| __recv_value.len()); __result }; __tmp_x == __tmp_y } && have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.elem()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.elem()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::CHAN as u8))))) {
                        // Special case:
                        // x is a bidirectional channel value, T is a channel type,
                        // and x's type V and T have identical element types.
            if { let __tmp_x = (*{ let __result = V.with_mut(|__recv_value| __recv_value.chan_dir()); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::r#type::ChanDir(Arc::new(Mutex::new(Some(internal_abi::BOTH_DIR as i32)))); __tmp_x == __tmp_y } && have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.elem()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.elem()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return true;
    }
                        // Otherwise continue test for identical underlying type.
            return { let __tmp_x = (*{ let __result = V.with_mut(|__recv_value| __recv_value.chan_dir()); __result }.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*{ let __result = T.with_mut(|__recv_value| __recv_value.chan_dir()); __result }.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } && have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.elem()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.elem()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::FUNC as u8))))) {
            let mut t: GoPtr<internal_abi::r#type::FuncType> = { let __ptr = Arc::new(Mutex::new(Some(T.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::FuncType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::FuncType")) } };
            let mut v: GoPtr<internal_abi::r#type::FuncType> = { let __ptr = Arc::new(Mutex::new(Some(V.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::FuncType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::FuncType")) } };
            if { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().out_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = v.borrow(); __ptr_value.as_ref().unwrap().out_count.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } || { let __tmp_x = (*{ let __ptr_value = t.borrow(); __ptr_value.as_ref().unwrap().in_count.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __ptr_value = v.borrow(); __ptr_value.as_ref().unwrap().in_count.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return false;
    }
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).num_in(); __result }; __tmp_x < __tmp_y } {
        if !have_identical_type({ let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).r#in(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }, { let __recv_value = v.borrow(); let __result = (*__recv_value.as_ref().unwrap()).r#in(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return false;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).num_out(); __result }; __tmp_x < __tmp_y } {
        if !have_identical_type({ let __recv_value = t.borrow(); let __result = (*__recv_value.as_ref().unwrap()).out(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }, { let __recv_value = v.borrow(); let __result = (*__recv_value.as_ref().unwrap()).out(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return false;
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            return true;
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(INTERFACE as u8))))) {
            let mut t: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(T.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
            let mut v: GoPtr<internal_abi::r#type::InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(V.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::InterfaceType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::InterfaceType")) } };
            if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.methods.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return true;
    }
                        // Might have the same methods but still
                        // need a run time conversion.
            return false;
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::MAP as u8))))) {
            return have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.key()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.key()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) && have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.elem()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.elem()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(PTR as u8))))) || _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::SLICE as u8))))) {
            return have_identical_type({ let __result = T.with_mut(|__recv_value| __recv_value.elem()); __result }, { let __result = V.with_mut(|__recv_value| __recv_value.elem()); __result }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        } else if _switch_val == (internal_abi::r#type::Kind(Arc::new(Mutex::new(Some(internal_abi::STRUCT as u8))))) {
            let mut t: GoPtr<internal_abi::r#type::StructType> = { let __ptr = Arc::new(Mutex::new(Some(T.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::StructType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::StructType")) } };
            let mut v: GoPtr<internal_abi::r#type::StructType> = { let __ptr = Arc::new(Mutex::new(Some(V.addr()))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<internal_abi::r#type::StructType>(*__ptr_guard.as_ref().unwrap(), "internal_abi::r#type::StructType")) } };
            if { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = { let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x != __tmp_y } {
        return false;
    }
            if { let __tmp_x = (*(*{ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*{ let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.pkg_path.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }
            for i in 0..(({ let __range_holder = { let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        let mut tf: Option<GoSliceElemPtr<internal_abi::r#type::StructField>> = Some(GoSliceElemPtr::new({ let __ptr_value = t.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(), (i) as usize));
        let mut vf: Option<GoSliceElemPtr<internal_abi::r#type::StructField>> = Some(GoSliceElemPtr::new({ let __ptr_value = v.with_mut(|__ptr_value| __ptr_value.fields.clone()); __ptr_value }.clone(), (i) as usize));
        if { let __tmp_x = (*(*(*tf.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*(*vf.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).name().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }
        if !have_identical_type({ let __field = (*tf.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); __field }, { let __field = (*vf.as_ref().unwrap().borrow().as_ref().unwrap()).typ.clone(); __field }, Arc::new(Mutex::new(Some({ let __arg_holder = cmpTags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return false;
    }
        if { let __v = (*cmpTags.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*(*(*tf.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).tag().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*(*(*vf.as_ref().unwrap().borrow().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).tag().lock().unwrap().as_ref().unwrap()).clone(); __tmp_x != __tmp_y } {
        return false;
    }
        if { let __tmp_x = (*{ let __field = (*tf.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*vf.as_ref().unwrap().borrow().as_ref().unwrap()).offset.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
        return false;
    }
        if { let __tmp_x = (*tf.as_ref().unwrap().borrow().as_ref().unwrap()).embedded(); let __tmp_y = (*vf.as_ref().unwrap().borrow().as_ref().unwrap()).embedded(); __tmp_x != __tmp_y } {
        return false;
    }
    }
            return true;
        }
    }

        // Special case:
        // x is a bidirectional channel value, T is a channel type,
        // and x's type V and T have identical element types.
        // Otherwise continue test for identical underlying type.
        // Might have the same methods but still
        // need a run time conversion.
    false
}

/// toType converts from a *rtype to a Type that can be returned
/// to the client of package reflect. In gc, the only concern is that
/// a nil *rtype must be replaced by a nil Type, but in gccgo this
/// function takes care of ensuring that multiple *rtype for the same
/// type are coalesced into a single Type.
pub fn to_type(t: GoPtr<internal_abi::r#type::Type>) -> Arc<Mutex<Option<Box<dyn Type + Send + Sync>>>> {
    if t.is_nil() {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some(Box::new((*to_r_type(t.clone()).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn Type + Send + Sync>)))
}

impl GoValueClone for rtype {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for name {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
