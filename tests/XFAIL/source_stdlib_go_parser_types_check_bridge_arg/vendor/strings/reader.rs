use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::builder::*;
use crate::clone::*;
use crate::compare::*;
use crate::iter::*;
use crate::replace::*;
use crate::search::*;
use crate::r#mod::*;

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Reader implements the [io.Reader], [io.ReaderAt], [io.ByteReader], [io.ByteScanner],
/// [io.RuneReader], [io.RuneScanner], [io.Seeker], and [io.WriterTo] interfaces by reading
/// from a string.
/// The zero value for Reader operates like a Reader of an empty string.
#[derive(Debug, Clone)]
pub struct Reader {
    pub s: Arc<Mutex<Option<String>>>,
    pub i: Arc<Mutex<Option<i64>>>,
    pub prev_rune: Arc<Mutex<Option<i32>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { s: { let __guard = self.s.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, i: { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, prev_rune: { let __guard = self.prev_rune.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Reader {
    fn default() -> Self {
        Self { s: Arc::new(Mutex::new(Some(String::new()))), i: Arc::new(Mutex::new(Some(0))), prev_rune: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", (*self.s.lock().unwrap().as_ref().unwrap()), (*self.i.lock().unwrap().as_ref().unwrap()), (*self.prev_rune.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Reader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Reader {
    /// Len returns the number of bytes of the unread portion of the
    /// string.
    pub fn len(&self) -> i32 {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return 0;
    }
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.i.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
    }

    /// Size returns the original length of the underlying string.
    /// Size is the number of bytes available for reading via [Reader.ReadAt].
    /// The returned value is always the same and is not affected by calls
    /// to any other method.
    pub fn size(&self) -> i64 {
        (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap())
    }

    /// Read implements the [io.Reader] interface.
    pub fn read(&mut self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        { let new_val = { let _src = (*Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return ((*n.lock().unwrap().as_ref().unwrap()), err);
    }

    /// ReadAt implements the [io.ReaderAt] interface.
    pub fn read_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // cannot modify state - see io.ReaderAt
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.ReadAt: negative offset".to_string())))));
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        { let new_val = { let _src = (*Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let __rhs_holder = io::EOF().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err);
    }

    /// ReadByte implements the [io.ByteReader] interface.
    pub fn read_byte(&mut self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        let mut b = Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize] })));
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// UnreadByte implements the [io.ByteScanner] interface.
    pub fn unread_byte(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.UnreadByte: at beginning of string".to_string()))));
    }
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
        return Arc::new(Mutex::new(None));
    }

    /// ReadRune implements the [io.RuneReader] interface.
    pub fn read_rune(&mut self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ch: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut size: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        return (0, 0, { let __return_value_2 = io::EOF().clone(); __return_value_2 });
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = self.i.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.prev_rune.lock().unwrap() = __moved_val; };
        {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize] })));;
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } {
            { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
            return ((*Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()), 1, Arc::new(Mutex::new(None)));;
        }
    }
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[__low..].to_string() })))); *ch.lock().unwrap() = Some(__tmp_0); *size.lock().unwrap() = Some(__tmp_1); };
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return ((*ch.lock().unwrap().as_ref().unwrap()), (*size.lock().unwrap().as_ref().unwrap()), err);
    }

    /// UnreadRune implements the [io.RuneScanner] interface.
    pub fn unread_rune(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.UnreadRune: at beginning of string".to_string()))));
    }
        if { let __tmp_x = (*self.prev_rune.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.UnreadRune: previous operation was not ReadRune".to_string()))));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = self.prev_rune.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.i.lock().unwrap() = __moved_val; };
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        return Arc::new(Mutex::new(None));
    }

    /// Seek implements the [io.Seeker] interface.
    pub fn seek(&mut self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        let mut abs: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        { let _switch_val = { let __v = (*whence.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (io::SEEK_START) {
            { let new_val = offset.lock().unwrap().as_ref().unwrap().clone(); *abs.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (io::SEEK_CURRENT) {
            { let new_val = { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *abs.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (io::SEEK_END) {
            { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *abs.lock().unwrap() = Some(new_val); };
        } else {
            return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.Seek: invalid whence".to_string())))));
        }
    }
        if { let __tmp_x = { let __v = (*abs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("strings.Reader.Seek: negative position".to_string())))));
    }
        { let new_val = abs.lock().unwrap().as_ref().unwrap().clone(); *self.i.lock().unwrap() = Some(new_val); };
        return ({ let __v = (*abs.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// WriteTo implements the [io.WriterTo] interface.
    pub fn write_to(&mut self, w: Arc<Mutex<Option<io_Writer>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some((*self.s.lock().unwrap().as_ref().unwrap()).len() as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, Arc::new(Mutex::new(None)));
    }
        let mut s = Arc::new(Mutex::new(Some({ let __s = &((*self.s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[__low..].to_string() })));
        let (mut m, __tmp_1) = io::write_string(w.clone(), { let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __tmp_x = (m as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x > __tmp_y } {
        panic!("strings.Reader.WriteTo: invalid WriteString count");
    }
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some(m as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some(m as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (m as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x != __tmp_y } && (*err.lock().unwrap()).is_none() {
        { let __rhs_holder = io::ErrShortWrite().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err);
    }

    /// Reset resets the [Reader] to be reading from s.
    pub fn reset(&mut self, s: Arc<Mutex<Option<String>>>) {
        { let new_val = Reader { s: Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), i: Arc::new(Mutex::new(Some(0 as i64))), prev_rune: Arc::new(Mutex::new(Some(-1))), ..Default::default() }; *self = new_val; };
    }
}

/// NewReader returns a new [Reader] reading from s.
/// It is similar to [bytes.NewBufferString] but more efficient and non-writable.
pub fn new_reader(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Reader>>> {
    Arc::new(Mutex::new(Some(Reader { s: Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), i: Arc::new(Mutex::new(Some(0 as i64))), prev_rune: Arc::new(Mutex::new(Some(-1))), ..Default::default() })))
}

impl GoValueClone for Reader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
