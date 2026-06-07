use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::atoi::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;

use std::sync::{Arc, Mutex};

pub(crate) const LOWERHEX: &'static str = "0123456789abcdef";
pub(crate) const UPPERHEX: &'static str = "0123456789ABCDEF";


pub fn quote_with(s: Arc<Mutex<Option<String>>>, quote: Arc<Mutex<Option<u8>>>, ASCIIonly: Arc<Mutex<Option<bool>>>, graphicOnly: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(String::from_utf8((*append_quoted_with(Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = ({ let __tmp_x = 3; let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x * __tmp_y } as i32); let __tmp_y = 2; __tmp_x / __tmp_y }) as usize)))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = quote.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ASCIIonly.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = graphicOnly.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())))
}

pub fn append_quoted_with(mut buf: Arc<Mutex<Option<Vec<u8>>>>, mut s: Arc<Mutex<Option<String>>>, quote: Arc<Mutex<Option<u8>>>, ASCIIonly: Arc<Mutex<Option<bool>>>, graphicOnly: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        // Often called with big strings, so preallocate. If there's quoting,
        // this is conservative but still helps a lot.
    if { let __tmp_x = ({ let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.capacity()).unwrap_or(0) as i32); let __tmp_y = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x - __tmp_y } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        let mut nBuf = Arc::new(Mutex::new(Some({ let mut v = Vec::with_capacity(({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y }) as usize); v.resize(((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)) as usize, 0); v })));
        { let _src = { let __copy_src_holder = buf.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().cloned().unwrap_or_default() }; let _n = std::cmp::min((*nBuf.lock().unwrap().as_ref().unwrap()).len(), _src.len()); for _i in 0.._n { (*nBuf.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone(); } Arc::new(Mutex::new(Some(_n as i32))) };
        { let new_val = nBuf.clone(); buf = new_val; };
    }
    { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*quote.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
    let mut width = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut r = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32)));
        { let new_val = 1; *width.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = utf8::decode_rune_in_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }); *r.lock().unwrap() = Some(__tmp_0); *width.lock().unwrap() = Some(__tmp_1); };
    }
        if { let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\x".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = 4; __tmp_x >> __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = 0xF as u8; __tmp_x & __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };; continue
    }
        { let new_val = append_escaped_rune(buf.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = quote.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = ASCIIonly.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = graphicOnly.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); buf = new_val; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
    { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*quote.lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
    return buf.clone();
}

pub fn append_escaped_rune(mut buf: Arc<Mutex<Option<Vec<u8>>>>, mut r: Arc<Mutex<Option<i32>>>, quote: Arc<Mutex<Option<u8>>>, ASCIIonly: Arc<Mutex<Option<bool>>>, graphicOnly: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*quote.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(('\\' as i32) as u8); __append_target.clone() }; buf = new_val; };
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
        return buf.clone();
    }
    if { let __v = (*ASCIIonly.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } && is_print(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
        return buf.clone();
    }
    } else if is_print(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __v = (*graphicOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_in_graphic_list(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return utf8::append_rune(buf.clone(), { let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
    }
    { let _switch_val = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('\u{7}' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\a".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\u{8}' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\b".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\u{c}' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\f".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\n' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\n".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\r' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\r".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\t' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\t".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else if _switch_val == (('\u{b}' as i32)) {
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\v".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
        } else {
            {
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (' ' as i32); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7f as i32; __tmp_x == __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\x".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 4; __tmp_x >> __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); let __tmp_y = 0xF as u8; __tmp_x & __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        }
        if !_matched && (!utf8::valid_rune({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = 0xFFFD as i32; *r.lock().unwrap() = Some(new_val); };
            _fallthrough = true;
        }
        if !_matched && ({ let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x10000 as i32; __tmp_x < __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\u".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            let mut s = Arc::new(Mutex::new(Some(12)));
    while { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >> __tmp_y }; let __tmp_y = 0xF as i32; __tmp_x & __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let __rhs = 4; let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).extend("\\U".to_string().as_bytes().iter().cloned()); __append_target.clone() }; buf = new_val; };
            let mut s = Arc::new(Mutex::new(Some(28)));
    while { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x >= __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &(LOWERHEX); __s.as_bytes()[({ let __tmp_x = { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*s.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x >> __tmp_y }; let __tmp_y = 0xF as i32; __tmp_x & __tmp_y }) as usize] }); __append_target.clone() }; buf = new_val; };
        { let __rhs = 4; let mut guard = s.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    }
        }
    }
        }
    }
    return buf.clone();
}

/// Quote returns a double-quoted Go string literal representing s. The
/// returned string uses Go escape sequences (\t, \n, \xFF, \u0100) for
/// control characters and non-printable characters as defined by
/// [IsPrint].
pub fn quote(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    quote_with(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(('"' as i32) as u8))), Arc::new(Mutex::new(Some(false))), Arc::new(Mutex::new(Some(false))))
}

/// bsearch is semantically the same as [slices.BinarySearch] (without NaN checks)
/// We copied this function because we can not import "slices" here.
pub fn bsearch<S, E: GoInteger + Clone + Send + Sync + 'static>(s: Arc<Mutex<Option<Vec<E>>>>, v: E) -> (i32, bool) {
    let mut n = Arc::new(Mutex::new(Some((*s.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32)));
    let (mut i, mut j) = (Arc::new(Mutex::new(Some(0))), { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) });
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let mut h = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __tmp_x = ({ let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y }); let __tmp_y = 1; __tmp_x >> __tmp_y }; __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = v.clone(); __tmp_x < __tmp_y } {
        { let new_val = { let __tmp_x = { let __v = (*h.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }; *i.lock().unwrap() = Some(new_val); };
    } else {
        { let new_val = h.lock().unwrap().as_ref().unwrap().clone(); *j.lock().unwrap() = Some(new_val); };
    }
    }
    return ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }, { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } && { let __tmp_x = { let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }; let __tmp_y = v.clone(); __tmp_x == __tmp_y });
}

/// IsPrint reports whether the rune is defined as printable by Go, with
/// the same definition as [unicode.IsPrint]: letters, numbers, punctuation,
/// symbols and ASCII space.
pub fn is_print(mut r: Arc<Mutex<Option<i32>>>) -> bool {
        // Fast check for Latin-1
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFF as i32; __tmp_x <= __tmp_y } {
        if { let __tmp_x = 0x20 as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x7E as i32; __tmp_x <= __tmp_y } {
                // All the ASCII is printable from space through DEL-1.
        return true;
    }
                // All the ASCII is printable from space through DEL-1.
        if { let __tmp_x = 0xA1 as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFF as i32; __tmp_x <= __tmp_y } {
                // Similarly for ¡ through ÿ...
        return { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xAD as i32; __tmp_x != __tmp_y };
    }
                // Similarly for ¡ through ÿ...
                // ...except for the bizarre soft hyphen.
        return false;
    }

        // All the ASCII is printable from space through DEL-1.
        // Similarly for ¡ through ÿ...
        // ...except for the bizarre soft hyphen.
        // Same algorithm, either on uint16 or uint32 value.
        // First, find first i such that isPrint[i] >= x.
        // This is the index of either the start or end of a pair that might span x.
        // The start is even (isPrint[i&^1]) and the end is odd (isPrint[i|1]).
        // If we find x in a range, make sure x is not in isNotPrint list.
    if { let __tmp_x = 0 as i32; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ((1 as i32) << (16 as i32)) as i32; __tmp_x < __tmp_y } {
        let (mut rr, mut isPrint, mut isNotPrint) = (Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u16))), Arc::new(Mutex::new(Some((*isPrint16.lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(Some((*isNotPrint16.lock().unwrap().as_ref().unwrap()).clone()))));
        let (mut i, _) = bsearch::<Vec<u16>, u16>(isPrint.clone(), (*rr.lock().unwrap().as_ref().unwrap()).clone());
        if { let __tmp_x = (i as i32); let __tmp_y = ((*isPrint.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } || { let __tmp_x = { let __v = (*rr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = isPrint.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x & ! __tmp_y }) as usize].clone() }; __tmp_x < __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = isPrint.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x | __tmp_y }) as usize].clone() }; let __tmp_y = { let __v = (*rr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return false;
    }
        let (_, mut found) = bsearch::<Vec<u16>, u16>(isNotPrint.clone(), (*rr.lock().unwrap().as_ref().unwrap()).clone());
        return !found;
    }

    let (mut rr, mut isPrint, mut isNotPrint) = (Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u32))), Arc::new(Mutex::new(Some((*isPrint32.lock().unwrap().as_ref().unwrap()).clone()))), Arc::new(Mutex::new(Some((*isNotPrint32.lock().unwrap().as_ref().unwrap()).clone()))));
    let (mut i, _) = bsearch::<Vec<u32>, u32>(isPrint.clone(), (*rr.lock().unwrap().as_ref().unwrap()).clone());
    if { let __tmp_x = (i as i32); let __tmp_y = ((*isPrint.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x >= __tmp_y } || { let __tmp_x = { let __v = (*rr.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __seq = { let __seq_holder = isPrint.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x & ! __tmp_y }) as usize].clone() }; __tmp_x < __tmp_y } || { let __tmp_x = { let __seq = { let __seq_holder = isPrint.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = i; let __tmp_y = 1; __tmp_x | __tmp_y }) as usize].clone() }; let __tmp_y = { let __v = (*rr.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        return false;
    }
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0x20000 as i32; __tmp_x >= __tmp_y } {
        return true;
    }
    { let __rhs = 0x10000 as i32; let mut guard = r.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - __rhs); };
    let (_, mut found) = bsearch::<Vec<u16>, u16>(isNotPrint.clone(), (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u16))).lock().unwrap().as_ref().unwrap()).clone());
    !found
}

/// isInGraphicList reports whether the rune is in the isGraphic list. This separation
/// from IsGraphic allows quoteWith to avoid two calls to IsPrint.
/// Should be called only if IsPrint fails.
pub fn is_in_graphic_list(r: Arc<Mutex<Option<i32>>>) -> bool {
        // We know r must fit in 16 bits - see makeisprint.go.
    if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0xFFFF as i32; __tmp_x > __tmp_y } {
        return false;
    }
    let (_, mut found) = bsearch::<Vec<u16>, u16>(isGraphic.clone(), (*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u16))).lock().unwrap().as_ref().unwrap()).clone());
    found
}