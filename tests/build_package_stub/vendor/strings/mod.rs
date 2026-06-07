use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::builder::*;
use crate::clone::*;
use crate::compare::*;
use crate::iter::*;
use crate::reader::*;
use crate::replace::*;
use crate::search::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const MAX_INT: i32 = i32::MAX;


pub(crate) const REPEATED_SPACES: &'static str = "                                                                                                                                ";
pub(crate) const REPEATED_DASHES: &'static str = "--------------------------------------------------------------------------------------------------------------------------------";
pub(crate) const REPEATED_ZEROES: &'static str = "0000000000000000000000000000000000000000000000000000000000000000";
pub(crate) const REPEATED_EQUALS: &'static str = "================================================================================================================================";
pub(crate) const REPEATED_TABS: &'static str = "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t";


/// asciiSet is a 32-byte value, where each bit represents the presence of a
/// given ASCII character in the set. The 128-bits of the lower 16 bytes,
/// starting with the least-significant bit of the lowest word to the
/// most-significant bit of the highest word, map to the full range of all
/// 128 ASCII characters. The 128-bits of the upper 16 bytes will be zeroed,
/// ensuring that any non-ASCII character will be reported as not in the set.
/// This allocates a total of 32 bytes even though the upper half
/// is unused to avoid bounds checks in asciiSet.contains.
#[derive(Debug, Clone)]
pub struct asciiSet(pub Arc<Mutex<Option<[u32; 8]>>>);

impl Default for asciiSet {
    fn default() -> Self {
        asciiSet(Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))))
    }
}

impl Display for asciiSet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


pub(crate) static asciiSpace: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<[u8; 256]>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *asciiSpace.lock().unwrap() = Some(std::array::from_fn(|_| 0));
    *asciiSpace.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))).lock().unwrap().as_ref().unwrap()).clone());
}


pub(crate) fn __go_zero_globals() {
    *asciiSpace.lock().unwrap() = Some(std::array::from_fn(|_| 0));
}


pub(crate) fn __go_init_order_0() {
    *asciiSpace.lock().unwrap() = Some((*Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1 as u8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]))).lock().unwrap().as_ref().unwrap()).clone());
}


impl asciiSet {
    /// contains reports whether c is inside the set.
    pub fn contains(&self, c: Arc<Mutex<Option<u8>>>) -> bool {
        return { let __tmp_x = ({ let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u8; __tmp_x / __tmp_y }) as usize].clone() }; let __tmp_y = ({ let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u8; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }); let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
    }
}

/// explode splits s into a slice of UTF-8 strings,
/// one string per Unicode character up to a maximum of n (n < 0 means no limit).
/// Invalid UTF-8 bytes are sliced individually.
pub fn explode(mut s: Arc<Mutex<Option<String>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut l = unicode_utf8::rune_count_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = l; __tmp_x > __tmp_y } {
        { let new_val = l; *n.lock().unwrap() = Some(new_val); };
    }
    let mut a = Arc::new(Mutex::new(Some(vec!["".to_string(); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        let (_, mut size) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (size) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone();
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (size) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*a.lock().unwrap().as_mut().unwrap())[({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    return a.clone();
}

/// Count counts the number of non-overlapping instances of substr in s.
/// If substr is an empty string, Count returns 1 + the number of Unicode code points in s.
pub fn count(mut s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> i32 {
        // special case
    if { let __tmp_x = ((*substr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __tmp_x = unicode_utf8::rune_count_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1; __tmp_x + __tmp_y };
    }
    if { let __tmp_x = ((*substr.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        return internal_bytealg::count_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }))));
    }
    let mut n = Arc::new(Mutex::new(Some(0)));
    loop {
        let mut i = index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = i; let __tmp_y = -1; __tmp_x == __tmp_y } {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = (i as i32); let __tmp_y = ((*substr.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
}

/// Contains reports whether substr is within s.
pub fn contains(s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

/// ContainsAny reports whether any Unicode code points in chars are within s.
pub fn contains_any(s: Arc<Mutex<Option<String>>>, chars: Arc<Mutex<Option<String>>>) -> bool {
    return { let __tmp_x = index_any(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

/// ContainsRune reports whether the Unicode code point r is within s.
pub fn contains_rune(s: Arc<Mutex<Option<String>>>, r: Arc<Mutex<Option<i32>>>) -> bool {
    return { let __tmp_x = index_rune(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y };
}

/// LastIndex returns the index of the last instance of substr in s, or -1 if substr is not present in s.
pub fn last_index(s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> i32 {
    let mut n = Arc::new(Mutex::new(Some((*substr.lock().unwrap().as_ref().unwrap()).len() as i32)));
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return (*s.lock().unwrap().as_ref().unwrap()).len() as i32;
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } {
            return internal_bytealg::last_index_byte_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __s = &((*substr.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }))));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x == __tmp_y } {
            if { let __tmp_x = (*substr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*s.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return 0;
    }
            return -(1);
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x > __tmp_y } {
            return -(1);
        }

        // Rabin-Karp search from the end of the string
    let (mut hashss, mut pow) = internal_bytealg::hash_str_rev::<String>(Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut last = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x - __tmp_y })));
    let mut h: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_bytealg::PRIME_R_K as u32; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *h.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    if { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = hashss; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*substr.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let __rhs = internal_bytealg::PRIME_R_K as u32; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        { let __rhs = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let __rhs = { let __tmp_x = pow; let __tmp_y = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize] } as u32))).lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y }; let mut guard = h.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = hashss; __tmp_x == __tmp_y } && { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*substr.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
    -(1)
}

/// IndexByte returns the index of the first instance of c in s, or -1 if c is not present in s.
pub fn index_byte(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> i32 {
    internal_stringslite::index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// IndexRune returns the index of the first instance of the Unicode code point
/// r, or -1 if rune is not present in s.
/// If r is [utf8.RuneError], it returns the first instance of any
/// invalid UTF-8 byte sequence.
pub fn index_rune(s: Arc<Mutex<Option<String>>>, r: Arc<Mutex<Option<i32>>>) -> i32 {
    const haveFastIndex: bool = internal_bytealg::MAX_BRUTE_FORCE > 0;

    if { let __tmp_x = 0 as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } {
            return index_byte(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))));
        } else if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } {
            for (i, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = (r as i32); let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } {
        return i as i32;
    }
    }
            return -(1);
        } else if !unicode_utf8::valid_rune(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return -(1);
        } else {
                        // Search for rune r using the last byte of its UTF-8 encoded form.
                        // The distribution of the last byte is more uniform compared to the
                        // first byte which has a 78% chance of being [240, 243, 244].
            let mut rs = Arc::new(Mutex::new(Some(char::from_u32(((*r.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string())));
            let mut last = Arc::new(Mutex::new(Some({ let __tmp_x = ((*rs.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
            let mut i = { let __owned = last.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
            let mut fails = Arc::new(Mutex::new(Some(0)));
            while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x != __tmp_y } {
        let mut o = index_byte(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }))));
        if { let __tmp_x = o; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return -(1);
    }
        { let __rhs = { let __tmp_x = o; let __tmp_y = 1; __tmp_x + __tmp_y }; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }

                // Step backwards comparing bytes.
        let mut j = Arc::new(Mutex::new(Some(1)));
    while { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*rs.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = { let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] }; __tmp_x != __tmp_y } {
        // TODO: unsupported goto next
    }
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
        { let mut guard = fails.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        if (haveFastIndex && { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = internal_bytealg::cutover(Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); __tmp_x > __tmp_y }) && { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } || (!haveFastIndex && { let __tmp_x = { let __v = (*fails.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = 4; let __tmp_y = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y }; __tmp_x >= __tmp_y } && { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y }) {
        // TODO: unsupported goto fallback
    }
    }
                        // Step backwards comparing bytes.
            return -(1);
                        // see comment in ../bytes/bytes.go
            if haveFastIndex {
        {
        let mut j = internal_bytealg::index_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some(char::from_u32(((*r.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))));;
        if { let __tmp_x = j; let __tmp_y = 0; __tmp_x >= __tmp_y } {
            return { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = j; __tmp_x + __tmp_y }; let __tmp_y = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };;
        }
    }
    } else {
        let mut c0 = Arc::new(Mutex::new(Some({ let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        let mut c1 = Arc::new(Mutex::new(Some({ let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] })));
        'r#loop: while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = { let __v = (*c0.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = { let __v = (*c1.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        let mut k = Arc::new(Mutex::new(Some(2)));
    while { let __tmp_x = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*rs.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] }; let __tmp_y = { let __s = &((*rs.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }) as usize] }; __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue 'r#loop
    }
        { let mut guard = k.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*last.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
            return -(1);
        }
    unreachable!()
}

/// IndexAny returns the index of the first instance of any Unicode code point
/// from chars in s, or -1 if no Unicode code point from chars is present in s.
pub fn index_any(s: Arc<Mutex<Option<String>>>, chars: Arc<Mutex<Option<String>>>) -> i32 {
    if { let __tmp_x = (*chars.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
                // Avoid scanning all of s.
        return -(1);
    }
        // Avoid scanning all of s.
    if { let __tmp_x = ((*chars.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
                // Avoid scanning all of s.
        let mut r = Arc::new(Mutex::new(Some({ let __s = &((*chars.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32)));
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let new_val = unicode_utf8::RUNE_ERROR as i32; *r.lock().unwrap() = Some(new_val); };
    }
        return index_rune(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
        // Avoid scanning all of s.
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 8; __tmp_x > __tmp_y } {
        {
        let (mut r#as, mut isASCII) = make_a_s_c_i_i_set(Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if isASCII {
            let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if (*r#as.lock().unwrap().as_ref().unwrap()).contains(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    };
            return -(1);;
        }
    }
    }
    for (i, c) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = index_rune(Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(c as i32)))); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return i as i32;
    }
    }
    -(1)
}

/// Generic split: splits after each instance of sep,
/// including sepSave bytes of sep in the subarrays.
pub fn gen_split(mut s: Arc<Mutex<Option<String>>>, sep: Arc<Mutex<Option<String>>>, sepSave: Arc<Mutex<Option<i32>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(None));
    }
    if { let __tmp_x = (*sep.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return explode(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = count(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 1; __tmp_x + __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }

    if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x + __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
    let mut a = Arc::new(Mutex::new(Some(vec!["".to_string(); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut m = index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = m; let __tmp_y = 0; __tmp_x < __tmp_y } {
        break
    }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = m; let __tmp_y = { let __v = (*sepSave.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone();
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = (m as i32); let __tmp_y = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v };
    return Arc::new(Mutex::new(Some({ let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v })));
}

/// SplitN slices s into substrings separated by sep and returns a slice of
/// the substrings between those separators.
///
/// The count determines the number of substrings to return:
///   - n > 0: at most n substrings; the last substring will be the unsplit remainder;
///   - n == 0: the result is nil (zero substrings);
///   - n < 0: all substrings.
///
/// Edge cases for s and sep (for example, empty strings) are handled
/// as described in the documentation for [Split].
///
/// To split around the first instance of a separator, see [Cut].
pub fn split_n(s: Arc<Mutex<Option<String>>>, sep: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    gen_split(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Split slices s into all substrings separated by sep and returns a slice of
/// the substrings between those separators.
///
/// If s does not contain sep and sep is not empty, Split returns a
/// slice of length 1 whose only element is s.
///
/// If sep is empty, Split splits after each UTF-8 sequence. If both s
/// and sep are empty, Split returns an empty slice.
///
/// It is equivalent to [SplitN] with a count of -1.
///
/// To split around the first instance of a separator, see [Cut].
pub fn split(s: Arc<Mutex<Option<String>>>, sep: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    gen_split(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(-1))))
}

/// Fields splits the string s around each instance of one or more consecutive white space
/// characters, as defined by [unicode.IsSpace], returning a slice of substrings of s or an
/// empty slice if s contains only white space.
pub fn fields(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        // First count the fields.
        // This is an exact count if s is ASCII, otherwise it is an approximation.
    let mut n = Arc::new(Mutex::new(Some(0)));
    let mut wasSpace = Arc::new(Mutex::new(Some(1)));

        // setBits is used to track which bits are set in the bytes of s.
    let mut setBits = Arc::new(Mutex::new(Some(0 as u8)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut r = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        { let __rhs = (*r.lock().unwrap().as_ref().unwrap()); let mut guard = setBits.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
        let mut isSpace = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() } as i32)));
        { let __rhs = { let __tmp_x = { let __v = (*wasSpace.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = !(*isSpace.lock().unwrap().as_ref().unwrap()); __tmp_x & __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let new_val = isSpace.lock().unwrap().as_ref().unwrap().clone(); *wasSpace.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    if { let __tmp_x = { let __v = (*setBits.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
                // Some runes in the input string are not ASCII.
        return fields_func(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
    }

        // Some runes in the input string are not ASCII.
        // ASCII fast path
    let mut a = Arc::new(Mutex::new(Some(vec!["".to_string(); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    let mut na = Arc::new(Mutex::new(Some(0)));
    let mut fieldStart = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));

        // Skip spaces in the front of the input.
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *fieldStart.lock().unwrap() = Some(new_val); };
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        continue
    }
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*na.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*fieldStart.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = na.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }

                // Skip spaces in between fields.
        while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *fieldStart.lock().unwrap() = Some(new_val); };
    }
        // Skip spaces in between fields.
    if { let __tmp_x = ({ let __v = (*fieldStart.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        (*a.lock().unwrap().as_mut().unwrap())[({ let __v = (*na.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*fieldStart.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))).lock().unwrap().as_ref().unwrap()).clone();
    }
    return a.clone();
}

/// FieldsFunc splits the string s at each run of Unicode code points c satisfying f(c)
/// and returns an array of slices of s. If all code points in s satisfy f(c) or the
/// string is empty, an empty slice is returned.
///
/// FieldsFunc makes no guarantees about the order in which it calls f(c)
/// and assumes that f always returns the same value for a given c.
pub fn fields_func(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
        // A span is used to record a slice of s of the form s[start:end].
        // The start index is inclusive and the end index is exclusive.
    type span = AnonymousStruct1;
    let mut spans: Arc<Mutex<Option<Vec<span>>>> = Arc::new(Mutex::new(Some(Vec::<span>::with_capacity((32) as usize))));

        // Find the field start and end indices.
        // Doing this in a separate pass (rather than slicing the string s
        // and collecting the result substrings right away) is significantly
        // more efficient, possibly due to cache effects.
    let mut start = Arc::new(Mutex::new(Some(-(1))));
    for (end, rune) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(rune as i32)))) } {
        if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = spans.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(span { start: Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some(end as i32))), ..Default::default() }); __append_target.clone() }; spans = new_val; };
                // Set start to a negative value.
                // Note: using -1 here consistently and reproducibly
                // slows down this code by a several percent on amd64.
        { let new_val = !(*start.lock().unwrap().as_ref().unwrap()); *start.lock().unwrap() = Some(new_val); };
    }
    } else {
        if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = end as i32; *start.lock().unwrap() = Some(new_val); };
    }
    }
    }

        // Set start to a negative value.
        // Note: using -1 here consistently and reproducibly
        // slows down this code by a several percent on amd64.
        // Last field might end at EOF.
    if { let __tmp_x = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = spans.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(span { start: Arc::new(Mutex::new(Some({ let __arg_holder = start.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), end: Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32))), ..Default::default() }); __append_target.clone() }; spans = new_val; };
    }

        // Create strings from recorded field indices.
    let mut a = Arc::new(Mutex::new(Some(vec!["".to_string(); ((*spans.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize])));
    { let __range_holder = spans.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, span) in __range_values.iter().enumerate() {
        (*a.lock().unwrap().as_mut().unwrap())[(i) as usize] = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ((*span.start.lock().unwrap().as_ref().unwrap())) as usize; let __high = ((*span.end.lock().unwrap().as_ref().unwrap())) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone();
    } }

    return a.clone();
}

/// Join concatenates the elements of its first argument to create a single string. The separator
/// string sep is placed between elements in the resulting string.
pub fn join(elems: Arc<Mutex<Option<Vec<String>>>>, sep: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = (*elems.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0);
    if _switch_val == (0) {
            return Arc::new(Mutex::new(Some("".to_string())));
        } else if _switch_val == (1) {
            return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })));
        }
    }

    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        if { let __tmp_x = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = i32::MAX; let __tmp_y = (({ let __tmp_x = ((*elems.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as i32); __tmp_x / __tmp_y } as i32); __tmp_x >= __tmp_y } {
        std::panic::panic_any(Box::new("strings: Join output length overflow".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = { let __tmp_x = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (({ let __tmp_x = ((*elems.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as i32); __tmp_x * __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    { let __range_holder = elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for elem in __range_values.iter() {
        if { let __tmp_x = (elem.len() as i32); let __tmp_y = ({ let __tmp_x = i32::MAX; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as i32); __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("strings: Join output length overflow".to_string()) as Box<dyn Any + Send + Sync>);
    }
        { let __rhs = elem.len() as i32; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }

    let mut b: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*b.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }))));
    for s in &{ let __seq_holder = elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (1) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v } {
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some((*s).clone()))));
    }
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

/// HasPrefix reports whether the string s begins with prefix.
pub fn has_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> bool {
    internal_stringslite::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// HasSuffix reports whether the string s ends with suffix.
pub fn has_suffix(s: Arc<Mutex<Option<String>>>, suffix: Arc<Mutex<Option<String>>>) -> bool {
    internal_stringslite::has_suffix(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = suffix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// TrimLeftFunc returns a slice of the string s with all leading
/// Unicode code points c satisfying f(c) removed.
pub fn trim_left_func(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut i = index_func_1(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), f.clone(), Arc::new(Mutex::new(Some(false))));
    if { let __tmp_x = i; let __tmp_y = -1; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }
    Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() })))
}

/// TrimRightFunc returns a slice of the string s with all trailing
/// Unicode code points c satisfying f(c) removed.
pub fn trim_right_func(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut i = last_index_func_1(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), f.clone(), Arc::new(Mutex::new(Some(false))));
    if { let __tmp_x = i; let __tmp_y = 0; __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(i) as usize] }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        let (_, mut wid) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() }))));
        { let __rhs = wid; i = i + __rhs; };
    } else {
        { i += 1; }
    }
    Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (i) as usize; __s[__low..__high].to_string() })))
}

/// TrimFunc returns a slice of the string s with all leading
/// and trailing Unicode code points c satisfying f(c) removed.
pub fn trim_func(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    trim_right_func(trim_left_func(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), f.clone()), f.clone())
}

/// indexFunc is the same as IndexFunc except that if
/// truth==false, the sense of the predicate function is
/// inverted.
pub fn index_func_1(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>, truth: Arc<Mutex<Option<bool>>>) -> i32 {
    for (i, r) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(r as i32)))) }; let __tmp_y = { let __v = (*truth.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return i as i32;
    }
    }
    -(1)
}

/// lastIndexFunc is the same as LastIndexFunc except that if
/// truth==false, the sense of the predicate function is
/// inverted.
pub fn last_index_func_1(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>, truth: Arc<Mutex<Option<bool>>>) -> i32 {
    let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut r, mut size) = unicode_utf8::decode_last_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))));
        { let __rhs = size; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(r)))) }; let __tmp_y = { let __v = (*truth.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
    -(1)
}

/// makeASCIISet creates a set of ASCII characters and reports whether all
/// characters in chars are ASCII.
pub fn make_a_s_c_i_i_set(chars: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<asciiSet>>>, bool) {
    let mut r#as: Arc<Mutex<Option<asciiSet>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*chars.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*chars.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        return ({ let __owned = r#as.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, false);
    }
        { let __idx = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u8; __tmp_x / __tmp_y } as usize; let __rhs = { let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u8; __tmp_x % __tmp_y }); __tmp_x << __tmp_y }; let __seq_holder = { let __named_array = (*r#as.lock().unwrap().as_ref().unwrap()).0.clone(); __named_array }; let mut __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] | __rhs; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return ({ let __owned = r#as.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, true);
}

/// TrimSpace returns a slice of the string s, with all leading
/// and trailing white space removed, as defined by Unicode.
pub fn trim_space(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        // Fast path for ASCII: look for the first ASCII non-space byte
    let mut start = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
                // If we run into a non-ASCII byte, fall back to the
                // slower unicode-aware method on the remaining bytes
        return trim_func(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
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
    let mut stop = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize] })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
                // start has been already trimmed above, should trim end only
        return trim_right_func(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> bool { unicode::is_space(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>))));
    }
                // start has been already trimmed above, should trim end only
        if { let __tmp_x = { let __seq = { let __seq_holder = asciiSpace.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        break
    }
        { let mut guard = stop.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }

        // start has been already trimmed above, should trim end only
        // At this point s[start:stop] starts and ends with an ASCII
        // non-space bytes, so we're done. Non-ASCII cases have already
        // been handled above.
    return Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() })));
}

/// TrimPrefix returns s without the provided leading prefix string.
/// If s doesn't start with prefix, s is returned unchanged.
pub fn trim_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_stringslite::trim_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
pub fn index(s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> i32 {
    internal_stringslite::index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// Cut slices s around the first instance of sep,
/// returning the text before and after sep.
/// The found result reports whether sep appears in s.
/// If sep does not appear in s, cut returns s, "", false.
pub fn cut(s: Arc<Mutex<Option<String>>>, sep: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, bool) {
    let mut before: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut after: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    internal_stringslite::cut(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = sep.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// CutPrefix returns s without the provided leading prefix string
/// and reports whether it found the prefix.
/// If s doesn't start with prefix, CutPrefix returns s, false.
/// If prefix is the empty string, CutPrefix returns s, true.
pub fn cut_prefix(s: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool) {
    let mut after: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut found: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    internal_stringslite::cut_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub start: Arc<Mutex<Option<i32>>>,
    pub end: Arc<Mutex<Option<i32>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { start: { let __guard = self.start.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, end: { let __guard = self.end.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { start: Arc::new(Mutex::new(Some(0))), end: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.start.lock().unwrap().as_ref().unwrap()), (*self.end.lock().unwrap().as_ref().unwrap()))
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
