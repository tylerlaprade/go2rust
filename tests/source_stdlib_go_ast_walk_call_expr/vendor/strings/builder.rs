use go2rust_stdlib_stubs::*;

use crate::clone::*;
use crate::compare::*;
use crate::iter::*;
use crate::reader::*;
use crate::replace::*;
use crate::search::*;
use crate::r#mod::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A Builder is used to efficiently build a string using [Builder.Write] methods.
/// It minimizes memory copying. The zero value is ready to use.
/// Do not copy a non-zero Builder.
#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub addr: Arc<Mutex<Option<Builder>>>,
    pub buf: Arc<Mutex<Option<Vec<u8>>>>,
}

impl Builder {
    pub fn __go_value_clone(&self) -> Self {
        Self { addr: self.addr.clone(), buf: self.buf.clone() }
    }
}

impl std::fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for Builder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Builder {
    pub fn copy_check(&mut self) {
        let _ = self;
    }

    /// String returns the accumulated string.
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __buf_guard = self.buf.lock().unwrap();
        let __text = __buf_guard.as_ref().map(|__buf| String::from_utf8_lossy(__buf).to_string()).unwrap_or_default();
        Arc::new(Mutex::new(Some(__text)))
    }

    /// Len returns the number of accumulated bytes; b.Len() == len(b.String()).
    pub fn len(&self) -> i32 {
        ({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32
    }

    /// Cap returns the capacity of the builder's underlying byte slice. It is the
    /// total space allocated for the string being built and includes any bytes
    /// already written.
    pub fn cap(&self) -> i32 {
        ({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32
    }

    /// Reset resets the [Builder] to be empty.
    pub fn reset(&mut self) {
        *self.addr.lock().unwrap() = None;
        *self.buf.lock().unwrap() = None;
    }

    /// grow copies the buffer to a new, larger buffer so that there are at least n
    /// bytes of capacity beyond len(b.buf).
    pub fn grow_1(&mut self, n: Arc<Mutex<Option<i32>>>) {
        let mut buf = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = bytealg::make_no_zero({ let __tmp_x = ({ let __tmp_x = 2; let __tmp_y = (({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); __tmp_x * __tmp_y } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x + __tmp_y }).clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) as usize; __seq[..__high].to_vec() })));
        { let _src = { let __copy_src_holder = self.buf.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*buf.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*buf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = buf.clone(); self.buf = new_val; };
    }

    /// Grow grows b's capacity, if necessary, to guarantee space for
    /// another n bytes. After Grow(n), at least n bytes can be written to b
    /// without another allocation. If n is negative, Grow panics.
    pub fn grow(&mut self, n: Arc<Mutex<Option<i32>>>) {
        self.copy_check();
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        panic!("strings.Builder.Grow: negative count");
    }
        if { let __tmp_x = ({ let __tmp_x = (({ let __cap_target = { let __field = self.buf.clone(); __field }; let __cap_guard = __cap_target.lock().unwrap(); __cap_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0) }) as i32); let __tmp_y = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        self.grow_1(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    }

    /// Write appends the contents of p to b's buffer.
    /// Write always returns len(p), nil.
    pub fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.copy_check();
        { let new_val = { let __append_target = self.buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend({ let __slice_holder = p.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned()); __append_target.clone() }; self.buf = new_val; };
        ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)))
    }

    /// WriteByte appends the byte c to b's buffer.
    /// The returned error is always nil.
    pub fn write_byte(&mut self, c: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.copy_check();
        { let new_val = { let __append_target = self.buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*c.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; self.buf = new_val; };
        return Arc::new(Mutex::new(None));
    }

    /// WriteRune appends the UTF-8 encoding of Unicode code point r to b's buffer.
    /// It returns the length of r and a nil error.
    pub fn write_rune(&mut self, r: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.copy_check();
        let mut n = Arc::new(Mutex::new(Some(({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32)));
        { let new_val = utf8::append_rune({ let __go_arg = { let __selector_holder = self.buf.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().cloned().unwrap_or_default(); drop(__selector_guard); __cloned }; __go_arg }, { let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); self.buf = new_val; };
        return ({ let __tmp_x = (({ let __len_target = { let __field = self.buf.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y }, Arc::new(Mutex::new(None)));
    }

    /// WriteString appends the contents of s to b's buffer.
    /// It returns the length of s and a nil error.
    pub fn write_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        self.copy_check();
        { let new_val = { let __append_target = self.buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend((*s.lock().unwrap().as_ref().unwrap()).clone().as_bytes().iter().cloned()); __append_target.clone() }; self.buf = new_val; };
        ((*s.lock().unwrap().as_ref().unwrap()).len() as i32, Arc::new(Mutex::new(None)))
    }
}

impl GoValueClone for Builder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
