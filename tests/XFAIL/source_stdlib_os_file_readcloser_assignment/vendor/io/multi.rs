use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_stringer, format_slice_wrapped_stringer_values};

use crate::r#mod::*;
use crate::pipe::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct eofReader {
}

impl eofReader {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for eofReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for eofReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct multiReader {
    pub readers: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>>>>>>,
}

impl multiReader {
    pub fn __go_value_clone(&self) -> Self {
        Self { readers: self.readers.clone() }
    }
}

impl std::fmt::Display for multiReader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped_stringer(&self.readers))
    }
}

impl GoJsonDecode for multiReader {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


#[derive(Clone, Default)]
pub struct multiWriter {
    pub writers: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>>>>>,
}

impl multiWriter {
    pub fn __go_value_clone(&self) -> Self {
        Self { writers: self.writers.clone() }
    }
}

impl std::fmt::Display for multiWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", format_slice_wrapped_stringer(&self.writers))
    }
}

impl GoJsonDecode for multiWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl eofReader {
    pub fn read(&self, __arg0: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (0, EOF.clone())
    }
}

impl Reader for eofReader {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        eofReader::read(self, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<eofReader>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct eofReaderPtr(pub Arc<Mutex<Option<eofReader>>>);

impl std::fmt::Display for eofReaderPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for eofReaderPtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        eofReader::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<eofReaderPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl multiReader {
    pub fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        while { let __tmp_x = (({ let __len_target = { let __field = self.readers.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // Optimization to flatten nested multiReaders (Issue 13558).
        if { let __tmp_x = (({ let __len_target = { let __field = self.readers.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        {
        let (mut r, mut ok) = ({
        let val = { let __seq = { let __seq_holder = self.readers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Reader + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<multiReaderPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<multiReader>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<multiReader>)), false)
        }
    });;
        if ok {
            { let new_val = (*r.lock().unwrap().as_ref().unwrap()).readers.clone(); self.readers = new_val; };;
            continue;
        }
    }
    }
        { let (__tmp_0, __tmp_1) = { let __recv = { let __seq = { let __seq_holder = self.readers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; let __result = (*__recv.lock().unwrap().as_mut().unwrap()).read(p.clone()); __result }; *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __left = err.clone(); let __right = EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
                // Use eofReader instead of nil to avoid nil panic
                // after performing flatten (Issue 18232).
        (*self.readers.lock().unwrap().as_mut().unwrap())[(0) as usize] = Arc::new(Mutex::new(Some(Box::new(eofReader {  }) as Box<dyn Reader + Send + Sync>)));
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.readers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.readers = new_val; };
    }
                // Use eofReader instead of nil to avoid nil panic
                // after performing flatten (Issue 18232).
                // permit earlier GC
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } || { let __left = err.clone(); let __right = EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; !__eq } {
        if { let __left = err.clone(); let __right = EOF.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } && { let __tmp_x = (({ let __len_target = { let __field = self.readers.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
                // Don't return EOF yet. More readers remain.
        *err.lock().unwrap() = None;
    }
                // Don't return EOF yet. More readers remain.
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    }
                // Optimization to flatten nested multiReaders (Issue 13558).
                // Use eofReader instead of nil to avoid nil panic
                // after performing flatten (Issue 18232).
                // permit earlier GC
                // Don't return EOF yet. More readers remain.
        (0, EOF.clone())
    }

    pub fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut sum: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        self.write_to_with_buffer(w.clone(), Arc::new(Mutex::new(Some(vec![0; ({ let __tmp_x = 1024; let __tmp_y = 32; __tmp_x * __tmp_y }) as usize]))))
    }

    pub fn write_to_with_buffer(&mut self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>, buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut sum: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let __range_holder = self.readers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, r) in __range_values.iter().enumerate() {
        let mut n: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        {
        let (mut subMr, mut ok) = ({
        let val = r.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Reader + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<multiReaderPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<multiReader>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<multiReader>)), false)
        }
    });;
        if ok {
            { let (__tmp_0, __tmp_1) = { let __recv = subMr.clone(); let __recv_ptr: *mut multiReader = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut multiReader }; let __result = unsafe { &mut *__recv_ptr }.write_to_with_buffer(w.clone(), buf.clone()); __result }; *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        } else {
            { let (__tmp_0, __tmp_1) = copy_buffer_1(w.clone(), r.clone(), buf.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        }
    }
        { let __rhs = (*n.lock().unwrap().as_ref().unwrap()); let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = self.readers.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); self.readers = new_val; };
        return ({ let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
                // permit resume / retry after error
        (*self.readers.lock().unwrap().as_mut().unwrap())[(i) as usize] = Arc::new(Mutex::new(None));
    } }
                // reuse buffer with nested multiReaders
                // permit resume / retry after error
                // permit early GC
        *self.readers.lock().unwrap() = None;
        return ({ let __v = (*sum.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }
}

impl Reader for multiReader {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        multiReader::read(self, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiReader>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct multiReaderPtr(pub Arc<Mutex<Option<multiReader>>>);

impl std::fmt::Display for multiReaderPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for multiReaderPtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        multiReader::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiReaderPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl WriterTo for multiReader {
    fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        multiReader::write_to(self, w)
    }
    fn __go_clone_box_writer_to(&self) -> Box<dyn WriterTo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn WriterTo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer_to(&self, other: &(dyn WriterTo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiReader>() {
            false
        } else {
            false
        }
    }
}

impl WriterTo for multiReaderPtr {
    fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        multiReader::write_to(__recv, w)
    }
    fn __go_clone_box_writer_to(&self) -> Box<dyn WriterTo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn WriterTo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer_to(&self, other: &(dyn WriterTo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiReaderPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl multiWriter {
    pub fn write(&self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let __range_holder = self.writers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_mut().unwrap()).write(p.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x != __tmp_y } {
        { let __rhs_holder = ErrShortWrite.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    } }
        ((*p.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32, Arc::new(Mutex::new(None)))
    }

    pub fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        let mut p: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
        { let __range_holder = self.writers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for w in __range_values.iter() {
        {
        let (mut sw, mut ok) = ({
        let val = w.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Writer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<crate::r#mod::discard>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn StringWriter + Send + Sync>))), true)
            } else if let Some(typed_val) = <dyn Writer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<multiWriterPtr>() {
                (Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn StringWriter + Send + Sync>))), true)
            } else {
                (Arc::new(Mutex::new(None::<Box<dyn StringWriter + Send + Sync>>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<Box<dyn StringWriter + Send + Sync>>)), false)
        }
    });;
        if ok {
            { let (__tmp_0, __tmp_1) = (*sw.lock().unwrap().as_ref().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        } else {
            if { let __nil_result = (*p.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec()))); p = new_val; };
    };
            { let (__tmp_0, __tmp_1) = (*w.lock().unwrap().as_mut().unwrap()).write(p.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        }
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
        if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x != __tmp_y } {
        { let __rhs_holder = ErrShortWrite.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*n.lock().unwrap().as_ref().unwrap()), err.clone());
    }
    } }
        ((*s.lock().unwrap().as_ref().unwrap()).len() as i32, Arc::new(Mutex::new(None)))
    }
}

impl StringWriter for multiWriter {
    fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        multiWriter::write_string(self, s)
    }
    fn __go_clone_box_string_writer(&self) -> Box<dyn StringWriter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn StringWriter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_string_writer(&self, other: &(dyn StringWriter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiWriter>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct multiWriterPtr(pub Arc<Mutex<Option<multiWriter>>>);

impl std::fmt::Display for multiWriterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl StringWriter for multiWriterPtr {
    fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        multiWriter::write_string(__recv, s)
    }
    fn __go_clone_box_string_writer(&self) -> Box<dyn StringWriter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn StringWriter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_string_writer(&self, other: &(dyn StringWriter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiWriterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Writer for multiWriter {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        multiWriter::write(self, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiWriter>() {
            false
        } else {
            false
        }
    }
}

impl Writer for multiWriterPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        multiWriter::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<multiWriterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
}


impl GoValueClone for eofReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for multiReader {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for multiWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
