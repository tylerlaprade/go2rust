use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use crate::{
    alg::{read_unaligned64},
    asan0::{ASANENABLED, asanwrite},
    cgo::{iscgo},
    cgocall::{cgoCallers, cgocall},
    mfinal::{FING_RUNNING_FINALIZER, fingStatus},
    msan0::{MSANENABLED, msanwrite},
    panic::{THROW_TYPE_RUNTIME, panicking, throw, throwType},
    print::{hex, hexdump_words},
    proc::{for_each_g_race, readgstatus},
    r#extern::{G_O_A_R_C_H, G_O_O_S},
    runtime1::{gotraceback},
    runtime2::{FRAMEPOINTER_ENABLED, WAIT_REASON_ZERO, __GCOPYSTACK, __GDEAD, __GIDLE, __GPREEMPTED, __GRUNNABLE, __GRUNNING, __GSCAN, __GSYSCALL, __GWAITING, ancestorInfo, g, goarm, gobuf, guintptr, m, muintptr, stack, waitReason},
    stkframe::{stkframe},
    string::{gostringnocopy},
    stubs::{add, align_up, asmcgocall, getg, noescape, systemstack},
    symtab::{findfunc, funcInfo, funcdata, funcline, funcname, funcspdelta, pcdatavalue, srcFunc},
    symtabinl::{inlineFrame, inlineUnwinder, new_inline_unwinder},
    synctest::{synctestGroup},
    time_nofake::{nanotime},
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const USES_L_R: bool = internal_runtime_sys::MIN_FRAME_SIZE > 0;


pub(crate) const TRACEBACK_INNER_FRAMES: i32 = 50;
pub(crate) const TRACEBACK_OUTER_FRAMES: i32 = 50;


pub(crate) const UNWIND_PRINT_ERRORS: u8 = 1 << 0;
pub(crate) const UNWIND_SILENT_ERRORS: u8 = 1 << 1;
pub(crate) const UNWIND_TRAP: u8 = 1 << 2;
pub(crate) const UNWIND_JUMP_STACK: u8 = 1 << 3;


/// unwindFlags control the behavior of various unwinders.
#[derive(Debug, Clone, Default)]
pub struct unwindFlags(pub Arc<Mutex<Option<u8>>>);

impl Display for unwindFlags {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for unwindFlags {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u8> for unwindFlags {
    fn eq(&self, other: &u8) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for unwindFlags {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u8> for unwindFlags {
    fn partial_cmp(&self, other: &u8) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<unwindFlags> for u8 {
    fn eq(&self, other: &unwindFlags) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<unwindFlags> for u8 {
    fn partial_cmp(&self, other: &unwindFlags) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for unwindFlags {
    type Output = unwindFlags;
    fn add(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u8> for unwindFlags {
    type Output = unwindFlags;
    fn add(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn add(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for unwindFlags {
    type Output = unwindFlags;
    fn sub(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u8> for unwindFlags {
    type Output = unwindFlags;
    fn sub(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn sub(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for unwindFlags {
    type Output = unwindFlags;
    fn mul(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u8> for unwindFlags {
    type Output = unwindFlags;
    fn mul(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn mul(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for unwindFlags {
    type Output = unwindFlags;
    fn div(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u8> for unwindFlags {
    type Output = unwindFlags;
    fn div(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn div(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for unwindFlags {
    type Output = unwindFlags;
    fn rem(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u8> for unwindFlags {
    type Output = unwindFlags;
    fn rem(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn rem(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for unwindFlags {
    type Output = unwindFlags;
    fn bitand(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u8> for unwindFlags {
    type Output = unwindFlags;
    fn bitand(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn bitand(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for unwindFlags {
    type Output = unwindFlags;
    fn bitor(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u8> for unwindFlags {
    type Output = unwindFlags;
    fn bitor(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn bitor(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for unwindFlags {
    type Output = unwindFlags;
    fn bitxor(self, other: Self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u8> for unwindFlags {
    type Output = unwindFlags;
    fn bitxor(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<unwindFlags> for u8 {
    type Output = unwindFlags;
    fn bitxor(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for unwindFlags {
    type Output = unwindFlags;
    fn not(self) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: i32) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: i8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: i16) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: i64) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: u32) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: u16) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: u64) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for unwindFlags {
    type Output = unwindFlags;
    fn shl(self, other: usize) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: unwindFlags) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: i32) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: i8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: i16) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: i64) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: u32) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: u8) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: u16) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: u64) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for unwindFlags {
    type Output = unwindFlags;
    fn shr(self, other: usize) -> unwindFlags {
        unwindFlags(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for unwindFlags {}

impl Ord for unwindFlags {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// An unwinder iterates the physical stack frames of a Go sack.
///
/// Typical use of an unwinder looks like:
///
///	var u unwinder
///	for u.init(gp, 0); u.valid(); u.next() {
///		// ... use frame info in u ...
///	}
///
/// Implementation note: This is carefully structured to be pointer-free because
/// tracebacks happen in places that disallow write barriers (e.g., signals).
/// Even if this is stack-allocated, its pointer-receiver methods don't know that
/// their receiver is on the stack, so they still emit write barriers. Here we
/// address that by carefully avoiding any pointers in this type. Another
/// approach would be to split this into a mutable part that's passed by pointer
/// but contains no pointers itself and an immutable part that's passed and
/// returned by value and can contain pointers. We could potentially hide that
/// we're doing that in trivial methods that are inlined into the caller that has
/// the stack allocation, but that's fragile.
#[derive(Clone)]
pub struct unwinder {
    pub frame: Arc<Mutex<Option<stkframe>>>,
    pub g: Arc<Mutex<Option<guintptr>>>,
    pub cgo_ctxt: Arc<Mutex<Option<i32>>>,
    pub callee_func_i_d: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>,
    pub flags: Arc<Mutex<Option<unwindFlags>>>,
}

impl unwinder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.frame.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.g.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.cgo_ctxt.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.callee_func_i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_4_0 = { let __guard = self.flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            frame: __go_clone_0_0,
            g: __go_clone_1_0,
            cgo_ctxt: __go_clone_2_0,
            callee_func_i_d: __go_clone_3_0,
            flags: __go_clone_4_0,
        }
    }
}


impl Default for unwinder {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(stkframe::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0)))))));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(0)))))));
        Self {
            frame: __go_default_0_0,
            g: __go_default_1_0,
            cgo_ctxt: __go_default_2_0,
            callee_func_i_d: __go_default_3_0,
            flags: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for unwinder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.frame.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.g.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.cgo_ctxt.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", (*self.callee_func_i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_4 = format!("{}", (*self.flags.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4)
    }
}

impl GoJsonDecode for unwinder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// cgoTracebackArg is the type passed to cgoTraceback.
#[derive(Debug, Clone)]
pub struct cgoTracebackArg {
    pub context: Arc<Mutex<Option<usize>>>,
    pub sig_context: Arc<Mutex<Option<usize>>>,
    pub buf: Arc<Mutex<Option<usize>>>,
    pub max: Arc<Mutex<Option<usize>>>,
}

impl cgoTracebackArg {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.context.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.sig_context.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.buf.clone();
        let __go_clone_3_0 = { let __guard = self.max.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            context: __go_clone_0_0,
            sig_context: __go_clone_1_0,
            buf: __go_clone_2_0,
            max: __go_clone_3_0,
        }
    }
}


impl Default for cgoTracebackArg {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            context: __go_default_0_0,
            sig_context: __go_default_1_0,
            buf: __go_default_2_0,
            max: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for cgoTracebackArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.context.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.sig_context.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.buf.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", (*self.max.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}

impl GoJsonDecode for cgoTracebackArg {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// cgoSymbolizerArg is the type passed to cgoSymbolizer.
#[derive(Debug, Clone)]
pub struct cgoSymbolizerArg {
    pub pc: Arc<Mutex<Option<usize>>>,
    pub file: Arc<Mutex<Option<u8>>>,
    pub lineno: Arc<Mutex<Option<usize>>>,
    pub func_name: Arc<Mutex<Option<u8>>>,
    pub entry: Arc<Mutex<Option<usize>>>,
    pub more: Arc<Mutex<Option<usize>>>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl cgoSymbolizerArg {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.pc.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.file.clone();
        let __go_clone_2_0 = { let __guard = self.lineno.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = self.func_name.clone();
        let __go_clone_4_0 = { let __guard = self.entry.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_5_0 = { let __guard = self.more.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_6_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            pc: __go_clone_0_0,
            file: __go_clone_1_0,
            lineno: __go_clone_2_0,
            func_name: __go_clone_3_0,
            entry: __go_clone_4_0,
            more: __go_clone_5_0,
            data: __go_clone_6_0,
        }
    }
}


impl Default for cgoSymbolizerArg {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_5_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_6_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            pc: __go_default_0_0,
            file: __go_default_1_0,
            lineno: __go_default_2_0,
            func_name: __go_default_3_0,
            entry: __go_default_4_0,
            more: __go_default_5_0,
            data: __go_default_6_0,
        }
    }
}

impl std::fmt::Display for cgoSymbolizerArg {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.pc.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.lineno.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", { let __guard = self.func_name.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_4 = format!("{}", (*self.entry.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_5 = format!("{}", (*self.more.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_6 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {} {} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3, __go_fmt_4, __go_fmt_5, __go_fmt_6)
    }
}

impl GoJsonDecode for cgoSymbolizerArg {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static gStatusStrings: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[String; 10]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoTraceback: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoContext: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static cgoSymbolizer: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<usize>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *gStatusStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *cgoTraceback.lock().unwrap() = Some(0);
    *cgoContext.lock().unwrap() = Some(0);
    *cgoSymbolizer.lock().unwrap() = Some(0);
    {
        let mut __go_array = Vec::<String>::with_capacity(10);
        __go_array.push("idle".to_string());
        __go_array.push("runnable".to_string());
        __go_array.push("running".to_string());
        __go_array.push("syscall".to_string());
        __go_array.push("waiting".to_string());
        __go_array.push(String::new());
        __go_array.push("dead".to_string());
        __go_array.push(String::new());
        __go_array.push("copystack".to_string());
        __go_array.push("preempted".to_string());
        let __go_array: [String; 10] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *gStatusStrings.lock().unwrap() = Some(__go_array);
    }
}


pub(crate) fn __go_zero_globals() {
    *gStatusStrings.lock().unwrap() = Some(std::array::from_fn(|_| String::new()));
    *cgoTraceback.lock().unwrap() = Some(0);
    *cgoContext.lock().unwrap() = Some(0);
    *cgoSymbolizer.lock().unwrap() = Some(0);
}


pub(crate) fn __go_init_order_82() {
    {
        let mut __go_array = Vec::<String>::with_capacity(10);
        __go_array.push("idle".to_string());
        __go_array.push("runnable".to_string());
        __go_array.push("running".to_string());
        __go_array.push("syscall".to_string());
        __go_array.push("waiting".to_string());
        __go_array.push(String::new());
        __go_array.push("dead".to_string());
        __go_array.push(String::new());
        __go_array.push("copystack".to_string());
        __go_array.push("preempted".to_string());
        let __go_array: [String; 10] = match __go_array.try_into() { Ok(__go_array) => __go_array, Err(_) => panic!("go2rust array literal length mismatch") };
        *gStatusStrings.lock().unwrap() = Some(__go_array);
    }
}


impl unwinder {
    /// init initializes u to start unwinding gp's stack and positions the
    /// iterator on gp's innermost frame. gp must not be the current G.
    ///
    /// A single unwinder can be reused for multiple unwinds.
    pub fn init(&mut self, gp: GoPtr<crate::runtime2::g>, flags: Arc<Mutex<Option<unwindFlags>>>) {
                // Implementation note: This starts the iterator on the first frame and we
                // provide a "valid" method. Alternatively, this could start in a "before
                // the first frame" state and "next" could return whether it was able to
                // move to the next frame, but that's both more awkward to use in a "for"
                // loop and is harder to implement because we have to do things differently
                // for the first frame.
        self.init_at(
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            gp.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        );
    }

    pub fn init_at(&mut self, mut pc0: Arc<Mutex<Option<usize>>>, mut sp0: Arc<Mutex<Option<usize>>>, mut lr0: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>, flags: Arc<Mutex<Option<unwindFlags>>>) {
                // Don't call this "g"; it's too easy get "g" and "gp" confused.
        {
        let mut ourg = getg();;
        if { let __left_addr = { let __ptr = GoPtr::local(ourg.clone()); __ptr.addr() }; let __right_addr = gp.addr(); let __eq = __left_addr == __right_addr; __eq } && { let __left_addr = { let __ptr = GoPtr::local(ourg.clone()); __ptr.addr() }; let __right_addr = (*(*ourg.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq } {
            throw(Arc::new(Mutex::new(Some("cannot trace user goroutine on its own stack".to_string()))));;
        }
    }
                // The starting sp has been passed in as a uintptr, and the caller may
                // have other uintptr-typed stack references as well.
                // If during one of the calls that got us here or during one of the
                // callbacks below the stack must be grown, all these uintptr references
                // to the stack will not be updated, and traceback will continue
                // to inspect the old stack memory, which may no longer be valid.
                // Even if all the variables were updated correctly, it is not clear that
                // we want to expose a traceback that begins on one stack and ends
                // on another stack. That could confuse callers quite a bit.
                // Instead, we require that initAt and any other function that
                // accepts an sp for the current goroutine (typically obtained by
                // calling GetCallerSP) must not run on that goroutine's stack but
                // instead on the g0 stack.
        if { let __tmp_x = { let __v = (*pc0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*sp0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(0 as usize) as usize; __tmp_x == __tmp_y } {
        if { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
        { let new_val = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.syscallpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *pc0.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.syscallsp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp0.lock().unwrap() = Some(new_val); };
        if USES_L_R {
        { let new_val = 0 as usize; *lr0.lock().unwrap() = Some(new_val); };
    }
    } else {
        { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *pc0.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp0.lock().unwrap() = Some(new_val); };
        if USES_L_R {
        { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *lr0.lock().unwrap() = Some(new_val); };
    }
    }
    }
        let mut frame: Arc<Mutex<Option<stkframe>>> = Arc::new(Mutex::new(Some(Default::default())));
        { let new_val = pc0.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        { let new_val = sp0.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
        if USES_L_R {
        { let new_val = lr0.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }
                // If the PC is zero, it's likely a nil function call.
                // Start in the caller's frame.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        if USES_L_R {
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        { let __target = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
                // internal/runtime/atomic functions call into kernel helpers on
                // arm < 7. See internal/runtime/atomic/sys_linux_arm.s.
                //
                // Start in the caller's frame.
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __tmp_x = "arm64".to_string(); let __tmp_y = "arm".to_string(); __tmp_x == __tmp_y };
                    if __go_cond_2 {
                        let __go_cond_3 = { let __tmp_x = (*goarm.lock().unwrap().as_ref().unwrap()); let __tmp_y = 7 as u8; __tmp_x < __tmp_y };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = { let __tmp_x = "darwin".to_string(); let __tmp_y = "linux".to_string(); __tmp_x == __tmp_y };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_5 = { let __tmp_x = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0xffff0000 as usize; __tmp_x & __tmp_y }; let __tmp_y = 0xffff0000 as usize; __tmp_x == __tmp_y };
                __go_cond_5
            } else {
                false
            }
        } {
                // Note that the calls are simple BL without pushing the return
                // address, so we use LR directly.
                //
                // The kernel helpers are frameless leaf functions, so SP and
                // LR are not touched.
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }
                // Note that the calls are simple BL without pushing the return
                // address, so we use LR directly.
                //
                // The kernel helpers are frameless leaf functions, so SP and
                // LR are not touched.
        let mut f = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        if {
            let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & UNWIND_SILENT_ERRORS as u8)))));
            let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: g ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " gp=".to_string());
            let __go_print_arg_3 = format!("{}", format!("0x{:x}", gp.addr()));
            let __go_print_arg_4 = format!("{}", ": unknown pc ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        traceback_hexdump(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), frame.clone(), Arc::new(Mutex::new(Some(0 as usize))));
    }
        if {
            let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*{ let __v = (*flags.lock().unwrap().as_ref().unwrap()).clone(); __v }.0.lock().unwrap().as_ref().unwrap()) & ((UNWIND_PRINT_ERRORS as u8 | UNWIND_SILENT_ERRORS as u8)))))));
            let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        } {
        throw(Arc::new(Mutex::new(Some("unknown pc".to_string()))));
    }
        { let new_val = unwinder { frame: Arc::new(Mutex::new(Some(Default::default()))), g: Arc::new(Mutex::new(Some(crate::runtime2::guintptr(Arc::new(Mutex::new(Some(0))))))), cgo_ctxt: Arc::new(Mutex::new(Some(0))), callee_func_i_d: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(0))))))), flags: Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(0))))))) }; *self = new_val; };
        return;
    }
        { let new_val = f.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = Some(new_val); };
                // Populate the unwinder.
        { let new_val = unwinder {
            frame: Arc::new(Mutex::new(Some({ let __arg_holder = frame.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            g: { let __result = gp.with_mut(|__recv_value| __recv_value.guintptr()); __result },
            cgo_ctxt: Arc::new(Mutex::new(Some({ let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.cgo_ctxt.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }))),
            callee_func_i_d: Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8))))))),
            flags: Arc::new(Mutex::new(Some({ let __arg_holder = flags.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            ..Default::default()
        }; *self = new_val; };
        let mut isSyscall = Arc::new(Mutex::new(Some({
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*pc0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y };
                    if __go_cond_2 {
                        let __go_cond_3 = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*sp0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = { let __tmp_x = { let __v = (*pc0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallpc.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_5 = { let __tmp_x = { let __v = (*sp0.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y };
                __go_cond_5
            } else {
                false
            }
        })));
        self.resolve_internal(Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some({ let __arg_holder = isSyscall.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    pub fn valid(&self) -> bool {
        return { let __tmp_x = (*(*self.frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
    }

    /// resolveInternal fills in u.frame based on u.frame.fn, pc, and sp.
    ///
    /// innermost indicates that this is the first resolve on this stack. If
    /// innermost is set, isSyscall indicates that the PC/SP was retrieved from
    /// gp.syscall*; this is otherwise ignored.
    ///
    /// On entry, u.frame contains:
    ///   - fn is the running function.
    ///   - pc is the PC in the running function.
    ///   - sp is the stack pointer at that program counter.
    ///   - For the innermost frame on LR machines, lr is the program counter that called fn.
    ///
    /// On return, u.frame contains:
    ///   - fp is the stack pointer of the caller.
    ///   - lr is the program counter that called fn.
    ///   - varp, argp, and continpc are populated for the current frame.
    ///
    /// If fn is a stack-jumping function, resolveInternal can change the entire
    /// frame state to follow that stack jump.
    ///
    /// This is internal to unwinder.
    pub fn resolve_internal(&mut self, innermost: Arc<Mutex<Option<bool>>>, isSyscall: Arc<Mutex<Option<bool>>>) {
        let mut frame = self.frame.clone();
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*self.g.lock().unwrap().as_ref().unwrap()));
        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = (*(*(*f.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap()).pcsp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
                // No frame information, must be external function, like race support.
                // See golang.org/issue/13568.
        self.finish_internal();
        return;
    }
                // No frame information, must be external function, like race support.
                // See golang.org/issue/13568.
                // Compute function info flags.
        let mut flag = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).flag.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if {
            let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_CGOCALLBACK as u8))));
            __tmp_x == __tmp_y
        } {
                // cgocallback does write SP to switch from the g0 to the curg stack,
                // but it carefully arranges that during the transition BOTH stacks
                // have cgocallback frame valid for unwinding through.
                // So we don't need to exclude it with the other SP-writing functions.
        { let __rhs = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); let mut guard = flag.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }
                // cgocallback does write SP to switch from the g0 to the curg stack,
                // but it carefully arranges that during the transition BOTH stacks
                // have cgocallback frame valid for unwinding through.
                // So we don't need to exclude it with the other SP-writing functions.
        if { let __v = (*isSyscall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Some Syscall functions write to SP, but they do so only after
                // saving the entry PC/SP using entersyscall.
                // Since we are using the entry PC/SP, the later SP write doesn't matter.
        { let __rhs = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); let mut guard = flag.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }
                // Some Syscall functions write to SP, but they do so only after
                // saving the entry PC/SP using entersyscall.
                // Since we are using the entry PC/SP, the later SP write doesn't matter.
                // Found an actual function.
                // Derive frame pointer.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
                // Jump over system stack transitions. If we're on g0 and there's a user
                // goroutine, try to jump. Otherwise this is a regular call.
                // We also defensively check that this won't switch M's on us,
                // which could happen at critical points in the scheduler.
                // This ensures gp.m doesn't change from a stack jump.
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = {
                        let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & UNWIND_JUMP_STACK as u8)))));
                        let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
                        __tmp_x != __tmp_y
                    };
                    if __go_cond_2 {
                        let __go_cond_3 = { let __left_addr = gp.addr(); let __right_addr = { let __ptr = GoPtr::local((*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).g0.clone()); __ptr.addr() }; let __eq = __left_addr == __right_addr; __eq };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = { let __ptr_field = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone(); !__ptr_field.is_nil() };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_5 = { let __left = { let __ptr_value = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __right = { let __ptr_value = gp.borrow(); let __field_value = __ptr_value.as_ref().unwrap().m.clone(); __field_value }; let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); __eq };
                __go_cond_5
            } else {
                false
            }
        } {
        '__go_switch_1: loop {
        { let _switch_val = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
    if _switch_val == (internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_MORESTACK as u8))))) {
                        // morestack does not return normally -- newstack()
                        // gogo's to curg.sched. Match that.
                        // This keeps morestack() from showing up in the backtrace,
                        // but that makes some sense since it'll never be returned
                        // to.
            gp = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone();
            (*self.g.lock().unwrap().as_mut().unwrap()).set(gp.clone());
            { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
            { let new_val = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = __moved_val; };
            { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *f.lock().unwrap() = Some(new_val); };
            { let new_val = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some((*(*(*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).flag.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *flag.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
            { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.cgo_ctxt.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }; *self.cgo_ctxt.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_SYSTEMSTACK as u8))))) {
                        // systemstack returns normally, so just follow the
                        // stack transition.
            if USES_L_R && { let __tmp_x = funcspdelta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
                // We're at the function prologue and the stack
                // switch hasn't happened, or epilogue where we're
                // about to return. Just unwind normally.
                // Do this only on LR machines because on x86
                // systemstack doesn't have an SP delta (the CALL
                // instruction opens the frame), therefore no way
                // to check.
        { let __rhs = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); let mut guard = flag.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
        break '__go_switch_1
    }
                        // We're at the function prologue and the stack
                        // switch hasn't happened, or epilogue where we're
                        // about to return. Just unwind normally.
                        // Do this only on LR machines because on x86
                        // systemstack doesn't have an SP delta (the CALL
                        // instruction opens the frame), therefore no way
                        // to check.
            gp = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.clone();
            (*self.g.lock().unwrap().as_mut().unwrap()).set(gp.clone());
            { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sched.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
            { let new_val = { let __tmp_x = (({ let __len_target = { let __field = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.cgo_ctxt.clone()); __ptr_value }.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }; *self.cgo_ctxt.lock().unwrap() = Some(new_val); };
            { let __rhs = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); let mut guard = flag.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
        }
    };
        break;
    }
    }
                // morestack does not return normally -- newstack()
                // gogo's to curg.sched. Match that.
                // This keeps morestack() from showing up in the backtrace,
                // but that makes some sense since it'll never be returned
                // to.
                // systemstack returns normally, so just follow the
                // stack transition.
                // We're at the function prologue and the stack
                // switch hasn't happened, or epilogue where we're
                // about to return. Just unwind normally.
                // Do this only on LR machines because on x86
                // systemstack doesn't have an SP delta (the CALL
                // instruction opens the frame), therefore no way
                // to check.
        { let new_val = {
            let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*Arc::new(Mutex::new(Some(funcspdelta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) as usize))).lock().unwrap().as_ref().unwrap());
            __tmp_x + __tmp_y
        }; *(*frame.lock().unwrap().as_ref().unwrap()).fp.lock().unwrap() = Some(new_val); };
        if !USES_L_R {
                // On x86, call instruction pushes return PC before entering new function.
        { let __target = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
                // Jump over system stack transitions. If we're on g0 and there's a user
                // goroutine, try to jump. Otherwise this is a regular call.
                // We also defensively check that this won't switch M's on us,
                // which could happen at critical points in the scheduler.
                // This ensures gp.m doesn't change from a stack jump.
                // morestack does not return normally -- newstack()
                // gogo's to curg.sched. Match that.
                // This keeps morestack() from showing up in the backtrace,
                // but that makes some sense since it'll never be returned
                // to.
                // systemstack returns normally, so just follow the
                // stack transition.
                // We're at the function prologue and the stack
                // switch hasn't happened, or epilogue where we're
                // about to return. Just unwind normally.
                // Do this only on LR machines because on x86
                // systemstack doesn't have an SP delta (the CALL
                // instruction opens the frame), therefore no way
                // to check.
                // On x86, call instruction pushes return PC before entering new function.
                // Derive link register.
        if {
            let __tmp_x = { let __tmp_x = (*flag.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_TOP_FRAME as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x != __tmp_y
        } {
                // This function marks the top of the stack. Stop the traceback.
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    } else if {
        let __go_cond_0 = {
            let __tmp_x = { let __tmp_x = (*flag.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(internal_abi::FUNC_FLAG_S_P_WRITE as u8)))); __tmp_x & __tmp_y };
            let __tmp_y = internal_abi::symtab::FuncFlag(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x != __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __go_cond_2 = !{ let __v = (*innermost.lock().unwrap().as_ref().unwrap()).clone(); __v };
                if __go_cond_2 {
                    true
                } else {
                    let __go_cond_3 = {
                        let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & ((UNWIND_PRINT_ERRORS as u8 | UNWIND_SILENT_ERRORS as u8)))))));
                        let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
                        __tmp_x != __tmp_y
                    };
                    __go_cond_3
                }
            };
            __go_cond_1
        } else {
            false
        }
    } {
        if {
            let __go_cond_0 = {
                let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & ((UNWIND_PRINT_ERRORS as u8 | UNWIND_SILENT_ERRORS as u8)))))));
                let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = !{ let __v = (*innermost.lock().unwrap().as_ref().unwrap()).clone(); __v };
                __go_cond_1
            } else {
                false
            }
        } {
        {
            let __go_print_arg_0 = format!("{}", "traceback: unexpected SPWRITE function".to_string());
            let __go_print_arg_1 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            eprintln!("{} {}", __go_print_arg_0, __go_print_arg_1)
        };
        throw(Arc::new(Mutex::new(Some("traceback".to_string()))));
    }
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    } else {
        let mut lrPtr: Arc<Mutex<Option<usize>>> = Arc::new(Mutex::new(Some(0)));
        if USES_L_R {
        if { let __v = (*innermost.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } || { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *lrPtr.lock().unwrap() = Some(new_val); };
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*lrPtr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }
    } else {
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        { let new_val = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_goarch::PTR_SIZE as usize; __tmp_x - __tmp_y }; *lrPtr.lock().unwrap() = Some(new_val); };
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some((*lrPtr.lock().unwrap().as_ref().unwrap())))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }
    }
    }
                // This function marks the top of the stack. Stop the traceback.
                // The function we are in does a write to SP that we don't know
                // how to encode in the spdelta table. Examples include context
                // switch routines like runtime.gogo but also any code that switches
                // to the g0 stack to run host C code.
                // We can't reliably unwind the SP (we might not even be on
                // the stack we think we are), so stop the traceback here.
                //
                // The one exception (encoded in the complex condition above) is that
                // we assume if we're doing a precise traceback, and this is the
                // innermost frame, that the SPWRITE function voluntarily preempted itself on entry
                // during the stack growth check. In that case, the function has
                // not yet had a chance to do any writes to SP and is safe to unwind.
                // isAsyncSafePoint does not allow assembly functions to be async preempted,
                // and preemptPark double-checks that SPWRITE functions are not async preempted.
                // So for GC stack traversal, we can safely ignore SPWRITE for the innermost frame,
                // but farther up the stack we'd better not find any.
                // This is somewhat imprecise because we're just guessing that we're in the stack
                // growth check. It would be better if SPWRITE were encoded in the spdelta
                // table so we would know for sure that we were still in safe code.
                //
                // uSE uPE inn | action
                //  T   _   _  | frame.lr = 0
                //  F   T   _  | frame.lr = 0
                //  F   F   F  | print; panic
                //  F   F   T  | ignore SPWrite
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).varp.lock().unwrap() = Some(new_val); };
        if !USES_L_R {
                // On x86, call instruction pushes return PC before entering new function.
        { let __target = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
                // On x86, call instruction pushes return PC before entering new function.
                // For architectures with frame pointers, if there's
                // a frame, then there's a saved frame pointer here.
                //
                // NOTE: This code is not as general as it looks.
                // On x86, the ABI is to save the frame pointer word at the
                // top of the stack frame, so we have to back down over it.
                // On arm64, the frame pointer should be at the bottom of
                // the stack (with R29 (aka FP) = RSP), in which case we would
                // not want to do the subtraction here. But we started out without
                // any frame pointer, and when we wanted to add it, we didn't
                // want to break all the assembly doing direct writes to 8(RSP)
                // to set the first parameter to a called function.
                // So we decided to write the FP link *below* the stack pointer
                // (with R29 = RSP - 8 in Go functions).
                // This is technically ABI-compatible but not standard.
                // And it happens to end up mimicking the x86 layout.
                // Other architectures may make different decisions.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } && FRAMEPOINTER_ENABLED {
        { let __target = (*frame.lock().unwrap().as_ref().unwrap()).varp.clone(); let __rhs = internal_goarch::PTR_SIZE as usize; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        { let new_val = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = internal_runtime_sys::MIN_FRAME_SIZE as usize; __tmp_x + __tmp_y }; *(*frame.lock().unwrap().as_ref().unwrap()).argp.lock().unwrap() = Some(new_val); };
                // Determine frame's 'continuation PC', where it can continue.
                // Normally this is the return address on the stack, but if sigpanic
                // is immediately below this function on the stack, then the frame
                // stopped executing due to a trap, and frame.pc is probably not
                // a safe point for looking up liveness information. In this panicking case,
                // the function either doesn't return at all (if it has no defers or if the
                // defers do not recover) or it returns from one of the calls to
                // deferproc a second time (if the corresponding deferred func recovers).
                // In the latter case, use a deferreturn call site as the continuation pc.
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).continpc.lock().unwrap() = Some(new_val); };
        if {
            let __tmp_x = (*self.callee_func_i_d.lock().unwrap().as_ref().unwrap()).clone();
            let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_SIGPANIC as u8))));
            __tmp_x == __tmp_y
        } {
        if { let __tmp_x = (*(*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().deferreturn.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        { let new_val = {
            let __tmp_x = {
                let __tmp_x = (*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap()).entry();
                let __tmp_y = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().deferreturn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap());
                __tmp_x + __tmp_y
            };
            let __tmp_y = 1 as usize;
            __tmp_x + __tmp_y
        }; *(*frame.lock().unwrap().as_ref().unwrap()).continpc.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).continpc.lock().unwrap() = Some(new_val); };
    }
    }
    }

    pub fn next(&mut self) {
        let mut frame = self.frame.clone();
        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*self.g.lock().unwrap().as_ref().unwrap()));
                // Do not unwind past the bottom of the stack.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        self.finish_internal();
        return;
    }
        let mut flr = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        if !(*flr.lock().unwrap().as_ref().unwrap()).valid() {
                // This happens if you get a profiling interrupt at just the wrong time.
                // In that context it is okay to stop early.
                // But if no error flags are set, we're doing a garbage collection and must
                // get everything, so crash loudly.
        let mut fail = Arc::new(Mutex::new(Some({
            let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & ((UNWIND_PRINT_ERRORS as u8 | UNWIND_SILENT_ERRORS as u8)))))));
            let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        })));
        let mut doPrint = Arc::new(Mutex::new(Some({
            let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & UNWIND_SILENT_ERRORS as u8)))));
            let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
            __tmp_x == __tmp_y
        })));
        if {
            let __go_cond_0 = {
                let __go_cond_1 = { let __v = (*doPrint.lock().unwrap().as_ref().unwrap()).clone(); __v };
                if __go_cond_1 {
                    let __go_cond_2 = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).incgo.lock().unwrap().as_ref().unwrap());
                    __go_cond_2
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_3 = {
                    let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                    let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_SIGPANIC as u8))));
                    __tmp_x == __tmp_y
                };
                __go_cond_3
            } else {
                false
            }
        } {
                // We can inject sigpanic
                // calls directly into C code,
                // in which case we'll see a C
                // return PC. Don't complain.
        { let new_val = false; *doPrint.lock().unwrap() = Some(new_val); };
    }
                // We can inject sigpanic
                // calls directly into C code,
                // in which case we'll see a C
                // return PC. Don't complain.
        if { let __v = (*fail.lock().unwrap().as_ref().unwrap()).clone(); __v } || { let __v = (*doPrint.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: g ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", ": unexpected return pc for ".to_string());
            let __go_print_arg_3 = format!("{}", (*funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_4 = format!("{}", " called from ".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        traceback_hexdump(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), frame.clone(), Arc::new(Mutex::new(Some(0 as usize))));
    }
        if { let __v = (*fail.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        throw(Arc::new(Mutex::new(Some("unknown caller pc".to_string()))));
    }
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
        self.finish_internal();
        return;
    }
                // This happens if you get a profiling interrupt at just the wrong time.
                // In that context it is okay to stop early.
                // But if no error flags are set, we're doing a garbage collection and must
                // get everything, so crash loudly.
                // We can inject sigpanic
                // calls directly into C code,
                // in which case we'll see a C
                // return PC. Don't complain.
        if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } && { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } {
                // If the next frame is identical to the current frame, we cannot make progress.
        {
            let __go_print_arg_0 = format!("{}", "runtime: traceback stuck. pc=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", " sp=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        traceback_hexdump(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), frame.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        throw(Arc::new(Mutex::new(Some("traceback stuck".to_string()))));
    }
                // If the next frame is identical to the current frame, we cannot make progress.
        let mut injectedCall = Arc::new(Mutex::new(Some({
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                    let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_SIGPANIC as u8))));
                    __tmp_x == __tmp_y
                };
                if __go_cond_1 {
                    true
                } else {
                    let __go_cond_2 = {
                        let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_ASYNC_PREEMPT as u8))));
                        __tmp_x == __tmp_y
                    };
                    __go_cond_2
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_3 = {
                    let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                    let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_DEBUG_CALL_V2 as u8))));
                    __tmp_x == __tmp_y
                };
                __go_cond_3
            }
        })));
        if { let __v = (*injectedCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let __target = self.flags.clone(); let __rhs = unwindFlags(Arc::new(Mutex::new(Some(UNWIND_TRAP as u8)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    } else {
        { let __target = self.flags.clone(); let __rhs = unwindFlags(Arc::new(Mutex::new(Some(UNWIND_TRAP as u8)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }
                // Unwind to next frame.
        { let new_val = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *self.callee_func_i_d.lock().unwrap() = Some(new_val); };
        { let new_val = flr.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).lr.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap() = Some(new_val); };
        { let new_val = 0 as usize; *(*frame.lock().unwrap().as_ref().unwrap()).fp.lock().unwrap() = Some(new_val); };
                // On link register architectures, sighandler saves the LR on stack
                // before faking a call.
        if USES_L_R && { let __v = (*injectedCall.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut x = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        { let __target = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __rhs = align_up(Arc::new(Mutex::new(Some(internal_runtime_sys::MIN_FRAME_SIZE as usize))), Arc::new(Mutex::new(Some(internal_runtime_sys::STACK_ALIGN as usize)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *f.lock().unwrap() = __moved_val; };
        { let new_val = f.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap() = Some(new_val); };
        if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = funcspdelta(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
        { let new_val = x.lock().unwrap().as_ref().unwrap().clone(); *(*frame.lock().unwrap().as_ref().unwrap()).lr.lock().unwrap() = Some(new_val); };
    }
    }
        self.resolve_internal(Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))));
    }

    /// finishInternal is an unwinder-internal helper called after the stack has been
    /// exhausted. It sets the unwinder to an invalid state and checks that it
    /// successfully unwound the entire stack.
    pub fn finish_internal(&mut self) {
        { let new_val = 0 as usize; *(*self.frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
                // Note that panic != nil is okay here: there can be leftover panics,
                // because the defers on the panic stack do not nest in frame order as
                // they do on the defer stack. If you have:
                //
                //	frame 1 defers d1
                //	frame 2 defers d2
                //	frame 3 defers d3
                //	frame 4 panics
                //	frame 4's panic starts running defers
                //	frame 5, running d3, defers d4
                //	frame 5 panics
                //	frame 5's panic starts running defers
                //	frame 6, running d4, garbage collects
                //	frame 6, running d2, garbage collects
                //
                // During the execution of d4, the panic stack is d4 -> d3, which
                // is nested properly, and we'll treat frame 3 as resumable, because we
                // can find d3. (And in fact frame 3 is resumable. If d4 recovers
                // and frame 5 continues running, d3, d3 can recover and we'll
                // resume execution in (returning from) frame 3.)
                //
                // During the execution of d2, however, the panic stack is d2 -> d3,
                // which is inverted. The scan will match d2 to frame 2 but having
                // d2 on the stack until then means it will not match d3 to frame 3.
                // This is okay: if we're running d2, then all the defers after d2 have
                // completed and their corresponding frames are dead. Not finding d3
                // for frame 3 means we'll set frame 3's continpc == 0, which is correct
                // (frame 3 is dead). At the end of the walk the panic stack can thus
                // contain defers (d3 in this case) for dead frames. The inversion here
                // always indicates a dead frame, and the effect of the inversion on the
                // scan is to hide those dead frames, so the scan is still okay:
                // what's left on the panic stack are exactly (and only) the dead frames.
                //
                // We require callback != nil here because only when callback != nil
                // do we know that gentraceback is being called in a "must be correct"
                // context as opposed to a "best effort" context. The tracebacks with
                // callbacks only happen when everything is stopped nicely.
                // At other times, such as when gathering a stack for a profiling signal
                // or when printing a traceback during a crash, everything may not be
                // stopped nicely, and the stack walk may not be able to complete.
        let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*self.g.lock().unwrap().as_ref().unwrap()));
        if {
            let __go_cond_0 = {
                let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & ((UNWIND_PRINT_ERRORS as u8 | UNWIND_SILENT_ERRORS as u8)))))));
                let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*(*self.frame.lock().unwrap().as_ref().unwrap()).sp.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().stktopsp.clone() }.lock().unwrap().as_ref().unwrap());
                    __tmp_x != __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
        {
            let __go_print_arg_0 = format!("{}", "runtime: g".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", ": frame.sp=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*self.frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " top=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stktopsp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6)
        };
        {
            let __go_print_arg_0 = format!("{}", "\tstack=[".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", "-".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.stack.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "\n".to_string());
            eprint!("{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
        throw(Arc::new(Mutex::new(Some("traceback did not unwind completely".to_string()))));
    }
    }

    /// symPC returns the PC that should be used for symbolizing the current frame.
    /// Specifically, this is the PC of the last instruction executed in this frame.
    ///
    /// If this frame did a normal call, then frame.pc is a return PC, so this will
    /// return frame.pc-1, which points into the CALL instruction. If the frame was
    /// interrupted by a signal (e.g., profiler, segv, etc) then frame.pc is for the
    /// trapped instruction, so this returns frame.pc. See issue #34123. Finally,
    /// frame.pc can be at function entry when the frame is initialized without
    /// actually running code, like in runtime.mstart, in which case this returns
    /// frame.pc because that's the best we can do.
    pub fn sym_p_c(&self) -> usize {
        if {
            let __go_cond_0 = {
                let __tmp_x = unwindFlags(Arc::new(Mutex::new(Some(((*(*self.flags.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & UNWIND_TRAP as u8)))));
                let __tmp_y = unwindFlags(Arc::new(Mutex::new(Some(0 as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = (*(*self.frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*(*self.frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap()).entry();
                    __tmp_x > __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
                // Regular call.
        return { let __tmp_x = (*(*self.frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x - __tmp_y };
    }
                // Regular call.
                // Trapping instruction or we're at the function entry point.
        return (*(*self.frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
    }

    /// cgoCallers populates pcBuf with the cgo callers of the current frame using
    /// the registered cgo unwinder. It returns the number of PCs written to pcBuf.
    /// If the current frame is not a cgo frame or if there's no registered cgo
    /// unwinder, it returns 0.
    pub fn cgo_callers(&mut self, pcBuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
        if {
            let __go_cond_0 = {
                let __go_cond_1 = { let __nil_result = (*cgoTraceback.lock().unwrap()).is_none(); __nil_result };
                if __go_cond_1 {
                    true
                } else {
                    let __go_cond_2 = {
                        let __tmp_x = { let __selector_holder = (*(*self.frame.lock().unwrap().as_ref().unwrap()).r#fn.lock().unwrap().as_ref().unwrap())._func.lock().unwrap().as_ref().unwrap().func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_CGOCALLBACK as u8))));
                        __tmp_x != __tmp_y
                    };
                    __go_cond_2
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_3 = { let __tmp_x = (*self.cgo_ctxt.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y };
                __go_cond_3
            }
        } {
                // We don't have a cgo unwinder (typical case), or we do but we're not
                // in a cgo frame or we're out of cgo context.
        return 0;
    }
                // We don't have a cgo unwinder (typical case), or we do but we're not
                // in a cgo frame or we're out of cgo context.
        let mut ctxt = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = { let __ptr = crate::runtime2::guintptr::ptr(&(*self.g.lock().unwrap().as_ref().unwrap())); let __ptr_value = __ptr.borrow(); __ptr_value.as_ref().unwrap().cgo_ctxt.clone() }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.cgo_ctxt.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        { let __target = self.cgo_ctxt.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        cgo_context_p_cs(Arc::new(Mutex::new(Some({ let __arg_holder = ctxt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pcBuf.clone());
        { let __range_holder = pcBuf.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, pc) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = pc; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return i as i32;
    }
    } }
        (*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32
    }
}

/// tracebackPCs populates pcBuf with the return addresses for each frame from u
/// and returns the number of PCs written to pcBuf. The returned PCs correspond
/// to "logical frames" rather than "physical frames"; that is if A is inlined
/// into B, this will still return a PCs for both A and B. This also includes PCs
/// generated by the cgo unwinder, if one is registered.
///
/// If skip != 0, this skips this many logical frames.
///
/// Callers should set the unwindSilentErrors flag on u.
pub fn traceback_p_cs(u: Arc<Mutex<Option<unwinder>>>, mut skip: Arc<Mutex<Option<i32>>>, pcBuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    let mut cgoBuf: Arc<Mutex<Option<[usize; 32]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    let mut n = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __recv = u.clone(); let __recv_ptr: *const unwinder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const unwinder }; let __result = unsafe { &*__recv_ptr }.valid(); __result } {
        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut cgoN = { let __recv = u.clone(); let __recv_ptr: *mut unwinder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut unwinder }; let __result = unsafe { &mut *__recv_ptr }.cgo_callers(Arc::new(Mutex::new(Some({
            let __seq_holder = cgoBuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = __seq.len();
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        })))); __result };

                // TODO: Why does &u.cache cause u to escape? (Same in traceback2)
        let (mut iu, mut uf) = new_inline_unwinder(
            Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __recv = u.clone(); let __recv_ptr: *const unwinder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const unwinder }; let __result = unsafe { &*__recv_ptr }.sym_p_c(); __result })))
        );
    while { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pcBuf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && (*uf.lock().unwrap().as_ref().unwrap()).valid() {
        let mut sf = (*iu.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if {
            let __go_cond_0 = {
                let __tmp_x = { let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_WRAPPER as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = elide_wrapper_calling(Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).callee_func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
                __go_cond_1
            } else {
                false
            }
        } {
    } else if { let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = skip.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    } else {
        (*pcBuf.lock().unwrap().as_mut().unwrap())[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __tmp_x = (*{ let __field = (*uf.lock().unwrap().as_ref().unwrap()).pc.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as usize; __tmp_x + __tmp_y };
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
                // ignore wrappers
                // Callers expect the pc buffer to contain return addresses
                // and do the -1 themselves, so we add 1 to the call pc to
                // create a "return pc". Since there is no actual call, here
                // "return pc" just means a pc you subtract 1 from to get
                // the pc of the "call". The actual no-op we insert may or
                // may not be 1 byte.
        { let new_val = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*sf.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*u.lock().unwrap().as_ref().unwrap()).callee_func_i_d.lock().unwrap() = Some(new_val); };
        { let new_val = (*iu.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };
    }

                // ignore wrappers
                // Callers expect the pc buffer to contain return addresses
                // and do the -1 themselves, so we add 1 to the call pc to
                // create a "return pc". Since there is no actual call, here
                // "return pc" just means a pc you subtract 1 from to get
                // the pc of the "call". The actual no-op we insert may or
                // may not be 1 byte.
                // Add cgo frames (if we're done skipping over the requested number of
                // Go frames).
        if { let __tmp_x = { let __v = (*skip.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __rhs = (*{
            let _dst_start = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let _dst_len = (*pcBuf.lock().unwrap().as_ref().unwrap()).len() - _dst_start;
            let _src = (*Arc::new(Mutex::new(Some({
                let __seq_holder = cgoBuf.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = (cgoN) as usize;
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }))).lock().unwrap().as_ref().unwrap()).clone();
            let _n = std::cmp::min(_dst_len, _src.len());
            for _i in 0.._n {
                (*pcBuf.lock().unwrap().as_mut().unwrap())[_dst_start + _i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i32)))
        }.lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        { let __recv = u.clone(); let __recv_ptr: *mut unwinder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut unwinder }; let __result = unsafe { &mut *__recv_ptr }.next(); __result };
    }
        // TODO: Why does &u.cache cause u to escape? (Same in traceback2)
        // ignore wrappers
        // Callers expect the pc buffer to contain return addresses
        // and do the -1 themselves, so we add 1 to the call pc to
        // create a "return pc". Since there is no actual call, here
        // "return pc" just means a pc you subtract 1 from to get
        // the pc of the "call". The actual no-op we insert may or
        // may not be 1 byte.
        // Add cgo frames (if we're done skipping over the requested number of
        // Go frames).
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

/// printArgs prints function arguments in traceback.
pub fn print_args(f: Arc<Mutex<Option<funcInfo>>>, argp: Arc<Mutex<Option<usize>>>, pc: Arc<Mutex<Option<usize>>>) {
    let mut p: GoPtr<[u8; 171]> = GoPtr::raw({ let __ptr = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__ARG_INFO as u8)))).clone(); let __ptr_guard = __ptr.lock().unwrap(); __ptr_guard.as_ref().copied().unwrap_or(0) });
    if p.is_nil() {
        return;
    }

    let mut liveInfo = funcdata(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::F_U_N_C_D_A_T_A__ARG_LIVE_INFO as u8))));
    let mut liveIdx = pcdatavalue(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(internal_abi::P_C_D_A_T_A__ARG_LIVE_INDEX as u32))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut startOffset = Arc::new(Mutex::new(Some(0xff as u8)));
    if { let __nil_result = (*liveInfo.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = { let __v = (*Arc::new(Mutex::new({ let __ptr = liveInfo.clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v }; *startOffset.lock().unwrap() = Some(new_val); };
    }

    let liveIdx_closure_clone = liveIdx.clone(); let liveInfo_closure_clone = liveInfo.clone(); let startOffset_closure_clone = startOffset.clone(); let mut isLive = Arc::new(Mutex::new(Some(Box::new(move |off: Arc<Mutex<Option<u8>>>, slotIdx: Arc<Mutex<Option<u8>>>| -> bool {
        if { let __nil_result = (*liveInfo_closure_clone.lock().unwrap()).is_none(); __nil_result } || { let __tmp_x = liveIdx_closure_clone; let __tmp_y = 0 as i32; __tmp_x <= __tmp_y } {
        return true;
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*startOffset_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return true;
    }
        let mut bits = Arc::new(Mutex::new(Some({ let __v = (*Arc::new(Mutex::new({ let __ptr = add(
            Arc::new(Mutex::new(Some({ let __arg_holder = liveInfo_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some(liveIdx_closure_clone as usize))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*slotIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u8; __tmp_x / __tmp_y }) as usize))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })))
        ).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<u8>(unimplemented!("unsafe.Pointer conversion to u8")) } })).lock().unwrap().as_ref().unwrap()).clone(); __v })));
        return { let __tmp_x = { let __tmp_x = { let __v = (*bits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = (1 as u8); let __tmp_y = ({ let __tmp_x = { let __v = (*slotIdx.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u8; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y };
    }) as Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> bool + Send + Sync>)));

        // no liveness info, always live
    let argp_closure_clone = argp.clone(); let isLive_closure_clone = isLive.clone(); let mut print1 = Arc::new(Mutex::new(Some(Box::new(move |off: Arc<Mutex<Option<u8>>>, sz: Arc<Mutex<Option<u8>>>, slotIdx: Arc<Mutex<Option<u8>>>| {
        let mut x = read_unaligned64(add(
            Arc::new(Mutex::new(Some({ let __arg_holder = argp_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as usize)))
        ));
        if { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u8; __tmp_x < __tmp_y } {
        let mut shift = Arc::new(Mutex::new(Some({ let __tmp_x = 64 as u8; let __tmp_y = { let __tmp_x = { let __v = (*sz.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8 as u8; __tmp_x * __tmp_y }; __tmp_x - __tmp_y })));
        if internal_goarch::BIG_ENDIAN {
        { let new_val = { let __tmp_x = x; let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }; x = new_val; };
    } else {
        { let new_val = { let __tmp_x = { let __tmp_x = x; let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x << __tmp_y }; let __tmp_y = { let __v = (*shift.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >> __tmp_y }; x = new_val; };
    }
    }
        {
            let __go_print_arg_0 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(x as u64)))));
            eprint!("{}", __go_print_arg_0)
        };
        if !{ let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> bool + Send + Sync> = { let mut __f_guard = isLive_closure_clone.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(off.clone(), slotIdx.clone()) } {
        {
            let __go_print_arg_0 = format!("{}", "?".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> () + Send + Sync>)));

        // mask out irrelevant bits
    let mut start = Arc::new(Mutex::new(Some(true)));
    let start_closure_clone = start.clone(); let mut printcomma = Arc::new(Mutex::new(Some(Box::new(move || {
        if !{ let __v = (*start_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", ", ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    }) as Box<dyn FnMut() -> () + Send + Sync>)));
    let mut pi = Arc::new(Mutex::new(Some(0)));
    let mut slotIdx = Arc::new(Mutex::new(Some(0 as u8)));
    'printloop: loop {
        let mut o = Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[({ let __v = (*pi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        { let mut guard = pi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let _switch_val = { let __v = (*o.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (internal_abi::TRACE_ARGS_END_SEQ as u8) {
            break 'printloop
        } else if _switch_val == (internal_abi::TRACE_ARGS_START_AGG as u8) {
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = printcomma.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
            {
            let __go_print_arg_0 = format!("{}", "{".to_string());
            eprint!("{}", __go_print_arg_0)
        };
            { let new_val = true; *start.lock().unwrap() = Some(new_val); };
            continue
        } else if _switch_val == (internal_abi::TRACE_ARGS_END_AGG as u8) {
            {
            let __go_print_arg_0 = format!("{}", "}".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (internal_abi::TRACE_ARGS_DOTDOTDOT as u8) {
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = printcomma.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
            {
            let __go_print_arg_0 = format!("{}", "...".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        } else if _switch_val == (internal_abi::TRACE_ARGS_OFFSET_TOO_LARGE as u8) {
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = printcomma.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
            {
            let __go_print_arg_0 = format!("{}", "_".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        } else {
            { let __f_ptr: *mut Box<dyn FnMut() -> () + Send + Sync> = { let mut __f_guard = printcomma.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
            let mut sz = Arc::new(Mutex::new(Some({ let __seq = p.borrow(); __seq.as_ref().unwrap()[({ let __v = (*pi.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
            { let mut guard = pi.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> () + Send + Sync> = { let mut __f_guard = print1.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>, Arc<Mutex<Option<u8>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(o.clone(), sz.clone(), slotIdx.clone()) };
            if { let __tmp_x = { let __v = (*o.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*startOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let mut guard = slotIdx.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        }
    }
        { let new_val = false; *start.lock().unwrap() = Some(new_val); };
    }
}

/// funcNamePiecesForPrint returns the function name for printing to the user.
/// It returns three pieces so it doesn't need an allocation for string
/// concatenation.
pub fn func_name_pieces_for_print(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
        // Replace the shape name in generic function with "...".
    let mut i = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('[' as i32) as u8))));
    if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return ({ let __owned = name.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))));
    }
    let mut j = Arc::new(Mutex::new(Some({ let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = (']' as i32) as u8; __tmp_x != __tmp_y } {
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i; __tmp_x <= __tmp_y } {
        return ({ let __owned = name.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some("".to_string()))), Arc::new(Mutex::new(Some("".to_string()))));
    }
    return (
        Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))),
        Arc::new(Mutex::new(Some("[...]".to_string()))),
        Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() })))
    );
}

/// funcNameForPrint returns the function name for printing to the user.
pub fn func_name_for_print(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let (mut a, mut b, mut c) = func_name_pieces_for_print(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s.push_str(&format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s.push_str(&format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s
    })));
}

/// printFuncName prints a function name. name is the function name in
/// the binary's func data table.
pub fn print_func_name(name: Arc<Mutex<Option<String>>>) {
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "runtime.gopanic".to_string(); __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "panic".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        return;
    }
    let (mut a, mut b, mut c) = func_name_pieces_for_print(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    {
            let __go_print_arg_0 = format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_1 = format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
}

pub fn printcreatedby(gp: GoPtr<crate::runtime2::g>) {
        // Show what created goroutine, except main goroutine (goid 1).
    let mut pc = Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.gopc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if (*f.lock().unwrap().as_ref().unwrap()).valid() && showframe(
        (*f.lock().unwrap().as_ref().unwrap()).src_func(),
        gp.clone(),
        Arc::new(Mutex::new(Some(false))),
        Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8)))))))
    ) && { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x != __tmp_y } {
        printcreatedby1(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.parent_goid.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
}

pub fn printcreatedby1(f: Arc<Mutex<Option<funcInfo>>>, pc: Arc<Mutex<Option<usize>>>, goid: Arc<Mutex<Option<u64>>>) {
    {
            let __go_print_arg_0 = format!("{}", "created by ".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    print_func_name(funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
    if { let __tmp_x = { let __v = (*goid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as u64; __tmp_x != __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", " in goroutine ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*goid.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
    {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    let mut tracepc = { let __owned = pc.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x > __tmp_y } {
        { let __rhs = internal_runtime_sys::P_C_QUANTUM as usize; let mut guard = tracepc.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
    let (mut file, mut line) = funcline(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = tracepc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    {
            let __go_print_arg_0 = format!("{}", "\t".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", ":".to_string());
            let __go_print_arg_3 = format!("{}", line);
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", " +".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x - __tmp_y } as u64)))));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
    {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
}

pub fn traceback(pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>) {
    traceback1(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = lr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gp.clone(), Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(0 as u8))))))));
}

/// tracebacktrap is like traceback but expects that the PC and SP were obtained
/// from a trap, not from gp->sched or gp->syscallpc/gp->syscallsp or GetCallerPC/GetCallerSP.
/// Because they are from a trap instead of from a saved pair,
/// the initial PC must not be rewound to the previous instruction.
/// (All the saved pairs record a PC that is a return address, so we
/// rewind it into the CALL instruction.)
/// If gp.m.libcall{g,pc,sp} information is available, it uses that information in preference to
/// the pc/sp/lr passed in.
pub fn tracebacktrap(pc: Arc<Mutex<Option<usize>>>, sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>) {
    if { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).libcallsp.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // We're in C code somewhere, traceback from the saved position.
        traceback1(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).libcallpc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some({ let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).libcallsp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some(0 as usize))),
            crate::runtime2::guintptr::ptr(&(*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).libcallg.lock().unwrap().as_ref().unwrap())),
            Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(0 as u8)))))))
        );
        return;
    }
        // We're in C code somewhere, traceback from the saved position.
    traceback1(Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = lr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), gp.clone(), Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(UNWIND_TRAP as u8))))))));
}

pub fn traceback1(mut pc: Arc<Mutex<Option<usize>>>, mut sp: Arc<Mutex<Option<usize>>>, lr: Arc<Mutex<Option<usize>>>, gp: GoPtr<crate::runtime2::g>, mut flags: Arc<Mutex<Option<unwindFlags>>>) {
        // If the goroutine is in cgo, and we have a cgo traceback, print that.
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = {
                    let __go_cond_3 = {
                        let __go_cond_4 = (*iscgo.lock().unwrap().as_ref().unwrap());
                        if __go_cond_4 {
                            let __go_cond_5 = { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                            __go_cond_5
                        } else {
                            false
                        }
                    };
                    if __go_cond_3 {
                        let __go_cond_6 = { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).ncgo.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x > __tmp_y };
                        __go_cond_6
                    } else {
                        false
                    }
                };
                if __go_cond_2 {
                    let __go_cond_7 = { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().syscallsp.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y };
                    __go_cond_7
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_8 = { let __nil_target = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                __go_cond_8
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_9 = {
                let __tmp_x = { let __seq_holder = { let __named_array = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[(0) as usize].clone() };
                let __tmp_y = 0 as usize;
                __tmp_x != __tmp_y
            };
            __go_cond_9
        } else {
            false
        }
    } {
                // Lock cgoCallers so that a signal handler won't
                // change it, copy the array, reset it, unlock it.
                // We are locked to the thread and are not running
                // concurrently with a signal handler.
                // We just have to stop a signal handler from interrupting
                // in the middle of our copy.
        (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers_use.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(1 as u32))));
        let mut cgoCallers_local = Arc::new(Mutex::new(Some({ let __v = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).clone(); __v })));
        (*{ let __named_array = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }.lock().unwrap().as_mut().unwrap())[(0) as usize] = 0 as usize;
        (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).cgo_callers_use.lock().unwrap().as_mut().unwrap()).store(Arc::new(Mutex::new(Some(0 as u32))));
        print_cgo_traceback(cgoCallers_local.clone());
    }

        // Lock cgoCallers so that a signal handler won't
        // change it, copy the array, reset it, unlock it.
        // We are locked to the thread and are not running
        // concurrently with a signal handler.
        // We just have to stop a signal handler from interrupting
        // in the middle of our copy.
    if { let __tmp_x = { let __tmp_x = readgstatus(gp.clone()); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GSYSCALL as u32; __tmp_x == __tmp_y } {
                // Override registers if blocked in system call.
        { let new_val = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.syscallpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *pc.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.syscallsp.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp.lock().unwrap() = Some(new_val); };
        { let __rhs = unwindFlags(Arc::new(Mutex::new(Some(UNWIND_TRAP as u8)))); let mut guard = flags.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }
        // Override registers if blocked in system call.
    if { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } && { let __tmp_x = (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).vdso_s_p.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } {
                // Override registers if running in VDSO. This comes after the
                // _Gsyscall check to cover VDSO calls after entersyscall.
        { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).vdso_p_c.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *pc.lock().unwrap() = Some(new_val); };
        { let new_val = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).vdso_s_p.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *sp.lock().unwrap() = Some(new_val); };
        { let __rhs = unwindFlags(Arc::new(Mutex::new(Some(UNWIND_TRAP as u8)))); let mut guard = flags.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() & ! __rhs); };
    }

        // Override registers if running in VDSO. This comes after the
        // _Gsyscall check to cover VDSO calls after entersyscall.
        // Print traceback.
        //
        // We print the first tracebackInnerFrames frames, and the last
        // tracebackOuterFrames frames. There are many possible approaches to this.
        // There are various complications to this:
        //
        // - We'd prefer to walk the stack once because in really bad situations
        //   traceback may crash (and we want as much output as possible) or the stack
        //   may be changing.
        //
        // - Each physical frame can represent several logical frames, so we might
        //   have to pause in the middle of a physical frame and pick up in the middle
        //   of a physical frame.
        //
        // - The cgo symbolizer can expand a cgo PC to more than one logical frame,
        //   and involves juggling state on the C side that we don't manage. Since its
        //   expansion state is managed on the C side, we can't capture the expansion
        //   state part way through, and because the output strings are managed on the
        //   C side, we can't capture the output. Thus, our only choice is to replay a
        //   whole expansion, potentially discarding some of it.
        //
        // Rejected approaches:
        //
        // - Do two passes where the first pass just counts and the second pass does
        //   all the printing. This is undesirable if the stack is corrupted or changing
        //   because we won't see a partial stack if we panic.
        //
        // - Keep a ring buffer of the last N logical frames and use this to print
        //   the bottom frames once we reach the end of the stack. This works, but
        //   requires keeping a surprising amount of state on the stack, and we have
        //   to run the cgo symbolizer twice—once to count frames, and a second to
        //   print them—since we can't retain the strings it returns.
        //
        // Instead, we print the outer frames, and if we reach that limit, we clone
        // the unwinder, count the remaining frames, and then skip forward and
        // finish printing from the clone. This makes two passes over the outer part
        // of the stack, but the single pass over the inner part ensures that's
        // printed immediately and not revisited. It keeps minimal state on the
        // stack. And through a combination of skip counts and limits, we can do all
        // of the steps we need with a single traceback printer implementation.
        //
        // We could be more lax about exactly how many frames we print, for example
        // always stopping and resuming on physical frame boundaries, or at least
        // cgo expansion boundaries. It's not clear that's much simpler.
    { let __rhs = unwindFlags(Arc::new(Mutex::new(Some(UNWIND_PRINT_ERRORS as u8)))); let mut guard = flags.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    let flags_closure_clone = flags.clone(); let gp_closure_clone = gp.clone(); let lr_closure_clone = lr.clone(); let pc_closure_clone = pc.clone(); let sp_closure_clone = sp.clone(); let u_closure_clone = u.clone(); let mut tracebackWithRuntime = Arc::new(Mutex::new(Some(Box::new(move |showRuntime: Arc<Mutex<Option<bool>>>| -> i32 {
        const maxInt: i32 = 0x7fffffff;

        (*u_closure_clone.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some({ let __arg_holder = pc_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = sp_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = lr_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            gp_closure_clone.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = flags_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        );
        let (mut n, mut lastN) = traceback2(u_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = showRuntime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(50))));
        if { let __tmp_x = n; let __tmp_y = 50; __tmp_x < __tmp_y } {
        return n;
    }
        let mut u2 = { let __owned = u_closure_clone.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        let (mut remaining, _) = traceback2(u_closure_clone.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = showRuntime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(2147483647))), Arc::new(Mutex::new(Some(0))));
        let mut elide = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = remaining; let __tmp_y = lastN; __tmp_x - __tmp_y }; let __tmp_y = 50; __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*elide.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "...".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*elide.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " frames elided...\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        traceback2(
            u2.clone(),
            Arc::new(Mutex::new(Some({ let __arg_holder = showRuntime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __tmp_x = lastN; let __tmp_y = { let __v = (*elide.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }))),
            Arc::new(Mutex::new(Some(50)))
        );
    } else if { let __tmp_x = { let __v = (*elide.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        traceback2(u2.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = showRuntime.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(lastN))), Arc::new(Mutex::new(Some(50))));
    }
        n
    }) as Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> i32 + Send + Sync>)));

        // We printed the whole stack.
        // Clone the unwinder and figure out how many frames are left. This
        // count will include any logical frames already printed for u's current
        // physical frame.
        // There are tracebackOuterFrames or fewer frames left to print.
        // Just print the rest of the stack.
        // By default, omits runtime frames. If that means we print nothing at all,
        // repeat forcing all frames printed.
    if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> i32 + Send + Sync> = { let mut __f_guard = tracebackWithRuntime.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(false)))) }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> i32 + Send + Sync> = { let mut __f_guard = tracebackWithRuntime.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<bool>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(true)))) };
    }
    printcreatedby(gp.clone());

    if { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.ancestors.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    for ancestor in &{ let __v = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.ancestors.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        print_ancestor_traceback(Arc::new(Mutex::new(Some((*ancestor).clone()))));
    }
}

/// traceback2 prints a stack trace starting at u. It skips the first "skip"
/// logical frames, after which it prints at most "max" logical frames. It
/// returns n, which is the number of logical frames skipped and printed, and
/// lastN, which is the number of logical frames skipped or printed just in the
/// physical frame that u references.
pub fn traceback2(u: Arc<Mutex<Option<unwinder>>>, showRuntime: Arc<Mutex<Option<bool>>>, skip: Arc<Mutex<Option<i32>>>, max: Arc<Mutex<Option<i32>>>) -> (i32, i32) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut lastN: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        // commitFrame commits to a logical frame and returns whether this frame
        // should be printed and whether iteration should stop.
    let mut lastN_closure_clone = lastN.clone(); let mut max_closure_clone = max.clone(); let mut n_closure_clone = n.clone(); let mut skip_closure_clone = skip.clone(); let mut commitFrame = Arc::new(Mutex::new(Some(Box::new(move || -> (bool, bool) {
    let mut pr: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut stop: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        if { let __tmp_x = { let __v = (*skip_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*max_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (false, true);
    }
        { let mut guard = n_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = lastN_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*skip_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let mut guard = skip_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return (false, false);
    }
        { let mut guard = max_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        (true, false)
    }) as Box<dyn FnMut() -> (bool, bool) + Send + Sync>)));

        // Stop
        // Skip
        // Print
    let mut gp: GoPtr<crate::runtime2::g> = crate::runtime2::guintptr::ptr(&(*(*u.lock().unwrap().as_ref().unwrap()).g.lock().unwrap().as_ref().unwrap()));
    let (mut level, _, _) = gotraceback();
    let mut cgoBuf: Arc<Mutex<Option<[usize; 32]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
    while { let __recv = u.clone(); let __recv_ptr: *const unwinder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const unwinder }; let __result = unsafe { &*__recv_ptr }.valid(); __result } {
        { let new_val = 0; *lastN.lock().unwrap() = Some(new_val); };
        let mut f = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).r#fn.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let (mut iu, mut uf) = new_inline_unwinder(
            Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __recv = u.clone(); let __recv_ptr: *const unwinder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const unwinder }; let __result = unsafe { &*__recv_ptr }.sym_p_c(); __result })))
        );
    while (*uf.lock().unwrap().as_ref().unwrap()).valid() {
        let mut sf = (*iu.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let mut callee = Arc::new(Mutex::new(Some({ let __selector_holder = (*u.lock().unwrap().as_ref().unwrap()).callee_func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        { let new_val = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some((*(*(*sf.lock().unwrap().as_ref().unwrap()).func_i_d.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()))))); *(*u.lock().unwrap().as_ref().unwrap()).callee_func_i_d.lock().unwrap() = Some(new_val); };
        if !({ let __v = (*showRuntime.lock().unwrap().as_ref().unwrap()).clone(); __v } || showframe(
            Arc::new(Mutex::new(Some({ let __arg_holder = sf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            gp.clone(),
            Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = callee.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))
        )) {
        { let new_val = (*iu.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };; continue
    }

        {
        let (mut pr, mut stop) = { let __f_ptr: *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> = { let mut __f_guard = commitFrame.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
        if stop {
            return ((*n.lock().unwrap().as_ref().unwrap()), (*lastN.lock().unwrap().as_ref().unwrap()));;
        } else if !pr {
        { let new_val = (*iu.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };; continue
    }
    }

        let mut name = (*sf.lock().unwrap().as_ref().unwrap()).name();
        let (mut file, mut line) = (*iu.lock().unwrap().as_ref().unwrap()).file_line(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

                // Print during crash.
                //	main(0x1, 0x2, 0x3)
                //		/home/rsc/go/src/runtime/x.go:23 +0xf
                //
        print_func_name(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        {
            let __go_print_arg_0 = format!("{}", "(".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        if (*iu.lock().unwrap().as_ref().unwrap()).is_inlined(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        {
            let __go_print_arg_0 = format!("{}", "...".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    } else {
        let mut argp = Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).argp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        print_args(
            Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __arg_holder = argp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            Arc::new(Mutex::new(Some({ let __recv = u.clone(); let __recv_ptr: *const unwinder = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const unwinder }; let __result = unsafe { &*__recv_ptr }.sym_p_c(); __result })))
        );
    }
        {
            let __go_print_arg_0 = format!("{}", ")\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        {
            let __go_print_arg_0 = format!("{}", "\t".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", ":".to_string());
            let __go_print_arg_3 = format!("{}", line);
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
        if !(*iu.lock().unwrap().as_ref().unwrap()).is_inlined(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        if {
            let __tmp_x = (*(*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry();
            __tmp_x > __tmp_y
        } {
        {
            let __go_print_arg_0 = format!("{}", " +".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({
                let __tmp_x = (*(*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap().as_ref().unwrap());
                let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry();
                __tmp_x - __tmp_y
            } as u64)))));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                    if __go_cond_2 {
                        let __go_cond_3 = {
                            let __tmp_x = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                            let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))));
                            __tmp_x >= __tmp_y
                        };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = { let __left_addr = gp.addr(); let __right_addr = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_5 = { let __tmp_x = level; let __tmp_y = 2 as i32; __tmp_x >= __tmp_y };
                __go_cond_5
            }
        } {
        {
            let __go_print_arg_0 = format!("{}", " fp=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", " sp=".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", " pc=".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*(*u.lock().unwrap().as_ref().unwrap()).frame.lock().unwrap().as_ref().unwrap()).pc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            eprint!("{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5)
        };
    }
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        { let new_val = (*iu.lock().unwrap().as_ref().unwrap()).next(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *uf.lock().unwrap() = __moved_val; };
    }

                // Print during crash.
                //	main(0x1, 0x2, 0x3)
                //		/home/rsc/go/src/runtime/x.go:23 +0xf
                //
                // Print cgo frames.
        {
        let mut cgoN = { let __recv = u.clone(); let __recv_ptr: *mut unwinder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut unwinder }; let __result = unsafe { &mut *__recv_ptr }.cgo_callers(Arc::new(Mutex::new(Some({
            let __seq_holder = cgoBuf.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = __seq.len();
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        })))); __result };;
        if { let __tmp_x = cgoN; let __tmp_y = 0; __tmp_x > __tmp_y } {
            let mut arg: Arc<Mutex<Option<cgoSymbolizerArg>>> = Arc::new(Mutex::new(Some(Default::default())));;
            let mut anySymbolized = Arc::new(Mutex::new(Some(false)));;
            let mut stop = Arc::new(Mutex::new(Some(false)));;
            for pc in {
                let __seq_holder = cgoBuf.clone();
                let __seq_guard = __seq_holder.lock().unwrap();
                let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
                let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
                drop(__seq_guard);
                let __low = 0;
                let __high = (cgoN) as usize;
                let __max = __source_cap;
                let _slice = &__seq[__low..__high];
                let mut _v = Vec::with_capacity((__max - __low) as usize);
                _v.extend_from_slice(_slice);
                _v
            }.iter().copied() {
        if { let __nil_result = (*cgoSymbolizer.lock().unwrap()).is_none(); __nil_result } {
        {
        let (mut pr, mut stop) = { let __f_ptr: *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> = { let mut __f_guard = commitFrame.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
        if stop {
            break;
        } else if pr {
        {
            let __go_print_arg_0 = format!("{}", "non-Go function at pc=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(pc as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    }
    }
    } else {
        { let new_val = print_one_cgo_traceback(Arc::new(Mutex::new(Some(pc.clone()))), commitFrame.clone(), arg.clone()); *stop.lock().unwrap() = Some(new_val); };
        { let new_val = true; *anySymbolized.lock().unwrap() = Some(new_val); };
        if { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break
    }
    }
    };
            if { let __v = (*anySymbolized.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = 0 as usize; *(*arg.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
        call_cgo_symbolizer(arg.clone());
    };
            if { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return ((*n.lock().unwrap().as_ref().unwrap()), (*lastN.lock().unwrap().as_ref().unwrap()));
    };
        }
    }
        { let __recv = u.clone(); let __recv_ptr: *mut unwinder = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut unwinder }; let __result = unsafe { &mut *__recv_ptr }.next(); __result };
    }
        // Print during crash.
        //	main(0x1, 0x2, 0x3)
        //		/home/rsc/go/src/runtime/x.go:23 +0xf
        //
        // Print cgo frames.
        // Free symbolization state.
    return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, 0);
}

/// printAncestorTraceback prints the traceback of the given ancestor.
/// TODO: Unify this with gentraceback and CallersFrames.
pub fn print_ancestor_traceback(ancestor: Arc<Mutex<Option<ancestorInfo>>>) {
    {
            let __go_print_arg_0 = format!("{}", "[originating from goroutine ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __field = (*ancestor.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", "]:\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    { let __range_holder = (*ancestor.lock().unwrap().as_ref().unwrap()).pcs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (fidx, pc) in __range_values.iter().copied().enumerate() {
        let mut f = findfunc(Arc::new(Mutex::new(Some(pc.clone()))));
        if showfuncinfo(
            (*f.lock().unwrap().as_ref().unwrap()).src_func(),
            Arc::new(Mutex::new(Some({ let __tmp_x = fidx as i32; let __tmp_y = 0; __tmp_x == __tmp_y }))),
            Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8)))))))
        ) {
        print_ancestor_traceback_func_info(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(pc.clone()))));
    }
    } }
        // f previously validated
    if { let __tmp_x = (({ let __len_target = { let __field = (*ancestor.lock().unwrap().as_ref().unwrap()).pcs.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 50; __tmp_x == __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", "...additional frames elided...\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }

        // Show what created goroutine, except main goroutine (goid 1).
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = (*ancestor.lock().unwrap().as_ref().unwrap()).gopc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if (*f.lock().unwrap().as_ref().unwrap()).valid() && showfuncinfo(
        (*f.lock().unwrap().as_ref().unwrap()).src_func(),
        Arc::new(Mutex::new(Some(false))),
        Arc::new(Mutex::new(Some(internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_NORMAL as u8)))))))
    ) && { let __tmp_x = (*{ let __field = (*ancestor.lock().unwrap().as_ref().unwrap()).goid.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as u64; __tmp_x != __tmp_y } {
                // In ancestor mode, we'll already print the goroutine ancestor.
                // Pass 0 for the goid parameter so we don't print it again.
        printcreatedby1(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*ancestor.lock().unwrap().as_ref().unwrap()).gopc.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(0 as u64))));
    }
}

/// printAncestorTracebackFuncInfo prints the given function info at a given pc
/// within an ancestor traceback. The precision of this info is reduced
/// due to only have access to the pcs at the time of the caller
/// goroutine being created.
pub fn print_ancestor_traceback_func_info(f: Arc<Mutex<Option<funcInfo>>>, pc: Arc<Mutex<Option<usize>>>) {
    let (mut u, mut uf) = new_inline_unwinder(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = pc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut file, mut line) = (*u.lock().unwrap().as_ref().unwrap()).file_line(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    print_func_name({
        let __recv = (*u.lock().unwrap().as_ref().unwrap()).src_func(Arc::new(Mutex::new(Some({ let __arg_holder = uf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let __result = (*__recv.lock().unwrap().as_ref().unwrap()).name();
        __result
    });
    {
            let __go_print_arg_0 = format!("{}", "(...)\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", "\t".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*file.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", ":".to_string());
            let __go_print_arg_3 = format!("{}", line);
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    if { let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x > __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", " +".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*pc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*f.lock().unwrap().as_ref().unwrap()).entry(); __tmp_x - __tmp_y } as u64)))));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    }
    {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
}

/// callers should be an internal detail,
/// (and is almost identical to Callers),
/// but widely used packages access it using linkname.
/// Notable members of the hall of shame include:
///   - github.com/phuslu/log
///
/// Do not remove or change the type signature.
/// See go.dev/issue/67401.
///
///go:linkname callers
pub fn callers_1(skip: Arc<Mutex<Option<i32>>>, pcbuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    let mut sp = internal_runtime_sys::get_caller_s_p();
    let mut pc = internal_runtime_sys::get_caller_p_c();
    let mut gp = getg();
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let gp_closure_clone = gp.clone(); let mut n_closure_clone = n.clone(); let pc_closure_clone = pc.clone(); let pcbuf_closure_clone = pcbuf.clone(); let skip_closure_clone = skip.clone(); let sp_closure_clone = sp.clone(); systemstack(Arc::new(Mutex::new(Some(Box::new(move || {
        let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*u.lock().unwrap().as_mut().unwrap()).init_at(
            Arc::new(Mutex::new(Some(pc_closure_clone))),
            Arc::new(Mutex::new(Some(sp_closure_clone))),
            Arc::new(Mutex::new(Some(0 as usize))),
            GoPtr::local(gp_closure_clone.clone()),
            Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(UNWIND_SILENT_ERRORS as u8))))))),
        );
        { let new_val = traceback_p_cs(u.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skip_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pcbuf_closure_clone.clone()); *n_closure_clone.lock().unwrap() = Some(new_val); };
    }) as Box<dyn FnMut() -> () + Send + Sync>))));
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn gcallers(gp: GoPtr<crate::runtime2::g>, skip: Arc<Mutex<Option<i32>>>, pcbuf: Arc<Mutex<Option<Vec<usize>>>>) -> i32 {
    let mut u: Arc<Mutex<Option<unwinder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*u.lock().unwrap().as_mut().unwrap()).init(gp.clone(), Arc::new(Mutex::new(Some(unwindFlags(Arc::new(Mutex::new(Some(UNWIND_SILENT_ERRORS as u8))))))));
    return traceback_p_cs(u.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = skip.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), pcbuf.clone());
}

/// showframe reports whether the frame with the given characteristics should
/// be printed during a traceback.
pub fn showframe(sf: Arc<Mutex<Option<srcFunc>>>, gp: GoPtr<crate::runtime2::g>, firstFrame: Arc<Mutex<Option<bool>>>, calleeID: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>) -> bool {
    let mut mp = (*getg().lock().unwrap().as_ref().unwrap()).m.clone();
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __tmp_x = { let __selector_holder = (*mp.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))));
                __tmp_x >= __tmp_y
            };
            if __go_cond_1 {
                let __go_cond_2 = !gp.is_nil();
                __go_cond_2
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_3 = {
                let __go_cond_4 = { let __left_addr = gp.addr(); let __right_addr = (*mp.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq };
                if __go_cond_4 {
                    true
                } else {
                    let __go_cond_5 = { let __left_addr = gp.addr(); let __right_addr = crate::runtime2::guintptr::ptr(&(*(*mp.lock().unwrap().as_ref().unwrap()).caughtsig.lock().unwrap().as_ref().unwrap())).addr(); let __eq = __left_addr == __right_addr; __eq };
                    __go_cond_5
                }
            };
            __go_cond_3
        } else {
            false
        }
    } {
        return true;
    }
    showfuncinfo(Arc::new(Mutex::new(Some({ let __arg_holder = sf.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = firstFrame.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = calleeID.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// showfuncinfo reports whether a function with the given characteristics should
/// be printed during a traceback.
pub fn showfuncinfo(sf: Arc<Mutex<Option<srcFunc>>>, firstFrame: Arc<Mutex<Option<bool>>>, calleeID: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>) -> bool {
    let (mut level, _, _) = gotraceback();
    if { let __tmp_x = level; let __tmp_y = 1 as i32; __tmp_x > __tmp_y } {
                // Show all frames.
        return true;
    }

        // Show all frames.
    if {
        let __go_cond_0 = {
            let __tmp_x = { let __selector_holder = (*sf.lock().unwrap().as_ref().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
            let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_WRAPPER as u8))));
            __tmp_x == __tmp_y
        };
        if __go_cond_0 {
            let __go_cond_1 = elide_wrapper_calling(Arc::new(Mutex::new(Some({ let __arg_holder = calleeID.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            __go_cond_1
        } else {
            false
        }
    } {
        return false;
    }

    let mut name = (*sf.lock().unwrap().as_ref().unwrap()).name();

        // Special case: always show runtime.gopanic frame
        // in the middle of a stack trace, so that we can
        // see the boundary between ordinary code and
        // panic-induced deferred code.
        // See golang.org/issue/5832.
    if { let __tmp_x = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "runtime.gopanic".to_string(); __tmp_x == __tmp_y } && !{ let __v = (*firstFrame.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return true;
    }

    return { let __tmp_x = internal_bytealg::index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('.' as i32) as u8)))); let __tmp_y = 0; __tmp_x >= __tmp_y } && (!internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime.".to_string())))) || is_exported_runtime(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))));
}

/// isExportedRuntime reports whether name is an exported runtime function.
/// It is only for runtime functions, so ASCII A-Z is fine.
pub fn is_exported_runtime(mut name: Arc<Mutex<Option<String>>>) -> bool {
        // Check and remove package qualifier.
    let (__tmp_0, mut found) = internal_stringslite::cut_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("runtime.".to_string())))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_tmp_0;;
    if !found {
        return false;
    }
    let mut rcvr = Arc::new(Mutex::new(Some("".to_string())));

        // Extract receiver type, if any.
        // For example, runtime.(*Func).Entry
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rcvr.lock().unwrap() = __moved_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *name.lock().unwrap() = __moved_val; };
                // Remove parentheses and star for pointer receivers.
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __tmp_x = ((*rcvr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x >= __tmp_y };
                    if __go_cond_2 {
                        let __go_cond_3 = { let __tmp_x = { let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('(' as i32) as u8; __tmp_x == __tmp_y };
                        __go_cond_3
                    } else {
                        false
                    }
                };
                if __go_cond_1 {
                    let __go_cond_4 = { let __tmp_x = { let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y };
                    __go_cond_4
                } else {
                    false
                }
            };
            if __go_cond_0 {
                let __go_cond_5 = { let __tmp_x = { let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = ((*rcvr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = (')' as i32) as u8; __tmp_x == __tmp_y };
                __go_cond_5
            } else {
                false
            }
        } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; let __high = ({ let __tmp_x = ((*rcvr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *rcvr.lock().unwrap() = __moved_val; };
    }
    }

        // Remove parentheses and star for pointer receivers.
        // Exported functions and exported methods on exported types.
    return {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = { let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y };
                if __go_cond_2 {
                    let __go_cond_3 = { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; __tmp_x <= __tmp_y };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            let __go_cond_5 = {
                let __go_cond_6 = { let __tmp_x = ((*rcvr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y };
                if __go_cond_6 {
                    true
                } else {
                    let __go_cond_7 = {
                        let __go_cond_8 = { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; __tmp_x <= __tmp_y };
                        if __go_cond_8 {
                            let __go_cond_9 = { let __tmp_x = { let __s = &((*rcvr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y };
                            __go_cond_9
                        } else {
                            false
                        }
                    };
                    __go_cond_7
                }
            };
            __go_cond_5
        } else {
            false
        }
    };
}

/// elideWrapperCalling reports whether a wrapper function that called
/// function id should be elided from stack traces.
pub fn elide_wrapper_calling(id: Arc<Mutex<Option<internal_abi::symtab::FuncID>>>) -> bool {
        // If the wrapper called a panic function instead of the
        // wrapped function, we want to include it in stacks.
    !({
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone();
                let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_GOPANIC as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_1 {
                true
            } else {
                let __go_cond_2 = {
                    let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone();
                    let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_SIGPANIC as u8))));
                    __tmp_x == __tmp_y
                };
                __go_cond_2
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __tmp_x = (*id.lock().unwrap().as_ref().unwrap()).clone();
                let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_PANICWRAP as u8))));
                __tmp_x == __tmp_y
            };
            __go_cond_3
        }
    })
}

pub fn goroutineheader(gp: GoPtr<crate::runtime2::g>) {
    let (mut level, _, _) = gotraceback();

    let mut gpstatus = readgstatus(gp.clone());

    let mut isScan = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = gpstatus; let __tmp_y = __GSCAN as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y })));
    { let __rhs = __GSCAN as u32; gpstatus = gpstatus & ! __rhs; };

        // Basic string status
    let mut status: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    if { let __tmp_x = 0 as u32; let __tmp_y = gpstatus; __tmp_x <= __tmp_y } && { let __tmp_x = gpstatus; let __tmp_y = (*Arc::new(Mutex::new(Some((*gStatusStrings.lock().unwrap().as_ref().unwrap()).len() as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x < __tmp_y } {
        { let new_val = { let __seq = { let __seq_holder = gStatusStrings.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(gpstatus) as usize].clone() }; *status.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = "???".to_string(); *status.lock().unwrap() = Some(new_val); };
    }

        // Override.
    if {
        let __go_cond_0 = { let __tmp_x = gpstatus; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = crate::runtime2::waitReason(Arc::new(Mutex::new(Some(WAIT_REASON_ZERO as u8))));
                __tmp_x != __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    } {
        { let new_val = crate::runtime2::waitReason::string(&(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.waitreason.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap())); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *status.lock().unwrap() = __moved_val; };
    }

        // approx time the G is blocked, in minutes
    let mut waitfor: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    if ({ let __tmp_x = gpstatus; let __tmp_y = __GWAITING as u32; __tmp_x == __tmp_y } || { let __tmp_x = gpstatus; let __tmp_y = __GSYSCALL as u32; __tmp_x == __tmp_y }) && { let __tmp_x = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().waitsince.clone() }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x != __tmp_y } {
        { let new_val = { let __tmp_x = ({ let __tmp_x = nanotime(); let __tmp_y = (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().waitsince.clone() }.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }); let __tmp_y = 60e9 as i64; __tmp_x / __tmp_y }; *waitfor.lock().unwrap() = Some(new_val); };
    }
    {
            let __go_print_arg_0 = format!("{}", "goroutine ".to_string());
            let __go_print_arg_1 = format!("{}", (*{ let __ptr_value = gp.borrow(); __ptr_value.as_ref().unwrap().goid.clone() }.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __go_cond_2 = { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
                if __go_cond_2 {
                    let __go_cond_3 = {
                        let __tmp_x = { let __selector_holder = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).throwing.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                        let __tmp_y = crate::panic::throwType(Arc::new(Mutex::new(Some(THROW_TYPE_RUNTIME as u32))));
                        __tmp_x >= __tmp_y
                    };
                    __go_cond_3
                } else {
                    false
                }
            };
            if __go_cond_1 {
                let __go_cond_4 = { let __left_addr = gp.addr(); let __right_addr = (*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).curg.addr(); let __eq = __left_addr == __right_addr; __eq };
                __go_cond_4
            } else {
                false
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_5 = { let __tmp_x = level; let __tmp_y = 2 as i32; __tmp_x >= __tmp_y };
            __go_cond_5
        }
    } {
        {
            let __go_print_arg_0 = format!("{}", " gp=".to_string());
            let __go_print_arg_1 = format!("{}", format!("0x{:x}", gp.addr()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
        if { let __nil_target = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", " m=".to_string());
            let __go_print_arg_1 = format!("{}", (*(*{ let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.lock().unwrap().as_ref().unwrap()).id.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_2 = format!("{}", " mp=".to_string());
            let __go_print_arg_3 = format!("{}", format!("&{}", (*{ let __field = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.m.clone()); __ptr_value }.clone(); __field }.lock().unwrap().as_ref().unwrap())));
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", " m=nil".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    }
    {
            let __go_print_arg_0 = format!("{}", " [".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*status.lock().unwrap().as_ref().unwrap()).clone(); __v });
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };
    if { let __v = (*isScan.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            let __go_print_arg_0 = format!("{}", " (scan)".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    if { let __tmp_x = { let __v = (*waitfor.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1 as i64; __tmp_x >= __tmp_y } {
        {
            let __go_print_arg_0 = format!("{}", ", ".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*waitfor.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", " minutes".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    }
    if {
        let __tmp_x = { let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.lockedm.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = crate::runtime2::muintptr(Arc::new(Mutex::new(Some(0 as usize))));
        __tmp_x != __tmp_y
    } {
        {
            let __go_print_arg_0 = format!("{}", ", locked to thread".to_string());
            eprint!("{}", __go_print_arg_0)
        };
    }
    {
        let mut sg = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.sync_group.clone()); __ptr_value }.clone();;
        if { let __nil_result = (*sg.lock().unwrap()).is_some(); __nil_result } {
            {
            let __go_print_arg_0 = format!("{}", ", synctest group ".to_string());
            let __go_print_arg_1 = format!("{}", (*(*(*sg.lock().unwrap().as_ref().unwrap()).root.lock().unwrap().as_ref().unwrap()).goid.lock().unwrap().as_ref().unwrap()));
            eprint!("{}{}", __go_print_arg_0, __go_print_arg_1)
        };;
        }
    }
    {
            let __go_print_arg_0 = format!("{}", "]:\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
}

pub fn tracebackothers(me: GoPtr<crate::runtime2::g>) {
    let (mut level, _, _) = gotraceback();

        // Show the current goroutine first, if we haven't already.
    let mut curgp: GoPtr<crate::runtime2::g> = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.clone();
    if !curgp.is_nil() && { let __left_addr = curgp.addr(); let __right_addr = me.addr(); let __eq = __left_addr == __right_addr; !__eq } {
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        goroutineheader(curgp.clone());
        traceback(
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(0 as usize))),
            curgp.clone()
        );
    }

        // We can't call locking forEachG here because this may be during fatal
        // throw/panic, where locking could be out-of-order or a direct
        // deadlock.
        //
        // Instead, use forEachGRace, which requires no locking. We don't lock
        // against concurrent creation of new Gs, but even with allglock we may
        // miss Gs created after this loop.
    let curgp_closure_clone = curgp.clone(); let level_closure_clone = level.clone(); let me_closure_clone = me.clone(); for_each_g_race(Arc::new(Mutex::new(Some(Box::new(move |gp: Arc<Mutex<Option<g>>>| {
        if {
            let __go_cond_0 = {
                let __go_cond_1 = {
                    let __go_cond_2 = { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = me_closure_clone.addr(); let __eq = __left_addr == __right_addr; __eq };
                    if __go_cond_2 {
                        true
                    } else {
                        let __go_cond_3 = { let __left_addr = { let __ptr = GoPtr::local(gp.clone()); __ptr.addr() }; let __right_addr = curgp_closure_clone.addr(); let __eq = __left_addr == __right_addr; __eq };
                        __go_cond_3
                    }
                };
                if __go_cond_1 {
                    true
                } else {
                    let __go_cond_4 = { let __tmp_x = readgstatus(GoPtr::local(gp.clone())); let __tmp_y = __GDEAD as u32; __tmp_x == __tmp_y };
                    __go_cond_4
                }
            };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_5 = {
                    let __go_cond_6 = is_system_goroutine(GoPtr::local(gp.clone()), Arc::new(Mutex::new(Some(false))));
                    if __go_cond_6 {
                        let __go_cond_7 = { let __tmp_x = level_closure_clone; let __tmp_y = 2 as i32; __tmp_x < __tmp_y };
                        __go_cond_7
                    } else {
                        false
                    }
                };
                __go_cond_5
            }
        } {
        return;
    }
        {
            let __go_print_arg_0 = format!("{}", "\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        goroutineheader(GoPtr::local(gp.clone()));
        if {
            let __go_cond_0 = { let __left = (*gp.lock().unwrap().as_ref().unwrap()).m.clone(); let __right = (*getg().lock().unwrap().as_ref().unwrap()).m.clone(); let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none(); let __eq = __both_nil || Arc::ptr_eq(&__left, &__right); !__eq };
            if __go_cond_0 {
                let __go_cond_1 = { let __tmp_x = { let __tmp_x = readgstatus(GoPtr::local(gp.clone())); let __tmp_y = __GSCAN as u32; __tmp_x & ! __tmp_y }; let __tmp_y = __GRUNNING as u32; __tmp_x == __tmp_y };
                __go_cond_1
            } else {
                false
            }
        } {
        {
            let __go_print_arg_0 = format!("{}", "\tgoroutine running on other thread; stack unavailable\n".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        printcreatedby(GoPtr::local(gp.clone()));
    } else {
        traceback(
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(!(0 as usize) as usize))),
            Arc::new(Mutex::new(Some(0 as usize))),
            GoPtr::local(gp.clone())
        );
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<g>>>) -> () + Send + Sync>))));
}

/// tracebackHexdump hexdumps part of stk around frame.sp and frame.fp
/// for debugging purposes. If the address bad is included in the
/// hexdumped range, it will mark it as well.
pub fn traceback_hexdump(stk: Arc<Mutex<Option<stack>>>, frame: Arc<Mutex<Option<stkframe>>>, bad: Arc<Mutex<Option<usize>>>) {
    const expand: i32 = 32 * internal_goarch::PTR_SIZE;

    const maxExpand: i32 = 256 * internal_goarch::PTR_SIZE;


        // Start around frame.sp.
    let (mut lo, mut hi) = (Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));

        // Expand to include frame.fp.
    if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *lo.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x != __tmp_y } && { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *hi.lock().unwrap() = Some(new_val); };
    }

        // Expand a bit more.
    {
        let __tmp_0 = { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = expand as usize; __tmp_x - __tmp_y };
        let __tmp_1 = { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = expand as usize; __tmp_x + __tmp_y };
        *lo.lock().unwrap() = Some(__tmp_0);
        *hi.lock().unwrap() = Some(__tmp_1);
    };

        // But don't go too far from frame.sp.
    if { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = maxExpand as usize; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = maxExpand as usize; __tmp_x - __tmp_y }; *lo.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = maxExpand as usize; __tmp_x + __tmp_y }; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = (*{ let __field = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = maxExpand as usize; __tmp_x + __tmp_y }; *hi.lock().unwrap() = Some(new_val); };
    }

        // And don't go outside the stack bounds.
    if { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *lo.lock().unwrap() = Some(new_val); };
    }
    if { let __tmp_x = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*{ let __field = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = { let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *hi.lock().unwrap() = Some(new_val); };
    }

        // Print the hex dump.
    {
            let __go_print_arg_0 = format!("{}", "stack: frame={sp:".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).sp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_2 = format!("{}", ", fp:".to_string());
            let __go_print_arg_3 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*frame.lock().unwrap().as_ref().unwrap()).fp.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_4 = format!("{}", "} stack=[".to_string());
            let __go_print_arg_5 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).lo.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_6 = format!("{}", ",".to_string());
            let __go_print_arg_7 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some({ let __selector_holder = (*stk.lock().unwrap().as_ref().unwrap()).hi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64)))));
            let __go_print_arg_8 = format!("{}", ")\n".to_string());
            eprint!("{}{}{}{}{}{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4, __go_print_arg_5, __go_print_arg_6, __go_print_arg_7, __go_print_arg_8)
        };
    let bad_closure_clone = bad.clone(); let frame_closure_clone = frame.clone(); hexdump_words(Arc::new(Mutex::new(Some({ let __arg_holder = lo.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = hi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move |p: Arc<Mutex<Option<usize>>>| -> u8 {
        { let _switch_val = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == ((*{ let __field = (*frame_closure_clone.lock().unwrap().as_ref().unwrap()).fp.clone(); __field }.lock().unwrap().as_ref().unwrap())) {
            return ('>' as u8);
        } else if _switch_val == ((*{ let __field = (*frame_closure_clone.lock().unwrap().as_ref().unwrap()).sp.clone(); __field }.lock().unwrap().as_ref().unwrap())) {
            return ('<' as u8);
        } else if _switch_val == ({ let __v = (*bad_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }) {
            return ('!' as u8);
        }
    }
        0
    }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> u8 + Send + Sync>))));
}

/// isSystemGoroutine reports whether the goroutine g must be omitted
/// in stack dumps and deadlock detector. This is any goroutine that
/// starts at a runtime.* entry point, except for runtime.main,
/// runtime.handleAsyncEvent (wasm only) and sometimes runtime.runfinq.
///
/// If fixed is true, any goroutine that can vary between user and
/// system (that is, the finalizer goroutine) is considered a user
/// goroutine.
pub fn is_system_goroutine(gp: GoPtr<crate::runtime2::g>, fixed: Arc<Mutex<Option<bool>>>) -> bool {
        // Keep this in sync with internal/trace.IsSystemGoroutine.
    let mut f = findfunc(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = gp.with_mut(|__ptr_value| __ptr_value.startpc.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if !(*f.lock().unwrap().as_ref().unwrap()).valid() {
        return false;
    }
    if {
        let __go_cond_0 = {
            let __go_cond_1 = {
                let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_RUNTIME_MAIN as u8))));
                __tmp_x == __tmp_y
            };
            if __go_cond_1 {
                true
            } else {
                let __go_cond_2 = {
                    let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                    let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_COROSTART as u8))));
                    __tmp_x == __tmp_y
                };
                __go_cond_2
            }
        };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_3 = {
                let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
                let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_HANDLE_ASYNC_EVENT as u8))));
                __tmp_x == __tmp_y
            };
            __go_cond_3
        }
    } {
        return false;
    }
    if {
        let __tmp_x = { let __selector_holder = (*(*f.lock().unwrap().as_mut().unwrap())._func.lock().unwrap().as_mut().unwrap()).func_i_d.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned };
        let __tmp_y = internal_abi::symtab::FuncID(Arc::new(Mutex::new(Some(internal_abi::FUNC_I_D_RUNFINQ as u8))));
        __tmp_x == __tmp_y
    } {
                // We include the finalizer goroutine if it's calling
                // back into user code.
        if { let __v = (*fixed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // This goroutine can vary. In fixed mode,
                // always consider it a user goroutine.
        return false;
    }
                // This goroutine can vary. In fixed mode,
                // always consider it a user goroutine.
        return { let __tmp_x = { let __tmp_x = (*fingStatus.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = FING_RUNNING_FINALIZER as u32; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y };
    }
        // We include the finalizer goroutine if it's calling
        // back into user code.
        // This goroutine can vary. In fixed mode,
        // always consider it a user goroutine.
    return internal_stringslite::has_prefix(funcname(Arc::new(Mutex::new(Some({ let __arg_holder = f.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), Arc::new(Mutex::new(Some("runtime.".to_string()))));
}

/// printCgoTraceback prints a traceback of callers.
pub fn print_cgo_traceback(callers: Arc<Mutex<Option<cgoCallers>>>) {
    if { let __nil_result = (*cgoSymbolizer.lock().unwrap()).is_none(); __nil_result } {
        { let __range_holder = { let __named_array = (*callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter().copied() {
        if { let __tmp_x = c; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
        {
            let __go_print_arg_0 = format!("{}", "non-Go function at pc=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some(c as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
    } }
        return;
    }

    let mut commitFrame = Arc::new(Mutex::new(Some(Box::new(move || -> (bool, bool) {
    let mut pr: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut stop: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

        (true, false)
    }) as Box<dyn FnMut() -> (bool, bool) + Send + Sync>)));
    let mut arg: Arc<Mutex<Option<cgoSymbolizerArg>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = { let __named_array = (*callers.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter().copied() {
        if { let __tmp_x = c; let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        break
    }
        print_one_cgo_traceback(Arc::new(Mutex::new(Some(c.clone()))), commitFrame.clone(), arg.clone());
    } }
    { let new_val = 0 as usize; *(*arg.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    call_cgo_symbolizer(arg.clone());
}

/// printOneCgoTraceback prints the traceback of a single cgo caller.
/// This can print more than one line because of inlining.
/// It returns the "stop" result of commitFrame.
pub fn print_one_cgo_traceback(pc: Arc<Mutex<Option<usize>>>, commitFrame: Arc<Mutex<Option<Box<dyn FnMut() -> (bool, bool) + Send + Sync>>>>, arg: Arc<Mutex<Option<cgoSymbolizerArg>>>) -> bool {
    { let new_val = pc.lock().unwrap().as_ref().unwrap().clone(); *(*arg.lock().unwrap().as_ref().unwrap()).pc.lock().unwrap() = Some(new_val); };
    loop {
        {
        let (mut pr, mut stop) = { let __f_ptr: *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> = { let mut __f_guard = commitFrame.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> (bool, bool) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };;
        if stop {
            return true;;
        } else if !pr {
        continue
    }
    }

        call_cgo_symbolizer(arg.clone());
        if { let __nil_target = (*arg.lock().unwrap().as_ref().unwrap()).func_name.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
                // Note that we don't print any argument
                // information here, not even parentheses.
                // The symbolizer must add that if appropriate.
        {
            let __go_print_arg_0 = format!("{}", (*gostringnocopy(GoPtr::local((*arg.lock().unwrap().as_ref().unwrap()).func_name.clone())).lock().unwrap().as_ref().unwrap()));
            eprintln!("{}", __go_print_arg_0)
        };
    } else {
        {
            let __go_print_arg_0 = format!("{}", "non-Go function".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
    }
                // Note that we don't print any argument
                // information here, not even parentheses.
                // The symbolizer must add that if appropriate.
        {
            let __go_print_arg_0 = format!("{}", "\t".to_string());
            eprint!("{}", __go_print_arg_0)
        };
        if { let __nil_target = (*arg.lock().unwrap().as_ref().unwrap()).file.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        {
            let __go_print_arg_0 = format!("{}", (*gostringnocopy(GoPtr::local((*arg.lock().unwrap().as_ref().unwrap()).file.clone())).lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_1 = format!("{}", ":".to_string());
            let __go_print_arg_2 = format!("{}", (*{ let __field = (*arg.lock().unwrap().as_ref().unwrap()).lineno.clone(); __field }.lock().unwrap().as_ref().unwrap()));
            let __go_print_arg_3 = format!("{}", " ".to_string());
            eprint!("{}{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3)
        };
    }
        {
            let __go_print_arg_0 = format!("{}", "pc=".to_string());
            let __go_print_arg_1 = format!("{}", crate::print::hex(Arc::new(Mutex::new(Some((*pc.lock().unwrap().as_ref().unwrap()) as u64)))));
            let __go_print_arg_2 = format!("{}", "\n".to_string());
            eprint!("{}{}{}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2)
        };
        if { let __tmp_x = (*{ let __field = (*arg.lock().unwrap().as_ref().unwrap()).more.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return false;
    }
    }
}

/// callCgoSymbolizer calls the cgoSymbolizer function.
pub fn call_cgo_symbolizer(arg: Arc<Mutex<Option<cgoSymbolizerArg>>>) {
    let mut call = Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> i32 { cgocall(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>)));
    if {
        let __go_cond_0 = { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __left_addr = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq };
            __go_cond_1
        }
    } {
                // We do not want to call into the scheduler when panicking
                // or when on the system stack.
        { let new_val = Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> i32 { asmcgocall(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>; *call.lock().unwrap() = Some(new_val); };
    }
        // We do not want to call into the scheduler when panicking
        // or when on the system stack.
    if MSANENABLED {
        msanwrite(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&arg) as usize))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<cgoSymbolizerArg>())))
        );
    }
    if ASANENABLED {
        asanwrite(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&arg) as usize))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<cgoSymbolizerArg>())))
        );
    }
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> = {
            let mut __f_guard = call.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            cgoSymbolizer.clone(),
            noescape(Arc::new(Mutex::new(Some(Arc::as_ptr(&arg) as usize))))
        )
    };
}

/// cgoContextPCs gets the PC values from a cgo traceback.
pub fn cgo_context_p_cs(ctxt: Arc<Mutex<Option<usize>>>, buf_local: Arc<Mutex<Option<Vec<usize>>>>) {
    if { let __nil_result = (*cgoTraceback.lock().unwrap()).is_none(); __nil_result } {
        return;
    }
    let mut call = Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> i32 { cgocall(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>)));
    if {
        let __go_cond_0 = { let __tmp_x = (*panicking.lock().unwrap().as_mut().unwrap()).load(); let __tmp_y = 0 as u32; __tmp_x > __tmp_y };
        if __go_cond_0 {
            true
        } else {
            let __go_cond_1 = { let __left_addr = (*(*getg().lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_ref().unwrap()).curg.addr(); let __right_addr = { let __ptr = GoPtr::local(getg()); __ptr.addr() }; let __eq = __left_addr == __right_addr; !__eq };
            __go_cond_1
        }
    } {
                // We do not want to call into the scheduler when panicking
                // or when on the system stack.
        { let new_val = Box::new(move |__arg0: Arc<Mutex<Option<usize>>>, __arg1: Arc<Mutex<Option<usize>>>| -> i32 { asmcgocall(__arg0, __arg1) }) as Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>; *call.lock().unwrap() = Some(new_val); };
    }
        // We do not want to call into the scheduler when panicking
        // or when on the system stack.
    let mut arg = Arc::new(Mutex::new(Some(cgoTracebackArg {
        context: Arc::new(Mutex::new(Some({ let __arg_holder = ctxt.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        buf: Arc::new(Mutex::new({ let __ptr = noescape(Arc::new(Mutex::new(Some({ let __seq_holder = buf_local.clone(); let __seq_guard = __seq_holder.lock().unwrap(); &__seq_guard.as_ref().unwrap()[(0) as usize] as *const _ as usize })))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<usize>(unimplemented!("unsafe.Pointer conversion to usize")) } })).clone(),
        max: Arc::new(Mutex::new(Some((*buf_local.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as usize))),
        ..Default::default()
    })));
    if MSANENABLED {
        msanwrite(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&arg.clone()) as usize))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<cgoTracebackArg>())))
        );
    }
    if ASANENABLED {
        asanwrite(
            Arc::new(Mutex::new(Some(Arc::as_ptr(&arg.clone()) as usize))),
            Arc::new(Mutex::new(Some(std::mem::size_of::<cgoTracebackArg>())))
        );
    }
    {
        let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync> = {
            let mut __f_guard = call.lock().unwrap();
            __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<usize>>>, Arc<Mutex<Option<usize>>>) -> i32 + Send + Sync>
        };
        let __f = unsafe { &mut *__f_ptr };
        (*__f)(
            cgoTraceback.clone(),
            noescape(Arc::new(Mutex::new(Some(Arc::as_ptr(&arg.clone()) as usize))))
        )
    };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for unwinder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cgoTracebackArg {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for cgoSymbolizerArg {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
