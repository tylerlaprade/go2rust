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

/// LastIndexAny returns the index of the last instance of any Unicode code
/// point from chars in s, or -1 if no Unicode code point from chars is
/// present in s.
pub fn last_index_any(s: Arc<Mutex<Option<String>>>, chars: Arc<Mutex<Option<String>>>) -> i32 {
    if { let __tmp_x = (*chars.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
                // Avoid scanning all of s.
        return -(1);
    }
        // Avoid scanning all of s.
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut rc = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32)));
        if { let __tmp_x = { let __v = (*rc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let new_val = unicode_utf8::RUNE_ERROR as i32; *rc.lock().unwrap() = Some(new_val); };
    }
        if { let __tmp_x = index_rune(Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = rc.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return 0;
    }
        return -(1);
    }
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 8; __tmp_x > __tmp_y } {
        {
        let (mut r#as, mut isASCII) = make_a_s_c_i_i_set(Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if isASCII {
            let mut i = Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x - __tmp_y })));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        if (*r#as.lock().unwrap().as_ref().unwrap()).contains(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))) {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    };
            return -(1);;
        }
    }
    }
    if { let __tmp_x = ((*chars.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x == __tmp_y } {
        let mut rc = Arc::new(Mutex::new(Some({ let __s = &((*chars.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32)));
        if { let __tmp_x = { let __v = (*rc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let new_val = unicode_utf8::RUNE_ERROR as i32; *rc.lock().unwrap() = Some(new_val); };
    }
        let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut r, mut size) = unicode_utf8::decode_last_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))));
        { let __rhs = size; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = { let __v = (*rc.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = r; __tmp_x == __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    }
        return -(1);
    }
    let mut i = Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut r, mut size) = unicode_utf8::decode_last_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))));
        { let __rhs = size; let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
        if { let __tmp_x = index_rune(Arc::new(Mutex::new(Some({ let __arg_holder = chars.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(r)))); let __tmp_y = 0; __tmp_x >= __tmp_y } {
        return { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v };
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
    return Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = a.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; let __high = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize; __seq[..__high].to_vec() })));
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
    for s in &{ let __seq = { let __seq_holder = elems.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize..].to_vec() } {
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

/// Map returns a copy of the string s with all its characters modified
/// according to the mapping function. If mapping returns a negative value, the character is
/// dropped from the string with no replacement.
pub fn map(mapping: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>>>>, mut s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        // In the worst case, the string can grow when mapped, making
        // things unpleasant. But it's so rare we barge in assuming it's
        // fine. It could also shrink but that falls out naturally.
        // The output buffer b is initialized on demand, the first
        // time a character differs.
    let mut b: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));

    for (i, __range_c) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        let mut c = __range_c as i32;
        let mut r = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = { let mut __f_guard = mapping.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(c)))) };
        if { let __tmp_x = r; let __tmp_y = c; __tmp_x == __tmp_y } && { let __tmp_x = c; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x != __tmp_y } {
        continue
    }
        let mut width: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = c; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (i) as usize; __s[__low..].to_string() })))); c = __tmp_0; *width.lock().unwrap() = Some(__tmp_1); };
        if { let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x != __tmp_y } && { let __tmp_x = r; let __tmp_y = c; __tmp_x == __tmp_y } {
        continue
    }
    } else {
        { let new_val = unicode_utf8::rune_len(Arc::new(Mutex::new(Some(c)))); *width.lock().unwrap() = Some(new_val); };
    }
        (*b.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = unicode_utf8::U_T_F_MAX; __tmp_x + __tmp_y }))));
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __high = (i) as usize; __s[..__high].to_string() }))));
        if { let __tmp_x = r; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(r))));
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __tmp_x = i as i32; let __tmp_y = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        break
    }

        // Fast path for unchanged input
    if { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).cap(); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

    for (_, c) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        let mut r = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> = { let mut __f_guard = mapping.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(c as i32)))) };
        if { let __tmp_x = r; let __tmp_y = 0 as i32; __tmp_x >= __tmp_y } {
                // common case
                // Due to inlining, it is more performant to determine if WriteByte should be
                // invoked rather than always call WriteRune
        if { let __tmp_x = r; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some(r as u8))));
    } else {
                // r is not an ASCII rune.
        (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(r))));
    }
    }
    }

        // common case
        // Due to inlining, it is more performant to determine if WriteByte should be
        // invoked rather than always call WriteRune
        // r is not an ASCII rune.
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

/// Repeat returns a new string consisting of count copies of the string s.
///
/// It panics if count is negative or if the result of (len(s) * count)
/// overflows.
pub fn repeat(s: Arc<Mutex<Option<String>>>, count: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (0) {
            return Arc::new(Mutex::new(Some("".to_string())));
        } else if _switch_val == (1) {
            return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        }
    }

        // Since we cannot return an error on overflow,
        // we should panic if the repeat will generate an overflow.
        // See golang.org/issue/16237.
    if { let __tmp_x = { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        std::panic::panic_any(Box::new("strings: negative Repeat count".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let (mut hi, mut lo) = math_bits::mul(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as u64))), Arc::new(Mutex::new(Some((*count.lock().unwrap().as_ref().unwrap()) as u64))));
    if { let __tmp_x = hi; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } || { let __tmp_x = lo; let __tmp_y = (*Arc::new(Mutex::new(Some(MAX_INT as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x > __tmp_y } {
        std::panic::panic_any(Box::new("strings: Repeat output length overflow".to_string()) as Box<dyn Any + Send + Sync>);
    }
    let mut n = Arc::new(Mutex::new(Some(lo as i32)));

    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return Arc::new(Mutex::new(Some("".to_string())));
    }

        // Optimize for commonly repeated strings of relatively short length.
    { let _switch_val = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] };
    if _switch_val == ((' ' as i32) as u8) || _switch_val == (('-' as i32) as u8) || _switch_val == (('0' as i32) as u8) || _switch_val == (('=' as i32) as u8) || _switch_val == (('\t' as i32) as u8) {
            if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 128; __tmp_x <= __tmp_y } && has_prefix(Arc::new(Mutex::new(Some(REPEATED_SPACES.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some({ let __s = &(REPEATED_SPACES); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 128; __tmp_x <= __tmp_y } && has_prefix(Arc::new(Mutex::new(Some(REPEATED_DASHES.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some({ let __s = &(REPEATED_DASHES); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 64; __tmp_x <= __tmp_y } && has_prefix(Arc::new(Mutex::new(Some(REPEATED_ZEROES.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some({ let __s = &(REPEATED_ZEROES); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 128; __tmp_x <= __tmp_y } && has_prefix(Arc::new(Mutex::new(Some(REPEATED_EQUALS.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some({ let __s = &(REPEATED_EQUALS); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        } else if { let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = 64; __tmp_x <= __tmp_y } && has_prefix(Arc::new(Mutex::new(Some(REPEATED_TABS.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
            return Arc::new(Mutex::new(Some({ let __s = &(REPEATED_TABS); let __high = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() })));
        }
        }
    }

        // Past a certain chunk size it is counterproductive to use
        // larger chunks as the source of the write, as when the source
        // is too large we are basically just thrashing the CPU D-cache.
        // So if the result length is larger than an empirically-found
        // limit (8KB), we stop growing the source string once the limit
        // is reached and keep reusing the same source string - that
        // should therefore be always resident in the L1 cache - until we
        // have completed the construction of the result.
        // This yields significant speedups (up to +100%) in cases where
        // the result length is large (roughly, over L2 cache size).
    const chunkLimit: i32 = 8 * 1024;

    let mut chunkMax = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8192; __tmp_x > __tmp_y } {
        { let new_val = { let __tmp_x = ({ let __tmp_x = 8192; let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x / __tmp_y } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x * __tmp_y }; *chunkMax.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*chunkMax.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = (*s.lock().unwrap().as_ref().unwrap()).len() as i32; *chunkMax.lock().unwrap() = Some(new_val); };
    }
    }

    let mut b: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*b.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    while { let __tmp_x = (*b.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut chunk = Arc::new(Mutex::new(Some(std::cmp::min(std::cmp::min(({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*b.lock().unwrap().as_ref().unwrap()).len(); __tmp_x - __tmp_y } as i32), ((*b.lock().unwrap().as_ref().unwrap()).len() as i32)), ({ let __v = (*chunkMax.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32)))));
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*(*b.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __v = (*chunk.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[..__high].to_string() }))));
    }
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

/// ToLower returns s with all Unicode letters mapped to their lower case.
pub fn to_lower(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    let (mut isASCII, mut hasUpper) = (Arc::new(Mutex::new(Some(true))), Arc::new(Mutex::new(Some(false))));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        { let new_val = false; *isASCII.lock().unwrap() = Some(new_val); };
        break
    }
        { let new_val = { let __v = (*hasUpper.lock().unwrap().as_ref().unwrap()).clone(); __v } || ({ let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y }); *hasUpper.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    if { let __v = (*isASCII.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if !{ let __v = (*hasUpper.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }
        let mut b: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));let mut pos: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        (*b.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as i32))));
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } {
        { let __rhs = { let __tmp_x = ('a' as i32); let __tmp_y = ('A' as i32); __tmp_x - __tmp_y } as u8; let mut guard = c.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        if { let __tmp_x = { let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))));
    }
        (*b.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *pos.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if { let __tmp_x = ({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*pos.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))));
    }
        return (*b.lock().unwrap().as_ref().unwrap()).string();
    }
    map(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<i32>>>| -> i32 { unicode::to_lower(__arg0) }) as Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> i32 + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

/// IndexFunc returns the index into s of the first Unicode
/// code point satisfying f(c), or -1 if none do.
pub fn index_func(s: Arc<Mutex<Option<String>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) -> i32 {
    index_func_1(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), f.clone(), Arc::new(Mutex::new(Some(true))))
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

/// Replace returns a copy of the string s with the first n
/// non-overlapping instances of old replaced by new.
/// If old is empty, it matches at the beginning of the string
/// and after each UTF-8 sequence, yielding up to k+1 replacements
/// for a k-rune string.
/// If n < 0, there is no limit on the number of replacements.
pub fn replace(s: Arc<Mutex<Option<String>>>, old: Arc<Mutex<Option<String>>>, new: Arc<Mutex<Option<String>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {
    if { let __tmp_x = (*old.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*new.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    }

        // avoid allocation
        // Compute number of replacements.
    {
        let mut m = count(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __tmp_x = m; let __tmp_y = 0; __tmp_x == __tmp_y } {
            return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        } else if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = m; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = m; *n.lock().unwrap() = Some(new_val); };
    }
    }

        // avoid allocation
        // Apply replacements to buffer.
    let mut b: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    (*b.lock().unwrap().as_mut().unwrap()).grow(Arc::new(Mutex::new(Some({ let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __tmp_x = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = (({ let __tmp_x = ((*new.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*old.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y }) as i32); __tmp_x * __tmp_y } as i32); __tmp_x + __tmp_y }))));
    let mut start = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut j = { let __owned = start.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = ((*old.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (_, mut wid) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))));
        { let __rhs = wid; let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    } else {
        { let __rhs = index(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..__high].to_string() }))));
        (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let new_val = { let __tmp_x = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*old.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y }; *start.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    (*b.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))));
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

/// ReplaceAll returns a copy of the string s with all
/// non-overlapping instances of old replaced by new.
/// If old is empty, it matches at the beginning of the string
/// and after each UTF-8 sequence, yielding up to k+1 replacements
/// for a k-rune string.
pub fn replace_all(s: Arc<Mutex<Option<String>>>, old: Arc<Mutex<Option<String>>>, new: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    replace(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = old.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = new.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(-1))))
}

/// EqualFold reports whether s and t, interpreted as UTF-8 strings,
/// are equal under simple Unicode case-folding, which is a more general
/// form of case-insensitivity.
pub fn equal_fold(mut s: Arc<Mutex<Option<String>>>, mut t: Arc<Mutex<Option<String>>>) -> bool {
        // ASCII fast path
    let mut i = Arc::new(Mutex::new(Some(0)));
    'has_unicode: {
        while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut sr = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        let mut tr = Arc::new(Mutex::new(Some({ let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
        break 'has_unicode;
    }

                // Easy case.
        if { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Make sr < tr to simplify what follows.
        if { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let __tmp_0 = (*sr.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_1 = (*tr.lock().unwrap().as_ref().unwrap()).clone(); *tr.lock().unwrap() = Some(__tmp_0); *sr.lock().unwrap() = Some(__tmp_1); };
    }

                // ASCII only, sr/tr must be upper/lower case
        if { let __tmp_x = ('A' as i32) as u8; let __tmp_y = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('Z' as i32) as u8; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = { let __v = (*sr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('a' as i32) as u8; __tmp_x + __tmp_y }; let __tmp_y = ('A' as i32) as u8; __tmp_x - __tmp_y }; __tmp_x == __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        return false;
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

                // Easy case.
                // Make sr < tr to simplify what follows.
                // ASCII only, sr/tr must be upper/lower case
                // Check if we've exhausted both strings.
        return { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x == __tmp_y };

    }
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *t.lock().unwrap() = __moved_val; };
    for (_, __range_sr) in (*s.lock().unwrap().as_ref().unwrap()).char_indices() {
        let mut sr = __range_sr as i32;
                // If t is exhausted the strings are not equal.
        if { let __tmp_x = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return false;
    }
                // Extract first rune from second string.
        let mut tr: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = { let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x < __tmp_y } {
        { let __tmp_0 = Arc::new(Mutex::new(Some({ let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32))); let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); *tr.lock().unwrap() = __tmp_0.lock().unwrap().take(); *t.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    } else {
        let (mut r, mut size) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        { let __tmp_0 = r; let __tmp_1 = Arc::new(Mutex::new(Some({ let __s = &((*t.lock().unwrap().as_ref().unwrap()).clone()); let __low = (size) as usize; __s[__low..].to_string() }))); *tr.lock().unwrap() = Some(__tmp_0); *t.lock().unwrap() = __tmp_1.lock().unwrap().take(); };
    }
                // If they match, keep going; if not, return false.
                // Easy case.
        if { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = sr; __tmp_x == __tmp_y } {
        continue
    }
                // Make sr < tr to simplify what follows.
        if { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = sr; __tmp_x < __tmp_y } {
        { let __tmp_0 = sr; let __tmp_1 = (*tr.lock().unwrap().as_ref().unwrap()).clone(); *tr.lock().unwrap() = Some(__tmp_0); sr = __tmp_1; };
    }
                // Fast check for ASCII.
        if { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } {
                // ASCII only, sr/tr must be upper/lower case
        if { let __tmp_x = ('A' as i32); let __tmp_y = sr; __tmp_x <= __tmp_y } && { let __tmp_x = sr; let __tmp_y = ('Z' as i32); __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __tmp_x = sr; let __tmp_y = ('a' as i32); __tmp_x + __tmp_y }; let __tmp_y = ('A' as i32); __tmp_x - __tmp_y }; __tmp_x == __tmp_y } {
        continue
    }
        return false;
    }
                // ASCII only, sr/tr must be upper/lower case
                // General case. SimpleFold(x) returns the next equivalent rune > x
                // or wraps around to smaller values.
        let mut r = unicode::simple_fold(Arc::new(Mutex::new(Some(sr))));
        while { let __tmp_x = r; let __tmp_y = sr; __tmp_x != __tmp_y } && { let __tmp_x = r; let __tmp_y = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        { let new_val = unicode::simple_fold(Arc::new(Mutex::new(Some(r)))); r = new_val; };
    }
        if { let __tmp_x = r; let __tmp_y = { let __v = (*tr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        continue
    }
        return false;
    }

        // If t is exhausted the strings are not equal.
        // Extract first rune from second string.
        // If they match, keep going; if not, return false.
        // Easy case.
        // Make sr < tr to simplify what follows.
        // Fast check for ASCII.
        // ASCII only, sr/tr must be upper/lower case
        // General case. SimpleFold(x) returns the next equivalent rune > x
        // or wraps around to smaller values.
        // First string is empty, so check if the second one is also empty.
    return { let __tmp_x = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y };
    unreachable!()
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

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
