use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::buffer::*;
use crate::r#mod::*;
use crate::iter::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Reader implements the [io.Reader], [io.ReaderAt], [io.WriterTo], [io.Seeker],
/// [io.ByteScanner], and [io.RuneScanner] interfaces by reading from
/// a byte slice.
/// Unlike a [Buffer], a Reader is read-only and supports seeking.
/// The zero value for Reader operates like a Reader of an empty slice.
#[derive(Debug, Clone)]
pub struct Reader {
    pub s: Arc<Mutex<Option<Vec<u8>>>>,
    pub i: Arc<Mutex<Option<i64>>>,
    pub prev_rune: Arc<Mutex<Option<i32>>>,
}

impl Reader {
    pub fn __go_value_clone(&self) -> Self {
        Self { s: self.s.clone(), i: { let __guard = self.i.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, prev_rune: { let __guard = self.prev_rune.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Reader {
    fn default() -> Self {
        Self { s: Arc::new(Mutex::new(None)), i: Arc::new(Mutex::new(Some(0))), prev_rune: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.s), (*self.i.lock().unwrap().as_ref().unwrap()), (*self.prev_rune.lock().unwrap().as_ref().unwrap()))
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
    /// slice.
    pub fn len(&self) -> i32 {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return 0;
    }
        (*Arc::new(Mutex::new(Some(({ let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.i.lock().unwrap().as_ref().unwrap()); __tmp_x - __tmp_y }) as i32))).lock().unwrap().as_ref().unwrap())
    }

    /// Size returns the original length of the underlying byte slice.
    /// Size is the number of bytes available for reading via [Reader.ReadAt].
    /// The result is unaffected by any method calls except [Reader.Reset].
    pub fn size(&self) -> i64 {
        (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap())
    }

    /// Read implements the [io.Reader] interface.
    pub fn read(&mut self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        { let new_val = { let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// ReadAt implements the [io.ReaderAt] interface.
    pub fn read_at(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // cannot modify state - see io.ReaderAt
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.ReadAt: negative offset".to_string())))));
    }
        if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        { let new_val = { let _src = (*Arc::new(Mutex::new(Some({ let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone(); let _n = std::cmp::min((*b.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*b.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        { let __rhs_holder = io::EOF().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// ReadByte implements the [io.ByteReader] interface.
    pub fn read_byte(&mut self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, { let __return_value_1 = io::EOF().clone(); __return_value_1 });
    }
        let mut b = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() })));
        { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return ({ let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// UnreadByte complements [Reader.ReadByte] in implementing the [io.ByteScanner] interface.
    pub fn unread_byte(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.UnreadByte: at beginning of slice".to_string()))));
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

        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        return (0, 0, { let __return_value_2 = io::EOF().clone(); __return_value_2 });
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = self.i.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *self.prev_rune.lock().unwrap() = __moved_val; };
        {
        let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize].clone() })));;
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } {
            { let __target = self.i.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); };
            return ((*Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()), 1, Arc::new(Mutex::new(None)));;
        }
    }
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune(Arc::new(Mutex::new(Some({ let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *ch.lock().unwrap() = Some(__tmp_0); *size.lock().unwrap() = Some(__tmp_1); };
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        return ((*ch.lock().unwrap().as_ref().unwrap()), (*size.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// UnreadRune complements [Reader.ReadRune] in implementing the [io.RuneScanner] interface.
    pub fn unread_rune(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.UnreadRune: at beginning of slice".to_string()))));
    }
        if { let __tmp_x = (*self.prev_rune.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.UnreadRune: previous operation was not ReadRune".to_string()))));
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
            { let new_val = { let __tmp_x = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*offset.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }; *abs.lock().unwrap() = Some(new_val); };
        } else {
            return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.Seek: invalid whence".to_string())))));
        }
    }
        if { let __tmp_x = { let __v = (*abs.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i64; __tmp_x < __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("bytes.Reader.Seek: negative position".to_string())))));
    }
        { let new_val = abs.lock().unwrap().as_ref().unwrap().clone(); *self.i.lock().unwrap() = Some(new_val); };
        return ({ let __v = (*abs.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    /// WriteTo implements the [io.WriterTo] interface.
    pub fn write_to(&mut self, w: Arc<Mutex<Option<io_Writer>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let new_val = -1; *self.prev_rune.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = (*self.i.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.s.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, Arc::new(Mutex::new(None)));
    }
        let mut b = Arc::new(Mutex::new(Some({ let __seq_holder = self.s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (*self.i.clone().lock().unwrap().as_ref().unwrap()) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
        let (mut m, __tmp_1) = (*w.lock().unwrap().as_ref().unwrap()).write(b.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __tmp_x = (m as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("bytes.Reader.WriteTo: invalid Write count".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __target = self.i.clone(); let __rhs = (*Arc::new(Mutex::new(Some(m as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = Arc::new(Mutex::new(Some(m as i64))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_val; };
        if { let __tmp_x = (m as i32); let __tmp_y = ((*b.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } && { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = io::ErrShortWrite().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }

    /// Reset resets the [Reader] to be reading from b.
    pub fn reset(&mut self, b: Arc<Mutex<Option<Vec<u8>>>>) {
        { let new_val = Reader { s: b.clone(), i: Arc::new(Mutex::new(Some(0 as i64))), prev_rune: Arc::new(Mutex::new(Some(-1))), ..Default::default() }; *self = new_val; };
    }
}

/// NewReader returns a new [Reader] reading from b.
pub fn new_reader(b: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Reader>>> {
    Arc::new(Mutex::new(Some(Reader { s: b.clone(), i: Arc::new(Mutex::new(Some(0 as i64))), prev_rune: Arc::new(Mutex::new(Some(-1))), ..Default::default() })))
}

impl GoValueClone for Reader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
