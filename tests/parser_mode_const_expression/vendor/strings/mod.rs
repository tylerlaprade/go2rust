use go2rust_stdlib_stubs::*;

use crate::builder::*;
use crate::clone::*;
use crate::compare::*;
use crate::iter::*;
use crate::reader::*;
use crate::replace::*;
use crate::search::*;

use std::sync::{Arc, Mutex};

pub(crate) const MAX_INT: i32 = i32::MAX;


pub(crate) const REPEATED_SPACES: &'static str = "                                                                                                                                ";
pub(crate) const REPEATED_DASHES: &'static str = "--------------------------------------------------------------------------------------------------------------------------------";
pub(crate) const REPEATED_ZEROES: &'static str = "0000000000000000000000000000000000000000000000000000000000000000";
pub(crate) const REPEATED_EQUALS: &'static str = "================================================================================================================================";
pub(crate) const REPEATED_TABS: &'static str = "\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t";


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


/// explode splits s into a slice of UTF-8 strings,
/// one string per Unicode character up to a maximum of n (n < 0 means no limit).
/// Invalid UTF-8 bytes are sliced individually.
pub fn explode(mut s: Arc<Mutex<Option<String>>>, mut n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut l = utf8::rune_count_in_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = l; __tmp_x > __tmp_y } {
        { let new_val = l; *n.lock().unwrap() = Some(new_val); };
    }
    let mut a = Arc::new(Mutex::new(Some(vec!["".to_string(); ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }; __tmp_x < __tmp_y } {
        let (_, mut size) = utf8::decode_rune_in_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
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
        return { let __tmp_x = utf8::rune_count_in_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); let __tmp_y = 1; __tmp_x + __tmp_y };
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
        panic!("strings: Join output length overflow");
    }
        { let __rhs = { let __tmp_x = ((*sep.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (({ let __tmp_x = ((*elems.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as i32); __tmp_x * __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    { let __range_holder = elems.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for elem in __range_values.iter() {
        if { let __tmp_x = (elem.len() as i32); let __tmp_y = ({ let __tmp_x = i32::MAX; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y } as i32); __tmp_x > __tmp_y } {
        panic!("strings: Join output length overflow");
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
        panic!("strings: negative Repeat count");
    }
    let (mut hi, mut lo) = bits::mul(Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()).len() as u64))), Arc::new(Mutex::new(Some((*count.lock().unwrap().as_ref().unwrap()) as u64))));
    if { let __tmp_x = hi; let __tmp_y = 0 as u64; __tmp_x > __tmp_y } || { let __tmp_x = lo; let __tmp_y = (*Arc::new(Mutex::new(Some(MAX_INT as u64))).lock().unwrap().as_ref().unwrap()) as u64; __tmp_x > __tmp_y } {
        panic!("strings: Repeat output length overflow");
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

/// Index returns the index of the first instance of substr in s, or -1 if substr is not present in s.
pub fn index(s: Arc<Mutex<Option<String>>>, substr: Arc<Mutex<Option<String>>>) -> i32 {
    internal_stringslite::index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = substr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
