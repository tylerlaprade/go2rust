use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::multi::*;
use crate::pipe::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const SEEK_START: i32 = 0;
pub const SEEK_CURRENT: i32 = 1;
pub const SEEK_END: i32 = 2;


/// Reader is the interface that wraps the basic Read method.
///
/// Read reads up to len(p) bytes into p. It returns the number of bytes
/// read (0 <= n <= len(p)) and any error encountered. Even if Read
/// returns n < len(p), it may use all of p as scratch space during the call.
/// If some data is available but not len(p) bytes, Read conventionally
/// returns what is available instead of waiting for more.
///
/// When Read encounters an error or end-of-file condition after
/// successfully reading n > 0 bytes, it returns the number of
/// bytes read. It may return the (non-nil) error from the same call
/// or return the error (and n == 0) from a subsequent call.
/// An instance of this general case is that a Reader returning
/// a non-zero number of bytes at the end of the input stream may
/// return either err == EOF or err == nil. The next Read should
/// return 0, EOF.
///
/// Callers should always process the n > 0 bytes returned before
/// considering the error err. Doing so correctly handles I/O errors
/// that happen after reading some bytes and also both of the
/// allowed EOF behaviors.
///
/// If len(p) == 0, Read should always return n == 0. It may return a
/// non-nil error if some error condition is known, such as EOF.
///
/// Implementations of Read are discouraged from returning a
/// zero byte count with a nil error, except when len(p) == 0.
/// Callers should treat a return of 0 and nil as indicating that
/// nothing happened; in particular it does not indicate EOF.
///
/// Implementations must not retain p.
pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool;
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Reader + Send + Sync> {
    fn clone(&self) -> Self {
        Reader::__go_clone_box_reader(self.as_ref())
    }
}

/// Writer is the interface that wraps the basic Write method.
///
/// Write writes len(p) bytes from p to the underlying data stream.
/// It returns the number of bytes written from p (0 <= n <= len(p))
/// and any error encountered that caused the write to stop early.
/// Write must return a non-nil error if it returns n < len(p).
/// Write must not modify the slice data, even temporarily.
///
/// Implementations must not retain p.
pub trait Writer: std::fmt::Display + Any {
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool;
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Writer + Send + Sync> {
    fn clone(&self) -> Self {
        Writer::__go_clone_box_writer(self.as_ref())
    }
}

/// ReaderFrom is the interface that wraps the ReadFrom method.
///
/// ReadFrom reads data from r until EOF or error.
/// The return value n is the number of bytes read.
/// Any error except EOF encountered during the read is also returned.
///
/// The [Copy] function uses [ReaderFrom] if available.
pub trait ReaderFrom: std::fmt::Display + Any {
    fn __go_clone_box_reader_from(&self) -> Box<dyn ReaderFrom + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader_from(&self, other: &(dyn ReaderFrom + Send + Sync)) -> bool;
    fn read_from(&mut self, r: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn ReaderFrom + Send + Sync> {
    fn clone(&self) -> Self {
        ReaderFrom::__go_clone_box_reader_from(self.as_ref())
    }
}

/// WriterTo is the interface that wraps the WriteTo method.
///
/// WriteTo writes data to w until there's no more data to write or
/// when an error occurs. The return value n is the number of bytes
/// written. Any error encountered during the write is also returned.
///
/// The Copy function uses WriterTo if available.
pub trait WriterTo: std::fmt::Display + Any {
    fn __go_clone_box_writer_to(&self) -> Box<dyn WriterTo + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_writer_to(&self, other: &(dyn WriterTo + Send + Sync)) -> bool;
    fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn WriterTo + Send + Sync> {
    fn clone(&self) -> Self {
        WriterTo::__go_clone_box_writer_to(self.as_ref())
    }
}

/// StringWriter is the interface that wraps the WriteString method.
pub trait StringWriter: std::fmt::Display + Any {
    fn __go_clone_box_string_writer(&self) -> Box<dyn StringWriter + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_string_writer(&self, other: &(dyn StringWriter + Send + Sync)) -> bool;
    fn write_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn StringWriter + Send + Sync> {
    fn clone(&self) -> Self {
        StringWriter::__go_clone_box_string_writer(self.as_ref())
    }
}

/// A LimitedReader reads from R but limits the amount of
/// data returned to just N bytes. Each call to Read
/// updates N to reflect the new amount remaining.
/// Read returns EOF when N <= 0 or when the underlying R returns EOF.
#[derive(Clone)]
pub struct LimitedReader {
    pub r: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>,
    pub n: Arc<Mutex<Option<i64>>>,
}

impl LimitedReader {
    pub fn __go_value_clone(&self) -> Self {
        Self { r: self.r.clone(), n: { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for LimitedReader {
    fn default() -> Self {
        Self { r: Arc::new(Mutex::new(None)), n: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for LimitedReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.r.lock().unwrap().as_ref().unwrap()), (*self.n.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for LimitedReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("N") {
            out.n = <Arc<Mutex<Option<i64>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


#[derive(Debug, Clone, Default)]
pub struct discard {
}

impl discard {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for discard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for discard {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub static ErrShortWrite: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errInvalidWrite: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrShortBuffer: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static EOF: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrUnexpectedEOF: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNoProgress: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errWhence: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static errOffset: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Discard: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn Writer + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static blackHolePool: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<sync::pool::Pool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrShortWrite.lock().unwrap() = None;
    *errInvalidWrite.lock().unwrap() = None;
    *ErrShortBuffer.lock().unwrap() = None;
    *EOF.lock().unwrap() = None;
    *ErrUnexpectedEOF.lock().unwrap() = None;
    *ErrNoProgress.lock().unwrap() = None;
    *errWhence.lock().unwrap() = None;
    *errOffset.lock().unwrap() = None;
    *Discard.lock().unwrap() = None;
    *blackHolePool.lock().unwrap() = Some(Default::default());
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("short write".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrShortWrite.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid write result".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidWrite.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("short buffer".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrShortBuffer.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("EOF".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *EOF.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unexpected EOF".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrUnexpectedEOF.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("multiple Read calls return no data or error".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoProgress.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Seek: invalid whence".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errWhence.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Seek: invalid offset".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errOffset.lock().unwrap() = new_val; }
    *Discard.lock().unwrap() = Some(Box::new(discard {  }) as Box<dyn Writer + Send + Sync>);
    *blackHolePool.lock().unwrap() = Some(sync::pool::Pool { new: Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut b = Arc::new(Mutex::new(Some(vec![0; (8192) as usize])));
        return Arc::new(Mutex::new(Some(Box::new(b.clone().clone()) as Box<dyn Any + Send + Sync>)));
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> + Send + Sync>))), ..Default::default() });
}


pub(crate) fn __go_zero_globals() {
    *ErrShortWrite.lock().unwrap() = None;
    *errInvalidWrite.lock().unwrap() = None;
    *ErrShortBuffer.lock().unwrap() = None;
    *EOF.lock().unwrap() = None;
    *ErrUnexpectedEOF.lock().unwrap() = None;
    *ErrNoProgress.lock().unwrap() = None;
    *errWhence.lock().unwrap() = None;
    *errOffset.lock().unwrap() = None;
    *Discard.lock().unwrap() = None;
    *blackHolePool.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("short write".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrShortWrite.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid write result".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidWrite.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("short buffer".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrShortBuffer.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("EOF".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *EOF.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("unexpected EOF".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrUnexpectedEOF.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("multiple Read calls return no data or error".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNoProgress.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_6() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Seek: invalid whence".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errWhence.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_7() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("Seek: invalid offset".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errOffset.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_8() {
    *Discard.lock().unwrap() = Some(Box::new(discard {  }) as Box<dyn Writer + Send + Sync>);
}


pub(crate) fn __go_init_order_10() {
    *blackHolePool.lock().unwrap() = Some(sync::pool::Pool { new: Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut b = Arc::new(Mutex::new(Some(vec![0; (8192) as usize])));
        return Arc::new(Mutex::new(Some(Box::new(b.clone().clone()) as Box<dyn Any + Send + Sync>)));
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> + Send + Sync>))), ..Default::default() });
}


impl LimitedReader {
    pub fn read(&mut self, mut p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if { let __tmp_x = (*self.n.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as i64; __tmp_x <= __tmp_y } {
        return (0, EOF.clone());
    }
        if { let __tmp_x = (*Arc::new(Mutex::new(Some((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.n.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = p.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (0) as usize; let __high = (*self.n.clone().lock().unwrap().as_ref().unwrap()) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); p = new_val; };
    }
        { let (__tmp_0, __tmp_1) = (*self.r.lock().unwrap().as_mut().unwrap()).read(p.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        { let __target = self.n.clone(); let __rhs = (*Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
}

impl Reader for LimitedReader {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        LimitedReader::read(self, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LimitedReader>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct LimitedReaderPtr(pub Arc<Mutex<Option<LimitedReader>>>);

impl std::fmt::Display for LimitedReaderPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for LimitedReaderPtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        LimitedReader::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<LimitedReaderPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl discard {
    pub fn write(&self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)))
    }

    pub fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        ((*s.lock().unwrap().as_ref().unwrap()).len() as i32, Arc::new(Mutex::new(None)))
    }

    pub fn read_from(&self, r: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut bufp = ({
        let val = (*blackHolePool.lock().unwrap().as_mut().unwrap()).get().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>().expect("type assertion failed").clone()
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        let mut readSize = Arc::new(Mutex::new(Some(0)));
        loop {
        { let (__tmp_0, __tmp_1) = (*r.lock().unwrap().as_mut().unwrap()).read(Arc::new(Mutex::new(Some({ let __v = (*bufp.lock().unwrap().as_ref().unwrap()).clone(); __v })))); *readSize.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        { let __rhs = (*Arc::new(Mutex::new(Some((*readSize.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        (*blackHolePool.lock().unwrap().as_mut().unwrap()).put(Arc::new(Mutex::new(Some(Box::new(bufp.clone()) as Box<dyn Any + Send + Sync>))));
        if { let __left = err.clone(); let __right = EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
    }
}

impl ReaderFrom for discard {
    fn read_from(&mut self, r: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        discard::read_from(self, r)
    }
    fn __go_clone_box_reader_from(&self) -> Box<dyn ReaderFrom + Send + Sync> {
        Box::new(self.clone()) as Box<dyn ReaderFrom + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader_from(&self, other: &(dyn ReaderFrom + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discard>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct discardPtr(pub Arc<Mutex<Option<discard>>>);

impl std::fmt::Display for discardPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl ReaderFrom for discardPtr {
    fn read_from(&mut self, r: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        discard::read_from(__recv, r)
    }
    fn __go_clone_box_reader_from(&self) -> Box<dyn ReaderFrom + Send + Sync> {
        Box::new(self.clone()) as Box<dyn ReaderFrom + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader_from(&self, other: &(dyn ReaderFrom + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discardPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl StringWriter for discard {
    fn write_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        discard::write_string(self, s)
    }
    fn __go_clone_box_string_writer(&self) -> Box<dyn StringWriter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn StringWriter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_string_writer(&self, other: &(dyn StringWriter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discard>() {
            false
        } else {
            false
        }
    }
}

impl StringWriter for discardPtr {
    fn write_string(&mut self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        discard::write_string(__recv, s)
    }
    fn __go_clone_box_string_writer(&self) -> Box<dyn StringWriter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn StringWriter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_string_writer(&self, other: &(dyn StringWriter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discardPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Writer for discard {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        discard::write(self, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discard>() {
            false
        } else {
            false
        }
    }
}

impl Writer for discardPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        discard::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<discardPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// copyBuffer is the actual implementation of Copy and CopyBuffer.
/// if buf is nil, one is allocated.
pub fn copy_buffer_1(dst: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, src: Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>, mut buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut written: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // If the reader has a WriteTo method, use it to do the copy.
        // Avoids an allocation and a copy.
    {
        let (mut wt, mut ok) = ({
        let val = src.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Reader + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::multi::multiReaderPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn WriterTo + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn WriterTo + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn WriterTo + Send + Sync>>)), false)
        }
    });;
        if ok {
            return (*wt.lock().unwrap().as_mut().unwrap()).write_to(dst.clone());;
        }
    }

        // Similarly, if the writer has a ReadFrom method, use it to do the copy.
    {
        let (mut rf, mut ok) = ({
        let val = dst.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Writer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<discard>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn ReaderFrom + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn ReaderFrom + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn ReaderFrom + Send + Sync>>)), false)
        }
    });;
        if ok {
            return (*rf.lock().unwrap().as_mut().unwrap()).read_from(src.clone());;
        }
    }
    if { let __nil_result = (*buf.lock().unwrap()).is_none(); __nil_result } {
        let mut size = Arc::new(Mutex::new(Some({ let __tmp_x = 32; let __tmp_y = 1024; __tmp_x * __tmp_y })));
        {
        let (mut l, mut ok) = ({
        let val = src.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Reader + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<LimitedReaderPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<LimitedReader>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<LimitedReader>)), false)
        }
    });;
        if ok && { let __tmp_x = (*Arc::new(Mutex::new(Some((*size.lock().unwrap().as_ref().unwrap()) as i64))).lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x > __tmp_y } {
            if { let __tmp_x = (*{ let __field = (*l.lock().unwrap().as_ref().unwrap()).n.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1 as i64; __tmp_x < __tmp_y } {
        { let new_val = 1; *size.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = Arc::new(Mutex::new(Some({ let __selector_holder = (*l.lock().unwrap().as_ref().unwrap()).n.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *size.lock().unwrap() = __moved_val; };
    };
        }
    }
        { let new_val = Arc::new(Mutex::new(Some(vec![0; ({ let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize]))); buf = new_val; };
    }
    loop {
        let (mut nr, mut er) = (*src.lock().unwrap().as_mut().unwrap()).read(buf.clone());
        if { let __tmp_x = nr; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut nw, mut ew) = (*dst.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (0) as usize; let __high = (nr) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        if { let __tmp_x = nw; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = nr; let __tmp_y = nw; __tmp_x < __tmp_y } {
        { let new_val = 0; nw = new_val; };
        if { let __nil_result = (*ew.lock().unwrap()).is_none(); __nil_result } {
        { let __rhs_holder = errInvalidWrite.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ew.lock().unwrap() = new_val; };
    }
    }
        { let __rhs = (*Arc::new(Mutex::new(Some(nw as i64))).lock().unwrap().as_ref().unwrap()); let mut guard = written.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __nil_result = (*ew.lock().unwrap()).is_some(); __nil_result } {
        { let __rhs_holder = ew.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break
    }
        if { let __tmp_x = nr; let __tmp_y = nw; __tmp_x != __tmp_y } {
        { let __rhs_holder = ErrShortWrite.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        break
    }
    }
        if { let __nil_result = (*er.lock().unwrap()).is_some(); __nil_result } {
        if { let __left = er.clone(); let __right = EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
        { let __rhs_holder = er.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
        break
    }
    }
    return ({ let __v = (*written.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for LimitedReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for discard {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
