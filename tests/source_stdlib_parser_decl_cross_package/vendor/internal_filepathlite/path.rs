use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::path_nonwindows::*;
use crate::path_unix::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// A lazybuf is a lazily constructed path buffer.
/// It supports append, reading previously appended bytes,
/// and retrieving the final string. It does not allocate a buffer
/// to hold the output until that output diverges from s.
#[derive(Debug, Clone)]
pub struct lazybuf {
    pub path: Arc<Mutex<Option<String>>>,
    pub buf: Arc<Mutex<Option<Vec<u8>>>>,
    pub w: Arc<Mutex<Option<i32>>>,
    pub vol_and_path: Arc<Mutex<Option<String>>>,
    pub vol_len: Arc<Mutex<Option<i32>>>,
}

impl lazybuf {
    pub fn __go_value_clone(&self) -> Self {
        Self { path: { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: self.buf.clone(), w: { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, vol_and_path: { let __guard = self.vol_and_path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, vol_len: { let __guard = self.vol_len.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for lazybuf {
    fn default() -> Self {
        Self { path: Arc::new(Mutex::new(Some(String::new()))), buf: Arc::new(Mutex::new(None)), w: Arc::new(Mutex::new(Some(0))), vol_and_path: Arc::new(Mutex::new(Some(String::new()))), vol_len: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for lazybuf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {}}}", (*self.path.lock().unwrap().as_ref().unwrap()), format_slice(&self.buf), (*self.w.lock().unwrap().as_ref().unwrap()), (*self.vol_and_path.lock().unwrap().as_ref().unwrap()), (*self.vol_len.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for lazybuf {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) static errInvalidPath: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *errInvalidPath.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid path".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidPath.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *errInvalidPath.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid path".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *errInvalidPath.lock().unwrap() = new_val; }
}


impl lazybuf {
    pub fn index(&self, i: Arc<Mutex<Option<i32>>>) -> u8 {
        if { let __nil_target = self.buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        return { let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() };
    }
        { let __s = &((*self.path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }
    }

    pub fn append(&mut self, c: Arc<Mutex<Option<u8>>>) {
        if { let __nil_target = self.buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        if { let __tmp_x = ((*self.w.clone().lock().unwrap().as_ref().unwrap()) as i32); let __tmp_y = ((*self.path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __s = &((*self.path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.w.clone().lock().unwrap().as_ref().unwrap()) as usize] }; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let __target = self.w.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return;
    }
        { let new_val = Arc::new(Mutex::new(Some(vec![0; ((*self.path.lock().unwrap().as_ref().unwrap()).len()) as usize]))); self.buf = new_val; };
        { let _src = (*Arc::new(Mutex::new(Some({ let __s = &((*self.path.lock().unwrap().as_ref().unwrap()).clone()); let __high = (*self.w.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone().as_bytes().to_vec(); let _n = std::cmp::min((*self.buf.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*self.buf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
    }
        (*self.buf.lock().unwrap().as_mut().unwrap())[(*self.w.clone().lock().unwrap().as_ref().unwrap()) as usize] = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
        { let __target = self.w.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn prepend(&mut self, prefix: Arc<Mutex<Option<Vec<u8>>>>) {
        { let new_val = { let __result = slices::insert::<Vec<u8>, u8>({ let __slice_holder = self.buf.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }, Arc::new(Mutex::new(Some(0))), { let __slice_holder = prefix.clone(); { let __slice_guard = __slice_holder.lock().unwrap(); Arc::new(Mutex::new(__slice_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| Arc::new(Mutex::new(Some(__elem)))).collect::<Vec<_>>()))) } }); let __result_guard = __result.lock().unwrap(); Arc::new(Mutex::new(__result_guard.as_ref().map(|__v| __v.iter().cloned().map(|__elem| (*__elem.lock().unwrap().as_ref().unwrap()).clone()).collect::<Vec<_>>()))) }; self.buf = new_val; };
        { let __target = self.w.clone(); let __rhs = (*prefix.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        if { let __nil_target = self.buf.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return Arc::new(Mutex::new(Some({ let __s = &((*self.vol_and_path.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = (*self.vol_len.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.w.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() })));
    }
        return Arc::new(Mutex::new(Some(format!("{}{}", (*Arc::new(Mutex::new(Some({ let __s = &((*self.vol_and_path.lock().unwrap().as_ref().unwrap()).clone()); let __high = (*self.vol_len.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()), (*Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = self.buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = (*self.w.clone().lock().unwrap().as_ref().unwrap()) as usize; __seq[..__high].to_vec() }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap())))));
    }
}

/// Clean is filepath.Clean.
pub fn clean(mut path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let mut originalPath = { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    let mut volLen = volume_name_len_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __low = (volLen) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *path.lock().unwrap() = __moved_val; };
    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        if { let __tmp_x = volLen; let __tmp_y = 1; __tmp_x > __tmp_y } && is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*originalPath.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })))) && is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*originalPath.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))) {
                // should be UNC
        return from_slash(Arc::new(Mutex::new(Some({ let __arg_holder = originalPath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
                // should be UNC
        return Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*originalPath.lock().unwrap().as_ref().unwrap()).clone(); __v }, ".".to_string()))));
    }
        // should be UNC
    let mut rooted = is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }))));

        // Invariants:
        //	reading from path; r is index of next byte to process.
        //	writing to buf; w is index of next byte to write.
        //	dotdot is index in buf where .. must stop, either because
        //		it is the leading slash or it is a leading ../../.. prefix.
    let mut n = Arc::new(Mutex::new(Some((*path.lock().unwrap().as_ref().unwrap()).len() as i32)));
    let mut out = Arc::new(Mutex::new(Some(lazybuf { path: Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), vol_and_path: Arc::new(Mutex::new(Some({ let __arg_holder = originalPath.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), vol_len: Arc::new(Mutex::new(Some(volLen))), ..Default::default() })));
    let (mut r, mut dotdot) = (Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(0))));
    if rooted {
        (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(SEPARATOR as u8))));
        { let __tmp_0 = 1; let __tmp_1 = 1; *r.lock().unwrap() = Some(__tmp_0); *dotdot.lock().unwrap() = Some(__tmp_1); };
    }

    while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        if is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
                        // empty path element
            { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if { let __tmp_x = { let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } || is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }))))) {
                        // . element
            { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        } else if { let __tmp_x = { let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } && { let __tmp_x = { let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize] }; let __tmp_y = ('.' as i32) as u8; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } || is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x + __tmp_y }) as usize] }))))) {
                        // .. element: remove to last separator
            { let __rhs = 2; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
            if { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dotdot.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                        // can backtrack
            { let __target = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
            while { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*dotdot.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } && !is_path_separator(Arc::new(Mutex::new(Some((*out.lock().unwrap().as_ref().unwrap()).index(Arc::new(Mutex::new(Some({ let __selector_holder = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))))))) {
        { let __target = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
        } else if !rooted {
                        // cannot backtrack, but not rooted, so append .. element.
            if { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(SEPARATOR as u8))));
    }
            (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(('.' as i32) as u8))));
            (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(('.' as i32) as u8))));
            { let new_val = { let __selector_holder = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *dotdot.lock().unwrap() = Some(new_val); };
        }
        } else {
                        // real path element.
                        // add slash if needed
            if rooted && { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 1; __tmp_x != __tmp_y } || !rooted && { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x != __tmp_y } {
        (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(SEPARATOR as u8))));
    }
                        // copy element
            while { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && !is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }))));
        { let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        }
    }

        // empty path element
        // . element
        // .. element: remove to last separator
        // can backtrack
        // cannot backtrack, but not rooted, so append .. element.
        // real path element.
        // add slash if needed
        // copy element
        // Turn empty string into "."
    if { let __tmp_x = (*{ let __field = (*out.lock().unwrap().as_ref().unwrap()).w.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0; __tmp_x == __tmp_y } {
        (*out.lock().unwrap().as_mut().unwrap()).append(Arc::new(Mutex::new(Some(('.' as i32) as u8))));
    }

    post_clean(out.clone());
    return from_slash((*out.lock().unwrap().as_ref().unwrap()).string());
}

/// FromSlash is filepath.ToSlash.
pub fn from_slash(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = SEPARATOR; let __tmp_y = ('/' as i32); __tmp_x == __tmp_y } {
        return { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    replace_string_byte(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('/' as i32) as u8))), Arc::new(Mutex::new(Some(SEPARATOR as u8))))
}

pub fn replace_string_byte(s: Arc<Mutex<Option<String>>>, old: Arc<Mutex<Option<u8>>>, new: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = internal_stringslite::index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = -1; __tmp_x == __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
    let mut n = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec())));
    for i in 0..(({ let __range_holder = n.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __tmp_x = { let __seq = { let __seq_holder = n.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }; let __tmp_y = { let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        (*n.lock().unwrap().as_mut().unwrap())[(i) as usize] = { let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
    return Arc::new(Mutex::new(Some(String::from_utf8((*n.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
}

/// Split is filepath.Split.
pub fn split(path: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    let mut dir: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut file: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let mut vol = volume_name(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*vol.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } && !is_path_separator(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    return (Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))));
}

/// VolumeName is filepath.VolumeName.
pub fn volume_name(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    from_slash(Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); let __high = (volume_name_len_1(Arc::new(Mutex::new(Some({ let __arg_holder = path.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) as usize; __s[..__high].to_string() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for lazybuf {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
