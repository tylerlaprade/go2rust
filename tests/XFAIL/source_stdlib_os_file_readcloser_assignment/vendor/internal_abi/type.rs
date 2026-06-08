use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped};

use crate::{escape::{no_escape}, iface::{EmptyInterface}, map_select_swiss::{mapType}, map_swiss::{SwissMapType}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const INVALID: u8 = 0;
pub const BOOL: u8 = 1;
pub const INT: u8 = 2;
pub const INT8: u8 = 3;
pub const INT16: u8 = 4;
pub const INT32: u8 = 5;
pub const INT64: u8 = 6;
pub const UINT: u8 = 7;
pub const UINT8: u8 = 8;
pub const UINT16: u8 = 9;
pub const UINT32: u8 = 10;
pub const UINT64: u8 = 11;
pub const UINTPTR: u8 = 12;
pub const FLOAT32: u8 = 13;
pub const FLOAT64: u8 = 14;
pub const COMPLEX64: u8 = 15;
pub const COMPLEX128: u8 = 16;
pub const ARRAY: u8 = 17;
pub const CHAN: u8 = 18;
pub const FUNC: u8 = 19;
pub const INTERFACE: u8 = 20;
pub const MAP: u8 = 21;
pub const POINTER: u8 = 22;
pub const SLICE: u8 = 23;
pub const STRING: u8 = 24;
pub const STRUCT: u8 = 25;
pub const UNSAFE_POINTER: u8 = 26;


pub const KIND_DIRECT_IFACE: u8 = ((1 as u8) << (5 as u8));
pub const KIND_MASK: u8 = (((1 as u8) << (5 as u8)) - (1 as u8));


pub const T_FLAG_UNCOMMON: u8 = ((1 as u8) << (0 as u8));
pub const T_FLAG_EXTRA_STAR: u8 = ((1 as u8) << (1 as u8));
pub const T_FLAG_NAMED: u8 = ((1 as u8) << (2 as u8));
pub const T_FLAG_REGULAR_MEMORY: u8 = ((1 as u8) << (3 as u8));
pub const T_FLAG_G_C_MASK_ON_DEMAND: u8 = ((1 as u8) << (4 as u8));


pub const RECV_DIR: i32 = 1 << 0;
pub const SEND_DIR: i32 = 1 << 1;
pub const BOTH_DIR: i32 = RECV_DIR as i32 | SEND_DIR as i32;
pub const INVALID_DIR: i32 = 0;


pub const TRACE_ARGS_LIMIT: i32 = 10;
pub const TRACE_ARGS_MAX_DEPTH: i32 = 5;
pub const TRACE_ARGS_MAX_LEN: i32 = (TRACE_ARGS_MAX_DEPTH * 3 + 2) * TRACE_ARGS_LIMIT + 1;


pub const TRACE_ARGS_END_SEQ: i32 = 0xff;
pub const TRACE_ARGS_START_AGG: i32 = 0xfe;
pub const TRACE_ARGS_END_AGG: i32 = 0xfd;
pub const TRACE_ARGS_DOTDOTDOT: i32 = 0xfc;
pub const TRACE_ARGS_OFFSET_TOO_LARGE: i32 = 0xfb;
pub const TRACE_ARGS_SPECIAL: i32 = 0xf0;


pub const MAX_PTRMASK_BYTES: i32 = 2048;


/// Type is the runtime representation of a Go type.
///
/// Be careful about accessing this type at build time, as the version
/// of this type in the compiler/linker may not have the same layout
/// as the version in the target binary, due to pointer width
/// differences and any experiments. Use cmd/compile/internal/rttype
/// or the functions in compiletype.go to access this type instead.
/// (TODO: this admonition applies to every type in this package.
/// Put it in some shared location?)
#[derive(Clone)]
pub struct Type {
    pub size_: Arc<Mutex<Option<usize>>>,
    pub ptr_bytes: Arc<Mutex<Option<usize>>>,
    pub hash: Arc<Mutex<Option<u32>>>,
    pub t_flag: Arc<Mutex<Option<TFlag>>>,
    pub align_: Arc<Mutex<Option<u8>>>,
    pub field_align_: Arc<Mutex<Option<u8>>>,
    pub kind_: Arc<Mutex<Option<Kind>>>,
    pub equal: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>,
    pub g_c_data: GoPtr<u8>,
    pub str: Arc<Mutex<Option<NameOff>>>,
    pub ptr_to_this: Arc<Mutex<Option<TypeOff>>>,
}

impl Type {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.size_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ptr_bytes.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.t_flag.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.align_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.field_align_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.kind_.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_7_0 = self.equal.clone();
        let __go_clone_8_0 = self.g_c_data.clone();
        let __go_clone_9_0 = { let __guard = self.str.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_10_0 = { let __guard = self.ptr_to_this.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            size_: __go_clone_0_0,
            ptr_bytes: __go_clone_1_0,
            hash: __go_clone_2_0,
            t_flag: __go_clone_3_0,
            align_: __go_clone_4_0,
            field_align_: __go_clone_5_0,
            kind_: __go_clone_6_0,
            equal: __go_clone_7_0,
            g_c_data: __go_clone_8_0,
            str: __go_clone_9_0,
            ptr_to_this: __go_clone_10_0,
        }
    }
}


impl Default for Type {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(TFlag(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_7_0 = Arc::new(Mutex::new(None));
        let __go_default_8_0 = GoPtr::nil();
        let __go_default_9_0 = Arc::new(Mutex::new(Some(NameOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_10_0 = Arc::new(Mutex::new(Some(TypeOff(Arc::new(Mutex::new(Some(0)))))));
        Self {
            size_: __go_default_0_0,
            ptr_bytes: __go_default_1_0,
            hash: __go_default_2_0,
            t_flag: __go_default_3_0,
            align_: __go_default_4_0,
            field_align_: __go_default_5_0,
            kind_: __go_default_6_0,
            equal: __go_default_7_0,
            g_c_data: __go_default_8_0,
            str: __go_default_9_0,
            ptr_to_this: __go_default_10_0,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.size_.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ptr_bytes.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.hash.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.t_flag.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.align_.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.field_align_.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.kind_.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_7 = format!("{}", "<func>");
        let __go_fmt_8 = format!("{}", { if self.g_c_data.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_9 = format!("{}", (*self.str.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_10 = format!("{}", (*self.ptr_to_this.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6, __go_fmt_7, __go_fmt_8, __go_fmt_9, __go_fmt_10)
    }
}

impl GoJsonDecode for Type {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Size_") {
            out.size_ = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("PtrBytes") {
            out.ptr_bytes = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Hash") {
            out.hash = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Align_") {
            out.align_ = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("FieldAlign_") {
            out.field_align_ = <Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("GCData") {
            out.g_c_data = GoPtr::local(<Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?);
        }
        Ok(out)
    }
}


/// A Kind represents the specific kind of type that a Type represents.
/// The zero Kind is not a valid kind.
#[derive(Debug, Clone, Default)]
pub struct Kind(pub Arc<Mutex<Option<u8>>>);

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for Kind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for Kind {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for Kind {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Kind> for u8 {
    fn eq(&self, other: &Kind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Kind> for u8 {
    fn partial_cmp(&self, other: &Kind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Kind {
    type Output = Kind;
    fn add(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for Kind {
    type Output = Kind;
    fn add(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Kind> for u8 {
    type Output = Kind;
    fn add(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Kind {
    type Output = Kind;
    fn sub(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for Kind {
    type Output = Kind;
    fn sub(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Kind> for u8 {
    type Output = Kind;
    fn sub(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Kind {
    type Output = Kind;
    fn mul(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for Kind {
    type Output = Kind;
    fn mul(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Kind> for u8 {
    type Output = Kind;
    fn mul(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Kind {
    type Output = Kind;
    fn div(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for Kind {
    type Output = Kind;
    fn div(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Kind> for u8 {
    type Output = Kind;
    fn div(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Kind {
    type Output = Kind;
    fn rem(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for Kind {
    type Output = Kind;
    fn rem(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Kind> for u8 {
    type Output = Kind;
    fn rem(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Kind {
    type Output = Kind;
    fn bitand(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for Kind {
    type Output = Kind;
    fn bitand(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Kind> for u8 {
    type Output = Kind;
    fn bitand(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Kind {
    type Output = Kind;
    fn bitor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for Kind {
    type Output = Kind;
    fn bitor(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Kind> for u8 {
    type Output = Kind;
    fn bitor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Kind {
    type Output = Kind;
    fn bitxor(self, other: Self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for Kind {
    type Output = Kind;
    fn bitxor(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Kind> for u8 {
    type Output = Kind;
    fn bitxor(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Kind {
    type Output = Kind;
    fn not(self) -> Kind {
        Kind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Kind {
    type Output = Kind;
    fn shl(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Kind {
    type Output = Kind;
    fn shl(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Kind {
    type Output = Kind;
    fn shl(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Kind {
    type Output = Kind;
    fn shl(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Kind {
    type Output = Kind;
    fn shl(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Kind {
    type Output = Kind;
    fn shl(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Kind {
    type Output = Kind;
    fn shl(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Kind {
    type Output = Kind;
    fn shl(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Kind {
    type Output = Kind;
    fn shl(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Kind {
    type Output = Kind;
    fn shl(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Kind {
    type Output = Kind;
    fn shr(self, other: Kind) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Kind {
    type Output = Kind;
    fn shr(self, other: i32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Kind {
    type Output = Kind;
    fn shr(self, other: i8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Kind {
    type Output = Kind;
    fn shr(self, other: i16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Kind {
    type Output = Kind;
    fn shr(self, other: i64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Kind {
    type Output = Kind;
    fn shr(self, other: u32) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Kind {
    type Output = Kind;
    fn shr(self, other: u8) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Kind {
    type Output = Kind;
    fn shr(self, other: u16) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Kind {
    type Output = Kind;
    fn shr(self, other: u64) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Kind {
    type Output = Kind;
    fn shr(self, other: usize) -> Kind {
        Kind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Kind {}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// TFlag is used by a Type to signal what extra type information is
/// available in the memory directly following the Type value.
#[derive(Debug, Clone, Default)]
pub struct TFlag(pub Arc<Mutex<Option<u8>>>);

impl Display for TFlag {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for TFlag {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for TFlag {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for TFlag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for TFlag {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<TFlag> for u8 {
    fn eq(&self, other: &TFlag) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<TFlag> for u8 {
    fn partial_cmp(&self, other: &TFlag) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for TFlag {
    type Output = TFlag;
    fn add(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for TFlag {
    type Output = TFlag;
    fn add(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<TFlag> for u8 {
    type Output = TFlag;
    fn add(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for TFlag {
    type Output = TFlag;
    fn sub(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for TFlag {
    type Output = TFlag;
    fn sub(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<TFlag> for u8 {
    type Output = TFlag;
    fn sub(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for TFlag {
    type Output = TFlag;
    fn mul(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for TFlag {
    type Output = TFlag;
    fn mul(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<TFlag> for u8 {
    type Output = TFlag;
    fn mul(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for TFlag {
    type Output = TFlag;
    fn div(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for TFlag {
    type Output = TFlag;
    fn div(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<TFlag> for u8 {
    type Output = TFlag;
    fn div(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for TFlag {
    type Output = TFlag;
    fn rem(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for TFlag {
    type Output = TFlag;
    fn rem(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<TFlag> for u8 {
    type Output = TFlag;
    fn rem(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for TFlag {
    type Output = TFlag;
    fn bitand(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for TFlag {
    type Output = TFlag;
    fn bitand(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<TFlag> for u8 {
    type Output = TFlag;
    fn bitand(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for TFlag {
    type Output = TFlag;
    fn bitor(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for TFlag {
    type Output = TFlag;
    fn bitor(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<TFlag> for u8 {
    type Output = TFlag;
    fn bitor(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for TFlag {
    type Output = TFlag;
    fn bitxor(self, other: Self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for TFlag {
    type Output = TFlag;
    fn bitxor(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<TFlag> for u8 {
    type Output = TFlag;
    fn bitxor(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for TFlag {
    type Output = TFlag;
    fn not(self) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for TFlag {
    type Output = TFlag;
    fn shl(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for TFlag {
    type Output = TFlag;
    fn shl(self, other: i32) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for TFlag {
    type Output = TFlag;
    fn shl(self, other: i8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for TFlag {
    type Output = TFlag;
    fn shl(self, other: i16) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for TFlag {
    type Output = TFlag;
    fn shl(self, other: i64) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for TFlag {
    type Output = TFlag;
    fn shl(self, other: u32) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for TFlag {
    type Output = TFlag;
    fn shl(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for TFlag {
    type Output = TFlag;
    fn shl(self, other: u16) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for TFlag {
    type Output = TFlag;
    fn shl(self, other: u64) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for TFlag {
    type Output = TFlag;
    fn shl(self, other: usize) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for TFlag {
    type Output = TFlag;
    fn shr(self, other: TFlag) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for TFlag {
    type Output = TFlag;
    fn shr(self, other: i32) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for TFlag {
    type Output = TFlag;
    fn shr(self, other: i8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for TFlag {
    type Output = TFlag;
    fn shr(self, other: i16) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for TFlag {
    type Output = TFlag;
    fn shr(self, other: i64) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for TFlag {
    type Output = TFlag;
    fn shr(self, other: u32) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for TFlag {
    type Output = TFlag;
    fn shr(self, other: u8) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for TFlag {
    type Output = TFlag;
    fn shr(self, other: u16) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for TFlag {
    type Output = TFlag;
    fn shr(self, other: u64) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for TFlag {
    type Output = TFlag;
    fn shr(self, other: usize) -> TFlag {
        TFlag(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for TFlag {}

impl Ord for TFlag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// NameOff is the offset to a name from moduledata.types.  See resolveNameOff in runtime.
#[derive(Debug, Clone, Default)]
pub struct NameOff(pub Arc<Mutex<Option<i32>>>);

impl Display for NameOff {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for NameOff {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for NameOff {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for NameOff {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for NameOff {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<NameOff> for i32 {
    fn eq(&self, other: &NameOff) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<NameOff> for i32 {
    fn partial_cmp(&self, other: &NameOff) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for NameOff {
    type Output = NameOff;
    fn add(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for NameOff {
    type Output = NameOff;
    fn add(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<NameOff> for i32 {
    type Output = NameOff;
    fn add(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for NameOff {
    type Output = NameOff;
    fn sub(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for NameOff {
    type Output = NameOff;
    fn sub(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<NameOff> for i32 {
    type Output = NameOff;
    fn sub(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for NameOff {
    type Output = NameOff;
    fn mul(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for NameOff {
    type Output = NameOff;
    fn mul(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<NameOff> for i32 {
    type Output = NameOff;
    fn mul(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for NameOff {
    type Output = NameOff;
    fn div(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for NameOff {
    type Output = NameOff;
    fn div(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<NameOff> for i32 {
    type Output = NameOff;
    fn div(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for NameOff {
    type Output = NameOff;
    fn neg(self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for NameOff {
    type Output = NameOff;
    fn rem(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for NameOff {
    type Output = NameOff;
    fn rem(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<NameOff> for i32 {
    type Output = NameOff;
    fn rem(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for NameOff {
    type Output = NameOff;
    fn bitand(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for NameOff {
    type Output = NameOff;
    fn bitand(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<NameOff> for i32 {
    type Output = NameOff;
    fn bitand(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for NameOff {
    type Output = NameOff;
    fn bitor(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for NameOff {
    type Output = NameOff;
    fn bitor(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<NameOff> for i32 {
    type Output = NameOff;
    fn bitor(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for NameOff {
    type Output = NameOff;
    fn bitxor(self, other: Self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for NameOff {
    type Output = NameOff;
    fn bitxor(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<NameOff> for i32 {
    type Output = NameOff;
    fn bitxor(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for NameOff {
    type Output = NameOff;
    fn not(self) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for NameOff {
    type Output = NameOff;
    fn shl(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for NameOff {
    type Output = NameOff;
    fn shl(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for NameOff {
    type Output = NameOff;
    fn shl(self, other: i8) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for NameOff {
    type Output = NameOff;
    fn shl(self, other: i16) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for NameOff {
    type Output = NameOff;
    fn shl(self, other: i64) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for NameOff {
    type Output = NameOff;
    fn shl(self, other: u32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for NameOff {
    type Output = NameOff;
    fn shl(self, other: u8) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for NameOff {
    type Output = NameOff;
    fn shl(self, other: u16) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for NameOff {
    type Output = NameOff;
    fn shl(self, other: u64) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for NameOff {
    type Output = NameOff;
    fn shl(self, other: usize) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for NameOff {
    type Output = NameOff;
    fn shr(self, other: NameOff) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for NameOff {
    type Output = NameOff;
    fn shr(self, other: i32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for NameOff {
    type Output = NameOff;
    fn shr(self, other: i8) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for NameOff {
    type Output = NameOff;
    fn shr(self, other: i16) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for NameOff {
    type Output = NameOff;
    fn shr(self, other: i64) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for NameOff {
    type Output = NameOff;
    fn shr(self, other: u32) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for NameOff {
    type Output = NameOff;
    fn shr(self, other: u8) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for NameOff {
    type Output = NameOff;
    fn shr(self, other: u16) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for NameOff {
    type Output = NameOff;
    fn shr(self, other: u64) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for NameOff {
    type Output = NameOff;
    fn shr(self, other: usize) -> NameOff {
        NameOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for NameOff {}

impl Ord for NameOff {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// TypeOff is the offset to a type from moduledata.types.  See resolveTypeOff in runtime.
#[derive(Debug, Clone, Default)]
pub struct TypeOff(pub Arc<Mutex<Option<i32>>>);

impl Display for TypeOff {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for TypeOff {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for TypeOff {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for TypeOff {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for TypeOff {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<TypeOff> for i32 {
    fn eq(&self, other: &TypeOff) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<TypeOff> for i32 {
    fn partial_cmp(&self, other: &TypeOff) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for TypeOff {
    type Output = TypeOff;
    fn add(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for TypeOff {
    type Output = TypeOff;
    fn add(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<TypeOff> for i32 {
    type Output = TypeOff;
    fn add(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for TypeOff {
    type Output = TypeOff;
    fn sub(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for TypeOff {
    type Output = TypeOff;
    fn sub(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<TypeOff> for i32 {
    type Output = TypeOff;
    fn sub(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for TypeOff {
    type Output = TypeOff;
    fn mul(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for TypeOff {
    type Output = TypeOff;
    fn mul(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<TypeOff> for i32 {
    type Output = TypeOff;
    fn mul(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for TypeOff {
    type Output = TypeOff;
    fn div(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for TypeOff {
    type Output = TypeOff;
    fn div(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<TypeOff> for i32 {
    type Output = TypeOff;
    fn div(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for TypeOff {
    type Output = TypeOff;
    fn neg(self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for TypeOff {
    type Output = TypeOff;
    fn rem(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for TypeOff {
    type Output = TypeOff;
    fn rem(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<TypeOff> for i32 {
    type Output = TypeOff;
    fn rem(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for TypeOff {
    type Output = TypeOff;
    fn bitand(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for TypeOff {
    type Output = TypeOff;
    fn bitand(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<TypeOff> for i32 {
    type Output = TypeOff;
    fn bitand(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for TypeOff {
    type Output = TypeOff;
    fn bitor(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for TypeOff {
    type Output = TypeOff;
    fn bitor(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<TypeOff> for i32 {
    type Output = TypeOff;
    fn bitor(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for TypeOff {
    type Output = TypeOff;
    fn bitxor(self, other: Self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for TypeOff {
    type Output = TypeOff;
    fn bitxor(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<TypeOff> for i32 {
    type Output = TypeOff;
    fn bitxor(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for TypeOff {
    type Output = TypeOff;
    fn not(self) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: i8) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: i16) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: i64) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: u32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: u8) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: u16) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: u64) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for TypeOff {
    type Output = TypeOff;
    fn shl(self, other: usize) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: TypeOff) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: i32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: i8) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: i16) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: i64) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: u32) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: u8) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: u16) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: u64) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for TypeOff {
    type Output = TypeOff;
    fn shr(self, other: usize) -> TypeOff {
        TypeOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for TypeOff {}

impl Ord for TypeOff {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// TextOff is an offset from the top of a text section.  See (rtype).textOff in runtime.
#[derive(Debug, Clone, Default)]
pub struct TextOff(pub Arc<Mutex<Option<i32>>>);

impl Display for TextOff {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for TextOff {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for TextOff {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for TextOff {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for TextOff {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<TextOff> for i32 {
    fn eq(&self, other: &TextOff) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<TextOff> for i32 {
    fn partial_cmp(&self, other: &TextOff) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for TextOff {
    type Output = TextOff;
    fn add(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for TextOff {
    type Output = TextOff;
    fn add(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<TextOff> for i32 {
    type Output = TextOff;
    fn add(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for TextOff {
    type Output = TextOff;
    fn sub(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for TextOff {
    type Output = TextOff;
    fn sub(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<TextOff> for i32 {
    type Output = TextOff;
    fn sub(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for TextOff {
    type Output = TextOff;
    fn mul(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for TextOff {
    type Output = TextOff;
    fn mul(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<TextOff> for i32 {
    type Output = TextOff;
    fn mul(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for TextOff {
    type Output = TextOff;
    fn div(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for TextOff {
    type Output = TextOff;
    fn div(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<TextOff> for i32 {
    type Output = TextOff;
    fn div(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for TextOff {
    type Output = TextOff;
    fn neg(self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for TextOff {
    type Output = TextOff;
    fn rem(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for TextOff {
    type Output = TextOff;
    fn rem(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<TextOff> for i32 {
    type Output = TextOff;
    fn rem(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for TextOff {
    type Output = TextOff;
    fn bitand(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for TextOff {
    type Output = TextOff;
    fn bitand(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<TextOff> for i32 {
    type Output = TextOff;
    fn bitand(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for TextOff {
    type Output = TextOff;
    fn bitor(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for TextOff {
    type Output = TextOff;
    fn bitor(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<TextOff> for i32 {
    type Output = TextOff;
    fn bitor(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for TextOff {
    type Output = TextOff;
    fn bitxor(self, other: Self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for TextOff {
    type Output = TextOff;
    fn bitxor(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<TextOff> for i32 {
    type Output = TextOff;
    fn bitxor(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for TextOff {
    type Output = TextOff;
    fn not(self) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for TextOff {
    type Output = TextOff;
    fn shl(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for TextOff {
    type Output = TextOff;
    fn shl(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for TextOff {
    type Output = TextOff;
    fn shl(self, other: i8) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for TextOff {
    type Output = TextOff;
    fn shl(self, other: i16) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for TextOff {
    type Output = TextOff;
    fn shl(self, other: i64) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for TextOff {
    type Output = TextOff;
    fn shl(self, other: u32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for TextOff {
    type Output = TextOff;
    fn shl(self, other: u8) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for TextOff {
    type Output = TextOff;
    fn shl(self, other: u16) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for TextOff {
    type Output = TextOff;
    fn shl(self, other: u64) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for TextOff {
    type Output = TextOff;
    fn shl(self, other: usize) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for TextOff {
    type Output = TextOff;
    fn shr(self, other: TextOff) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for TextOff {
    type Output = TextOff;
    fn shr(self, other: i32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for TextOff {
    type Output = TextOff;
    fn shr(self, other: i8) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for TextOff {
    type Output = TextOff;
    fn shr(self, other: i16) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for TextOff {
    type Output = TextOff;
    fn shr(self, other: i64) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for TextOff {
    type Output = TextOff;
    fn shr(self, other: u32) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for TextOff {
    type Output = TextOff;
    fn shr(self, other: u8) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for TextOff {
    type Output = TextOff;
    fn shr(self, other: u16) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for TextOff {
    type Output = TextOff;
    fn shr(self, other: u64) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for TextOff {
    type Output = TextOff;
    fn shr(self, other: usize) -> TextOff {
        TextOff(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for TextOff {}

impl Ord for TextOff {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// Method on non-interface type
#[derive(Debug, Clone)]
pub struct Method {
    pub name: Arc<Mutex<Option<NameOff>>>,
    pub mtyp: Arc<Mutex<Option<TypeOff>>>,
    pub ifn: Arc<Mutex<Option<TextOff>>>,
    pub tfn: Arc<Mutex<Option<TextOff>>>,
}

impl Method {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.mtyp.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.ifn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tfn.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            mtyp: __go_clone_1_0,
            ifn: __go_clone_2_0,
            tfn: __go_clone_3_0,
        }
    }
}


impl Default for Method {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(NameOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(TypeOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(TextOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(TextOff(Arc::new(Mutex::new(Some(0)))))));
        Self {
            name: __go_default_0_0,
            mtyp: __go_default_1_0,
            ifn: __go_default_2_0,
            tfn: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.mtyp.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.ifn.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.tfn.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for Method {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// UncommonType is present only for defined types or types with methods
/// (if T is a defined type, the uncommonTypes for T and *T have methods).
/// Using a pointer to this struct reduces the overall size required
/// to describe a non-defined type with no methods.
#[derive(Debug, Clone)]
pub struct UncommonType {
    pub pkg_path: Arc<Mutex<Option<NameOff>>>,
    pub mcount: Arc<Mutex<Option<u16>>>,
    pub xcount: Arc<Mutex<Option<u16>>>,
    pub moff: Arc<Mutex<Option<u32>>>,
    pub __blank_4_0: Arc<Mutex<Option<u32>>>,
}

impl UncommonType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pkg_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.mcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.xcount.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.moff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.__blank_4_0.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            pkg_path: __go_clone_0_0,
            mcount: __go_clone_1_0,
            xcount: __go_clone_2_0,
            moff: __go_clone_3_0,
            __blank_4_0: __go_clone_4_0,
        }
    }
}


impl Default for UncommonType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(NameOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            pkg_path: __go_default_0_0,
            mcount: __go_default_1_0,
            xcount: __go_default_2_0,
            moff: __go_default_3_0,
            __blank_4_0: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for UncommonType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pkg_path.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.mcount.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.xcount.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.moff.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.__blank_4_0.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for UncommonType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Mcount") {
            out.mcount = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Xcount") {
            out.xcount = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Moff") {
            out.moff = <Arc<Mutex<Option<u32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// Imethod represents a method on an interface type
#[derive(Debug, Clone)]
pub struct Imethod {
    pub name: Arc<Mutex<Option<NameOff>>>,
    pub typ: Arc<Mutex<Option<TypeOff>>>,
}

impl Imethod {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.typ.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            typ: __go_clone_1_0,
        }
    }
}


impl Default for Imethod {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(NameOff(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(TypeOff(Arc::new(Mutex::new(Some(0)))))));
        Self {
            name: __go_default_0_0,
            typ: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Imethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.typ.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for Imethod {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// ArrayType represents a fixed array type.
#[derive(Clone)]
pub struct ArrayType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub elem: Arc<Mutex<Option<Type>>>,
    pub slice: Arc<Mutex<Option<Type>>>,
    pub len: Arc<Mutex<Option<usize>>>,
}

impl ArrayType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.elem.clone();
        let __go_clone_2_0 = self.slice.clone();
        let __go_clone_3_0 = { let __guard = self.len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            elem: __go_clone_1_0,
            slice: __go_clone_2_0,
            len: __go_clone_3_0,
        }
    }
}


impl Default for ArrayType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#type: __go_default_0_0,
            elem: __go_default_1_0,
            slice: __go_default_2_0,
            len: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", { let __guard = self.slice.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.len.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for ArrayType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Len") {
            out.len = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


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


/// ChanType represents a channel type
#[derive(Clone)]
pub struct ChanType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub elem: Arc<Mutex<Option<Type>>>,
    pub dir: Arc<Mutex<Option<ChanDir>>>,
}

impl ChanType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.elem.clone();
        let __go_clone_2_0 = { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            elem: __go_clone_1_0,
            dir: __go_clone_2_0,
        }
    }
}


impl Default for ChanType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some(0)))))));
        Self {
            r#type: __go_default_0_0,
            elem: __go_default_1_0,
            dir: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for ChanType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.dir.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for ChanType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct structTypeUncommon {
    pub struct_type: Arc<Mutex<Option<StructType>>>,
    pub u: Arc<Mutex<Option<UncommonType>>>,
}

impl structTypeUncommon {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.struct_type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            struct_type: __go_clone_0_0,
            u: __go_clone_1_0,
        }
    }
}


impl Default for structTypeUncommon {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(StructType::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(UncommonType::default())));
        Self {
            struct_type: __go_default_0_0,
            u: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for structTypeUncommon {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.struct_type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for structTypeUncommon {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct InterfaceType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub pkg_path: Arc<Mutex<Option<Name>>>,
    pub methods: Arc<Mutex<Option<Vec<Imethod>>>>,
}

impl InterfaceType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pkg_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.methods.clone();
        Self {
            r#type: __go_clone_0_0,
            pkg_path: __go_clone_1_0,
            methods: __go_clone_2_0,
        }
    }
}


impl Default for InterfaceType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Name::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            r#type: __go_default_0_0,
            pkg_path: __go_default_1_0,
            methods: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for InterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pkg_path.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.methods));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for InterfaceType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct SliceType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub elem: Arc<Mutex<Option<Type>>>,
}

impl SliceType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.elem.clone();
        Self {
            r#type: __go_clone_0_0,
            elem: __go_clone_1_0,
        }
    }
}


impl Default for SliceType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            r#type: __go_default_0_0,
            elem: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for SliceType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// funcType represents a function type.
///
/// A *Type for each in and out parameter is stored in an array that
/// directly follows the funcType (and possibly its uncommonType). So
/// a function type with one method, one input, and one output is:
///
///	struct {
///		funcType
///		uncommonType
///		[2]*rtype    // [0] is in, [1] is out
///	}
#[derive(Clone)]
pub struct FuncType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub in_count: Arc<Mutex<Option<u16>>>,
    pub out_count: Arc<Mutex<Option<u16>>>,
}

impl FuncType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.in_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.out_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            in_count: __go_clone_1_0,
            out_count: __go_clone_2_0,
        }
    }
}


impl Default for FuncType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#type: __go_default_0_0,
            in_count: __go_default_1_0,
            out_count: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for FuncType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.in_count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.out_count.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for FuncType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("InCount") {
            out.in_count = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("OutCount") {
            out.out_count = <Arc<Mutex<Option<u16>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Clone)]
pub struct PtrType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub elem: Arc<Mutex<Option<Type>>>,
}

impl PtrType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.elem.clone();
        Self {
            r#type: __go_clone_0_0,
            elem: __go_clone_1_0,
        }
    }
}


impl Default for PtrType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            r#type: __go_default_0_0,
            elem: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for PtrType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.elem.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for PtrType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone)]
pub struct StructField {
    pub name: Arc<Mutex<Option<Name>>>,
    pub typ: Arc<Mutex<Option<Type>>>,
    pub offset: Arc<Mutex<Option<usize>>>,
}

impl StructField {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.typ.clone();
        let __go_clone_2_0 = { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            typ: __go_clone_1_0,
            offset: __go_clone_2_0,
        }
    }
}


impl Default for StructField {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Name::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            typ: __go_default_1_0,
            offset: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for StructField {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.typ.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.offset.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for StructField {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Offset") {
            out.offset = <Arc<Mutex<Option<usize>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Clone)]
pub struct StructType {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub pkg_path: Arc<Mutex<Option<Name>>>,
    pub fields: Arc<Mutex<Option<Vec<StructField>>>>,
}

impl StructType {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pkg_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.fields.clone();
        Self {
            r#type: __go_clone_0_0,
            pkg_path: __go_clone_1_0,
            fields: __go_clone_2_0,
        }
    }
}


impl Default for StructType {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Type::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Name::default())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            r#type: __go_default_0_0,
            pkg_path: __go_default_1_0,
            fields: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for StructType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pkg_path.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", format_slice(&self.fields));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}

impl GoJsonDecode for StructType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Name {
    pub bytes: GoPtr<u8>,
}

impl Name {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.bytes.clone();
        Self {
            bytes: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.bytes.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for Name {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Bytes") {
            out.bytes = GoPtr::local(<Arc<Mutex<Option<u8>>> as GoJsonDecode>::go_json_decode(field_value)?);
        }
        Ok(out)
    }
}


pub(crate) static kindNames: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<String>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *kindNames.lock().unwrap() = Some(vec![]);
    {
        let mut __go_slice = Vec::<String>::with_capacity(27);
        __go_slice.push("invalid".to_string());
        __go_slice.push("bool".to_string());
        __go_slice.push("int".to_string());
        __go_slice.push("int8".to_string());
        __go_slice.push("int16".to_string());
        __go_slice.push("int32".to_string());
        __go_slice.push("int64".to_string());
        __go_slice.push("uint".to_string());
        __go_slice.push("uint8".to_string());
        __go_slice.push("uint16".to_string());
        __go_slice.push("uint32".to_string());
        __go_slice.push("uint64".to_string());
        __go_slice.push("uintptr".to_string());
        __go_slice.push("float32".to_string());
        __go_slice.push("float64".to_string());
        __go_slice.push("complex64".to_string());
        __go_slice.push("complex128".to_string());
        __go_slice.push("array".to_string());
        __go_slice.push("chan".to_string());
        __go_slice.push("func".to_string());
        __go_slice.push("interface".to_string());
        __go_slice.push("map".to_string());
        __go_slice.push("ptr".to_string());
        __go_slice.push("slice".to_string());
        __go_slice.push("string".to_string());
        __go_slice.push("struct".to_string());
        __go_slice.push("unsafe.Pointer".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *kindNames.lock().unwrap() = Some(__go_slice);
    }
}


pub(crate) fn __go_zero_globals() {
    *kindNames.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    {
        let mut __go_slice = Vec::<String>::with_capacity(27);
        __go_slice.push("invalid".to_string());
        __go_slice.push("bool".to_string());
        __go_slice.push("int".to_string());
        __go_slice.push("int8".to_string());
        __go_slice.push("int16".to_string());
        __go_slice.push("int32".to_string());
        __go_slice.push("int64".to_string());
        __go_slice.push("uint".to_string());
        __go_slice.push("uint8".to_string());
        __go_slice.push("uint16".to_string());
        __go_slice.push("uint32".to_string());
        __go_slice.push("uint64".to_string());
        __go_slice.push("uintptr".to_string());
        __go_slice.push("float32".to_string());
        __go_slice.push("float64".to_string());
        __go_slice.push("complex64".to_string());
        __go_slice.push("complex128".to_string());
        __go_slice.push("array".to_string());
        __go_slice.push("chan".to_string());
        __go_slice.push("func".to_string());
        __go_slice.push("interface".to_string());
        __go_slice.push("map".to_string());
        __go_slice.push("ptr".to_string());
        __go_slice.push("slice".to_string());
        __go_slice.push("string".to_string());
        __go_slice.push("struct".to_string());
        __go_slice.push("unsafe.Pointer".to_string());
        let __go_slice = __go_slice.into_boxed_slice().into_vec();
        *kindNames.lock().unwrap() = Some(__go_slice);
    }
}


impl Kind {
    /// String returns the name of k.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __tmp_x = ((*Arc::new(Mutex::new(Some((*self.0.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*kindNames.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = kindNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.0.lock().unwrap().as_ref().unwrap()) as usize].clone() })));
    }
        Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = kindNames.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })))
    }
}

impl Type {
    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        return Arc::new(Mutex::new(Some(Kind(Arc::new(Mutex::new(Some(((*(*self.kind_.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & KIND_MASK as u8))))))));
    }

    pub fn has_name(&self) -> bool {
        return { let __tmp_x = TFlag(Arc::new(Mutex::new(Some(((*(*self.t_flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & T_FLAG_NAMED as u8))))); let __tmp_y = TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y };
    }

    /// Pointers reports whether t contains pointers.
    pub fn pointers(&self) -> bool {
        return { let __tmp_x = (*self.ptr_bytes.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }

    /// IfaceIndir reports whether t is stored indirectly in an interface value.
    pub fn iface_indir(&self) -> bool {
        return { let __tmp_x = Kind(Arc::new(Mutex::new(Some(((*(*self.kind_.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & KIND_DIRECT_IFACE as u8))))); let __tmp_y = Kind(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y };
    }

    /// isDirectIface reports whether t is stored directly in an interface value.
    pub fn is_direct_iface(&self) -> bool {
        return { let __tmp_x = Kind(Arc::new(Mutex::new(Some(((*(*self.kind_.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & KIND_DIRECT_IFACE as u8))))); let __tmp_y = Kind(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y };
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        if { let __tmp_x = TFlag(Arc::new(Mutex::new(Some(((*(*self.t_flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & T_FLAG_G_C_MASK_ON_DEMAND as u8))))); let __tmp_y = TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
        std::panic::panic_any(Box::new("GcSlice can't handle on-demand gcdata types".to_string()) as Box<dyn Any + Send + Sync>);
    }
        Arc::new(Mutex::new(Some({ let __seq_holder = { let __go_unsafe_result: Arc<Mutex<Option<Vec<u8>>>> = unimplemented!("unsafe.Slice requires unsafe intrinsic support"); __go_unsafe_result }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*begin.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }

    /// Len returns the length of t if t is an array type, otherwise 0
    pub fn len(&self) -> i32 {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(ARRAY as u8)))); __tmp_x == __tmp_y } {
        return (*Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { Arc::new(Mutex::new(None::<ArrayType>)) } else { go_lookup_embedded_owner::<ArrayType>(*__ptr_guard.as_ref().unwrap(), "ArrayType") } }.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap());
    }
        0
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        Arc::new(Mutex::new(Some(self.clone())))
    }

    /// ChanDir returns the direction of t if t is a channel type, otherwise InvalidDir (0).
    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(CHAN as u8)))); __tmp_x == __tmp_y } {
        let mut ch: GoPtr<ChanType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<ChanType>(*__ptr_guard.as_ref().unwrap(), "ChanType")) } };
        return Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some((*(*{ let __ptr_value = ch.with_mut(|__ptr_value| __ptr_value.dir.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))))));
    }
        Arc::new(Mutex::new(Some(ChanDir(Arc::new(Mutex::new(Some(INVALID_DIR as i32)))))))
    }

    /// Uncommon returns a pointer to T's "uncommon" data if there is any, otherwise nil
    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        if { let __tmp_x = TFlag(Arc::new(Mutex::new(Some(((*(*self.t_flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & T_FLAG_UNCOMMON as u8))))); let __tmp_y = TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        { let _switch_val = { let __v = self.kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (Kind(Arc::new(Mutex::new(Some(STRUCT as u8))))) {
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<structTypeUncommon>(unimplemented!("unsafe.Pointer conversion to structTypeUncommon")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(POINTER as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(FUNC as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(SLICE as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(ARRAY as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(CHAN as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(MAP as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(INTERFACE as u8))))) {
            type u = AnonymousStruct1;
            return (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u>(unimplemented!("unsafe.Pointer conversion to u")) } })).lock().unwrap().as_ref().unwrap()).u.clone();
        } else {
            type u = AnonymousStruct1;
            return (*{ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { Arc::new(Mutex::new(None::<u>)) } else { go_lookup_embedded_owner::<u>(*__ptr_guard.as_ref().unwrap(), "u") } }.lock().unwrap().as_ref().unwrap()).u.clone();
        }
    }
    }

    /// Elem returns the element type for t if t is an array, channel, map, pointer, or slice, otherwise nil.
    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        { let _switch_val = { let __v = self.kind(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (Kind(Arc::new(Mutex::new(Some(ARRAY as u8))))) {
            let mut tt: GoPtr<ArrayType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<ArrayType>(*__ptr_guard.as_ref().unwrap(), "ArrayType")) } };
            return { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(CHAN as u8))))) {
            let mut tt: GoPtr<ChanType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<ChanType>(*__ptr_guard.as_ref().unwrap(), "ChanType")) } };
            return { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(MAP as u8))))) {
            let mut tt: GoPtr<crate::map_swiss::SwissMapType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<crate::map_swiss::SwissMapType>(*__ptr_guard.as_ref().unwrap(), "crate::map_swiss::SwissMapType")) } };
            return { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(POINTER as u8))))) {
            let mut tt: GoPtr<PtrType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<PtrType>(*__ptr_guard.as_ref().unwrap(), "PtrType")) } };
            return { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
        } else if _switch_val == (Kind(Arc::new(Mutex::new(Some(SLICE as u8))))) {
            let mut tt: GoPtr<SliceType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<SliceType>(*__ptr_guard.as_ref().unwrap(), "SliceType")) } };
            return { let __ptr_value = tt.with_mut(|__ptr_value| __ptr_value.elem.clone()); __ptr_value }.clone();
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// StructType returns t cast to a *StructType, or nil if its tag does not match.
    pub fn struct_type(&self) -> GoPtr<StructType> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(STRUCT as u8)))); __tmp_x != __tmp_y } {
        return GoPtr::nil();
    }
        { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<StructType>(*__ptr_guard.as_ref().unwrap(), "StructType")) } }
    }

    /// MapType returns t cast to a *OldMapType or *SwissMapType, or nil if its tag does not match.
    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(MAP as u8)))); __tmp_x != __tmp_y } {
        return GoPtr::nil();
    }
        { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<crate::map_swiss::SwissMapType>(*__ptr_guard.as_ref().unwrap(), "crate::map_swiss::SwissMapType")) } }
    }

    /// ArrayType returns t cast to a *ArrayType, or nil if its tag does not match.
    pub fn array_type(&self) -> GoPtr<ArrayType> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(ARRAY as u8)))); __tmp_x != __tmp_y } {
        return GoPtr::nil();
    }
        { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<ArrayType>(*__ptr_guard.as_ref().unwrap(), "ArrayType")) } }
    }

    /// FuncType returns t cast to a *FuncType, or nil if its tag does not match.
    pub fn func_type(&self) -> GoPtr<FuncType> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(FUNC as u8)))); __tmp_x != __tmp_y } {
        return GoPtr::nil();
    }
        { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<FuncType>(*__ptr_guard.as_ref().unwrap(), "FuncType")) } }
    }

    /// InterfaceType returns t cast to a *InterfaceType, or nil if its tag does not match.
    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(INTERFACE as u8)))); __tmp_x != __tmp_y } {
        return GoPtr::nil();
    }
        { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<InterfaceType>(*__ptr_guard.as_ref().unwrap(), "InterfaceType")) } }
    }

    /// Size returns the size of data with type t.
    pub fn size(&self) -> usize {
        return (*self.size_.lock().unwrap().as_ref().unwrap());
    }

    /// Align returns the alignment of data with type t.
    pub fn align(&self) -> i32 {
        (*Arc::new(Mutex::new(Some({ let __selector_holder = self.align_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn field_align(&self) -> i32 {
        (*Arc::new(Mutex::new(Some({ let __selector_holder = self.field_align_.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        let mut ut = self.uncommon();
        if { let __nil_result = (*ut.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(None));
    }
        return { let __recv = ut.clone(); let __recv_ptr: *const UncommonType = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const UncommonType }; let __result = unsafe { &*__recv_ptr }.exported_methods(); __result };
    }

    pub fn num_method(&self) -> i32 {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(INTERFACE as u8)))); __tmp_x == __tmp_y } {
        let mut tt: GoPtr<InterfaceType> = { let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { GoPtr::nil() } else { GoPtr::local(go_lookup_embedded_owner::<InterfaceType>(*__ptr_guard.as_ref().unwrap(), "InterfaceType")) } };
        return { let __recv_value = tt.borrow(); let __result = (*__recv_value.as_ref().unwrap()).num_method(); __result };
    }
        (*self.exported_methods().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        if { let __tmp_x = (*self.kind().lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = Kind(Arc::new(Mutex::new(Some(MAP as u8)))); __tmp_x == __tmp_y } {
        return (*{ let __ptr = Arc::new(Mutex::new(Some(self as *const _ as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { Arc::new(Mutex::new(None::<crate::map_swiss::SwissMapType>)) } else { go_lookup_embedded_owner::<crate::map_swiss::SwissMapType>(*__ptr_guard.as_ref().unwrap(), "crate::map_swiss::SwissMapType") } }.lock().unwrap().as_ref().unwrap()).key.clone();
    }
        return Arc::new(Mutex::new(None));
    }
}

impl UncommonType {
    pub fn methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        if { let __tmp_x = (*self.mcount.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = self.moff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("t.mcount > 0".to_string())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[Method; 65536]>(unimplemented!("unsafe.Pointer conversion to [Method; 65536]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = 0; let __high = (*self.mcount.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = (*self.mcount.clone().lock().unwrap().as_ref().unwrap()) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        if { let __tmp_x = (*self.xcount.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __selector_holder = self.moff.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))), Arc::new(Mutex::new(Some("t.xcount > 0".to_string())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[Method; 65536]>(unimplemented!("unsafe.Pointer conversion to [Method; 65536]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = 0; let __high = (*self.xcount.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = (*self.xcount.clone().lock().unwrap().as_ref().unwrap()) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
    }
}

impl InterfaceType {
    /// NumMethod returns the number of interface methods in the type's method set.
    pub fn num_method(&self) -> i32 {
        ({ let __len_target = { let __field = self.methods.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl FuncType {
    pub fn r#in(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Type>>> {
        { let __seq = { let __seq_holder = self.in_slice().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }
    }

    pub fn num_in(&self) -> i32 {
        (*Arc::new(Mutex::new(Some({ let __selector_holder = self.in_count.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn num_out(&self) -> i32 {
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*self.out_count.lock().unwrap().as_ref().unwrap()); let __tmp_y = (((1 as u16) << (15 as u16)) - (1 as u16)) as u16; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
    }

    pub fn out(&self, i: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Type>>> {
        ({ let __seq = { let __seq_holder = self.out_slice().clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })
    }

    pub fn in_slice(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Type>>>>>>> {
        let mut uadd = Arc::new(Mutex::new(Some(std::mem::size_of::<FuncType>())));
        if { let __tmp_x = TFlag(Arc::new(Mutex::new(Some(((*(*(*self.r#type.lock().unwrap().as_ref().unwrap()).t_flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & T_FLAG_UNCOMMON as u8))))); let __tmp_y = TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some(std::mem::size_of::<UncommonType>()))).lock().unwrap().as_ref().unwrap()) as usize; let mut guard = uadd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = (*self.in_count.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        return Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = uadd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("t.inCount > 0".to_string())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[Arc<Mutex<Option<Type>>>; 65536]>(unimplemented!("unsafe.Pointer conversion to [Arc<Mutex<Option<Type>>>; 65536]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = 0; let __high = (*self.in_count.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = (*self.in_count.clone().lock().unwrap().as_ref().unwrap()) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    }

    pub fn out_slice(&self) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Type>>>>>>> {
        let mut outCount = Arc::new(Mutex::new(Some(self.num_out() as u16)));
        if { let __tmp_x = { let __v = (*outCount.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u16; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
        let mut uadd = Arc::new(Mutex::new(Some(std::mem::size_of::<FuncType>())));
        if { let __tmp_x = TFlag(Arc::new(Mutex::new(Some(((*(*(*self.r#type.lock().unwrap().as_ref().unwrap()).t_flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & T_FLAG_UNCOMMON as u8))))); let __tmp_y = TFlag(Arc::new(Mutex::new(Some(0 as u8)))); __tmp_x != __tmp_y } {
        { let __rhs = (*Arc::new(Mutex::new(Some(std::mem::size_of::<UncommonType>()))).lock().unwrap().as_ref().unwrap()) as usize; let mut guard = uadd.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return Arc::new(Mutex::new(Some({ let mut __seq = { let __seq_holder = Arc::new(Mutex::new({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self as *const _ as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = uadd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("outCount > 0".to_string())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[Arc<Mutex<Option<Type>>>; 131072]>(unimplemented!("unsafe.Pointer conversion to [Arc<Mutex<Option<Type>>>; 131072]")) } })).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; let __low = (*self.in_count.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = ({ let __tmp_x = (*self.in_count.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*outCount.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = ({ let __tmp_x = (*self.in_count.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*outCount.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
    }

    pub fn is_variadic(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = (*self.out_count.lock().unwrap().as_ref().unwrap()); let __tmp_y = ((1 as u16) << (15 as u16)) as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y };
    }

    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl StructField {
    pub fn embedded(&self) -> bool {
        (*self.name.lock().unwrap().as_ref().unwrap()).is_embedded()
    }
}

impl Name {
    /// DataChecked does pointer arithmetic on n's Bytes, and that arithmetic is asserted to
    /// be safe for the reason in whySafe (which can appear in a backtrace, etc.)
    pub fn data_checked(&self, off: Arc<Mutex<Option<i32>>>, whySafe: Arc<Mutex<Option<String>>>) -> GoPtr<u8> {
        GoPtr::raw({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self.bytes.addr()))), Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some({ let __arg_holder = whySafe.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// Data does pointer arithmetic on n's Bytes, and that arithmetic is asserted to
    /// be safe because the runtime made the call (other packages use DataChecked)
    pub fn data(&self, off: Arc<Mutex<Option<i32>>>) -> GoPtr<u8> {
        GoPtr::raw({ let __ptr = add_checked(Arc::new(Mutex::new(Some(self.bytes.addr()))), Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize))), Arc::new(Mutex::new(Some("the runtime doesn't need to give you a reason".to_string())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) })
    }

    /// IsExported returns "is n exported?"
    pub fn is_exported(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __ptr_value = self.bytes.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ((1 as u8) << (0 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// HasTag returns true iff there is tag data following this name
    pub fn has_tag(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __ptr_value = self.bytes.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ((1 as u8) << (1 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// IsEmbedded returns true iff n is embedded (an anonymous field).
    pub fn is_embedded(&self) -> bool {
        return { let __tmp_x = { let __tmp_x = ({ let __ptr_value = self.bytes.borrow(); __ptr_value.as_ref().unwrap().clone() }); let __tmp_y = ((1 as u8) << (3 as u8)) as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }

    /// ReadVarint parses a varint as encoded by encoding/binary.
    /// It returns the number of encoded bytes and the encoded value.
    pub fn read_varint(&self, off: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
        let mut v = Arc::new(Mutex::new(Some(0)));
        let mut i = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut x = Arc::new(Mutex::new(Some({ let __ptr_handle = self.data_checked(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some("read varint".to_string())))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() })));
        { let __rhs = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7f as u8; __tmp_x & __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ({ let __tmp_x = 7; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }); __tmp_x << __tmp_y }; let mut guard = v.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x80 as u8; __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        return ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }, { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v });
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }

    /// IsBlank indicates whether n is "_".
    pub fn is_blank(&self) -> bool {
        if { let __ptr_field = self.bytes.clone(); __ptr_field.is_nil() } {
        return false;
    }
        let (_, mut l) = self.read_varint(Arc::new(Mutex::new(Some(1))));
        return { let __tmp_x = l; let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __ptr_handle = self.data(Arc::new(Mutex::new(Some(2)))); let __ptr_value = __ptr_handle.borrow(); __ptr_value.as_ref().unwrap().clone() }; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y };
    }

    /// Name returns the tag string for n, or empty if there is none.
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        if { let __ptr_field = self.bytes.clone(); __ptr_field.is_nil() } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let (mut i, mut l) = self.read_varint(Arc::new(Mutex::new(Some(1))));
        { let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }
    }

    /// Tag returns the tag string for n, or empty if there is none.
    pub fn tag(&self) -> Arc<Mutex<Option<String>>> {
        if !self.has_tag() {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
        let (mut i, mut l) = self.read_varint(Arc::new(Mutex::new(Some(1))));
        let (mut i2, mut l2) = self.read_varint(Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = 1; let __tmp_y = i; __tmp_x + __tmp_y }; let __tmp_y = l; __tmp_x + __tmp_y }))));
        { let __go_unsafe_result: Arc<Mutex<Option<String>>> = unimplemented!("unsafe.String requires unsafe intrinsic support"); __go_unsafe_result }
    }
}

impl ArrayType {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl ChanType {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl PtrType {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl SliceType {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl StructType {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.r#type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

impl structTypeUncommon {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.struct_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}

/// TypeOf returns the abi.Type of some value.
pub fn type_of(a: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> GoPtr<Type> {
    let __guard = a.lock().unwrap();
    let __value = match __guard.as_ref() { Some(__value) => __value.as_ref(), None => return GoPtr::nil() };
    let mut __typ = Type::default();
    let __go_any_metadata = go_any_type_metadata(__value);
    let __kind: u8 = if let Some(__go_meta) = __go_any_metadata { match __go_meta.kind { "struct" => STRUCT, "pointer" => POINTER, "slice" => SLICE, "map" => MAP, "interface" => INTERFACE, "chan" => CHAN, "func" => FUNC, "array" => ARRAY, "basic" => INVALID, _ => panic!("internal/abi.TypeOf unsupported Go metadata kind: {}", __go_meta.kind) } } else if <dyn std::any::Any>::is::<bool>(__value) { BOOL } else if <dyn std::any::Any>::is::<i32>(__value) { INT } else if <dyn std::any::Any>::is::<isize>(__value) { INT } else if <dyn std::any::Any>::is::<i8>(__value) { INT8 } else if <dyn std::any::Any>::is::<i16>(__value) { INT16 } else if <dyn std::any::Any>::is::<i64>(__value) { INT64 } else if <dyn std::any::Any>::is::<u8>(__value) { UINT8 } else if <dyn std::any::Any>::is::<u16>(__value) { UINT16 } else if <dyn std::any::Any>::is::<u32>(__value) { UINT32 } else if <dyn std::any::Any>::is::<u64>(__value) { UINT64 } else if <dyn std::any::Any>::is::<usize>(__value) { UINTPTR } else if <dyn std::any::Any>::is::<f32>(__value) { FLOAT32 } else if <dyn std::any::Any>::is::<f64>(__value) { FLOAT64 } else if <dyn std::any::Any>::is::<String>(__value) { STRING } else if <dyn std::any::Any>::is::<&'static str>(__value) { STRING } else if <dyn std::any::Any>::is::<char>(__value) { INT32 } else if <dyn std::any::Any>::is::<Vec<u8>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<i32>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<i64>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<f64>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<String>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<bool>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Arc<Mutex<Option<Vec<u8>>>>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Vec<Box<dyn Any + Send + Sync>>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>>(__value) { SLICE } else if <dyn std::any::Any>::is::<Box<dyn Any + Send + Sync>>(__value) { INTERFACE } else if <dyn std::any::Any>::is::<Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>(__value) { INTERFACE } else { panic!("internal/abi.TypeOf unsupported Rust Any payload: {}", std::any::type_name_of_val(__value)) };
    *__typ.kind_.lock().unwrap() = Some(Kind(Arc::new(Mutex::new(Some(__kind)))));
    *__typ.size_.lock().unwrap() = Some(std::mem::size_of_val(__value));
    if let Some(__go_meta) = __go_any_metadata { if __go_meta.comparable { *__typ.equal.lock().unwrap() = Some(Box::new(|_, _| false) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>); } }
    if let Some(__go_meta) = __go_any_metadata { if __go_meta.kind == "pointer" { let mut __ptr_type = PtrType::default(); *__ptr_type.r#type.lock().unwrap() = Some(__typ); if let Some(__go_elem_kind) = __go_meta.elem_kind { let mut __elem_type = Type::default(); let __elem_kind: u8 = match __go_elem_kind { "struct" => STRUCT, "pointer" => POINTER, "slice" => SLICE, "map" => MAP, "interface" => INTERFACE, "chan" => CHAN, "func" => FUNC, "array" => ARRAY, "basic" => INVALID, _ => panic!("internal/abi.TypeOf unsupported Go metadata kind: {}", __go_elem_kind) }; *__elem_type.kind_.lock().unwrap() = Some(Kind(Arc::new(Mutex::new(Some(__elem_kind))))); if __go_meta.elem_comparable { *__elem_type.equal.lock().unwrap() = Some(Box::new(|_, _| false) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>); } *__ptr_type.elem.lock().unwrap() = Some(__elem_type); } let __owner = Arc::new(Mutex::new(Some(__ptr_type))); let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().r#type.clone() }; let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); return GoPtr::local(__embedded); } }
    GoPtr::local(Arc::new(Mutex::new(Some(__typ))))
}


/// addChecked returns p+x.
///
/// The whySafe string is ignored, so that the function still inlines
/// as efficiently as p+x, but all call sites should use the string to
/// record why the addition is safe, which is to say why the addition
/// does not cause x to advance to the very end of p's allocation
/// and therefore point incorrectly at the next block in memory.
pub fn add_checked(p: Arc<Mutex<Option<usize>>>, x: Arc<Mutex<Option<usize>>>, whySafe: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<usize>>> {
    Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap().as_ref().unwrap()) as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })))
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub ptr_type: Arc<Mutex<Option<PtrType>>>,
    pub u: Arc<Mutex<Option<UncommonType>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.ptr_type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.u.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            ptr_type: __go_clone_0_0,
            u: __go_clone_1_0,
        }
    }
}

impl AnonymousStruct1 {
    pub fn align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.align()
    }

    pub fn array_type(&self) -> GoPtr<ArrayType> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.array_type()
    }

    pub fn chan_dir(&self) -> Arc<Mutex<Option<ChanDir>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.chan_dir()
    }

    pub fn common(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.common()
    }

    pub fn elem(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.elem()
    }

    pub fn exported_methods(&self) -> Arc<Mutex<Option<Vec<Method>>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.exported_methods()
    }

    pub fn field_align(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.field_align()
    }

    pub fn func_type(&self) -> GoPtr<FuncType> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.func_type()
    }

    pub fn gc_slice(&self, begin: Arc<Mutex<Option<usize>>>, end: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.gc_slice(begin, end)
    }

    pub fn has_name(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.has_name()
    }

    pub fn iface_indir(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.iface_indir()
    }

    pub fn interface_type(&self) -> GoPtr<InterfaceType> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.interface_type()
    }

    pub fn is_direct_iface(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.is_direct_iface()
    }

    pub fn key(&self) -> Arc<Mutex<Option<Type>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.key()
    }

    pub fn kind(&self) -> Arc<Mutex<Option<Kind>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.kind()
    }

    pub fn len(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn map_type(&self) -> GoPtr<crate::map_swiss::SwissMapType> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.map_type()
    }

    pub fn num_method(&self) -> i32 {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.num_method()
    }

    pub fn pointers(&self) -> bool {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.pointers()
    }

    pub fn size(&self) -> usize {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.size()
    }

    pub fn struct_type(&self) -> GoPtr<StructType> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.struct_type()
    }

    pub fn uncommon(&self) -> Arc<Mutex<Option<UncommonType>>> {
        // Forward to embedded type's method
        let embedded = self.ptr_type.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.uncommon()
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(PtrType::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(UncommonType::default())));
        Self {
            ptr_type: __go_default_0_0,
            u: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.ptr_type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.u.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Type {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Method {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for UncommonType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Imethod {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ArrayType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ChanType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for structTypeUncommon {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for InterfaceType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for SliceType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for FuncType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for PtrType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for StructField {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for StructType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Name {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
