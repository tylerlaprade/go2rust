use go2rust_stdlib_stubs::*;

use crate::buffer::*;
use crate::iter::*;
use crate::reader::*;

use std::sync::{Arc, Mutex};

pub(crate) static asciiSpace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *asciiSpace.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *asciiSpace.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *asciiSpace.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_3() {
    *asciiSpace.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))).lock().unwrap().as_ref().unwrap()).clone());
}


/// Equal reports whether a and b
/// are the same length and contain the same bytes.
/// A nil argument is equivalent to an empty slice.
pub fn equal(a: Arc<Mutex<Option<Vec<u8>>>>, b: Arc<Mutex<Option<Vec<u8>>>>) -> bool {
        // Neither cmd/compile nor gccgo allocates for these string conversions.
    return { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*a.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*Arc::new(Mutex::new(Some(String::from_utf8((*b.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y };
}

/// Count counts the number of non-overlapping instances of sep in s.
/// If sep is an empty slice, Count returns 1 + the number of UTF-8-encoded code points in s.
pub fn count(mut s: Arc<Mutex<Option<Vec<u8>>>>, sep: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
        // special case
    if { let __tmp_x = ((*sep.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __tmp_x = unicode_utf8::rune_count(s.clone()); let __tmp_y = 1; __tmp_x + __tmp_y };
    }
    if { let __tmp_x = ((*sep.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        return internal_bytealg::count(s.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
    }
    let mut n = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut i = index(s.clone(), sep.clone());
        if { let __tmp_x = i; let __tmp_y = -1; __tmp_x == __tmp_y } {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = (i as i32); let __tmp_y = ((*sep.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); s = new_val; };
    }
}

/// Contains reports whether subslice is within b.
pub fn contains(b: Arc<Mutex<Option<Vec<u8>>>>, subslice: Arc<Mutex<Option<Vec<u8>>>>) -> bool {
    return { let __tmp_x = index(b.clone(), subslice.clone()); let __tmp_y = -1; __tmp_x != __tmp_y };
}

/// IndexByte returns the index of the first instance of c in b, or -1 if c is not present in b.
pub fn index_byte(b: Arc<Mutex<Option<Vec<u8>>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_bytealg::index_byte(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// HasPrefix reports whether the byte slice s begins with prefix.
pub fn has_prefix(s: Arc<Mutex<Option<Vec<u8>>>>, prefix: Arc<Mutex<Option<Vec<u8>>>>) -> bool {
    return { let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*prefix.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } && equal(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ((*prefix.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), prefix.clone());
}

/// TrimLeftFunc treats s as UTF-8-encoded bytes and returns a subslice of s by slicing off
/// all leading UTF-8-encoded code points c that satisfy f(c).
pub fn trim_left_func(s: Arc<Mutex<Option<Vec<u8>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut i = index_func_1(s.clone(), f.clone(), Arc::new(Mutex::new(Some(false))));
    if { let __tmp_x = i; let __tmp_y = -1; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
}

/// TrimRightFunc returns a subslice of s by slicing off all trailing
/// UTF-8-encoded code points c that satisfy f(c).
pub fn trim_right_func(s: Arc<Mutex<Option<Vec<u8>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    let mut i = last_index_func_1(s.clone(), f.clone(), Arc::new(Mutex::new(Some(false))));
    if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        let (_, mut wid) = unicode_utf8::decode_rune(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (i) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))));
        { let __rhs = wid; i = i + __rhs; };
    } else {
        { i += 1; }
    }
    Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (0) as usize; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))
}

/// TrimFunc returns a subslice of s by slicing off all leading and trailing
/// UTF-8-encoded code points c that satisfy f(c).
pub fn trim_func(s: Arc<Mutex<Option<Vec<u8>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    trim_right_func(trim_left_func(s.clone(), f.clone()), f.clone())
}

/// indexFunc is the same as IndexFunc except that if
/// truth==false, the sense of the predicate function is
/// inverted.
pub fn index_func_1(s: Arc<Mutex<Option<Vec<u8>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>, truth: Arc<Mutex<Option<bool>>>) -> i32 {
    let mut start = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut wid = Arc::new(Mutex::new(Some(1)));
        let mut r = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i32)));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *r.lock().unwrap() = Some(__tmp_0); *wid.lock().unwrap() = Some(__tmp_1); };
    }
        if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(r.clone()) }; let __tmp_y = { let __v = (*truth.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let __rhs = (*wid.lock().unwrap().as_ref().unwrap()); let mut guard = start.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    -(1)
}

/// lastIndexFunc is the same as LastIndexFunc except that if
/// truth==false, the sense of the predicate function is
/// inverted.
pub fn last_index_func_1(s: Arc<Mutex<Option<Vec<u8>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>, truth: Arc<Mutex<Option<bool>>>) -> i32 {
    let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut r, mut size) = (Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() } as i32))), Arc::new(Mutex::new(Some(1))));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_last_rune(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })))); *r.lock().unwrap() = Some(__tmp_0); *size.lock().unwrap() = Some(__tmp_1); };
    }
        { let __rhs = (*size.lock().unwrap().as_ref().unwrap()); let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(r.clone()) }; let __tmp_y = { let __v = (*truth.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
    -(1)
}

/// TrimSpace returns a subslice of s by slicing off all leading and
/// trailing white space, as defined by Unicode.
pub fn trim_space(s: Arc<Mutex<Option<Vec<u8>>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Fast path for ASCII: look for the first ASCII non-space byte
    let mut start = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
                // If we run into a non-ASCII byte, fall back to the
                // slower unicode-aware method on the remaining bytes
        return trim_func(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
    }
                // If we run into a non-ASCII byte, fall back to the
                // slower unicode-aware method on the remaining bytes
        if { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let mut guard = start.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

        // If we run into a non-ASCII byte, fall back to the
        // slower unicode-aware method on the remaining bytes
        // Now look for the first ASCII non-space byte from the end
    let mut stop = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    while { let __tmp_x = { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize].clone() })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        return trim_func(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
    }
        if { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let mut guard = stop.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // At this point s[start:stop] starts and ends with an ASCII
        // non-space bytes, so we're done. Non-ASCII cases have already
        // been handled above.
    if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
                // Special case to preserve previous TrimLeftFunc behavior,
                // returning nil instead of empty slice if all spaces.
        return Arc::new(Mutex::new(None));
    }
        // Special case to preserve previous TrimLeftFunc behavior,
        // returning nil instead of empty slice if all spaces.
    return Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
}

/// Index returns the index of the first instance of sep in s, or -1 if sep is not present in s.
pub fn index(s: Arc<Mutex<Option<Vec<u8>>>>, sep: Arc<Mutex<Option<Vec<u8>>>>) -> i32 {
    let mut n = Arc::new(Mutex::new(Some((*sep.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return 0;
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            return index_byte(s.clone(), Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x == __tmp_y } {
            if equal(sep.clone(), s.clone()) {
        return 0;
    }
            return -(1);
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x > __tmp_y } {
            return -(1);
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*internal_bytealg::MaxLen.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x <= __tmp_y } {
                        // Use brute force when s and sep both are small
            if { let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = internal_bytealg::MAX_BRUTE_FORCE; __tmp_x <= __tmp_y } {
        return internal_bytealg::index(s.clone(), sep.clone());
    }
            let mut c0 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
            let mut c1 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })));
            let mut i = Arc::new(Mutex::new(Some(0)));
            let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })));
            let mut fails = Arc::new(Mutex::new(Some(0)));
            while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __v = (*c0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // IndexByte is faster than bytealg.Index, so use it as long as
                // we're not getting lots of false positives.
        let mut o = index_byte(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = c0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = o; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __rhs = { let __tmp_x = o; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
                // IndexByte is faster than bytealg.Index, so use it as long as
                // we're not getting lots of false positives.
        if { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; let __tmp_y = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && equal(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), sep.clone()) {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = fails.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

                // Switch to bytealg.Index when IndexByte produces too many false positives.
        if { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_bytealg::cutover(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x > __tmp_y } {
        let mut r = internal_bytealg::index(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), sep.clone());
        if { let __tmp_x = r; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return { let __tmp_x = r; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y };
    }
        return -(1);
    }
    }
                        // IndexByte is faster than bytealg.Index, so use it as long as
                        // we're not getting lots of false positives.
                        // Switch to bytealg.Index when IndexByte produces too many false positives.
            return -(1);
        }
        // Use brute force when s and sep both are small
        // IndexByte is faster than bytealg.Index, so use it as long as
        // we're not getting lots of false positives.
        // Switch to bytealg.Index when IndexByte produces too many false positives.
    let mut c0 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
    let mut c1 = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = sep.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() })));
    let mut i = Arc::new(Mutex::new(Some(0)));
    let mut fails = Arc::new(Mutex::new(Some(0)));
    let mut t = Arc::new(Mutex::new(Some({ let __tmp_x = ({ let __tmp_x = ((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = { let __v = (*c0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        let mut o = index_byte(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __high = ({ let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __arg_holder = c0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = o; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        { let __rhs = { let __tmp_x = o; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        if { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize].clone() }; let __tmp_y = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && equal(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), sep.clone()) {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = fails.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 4; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }; __tmp_x >= __tmp_y } && { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
                // Give up on IndexByte, it isn't skipping ahead
                // far enough to be better than Rabin-Karp.
                // Experiments (using IndexPeriodic) suggest
                // the cutover is about 16 byte skips.
                // TODO: if large prefixes of sep are matching
                // we should cutover at even larger average skips,
                // because Equal becomes that much more expensive.
                // This code does not take that effect into account.
        let mut j = internal_bytealg::index_rabin_karp::<Vec<u8>>(Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), sep.clone());
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = j; __tmp_x + __tmp_y };
    }
    }
        // Give up on IndexByte, it isn't skipping ahead
        // far enough to be better than Rabin-Karp.
        // Experiments (using IndexPeriodic) suggest
        // the cutover is about 16 byte skips.
        // TODO: if large prefixes of sep are matching
        // we should cutover at even larger average skips,
        // because Equal becomes that much more expensive.
        // This code does not take that effect into account.
    -(1)
}

/// Cut slices s around the first instance of sep,
/// returning the text before and after sep.
/// The found result reports whether sep appears in s.
/// If sep does not appear in s, cut returns s, nil, false.
///
/// Cut returns slices of the original slice s, not copies.
pub fn cut(s: Arc<Mutex<Option<Vec<u8>>>>, sep: Arc<Mutex<Option<Vec<u8>>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Vec<u8>>>>, bool) {
    let mut before: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut after: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    {
        let mut i = index(s.clone(), sep.clone());;
        if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            return (Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (i) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), Arc::new(Mutex::new(Some({ let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = ({ let __tmp_x = (i as i32); let __tmp_y = ((*sep.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y }) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))), true);;
        }
    }
    return (s.clone(), Arc::new(Mutex::new(None)), false);
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
