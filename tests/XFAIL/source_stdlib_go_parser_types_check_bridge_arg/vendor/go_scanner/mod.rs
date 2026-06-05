use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::errors::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const BOM: i32 = 0xFEFF;
pub(crate) const EOF: i32 = -1;


pub const SCAN_COMMENTS: u64 = 1 << 0;
pub(crate) const DONT_INSERT_SEMIS: u64 = 1 << 1;


/// An ErrorHandler may be provided to [Scanner.Init]. If a syntax error is
/// encountered and a handler was installed, the handler is called with a
/// position and an error message. The position points to the beginning of
/// the offending token.
pub type ErrorHandler = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Position>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync>>>>;


/// A Scanner holds the scanner's internal state while processing
/// a given text. It can be allocated as part of another data
/// structure but must be initialized via [Scanner.Init] before use.
#[derive(Clone)]
pub struct Scanner {
    pub file: Arc<Mutex<Option<go_token::position::File>>>,
    pub dir: Arc<Mutex<Option<String>>>,
    pub src: Arc<Mutex<Option<Vec<u8>>>>,
    pub err: ErrorHandler,
    pub mode: Arc<Mutex<Option<Mode>>>,
    pub ch: Arc<Mutex<Option<i32>>>,
    pub offset: Arc<Mutex<Option<i32>>>,
    pub rd_offset: Arc<Mutex<Option<i32>>>,
    pub line_offset: Arc<Mutex<Option<i32>>>,
    pub insert_semi: Arc<Mutex<Option<bool>>>,
    pub nl_pos: Arc<Mutex<Option<go_token::position::Pos>>>,
    pub error_count: Arc<Mutex<Option<i32>>>,
}

impl Scanner {
    pub fn __go_value_clone(&self) -> Self {
        Self { file: self.file.clone(), dir: { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, src: self.src.clone(), err: self.err.clone(), mode: { let __guard = self.mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ch: { let __guard = self.ch.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, offset: { let __guard = self.offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, rd_offset: { let __guard = self.rd_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, line_offset: { let __guard = self.line_offset.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, insert_semi: { let __guard = self.insert_semi.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nl_pos: { let __guard = self.nl_pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, error_count: { let __guard = self.error_count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Scanner {
    fn default() -> Self {
        Self { file: Arc::new(Mutex::new(None)), dir: Arc::new(Mutex::new(Some(String::new()))), src: Arc::new(Mutex::new(None)), err: Arc::new(Mutex::new(None)), mode: Arc::new(Mutex::new(Some(Mode(Arc::new(Mutex::new(Some(0))))))), ch: Arc::new(Mutex::new(Some(0))), offset: Arc::new(Mutex::new(Some(0))), rd_offset: Arc::new(Mutex::new(Some(0))), line_offset: Arc::new(Mutex::new(Some(0))), insert_semi: Arc::new(Mutex::new(Some(false))), nl_pos: Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(0))))))), error_count: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Scanner {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {} {} {} {} {}}}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, (*self.dir.lock().unwrap().as_ref().unwrap()), format_slice(&self.src), "<func>", (*self.mode.lock().unwrap().as_ref().unwrap()), (*self.ch.lock().unwrap().as_ref().unwrap()), (*self.offset.lock().unwrap().as_ref().unwrap()), (*self.rd_offset.lock().unwrap().as_ref().unwrap()), (*self.line_offset.lock().unwrap().as_ref().unwrap()), (*self.insert_semi.lock().unwrap().as_ref().unwrap()), (*self.nl_pos.lock().unwrap().as_ref().unwrap()), (*self.error_count.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Scanner {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("ErrorCount") {
            out.error_count = <Arc<Mutex<Option<i32>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


/// A mode value is a set of flags (or 0).
/// They control scanner behavior.
#[derive(Debug, Clone, Default)]
pub struct Mode(pub Arc<Mutex<Option<u64>>>);

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u64> for Mode {
    fn eq(&self, other: &u64) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for Mode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u64> for Mode {
    fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<Mode> for u64 {
    fn eq(&self, other: &Mode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<Mode> for u64 {
    fn partial_cmp(&self, other: &Mode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for Mode {
    type Output = Mode;
    fn add(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u64> for Mode {
    type Output = Mode;
    fn add(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<Mode> for u64 {
    type Output = Mode;
    fn add(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for Mode {
    type Output = Mode;
    fn sub(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u64> for Mode {
    type Output = Mode;
    fn sub(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<Mode> for u64 {
    type Output = Mode;
    fn sub(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for Mode {
    type Output = Mode;
    fn mul(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u64> for Mode {
    type Output = Mode;
    fn mul(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<Mode> for u64 {
    type Output = Mode;
    fn mul(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for Mode {
    type Output = Mode;
    fn div(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u64> for Mode {
    type Output = Mode;
    fn div(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<Mode> for u64 {
    type Output = Mode;
    fn div(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for Mode {
    type Output = Mode;
    fn rem(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u64> for Mode {
    type Output = Mode;
    fn rem(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<Mode> for u64 {
    type Output = Mode;
    fn rem(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for Mode {
    type Output = Mode;
    fn bitand(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u64> for Mode {
    type Output = Mode;
    fn bitand(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<Mode> for u64 {
    type Output = Mode;
    fn bitand(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for Mode {
    type Output = Mode;
    fn bitor(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u64> for Mode {
    type Output = Mode;
    fn bitor(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<Mode> for u64 {
    type Output = Mode;
    fn bitor(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for Mode {
    type Output = Mode;
    fn bitxor(self, other: Self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u64> for Mode {
    type Output = Mode;
    fn bitxor(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<Mode> for u64 {
    type Output = Mode;
    fn bitxor(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for Mode {
    type Output = Mode;
    fn not(self) -> Mode {
        Mode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for Mode {
    type Output = Mode;
    fn shl(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for Mode {
    type Output = Mode;
    fn shl(self, other: i32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for Mode {
    type Output = Mode;
    fn shl(self, other: i8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for Mode {
    type Output = Mode;
    fn shl(self, other: i16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for Mode {
    type Output = Mode;
    fn shl(self, other: i64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for Mode {
    type Output = Mode;
    fn shl(self, other: u32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for Mode {
    type Output = Mode;
    fn shl(self, other: u8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for Mode {
    type Output = Mode;
    fn shl(self, other: u16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for Mode {
    type Output = Mode;
    fn shl(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for Mode {
    type Output = Mode;
    fn shl(self, other: usize) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for Mode {
    type Output = Mode;
    fn shr(self, other: Mode) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for Mode {
    type Output = Mode;
    fn shr(self, other: i32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for Mode {
    type Output = Mode;
    fn shr(self, other: i8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for Mode {
    type Output = Mode;
    fn shr(self, other: i16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for Mode {
    type Output = Mode;
    fn shr(self, other: i64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for Mode {
    type Output = Mode;
    fn shr(self, other: u32) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for Mode {
    type Output = Mode;
    fn shr(self, other: u8) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for Mode {
    type Output = Mode;
    fn shr(self, other: u16) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for Mode {
    type Output = Mode;
    fn shr(self, other: u64) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for Mode {
    type Output = Mode;
    fn shr(self, other: usize) -> Mode {
        Mode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for Mode {}

impl Ord for Mode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


pub(crate) static prefix: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<u8>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *prefix.lock().unwrap() = Some(vec![]);
    *prefix.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("line ".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *prefix.lock().unwrap() = Some(vec![]);
}


pub(crate) fn __go_init_order_0() {
    *prefix.lock().unwrap() = Some((*Arc::new(Mutex::new(Some(("line ".to_string()).as_bytes().to_vec()))).lock().unwrap().as_ref().unwrap()).clone());
}


impl Scanner {
    /// Read the next Unicode char into s.ch.
    /// s.ch < 0 means end-of-file.
    ///
    /// For optimization, there is some overlap between this method and
    /// s.scanIdentifier.
    pub fn next(&mut self) {
        if { let __tmp_x = ((*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = self.src.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        { let new_val = { let __selector_holder = self.rd_offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.offset.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.line_offset.lock().unwrap() = Some(new_val); };
        (*self.file.lock().unwrap().as_mut().unwrap()).add_line(Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        let (mut r, mut w) = (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() } as i32))), Arc::new(Mutex::new(Some(1))));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x == __tmp_y } {
            { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("illegal character NUL".to_string()))); self.error(__method_arg0, __method_arg1) };
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
                        // not ASCII
            { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = (*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[__low..].to_vec() })))); *r.lock().unwrap() = Some(__tmp_0); *w.lock().unwrap() = Some(__tmp_1); };
            if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("illegal UTF-8 encoding".to_string()))); self.error(__method_arg0, __method_arg1) };
    } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = BOM as i32; __tmp_x == __tmp_y } && { let __tmp_x = (*self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("illegal byte order mark".to_string()))); self.error(__method_arg0, __method_arg1) };
    }
        }
                // not ASCII
        { let __target = self.rd_offset.clone(); let __rhs = (*w.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = r.lock().unwrap().as_ref().unwrap().clone(); *self.ch.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = ({ let __len_target = { let __field = self.src.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *self.offset.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.line_offset.lock().unwrap() = Some(new_val); };
        (*self.file.lock().unwrap().as_mut().unwrap()).add_line(Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    }
        { let new_val = EOF as i32; *self.ch.lock().unwrap() = Some(new_val); };
    }
    }

    /// peek returns the byte following the most recently read character without
    /// advancing the scanner. If the scanner is at EOF, peek returns 0.
    pub fn peek(&self) -> u8 {
        if { let __tmp_x = ((*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = (({ let __len_target = { let __field = self.src.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x < __tmp_y } {
        return { let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() };
    }
        0
    }

    /// Init prepares the scanner s to tokenize the text src by setting the
    /// scanner at the beginning of src. The scanner uses the file set file
    /// for position information and it adds line information for each line.
    /// It is ok to re-use the same file when re-scanning the same file as
    /// line information which is already present is ignored. Init causes a
    /// panic if the file size does not match the src size.
    ///
    /// Calls to [Scanner.Scan] will invoke the error handler err if they encounter a
    /// syntax error and err is not nil. Also, for each error encountered,
    /// the [Scanner] field ErrorCount is incremented by one. The mode parameter
    /// determines how comments are handled.
    ///
    /// Note that Init may call err if there is an error in the first character
    /// of the file.
    pub fn init(&mut self, file: Arc<Mutex<Option<go_token::position::File>>>, src: Arc<Mutex<Option<Vec<u8>>>>, err: ErrorHandler, mode: Arc<Mutex<Option<Mode>>>) {
                // Explicitly initialize all fields since a scanner may be reused.
        if { let __tmp_x = ({ let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.size(); __result } as i32); let __tmp_y = ((*src.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        panic!("file size ({}) does not match src len ({})", { let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.size(); __result }, (*src.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0));
    }
        { let new_val = file.clone(); self.file = new_val; };
        { let (__tmp_0, __tmp_1) = path_filepath::split({ let __recv = file.clone(); let __recv_ptr: *const go_token::position::File = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_token::position::File }; let __result = unsafe { &*__recv_ptr }.name(); __result }); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *self.dir.lock().unwrap() = __moved_tmp_0; };
        { let new_val = src.clone(); self.src = new_val; };
        { let new_val = err.clone(); self.err = new_val; };
        { let new_val = mode.lock().unwrap().as_ref().unwrap().clone(); *self.mode.lock().unwrap() = Some(new_val); };
        { let new_val = (' ' as i32); *self.ch.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *self.offset.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *self.rd_offset.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *self.line_offset.lock().unwrap() = Some(new_val); };
        { let new_val = false; *self.insert_semi.lock().unwrap() = Some(new_val); };
        { let new_val = 0; *self.error_count.lock().unwrap() = Some(new_val); };
        self.next();
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = BOM as i32; __tmp_x == __tmp_y } {
        self.next();
    }
    }

    pub fn error(&mut self, offs: Arc<Mutex<Option<i32>>>, msg: Arc<Mutex<Option<String>>>) {
        if { let __nil_target = self.err.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let __f_holder = self.err.clone(); let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Position>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> = { let mut __f_guard = __f_holder.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<go_token::position::Position>>>, Arc<Mutex<Option<String>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)((*self.file.lock().unwrap().as_ref().unwrap()).position((*self.file.lock().unwrap().as_ref().unwrap()).pos(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) };
    }
        { let __target = self.error_count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn errorf(&mut self, offs: Arc<Mutex<Option<i32>>>, format: Arc<Mutex<Option<String>>>, args: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(format!("{}", (*format.lock().unwrap().as_ref().unwrap()).clone())))));
    }

    /// scanComment returns the text of the comment and (if nonzero)
    /// the offset of the first newline within it, which implies a
    /// /*...*/ comment.
    pub fn scan_comment(&mut self) -> (Arc<Mutex<Option<String>>>, i32) {
                // initial '/' already consumed; s.ch == '/' || s.ch == '*'
        let mut offs = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut next = Arc::new(Mutex::new(Some(-(1))));
        let mut numCR = Arc::new(Mutex::new(Some(0)));
        let mut nlOffset = Arc::new(Mutex::new(Some(0)));

        'exit: {
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('/' as i32); __tmp_x == __tmp_y } {
                //-style comment
                // (the final '\n' is not considered part of the comment)
        self.next();
        while { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\n' as i32); __tmp_x != __tmp_y } && { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\r' as i32); __tmp_x == __tmp_y } {
        { let mut guard = numCR.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        self.next();
    }
                // if we are at '\n', the position following the comment is afterwards
        { let new_val = { let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *next.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } {
        { let mut guard = next.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        break 'exit;
    }

                        //-style comment
                        // (the final '\n' is not considered part of the comment)
                        // if we are at '\n', the position following the comment is afterwards
                        /*-style comment */
            self.next();
            while { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        let mut ch = Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\r' as i32); __tmp_x == __tmp_y } {
        { let mut guard = numCR.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*nlOffset.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = { let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *nlOffset.lock().unwrap() = Some(new_val); };
    }
        self.next();
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('*' as i32); __tmp_x == __tmp_y } && { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('/' as i32); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = { let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *next.lock().unwrap() = Some(new_val); };
        break 'exit;
    }
    }

            self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("comment not terminated".to_string()))));

        }
        let mut lit = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() })));

                // On Windows, a (//-comment) line may end in "\r\n".
                // Remove the final '\r' before analyzing the text for
                // line directives (matching the compiler). Remove any
                // other '\r' afterwards (matching the pre-existing be-
                // havior of the scanner).
        if { let __tmp_x = { let __v = (*numCR.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = ((*lit.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x >= __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = ((*lit.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('\r' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = ((*lit.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))); lit = new_val; };
        { let mut guard = numCR.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

                // interpret line directives
                // (//line directives must start at the beginning of the current line)
        if { let __tmp_x = { let __v = (*next.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } && ({ let __tmp_x = { let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*self.line_offset.lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y }) && bytes::has_prefix(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(2) as usize..].to_vec() }))), prefix.clone()) {
        self.update_line_info(Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), lit.clone());
    }

        if { let __tmp_x = { let __v = (*numCR.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = strip_c_r(lit.clone(), Arc::new(Mutex::new(Some({ let __tmp_x = { let __seq = { let __seq_holder = lit.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y })))); lit = new_val; };
    }

        return (Arc::new(Mutex::new(Some(String::from_utf8((*lit.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), { let __v = (*nlOffset.lock().unwrap().as_ref().unwrap()).clone(); __v });
        unreachable!()
    }

    /// updateLineInfo parses the incoming comment text at offset offs
    /// as a line directive. If successful, it updates the line info table
    /// for the position next per the line directive.
    pub fn update_line_info(&mut self, next: Arc<Mutex<Option<i32>>>, mut offs: Arc<Mutex<Option<i32>>>, mut text: Arc<Mutex<Option<Vec<u8>>>>) {
                // extract comment text
        if { let __tmp_x = { let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = ((*text.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 2; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))); text = new_val; };
    }
                // lop off trailing "*/"
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(7) as usize..].to_vec() }))); text = new_val; };
        { let __rhs = 7; let mut guard = offs.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        let (mut i, mut n, mut ok) = trailing_digits(text.clone());
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return;
    }
                // ignore (not a line directive)
                // i > 0
        if !ok {
                // text has a suffix :xxx but xxx is not a number
        self.error(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(format!("{}{}", "invalid line number: ".to_string(), (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()))))));
        return;
    }
                // text has a suffix :xxx but xxx is not a number
                // Put a cap on the maximum size of line and column numbers.
                // 30 bits allows for some additional space before wrapping an int32.
                // Keep this consistent with cmd/compile/internal/syntax.PosMax.
        const maxLineCol: i32 = 1 << 30;

        let mut line: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut col: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let (mut i2, mut n2, mut ok2) = trailing_digits(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))));
        if ok2 {
                //line filename:line:col
        { let __tmp_0 = i2; let __tmp_1 = i; i = __tmp_0; i2 = __tmp_1; };
        { let __tmp_0 = n2; let __tmp_1 = n; *line.lock().unwrap() = Some(__tmp_0); *col.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*col.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*col.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i2; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(format!("{}{}", "invalid column number: ".to_string(), (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i2) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()))))));
        return;
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = i2; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))); text = new_val; };
    } else {
                //line filename:line
        { let new_val = n; *line.lock().unwrap() = Some(new_val); };
    }
                //line filename:line:col
                // lop off ":col"
                //line filename:line
        if { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*line.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1073741824; __tmp_x > __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some(format!("{}{}", "invalid line number: ".to_string(), (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()))))));
        return;
    }
                // If we have a column (//line filename:line:col form),
                // an empty filename means to use the previous filename.
        let mut filename = Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __seq[..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
        if { let __tmp_x = (*filename.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } && ok2 {
        { let new_val = { let __selector_holder = (*(*self.file.lock().unwrap().as_ref().unwrap()).position((*self.file.lock().unwrap().as_ref().unwrap()).pos(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))).lock().unwrap().as_ref().unwrap()).filename.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *filename.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*filename.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
        { let new_val = path_filepath::clean(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *filename.lock().unwrap() = __moved_val; };
        if !path_filepath::is_abs(Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = path_filepath::join(Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }, (*filename.lock().unwrap().as_ref().unwrap()).clone()])))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *filename.lock().unwrap() = __moved_val; };
    }
    }
                // Put a relative filename in the current directory.
                // This is for compatibility with earlier releases.
                // See issue 26671.
        (*self.file.lock().unwrap().as_mut().unwrap()).add_line_column_info(Arc::new(Mutex::new(Some({ let __arg_holder = next.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = filename.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = line.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = col.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    /// scanIdentifier reads the string of valid identifier characters at s.offset.
    /// It must only be called when s.ch is known to be a valid letter.
    ///
    /// Be careful when making changes to this function: it is optimized and affects
    /// scanning performance significantly.
    pub fn scan_identifier(&mut self) -> Arc<Mutex<Option<String>>> {
        let mut offs = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));

        'exit: {
                        // Optimize for the common case of an ASCII identifier.
                        //
                        // Ranging over s.src[s.rdOffset:] lets us avoid some bounds checks, and
                        // avoids conversions to runes.
                        //
                        // In case we encounter a non-ASCII character, fall back on the slower path
                        // of calling into s.next().
            for (rdOffset, b) in { let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = (*self.rd_offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[__low..].to_vec() }.iter().copied().enumerate() {
        if { let __tmp_x = ('a' as i32) as u8; let __tmp_y = b; __tmp_x <= __tmp_y } && { let __tmp_x = b; let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = ('A' as i32) as u8; let __tmp_y = b; __tmp_x <= __tmp_y } && { let __tmp_x = b; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } || { let __tmp_x = b; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = ('0' as i32) as u8; let __tmp_y = b; __tmp_x <= __tmp_y } && { let __tmp_x = b; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
                // Avoid assigning a rune for the common case of an ascii character.
        continue
    }
                // Avoid assigning a rune for the common case of an ascii character.
        { let __target = self.rd_offset.clone(); let __rhs = (rdOffset as i32); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = 0 as u8; let __tmp_y = b; __tmp_x < __tmp_y } && { let __tmp_x = b; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } {
                // Optimization: we've encountered an ASCII character that's not a letter
                // or number. Avoid the call into s.next() and corresponding set up.
                //
                // Note that s.next() does some line accounting if s.ch is '\n', so this
                // shortcut is only possible because we know that the preceding character
                // is not '\n'.
        { let new_val = Arc::new(Mutex::new(Some(b as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.ch.lock().unwrap() = __moved_val; };
        { let new_val = { let __selector_holder = self.rd_offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *self.offset.lock().unwrap() = Some(new_val); };
        { let __target = self.rd_offset.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        break 'exit;
    }
                // Optimization: we've encountered an ASCII character that's not a letter
                // or number. Avoid the call into s.next() and corresponding set up.
                //
                // Note that s.next() does some line accounting if s.ch is '\n', so this
                // shortcut is only possible because we know that the preceding character
                // is not '\n'.
                // We know that the preceding character is valid for an identifier because
                // scanIdentifier is only called when s.ch is a letter, so calling s.next()
                // at s.rdOffset resets the scanner state.
        self.next();
        while is_letter(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) || is_digit(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) {
        self.next();
    }
        break 'exit;
    }
                        // Avoid assigning a rune for the common case of an ascii character.
                        // Optimization: we've encountered an ASCII character that's not a letter
                        // or number. Avoid the call into s.next() and corresponding set up.
                        //
                        // Note that s.next() does some line accounting if s.ch is '\n', so this
                        // shortcut is only possible because we know that the preceding character
                        // is not '\n'.
                        // We know that the preceding character is valid for an identifier because
                        // scanIdentifier is only called when s.ch is a letter, so calling s.next()
                        // at s.rdOffset resets the scanner state.
            { let new_val = ({ let __len_target = { let __field = self.src.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *self.offset.lock().unwrap() = Some(new_val); };
            { let new_val = ({ let __len_target = { let __field = self.src.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32; *self.rd_offset.lock().unwrap() = Some(new_val); };
            { let new_val = EOF as i32; *self.ch.lock().unwrap() = Some(new_val); };

        }
        return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
        unreachable!()
    }

    /// digits accepts the sequence { digit | '_' }.
    /// If base <= 10, digits accepts any decimal digit but records
    /// the offset (relative to the source start) of a digit >= base
    /// in *invalid, if *invalid < 0.
    /// digits returns a bitset describing whether the sequence contained
    /// digits (bit 0 is set), or separators '_' (bit 1 is set).
    pub fn digits(&mut self, base: Arc<Mutex<Option<i32>>>, invalid: Arc<Mutex<Option<i32>>>) -> i32 {
    let mut digsep: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));

        if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x <= __tmp_y } {
        let mut max = Arc::new(Mutex::new(Some(({ let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as i32)));
        while is_decimal(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        let mut ds = Arc::new(Mutex::new(Some(1)));
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        { let new_val = 2; *ds.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __v = self.offset.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }; *invalid.lock().unwrap() = Some(new_val); };
    }
                // record invalid rune offset
        { let __rhs = (*ds.lock().unwrap().as_ref().unwrap()); let mut guard = digsep.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        self.next();
    }
    } else {
        while is_hex(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        let mut ds = Arc::new(Mutex::new(Some(1)));
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        { let new_val = 2; *ds.lock().unwrap() = Some(new_val); };
    }
        { let __rhs = (*ds.lock().unwrap().as_ref().unwrap()); let mut guard = digsep.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        self.next();
    }
    }
                // record invalid rune offset
        return (*digsep.lock().unwrap().as_ref().unwrap());
    }

    pub fn scan_number(&mut self) -> (Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<String>>>) {
        let mut offs = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut tok = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_L_L_E_G_A_L as i32)))))));
        let mut base = Arc::new(Mutex::new(Some(10)));
        let mut prefix_local = Arc::new(Mutex::new(Some(0 as i32)));
        let mut digsep = Arc::new(Mutex::new(Some(0)));
        let mut invalid = Arc::new(Mutex::new(Some(-(1))));
                // integer part
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('.' as i32); __tmp_x != __tmp_y } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32)))); *tok.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32); __tmp_x == __tmp_y } {
        self.next();
        { let _switch_val = lower(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
    if _switch_val == (('x' as i32)) {
            self.next();
            { let __tmp_0 = 16; let __tmp_1 = ('x' as i32); *base.lock().unwrap() = Some(__tmp_0); *prefix_local.lock().unwrap() = Some(__tmp_1); };
        } else if _switch_val == (('o' as i32)) {
            self.next();
            { let __tmp_0 = 8; let __tmp_1 = ('o' as i32); *base.lock().unwrap() = Some(__tmp_0); *prefix_local.lock().unwrap() = Some(__tmp_1); };
        } else if _switch_val == (('b' as i32)) {
            self.next();
            { let __tmp_0 = 2; let __tmp_1 = ('b' as i32); *base.lock().unwrap() = Some(__tmp_0); *prefix_local.lock().unwrap() = Some(__tmp_1); };
        } else {
            { let __tmp_0 = 8; let __tmp_1 = ('0' as i32); *base.lock().unwrap() = Some(__tmp_0); *prefix_local.lock().unwrap() = Some(__tmp_1); };
            { let new_val = 1; *digsep.lock().unwrap() = Some(new_val); };
        }
    }
    }
                // leading 0
        { let __rhs = self.digits(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), invalid.clone()); let mut guard = digsep.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
                // leading 0
                // fractional part
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('.' as i32); __tmp_x == __tmp_y } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32)))); *tok.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('o' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('b' as i32); __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some(format!("{}{}", "invalid radix point in ".to_string(), (*litname(Arc::new(Mutex::new(Some({ let __arg_holder = prefix_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()))))); self.error(__method_arg0, __method_arg1) };
    }
        self.next();
        { let __rhs = self.digits(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), invalid.clone()); let mut guard = digsep.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*digsep.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some(format!("{}{}", (*litname(Arc::new(Mutex::new(Some({ let __arg_holder = prefix_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()), " has no digits".to_string())))); self.error(__method_arg0, __method_arg1) };
    }
                // exponent
        {
        let mut e = lower(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = e; let __tmp_y = ('e' as i32); __tmp_x == __tmp_y } || { let __tmp_x = e; let __tmp_y = ('p' as i32); __tmp_x == __tmp_y } {
            if { let __tmp_x = e; let __tmp_y = ('e' as i32); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x != __tmp_y } && { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
            { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("%q exponent requires decimal mantissa".to_string()))); self.errorf(__method_arg0, __method_arg1, Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) };
        } else if { let __tmp_x = e; let __tmp_y = ('p' as i32); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32); __tmp_x != __tmp_y } {
            { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("%q exponent requires hexadecimal mantissa".to_string()))); self.errorf(__method_arg0, __method_arg1, Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))) };
        };
            self.next();;
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32)))); *tok.lock().unwrap() = Some(new_val); };;
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('+' as i32); __tmp_x == __tmp_y } || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('-' as i32); __tmp_x == __tmp_y } {
        self.next();
    };
            let mut ds = self.digits(Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(None)));;
            { let __rhs = ds; let mut guard = digsep.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };;
            if { let __tmp_x = { let __tmp_x = ds; let __tmp_y = 1; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("exponent has no digits".to_string()))); self.error(__method_arg0, __method_arg1) };
    };
        } else if { let __tmp_x = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32); __tmp_x == __tmp_y } && { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_L_O_A_T as i32)))); __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some("hexadecimal mantissa requires a 'p' exponent".to_string()))); self.error(__method_arg0, __method_arg1) };
    }
    }
                // suffix 'i'
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('i' as i32); __tmp_x == __tmp_y } {
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_M_A_G as i32)))); *tok.lock().unwrap() = Some(new_val); };
        self.next();
    }
        let mut lit = Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
        if { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i32)))); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        self.errorf(Arc::new(Mutex::new(Some({ let __arg_holder = invalid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("invalid digit %q in %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __s = &((*lit.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*invalid.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] }) as Box<dyn Any + Send + Sync>, Box::new({ let __v = litname(Arc::new(Mutex::new(Some({ let __arg_holder = prefix_local.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any + Send + Sync>]))));
    }
        if { let __tmp_x = { let __tmp_x = { let __v = (*digsep.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } {
        {
        let mut i = invalid_sep(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            self.error(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i; __tmp_x + __tmp_y }))), Arc::new(Mutex::new(Some("'_' must separate successive digits".to_string()))));;
        }
    }
    }
        return ({ let __owned = tok.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __owned = lit.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    }

    /// scanEscape parses an escape sequence where rune is the accepted
    /// escaped quote. In case of a syntax error, it stops at the offending
    /// character (without consuming it) and returns false. Otherwise
    /// it returns true.
    pub fn scan_escape(&mut self, quote: Arc<Mutex<Option<i32>>>) -> bool {
        let mut offs = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        let mut base: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));let mut max: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = { let __v = self.ch.clone(); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned };
    if _switch_val == (('a' as i32)) || _switch_val == (('b' as i32)) || _switch_val == (('f' as i32)) || _switch_val == (('n' as i32)) || _switch_val == (('r' as i32)) || _switch_val == (('t' as i32)) || _switch_val == (('v' as i32)) || _switch_val == (('\\' as i32)) || _switch_val == ({ let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }) {
            self.next();
            return true;
        } else if _switch_val == (('0' as i32)) || _switch_val == (('1' as i32)) || _switch_val == (('2' as i32)) || _switch_val == (('3' as i32)) || _switch_val == (('4' as i32)) || _switch_val == (('5' as i32)) || _switch_val == (('6' as i32)) || _switch_val == (('7' as i32)) {
            { let __tmp_0 = 3; let __tmp_1 = 8; let __tmp_2 = 255; *n.lock().unwrap() = Some(__tmp_0); *base.lock().unwrap() = Some(__tmp_1 as u32); *max.lock().unwrap() = Some(__tmp_2 as u32); };
        } else if _switch_val == (('x' as i32)) {
            self.next();
            { let __tmp_0 = 2; let __tmp_1 = 16; let __tmp_2 = 255; *n.lock().unwrap() = Some(__tmp_0); *base.lock().unwrap() = Some(__tmp_1 as u32); *max.lock().unwrap() = Some(__tmp_2 as u32); };
        } else if _switch_val == (('u' as i32)) {
            self.next();
            { let __tmp_0 = 4; let __tmp_1 = 16; let __tmp_2 = unicode::MAX_RUNE; *n.lock().unwrap() = Some(__tmp_0); *base.lock().unwrap() = Some(__tmp_1 as u32); *max.lock().unwrap() = Some(__tmp_2 as u32); };
        } else if _switch_val == (('U' as i32)) {
            self.next();
            { let __tmp_0 = 8; let __tmp_1 = 16; let __tmp_2 = unicode::MAX_RUNE; *n.lock().unwrap() = Some(__tmp_0); *base.lock().unwrap() = Some(__tmp_1 as u32); *max.lock().unwrap() = Some(__tmp_2 as u32); };
        } else {
            let mut msg = Arc::new(Mutex::new(Some("unknown escape sequence".to_string())));
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let new_val = "escape sequence not terminated".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
            self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            return false;
        }
    }
        let mut x: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
        while { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut d = Arc::new(Mutex::new(Some(digit_val(Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))) as u32)));
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        let mut msg = Arc::new(Mutex::new(Some(format!("illegal character U+{:04X} in escape sequence", (*self.ch.lock().unwrap().as_ref().unwrap()) as u32))));
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        { let new_val = "escape sequence not terminated".to_string(); *msg.lock().unwrap() = Some(new_val); };
    }
        { let __method_arg0 = Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))); let __method_arg1 = Arc::new(Mutex::new(Some({ let __arg_holder = msg.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); self.error(__method_arg0, __method_arg1) };
        return false;
    }
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }; let __tmp_y = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *x.lock().unwrap() = Some(new_val); };
        self.next();
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*max.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } || { let __tmp_x = 0xD800 as u32; let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xE000 as u32; __tmp_x < __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("escape sequence is invalid Unicode code point".to_string()))));
        return false;
    }
        true
    }

    pub fn scan_rune(&mut self) -> Arc<Mutex<Option<String>>> {
                // '\'' opening already consumed
        let mut offs = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut valid = Arc::new(Mutex::new(Some(true)));
        let mut n = Arc::new(Mutex::new(Some(0)));
        loop {
        let mut ch = Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
                // only report error if we don't have one already
        if { let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("rune literal not terminated".to_string()))));
        { let new_val = false; *valid.lock().unwrap() = Some(new_val); };
    }
        break
    }
                // only report error if we don't have one already
        self.next();
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\'' as i32); __tmp_x == __tmp_y } {
        break
    }
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32); __tmp_x == __tmp_y } {
        if !self.scan_escape(Arc::new(Mutex::new(Some(('\'' as i32))))) {
        { let new_val = false; *valid.lock().unwrap() = Some(new_val); };
    }
    }
    }
                // only report error if we don't have one already
                // continue to read to closing quote
        if { let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("illegal rune literal".to_string()))));
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    pub fn scan_string(&mut self) -> Arc<Mutex<Option<String>>> {
                // '"' opening already consumed
        let mut offs = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));
        loop {
        let mut ch = Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("string literal not terminated".to_string()))));
        break
    }
        self.next();
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('"' as i32); __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32); __tmp_x == __tmp_y } {
        self.scan_escape(Arc::new(Mutex::new(Some(('"' as i32)))));
    }
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    pub fn scan_raw_string(&mut self) -> Arc<Mutex<Option<String>>> {
                // '`' opening already consumed
        let mut offs = Arc::new(Mutex::new(Some({ let __tmp_x = (*self.offset.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x - __tmp_y })));
        let mut hasCR = Arc::new(Mutex::new(Some(false)));
        loop {
        let mut ch = Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } {
        self.error(Arc::new(Mutex::new(Some({ let __arg_holder = offs.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("raw string literal not terminated".to_string()))));
        break
    }
        self.next();
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('`' as i32); __tmp_x == __tmp_y } {
        break
    }
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\r' as i32); __tmp_x == __tmp_y } {
        { let new_val = true; *hasCR.lock().unwrap() = Some(new_val); };
    }
    }
        let mut lit = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.src.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.offset.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[({ let __v = (*offs.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..__high].to_vec() })));
        if { let __v = (*hasCR.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = strip_c_r(lit.clone(), Arc::new(Mutex::new(Some(false)))); lit = new_val; };
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*lit.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    pub fn skip_whitespace(&mut self) {
        while { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = (' ' as i32); __tmp_x == __tmp_y } || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\t' as i32); __tmp_x == __tmp_y } || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\n' as i32); __tmp_x == __tmp_y } && !(*self.insert_semi.clone().lock().unwrap().as_ref().unwrap()) || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('\r' as i32); __tmp_x == __tmp_y } {
        self.next();
    }
    }

    pub fn switch2(&mut self, tok0: Arc<Mutex<Option<go_token::r#mod::Token>>>, tok1: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::r#mod::Token>>> {
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('=' as i32); __tmp_x == __tmp_y } {
        self.next();
        return { let __owned = tok1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        return { let __owned = tok0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn switch3(&mut self, tok0: Arc<Mutex<Option<go_token::r#mod::Token>>>, tok1: Arc<Mutex<Option<go_token::r#mod::Token>>>, ch2: Arc<Mutex<Option<i32>>>, tok2: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::r#mod::Token>>> {
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('=' as i32); __tmp_x == __tmp_y } {
        self.next();
        return { let __owned = tok1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ch2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        self.next();
        return { let __owned = tok2.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        return { let __owned = tok0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    pub fn switch4(&mut self, tok0: Arc<Mutex<Option<go_token::r#mod::Token>>>, tok1: Arc<Mutex<Option<go_token::r#mod::Token>>>, ch2: Arc<Mutex<Option<i32>>>, tok2: Arc<Mutex<Option<go_token::r#mod::Token>>>, tok3: Arc<Mutex<Option<go_token::r#mod::Token>>>) -> Arc<Mutex<Option<go_token::r#mod::Token>>> {
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('=' as i32); __tmp_x == __tmp_y } {
        self.next();
        return { let __owned = tok1.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*ch2.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        self.next();
        if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('=' as i32); __tmp_x == __tmp_y } {
        self.next();
        return { let __owned = tok3.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        return { let __owned = tok2.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        return { let __owned = tok0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    /// Scan scans the next token and returns the token position, the token,
    /// and its literal string if applicable. The source end is indicated by
    /// [token.EOF].
    ///
    /// If the returned token is a literal ([token.IDENT], [token.INT], [token.FLOAT],
    /// [token.IMAG], [token.CHAR], [token.STRING]) or [token.COMMENT], the literal string
    /// has the corresponding value.
    ///
    /// If the returned token is a keyword, the literal string is the keyword.
    ///
    /// If the returned token is [token.SEMICOLON], the corresponding
    /// literal string is ";" if the semicolon was present in the source,
    /// and "\n" if the semicolon was inserted because of a newline or
    /// at EOF.
    ///
    /// If the returned token is [token.ILLEGAL], the literal string is the
    /// offending character.
    ///
    /// In all other cases, Scan returns an empty literal string.
    ///
    /// For more tolerant parsing, Scan will return a valid token if
    /// possible even if a syntax error was encountered. Thus, even
    /// if the resulting token sequence contains no illegal tokens,
    /// a client may not assume that no error occurred. Instead it
    /// must check the scanner's ErrorCount or the number of calls
    /// of the error handler, if there was one installed.
    ///
    /// Scan adds line information to the file added to the file
    /// set with Init. Token positions are relative to that file
    /// and thus relative to the file set.
    pub fn scan(&mut self) -> (Arc<Mutex<Option<go_token::position::Pos>>>, Arc<Mutex<Option<go_token::r#mod::Token>>>, Arc<Mutex<Option<String>>>) {
    let mut pos: Arc<Mutex<Option<go_token::position::Pos>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut tok: Arc<Mutex<Option<go_token::r#mod::Token>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut lit: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

        'scan_again: loop {
            if go_token::position::Pos::is_valid(&(*self.nl_pos.lock().unwrap().as_ref().unwrap())) {
                // Return artificial ';' token after /*...*/ comment
                // containing newline, at position of first newline.
        { let __tmp_0 = { let __selector_holder = self.nl_pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_1 = go_token::S_E_M_I_C_O_L_O_N; let __tmp_2 = "\n".to_string(); *pos.lock().unwrap() = Some(__tmp_0); *tok.lock().unwrap() = Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(__tmp_1 as i32))))); *lit.lock().unwrap() = Some(__tmp_2); };
        { let new_val = go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32)))); *self.nl_pos.lock().unwrap() = Some(new_val); };
        return (pos, tok, lit);
    }

                        // Return artificial ';' token after /*...*/ comment
                        // containing newline, at position of first newline.
            self.skip_whitespace();

                        // current token start
            { let new_val = (*self.file.lock().unwrap().as_ref().unwrap()).pos(Arc::new(Mutex::new(Some({ let __selector_holder = self.offset.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pos.lock().unwrap() = __moved_val; };

                        // determine token value
            let mut insertSemi = Arc::new(Mutex::new(Some(false)));
            let mut ch = Arc::new(Mutex::new(Some({ let __selector_holder = self.ch.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
    if is_letter(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let new_val = self.scan_identifier(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_val; };
            if { let __tmp_x = ((*lit.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x > __tmp_y } {
                // keywords are longer than one letter - avoid lookup otherwise
        { let new_val = go_token::lookup(Arc::new(Mutex::new(Some({ let __arg_holder = lit.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        { let _switch_val = (*tok.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::B_R_E_A_K as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_N_T_I_N_U_E as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::F_A_L_L_T_H_R_O_U_G_H as i32))))) || _switch_val == (go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_T_U_R_N as i32))))) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
        }
    }
    } else {
        { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_D_E_N_T as i32)))); *tok.lock().unwrap() = Some(new_val); };
    }
        } else if is_decimal(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('.' as i32); __tmp_x == __tmp_y } && is_decimal(Arc::new(Mutex::new(Some(self.peek() as i32)))) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let (__tmp_0, __tmp_1) = self.scan_number(); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_tmp_1; };
        } else {
            self.next();
            { let _switch_val = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (EOF as i32) {
            if (*self.insert_semi.clone().lock().unwrap().as_ref().unwrap()) {
        { let new_val = false; *self.insert_semi.lock().unwrap() = Some(new_val); };
        return ({ let __owned = pos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __return_value_1 = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))))); __return_value_1 }, Arc::new(Mutex::new(Some("\n".to_string()))));
    }
                        // EOF consumed
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_O_F as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('\n' as i32)) {
                        // we only reach here if s.insertSemi was
                        // set in the first place and exited early
                        // from s.skipWhitespace()
            { let new_val = false; *self.insert_semi.lock().unwrap() = Some(new_val); };
            return ({ let __owned = pos.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, { let __return_value_1 = Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32))))))); __return_value_1 }, Arc::new(Mutex::new(Some("\n".to_string()))));
        } else if _switch_val == (('"' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); *tok.lock().unwrap() = Some(new_val); };
            { let new_val = self.scan_string(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('\'' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_H_A_R as i32)))); *tok.lock().unwrap() = Some(new_val); };
            { let new_val = self.scan_rune(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('`' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_T_R_I_N_G as i32)))); *tok.lock().unwrap() = Some(new_val); };
            { let new_val = self.scan_raw_string(); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_val; };
        } else if _switch_val == ((':' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_L_O_N as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_F_I_N_E as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('.' as i32)) {
                        // fractions starting with a '.' are handled by outer switch
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::P_E_R_I_O_D as i32)))); *tok.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('.' as i32); __tmp_x == __tmp_y } && { let __tmp_x = self.peek(); let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } {
        self.next();
        self.next();
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_L_L_I_P_S_I_S as i32)))); *tok.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == ((',' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_A as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ((';' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_E_M_I_C_O_L_O_N as i32)))); *tok.lock().unwrap() = Some(new_val); };
            { let new_val = ";".to_string(); *lit.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('(' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_P_A_R_E_N as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ((')' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_P_A_R_E_N as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('[' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_K as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ((']' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_K as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('{' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_B_R_A_C_E as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('}' as i32)) {
            { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_B_R_A_C_E as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('+' as i32)) {
            { let new_val = self.switch3(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_D_D__A_S_S_I_G_N as i32))))))), Arc::new(Mutex::new(Some(('+' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_C as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
            if { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_C as i32)))); __tmp_x == __tmp_y } {
        { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (('-' as i32)) {
            { let new_val = self.switch3(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_U_B__A_S_S_I_G_N as i32))))))), Arc::new(Mutex::new(Some(('-' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_C as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
            if { let __tmp_x = (*tok.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::D_E_C as i32)))); __tmp_x == __tmp_y } {
        { let new_val = true; *insertSemi.lock().unwrap() = Some(new_val); };
    }
        } else if _switch_val == (('*' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::M_U_L__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('/' as i32)) {
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('/' as i32); __tmp_x == __tmp_y } || { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('*' as i32); __tmp_x == __tmp_y } {
                // comment
        let (mut comment, mut nlOffset) = self.scan_comment();
        if (*self.insert_semi.clone().lock().unwrap().as_ref().unwrap()) && { let __tmp_x = nlOffset; let __tmp_y = 0; __tmp_x != __tmp_y } {
                // For /*...*/ containing \n, return
                // COMMENT then artificial SEMICOLON.
        { let new_val = (*self.file.lock().unwrap().as_ref().unwrap()).pos(Arc::new(Mutex::new(Some(nlOffset)))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.nl_pos.lock().unwrap() = __moved_val; };
        { let new_val = false; *self.insert_semi.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = { let __selector_holder = self.insert_semi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *insertSemi.lock().unwrap() = Some(new_val); };
    }
                // For /*...*/ containing \n, return
                // COMMENT then artificial SEMICOLON.
                // preserve insertSemi info
        if { let __tmp_x = Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & SCAN_COMMENTS as u64))))); let __tmp_y = Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
                // skip comment
        continue 'scan_again;
    }
                // skip comment
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::C_O_M_M_E_N_T as i32)))); *tok.lock().unwrap() = Some(new_val); };
        { let new_val = comment.lock().unwrap().as_ref().unwrap().clone(); *lit.lock().unwrap() = Some(new_val); };
    } else {
                // division
        { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::Q_U_O__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
    }
        } else if _switch_val == (('%' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::R_E_M__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('^' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::X_O_R__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('<' as i32)) {
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('-' as i32); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_R_R_O_W as i32)))); *tok.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = self.switch4(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_S_S as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_E_Q as i32))))))), Arc::new(Mutex::new(Some(('<' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_L__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
    }
        } else if _switch_val == (('>' as i32)) {
            { let new_val = self.switch4(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_T_R as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::G_E_Q as i32))))))), Arc::new(Mutex::new(Some(('>' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::S_H_R__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('=' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_S_S_I_G_N as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::E_Q_L as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('!' as i32)) {
            { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_O_T as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::N_E_Q as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('&' as i32)) {
            if { let __tmp_x = (*self.ch.lock().unwrap().as_ref().unwrap()); let __tmp_y = ('^' as i32); __tmp_x == __tmp_y } {
        self.next();
        { let new_val = self.switch2(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__N_O_T__A_S_S_I_G_N as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
    } else {
        { let new_val = self.switch3(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::A_N_D__A_S_S_I_G_N as i32))))))), Arc::new(Mutex::new(Some(('&' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_A_N_D as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
    }
        } else if _switch_val == (('|' as i32)) {
            { let new_val = self.switch3(Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R as i32))))))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::O_R__A_S_S_I_G_N as i32))))))), Arc::new(Mutex::new(Some(('|' as i32)))), Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::L_O_R as i32)))))))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *tok.lock().unwrap() = __moved_val; };
        } else if _switch_val == (('~' as i32)) {
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::T_I_L_D_E as i32)))); *tok.lock().unwrap() = Some(new_val); };
        } else {
                        // next reports unexpected BOMs - don't repeat
            if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = BOM as i32; __tmp_x != __tmp_y } {
                // Report an informative error for U+201[CD] quotation
                // marks, which are easily introduced via copy and paste.
        if { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\u{201c}' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\u{201d}' as i32); __tmp_x == __tmp_y } {
        { let __method_arg0 = Arc::new(Mutex::new(Some((*self.file.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))); let __method_arg1 = Arc::new(Mutex::new(Some("curly quotation mark %q (use neutral %q)".to_string()))); self.errorf(__method_arg0, __method_arg1, Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>, Box::new(('"' as i32)) as Box<dyn Any + Send + Sync>])))) };
    } else {
        { let __method_arg0 = Arc::new(Mutex::new(Some((*self.file.lock().unwrap().as_ref().unwrap()).offset(Arc::new(Mutex::new(Some({ let __arg_holder = pos.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))); let __method_arg1 = Arc::new(Mutex::new(Some("illegal character %#U".to_string()))); self.errorf(__method_arg0, __method_arg1, Arc::new(Mutex::new(Some(vec![Box::new({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>])))) };
    }
    }
                        // Report an informative error for U+201[CD] quotation
                        // marks, which are easily introduced via copy and paste.
            { let new_val = { let __selector_holder = self.insert_semi.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *insertSemi.lock().unwrap() = Some(new_val); };
            { let new_val = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_L_L_E_G_A_L as i32)))); *tok.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some(char::from_u32(((*ch.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *lit.lock().unwrap() = __moved_val; };
        }
    }
        }
                        // keywords are longer than one letter - avoid lookup otherwise
                        // always make progress
                        // EOF consumed
                        // we only reach here if s.insertSemi was
                        // set in the first place and exited early
                        // from s.skipWhitespace()
                        // newline consumed
                        // fractions starting with a '.' are handled by outer switch
                        // consume last '.'
                        // comment
                        // For /*...*/ containing \n, return
                        // COMMENT then artificial SEMICOLON.
                        // preserve insertSemi info
                        // skip comment
                        // division
                        // next reports unexpected BOMs - don't repeat
                        // Report an informative error for U+201[CD] quotation
                        // marks, which are easily introduced via copy and paste.
                        // preserve insertSemi info
            if { let __tmp_x = Mode(Arc::new(Mutex::new(Some(((*(*self.mode.lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) & DONT_INSERT_SEMIS as u64))))); let __tmp_y = Mode(Arc::new(Mutex::new(Some(0 as u64)))); __tmp_x == __tmp_y } {
        { let new_val = insertSemi.lock().unwrap().as_ref().unwrap().clone(); *self.insert_semi.lock().unwrap() = Some(new_val); };
    }

            return (pos, tok, lit);
        };
        unreachable!()
    }
}

pub fn trailing_digits(text: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, i32, bool) {
    let mut i = bytes::last_index_byte(text.clone(), (':' as i32) as u8);
    if { let __tmp_x = i; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return (0, 0, false);
    }

        // no ":"
        // i >= 0
    let (mut n, mut err) = strconv::parse_uint(Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = text.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __low = ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __seq[__low..].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), 10, 0);
    return ({ let __tmp_x = i; let __tmp_y = 1; __tmp_x + __tmp_y }, (*Arc::new(Mutex::new(Some(n as i32))).lock().unwrap().as_ref().unwrap()), (*err.lock().unwrap()).is_none());
}

pub fn is_letter(ch: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = ('a' as i32); let __tmp_y = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x <= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = ('z' as i32); __tmp_x <= __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } && unicode::is_letter(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
}

pub fn is_digit(ch: Arc<Mutex<Option<i32>>>) -> bool {
    is_decimal(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } && unicode::is_digit(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub fn digit_val(ch: Arc<Mutex<Option<i32>>>) -> i32 {
    if { let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y } {
            return (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
        } else if { let __tmp_x = ('a' as i32); let __tmp_y = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x <= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = ('f' as i32); __tmp_x <= __tmp_y } {
            return (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = ('a' as i32); __tmp_x - __tmp_y }; let __tmp_y = 10 as i32; __tmp_x + __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap());
        }
    16
}

pub fn lower(ch: Arc<Mutex<Option<i32>>>) -> i32 {
    return { let __tmp_x = ({ let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y }) as i32; let __tmp_y = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y };
}

pub fn is_decimal(ch: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y };
}

pub fn is_hex(ch: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*ch.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y } || { let __tmp_x = ('a' as i32); let __tmp_y = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x <= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __arg_holder = ch.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = ('f' as i32); __tmp_x <= __tmp_y };
}

pub fn litname(prefix_local: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = { let __v = (*prefix_local.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('x' as i32)) {
            return Arc::new(Mutex::new(Some("hexadecimal literal".to_string())));
        } else if _switch_val == (('o' as i32)) || _switch_val == (('0' as i32)) {
            return Arc::new(Mutex::new(Some("octal literal".to_string())));
        } else if _switch_val == (('b' as i32)) {
            return Arc::new(Mutex::new(Some("binary literal".to_string())));
        }
    }
    Arc::new(Mutex::new(Some("decimal literal".to_string())))
}

/// invalidSep returns the index of the first invalid separator in x, or -1.
pub fn invalid_sep(x: Arc<Mutex<Option<String>>>) -> i32 {
    let mut x1 = Arc::new(Mutex::new(Some((' ' as i32))));
    let mut d = Arc::new(Mutex::new(Some(('.' as i32))));
    let mut i = Arc::new(Mutex::new(Some(0)));

        // a prefix counts as a digit
    if { let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        { let new_val = lower(Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] } as i32)))); *x1.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('o' as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('b' as i32); __tmp_x == __tmp_y } {
        { let new_val = ('0' as i32); *d.lock().unwrap() = Some(new_val); };
        { let new_val = 2; *i.lock().unwrap() = Some(new_val); };
    }
    }

        // mantissa and exponent
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut p = { let __owned = d.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*x.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *d.lock().unwrap() = __moved_val; };
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
            if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        } else if is_decimal(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __tmp_x = { let __v = (*x1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32); __tmp_x == __tmp_y } && is_hex(Arc::new(Mutex::new(Some({ let __arg_holder = d.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            { let new_val = ('0' as i32); *d.lock().unwrap() = Some(new_val); };
        } else {
            if { let __tmp_x = { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y };
    }
            { let new_val = ('.' as i32); *d.lock().unwrap() = Some(new_val); };
        }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // previous digit
    if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        return { let __tmp_x = ((*x.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y };
    }

    -(1)
}

pub fn strip_c_r(b: Arc<Mutex<Option<Vec<u8>>>>, comment: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut c = Arc::new(Mutex::new(Some(vec![0; ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    let mut i = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = b.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (j, ch) in __range_values.iter().copied().enumerate() {
                // In a /*-style comment, don't strip \r from *\r/ (incl.
                // sequences of \r from *\r\r...\r/) since the resulting
                // */ would terminate the comment too early unless the \r
                // is immediately following the opening /* in which case
                // it's ok because /*/ is not closed yet (issue #11151).
        if { let __tmp_x = ch; let __tmp_y = ('\r' as i32) as u8; __tmp_x != __tmp_y } || { let __v = (*comment.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 2; __tmp_x > __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = c.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() }; let __tmp_y = ('*' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = ({ let __tmp_x = j as i32; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = b.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = j as i32; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; let __tmp_y = ('/' as i32) as u8; __tmp_x == __tmp_y } {
        (*c.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ch;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }
        // In a /*-style comment, don't strip \r from *\r/ (incl.
        // sequences of \r from *\r\r...\r/) since the resulting
        // */ would terminate the comment too early unless the \r
        // is immediately following the opening /* in which case
        // it's ok because /*/ is not closed yet (issue #11151).
    return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = c.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_vec() })));
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Scanner {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
