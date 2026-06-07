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

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub(crate) const LOWERHEX: &'static str = "0123456789abcdef";
pub(crate) const UPPERHEX: &'static str = "0123456789ABCDEF";


/// contains reports whether the string contains the byte c.
pub fn contains(s: Arc<Mutex<Option<String>>>, c: Arc<Mutex<Option<u8>>>) -> bool {
    return { let __tmp_x = index(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = c.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __tmp_y = -1; __tmp_x != __tmp_y };
}

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
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x >= __tmp_y } {
        { let (__tmp_0, __tmp_1) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *r.lock().unwrap() = Some(__tmp_0); *width.lock().unwrap() = Some(__tmp_1); };
    }
        if { let __tmp_x = { let __v = (*width.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } {
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
        if { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } && is_print(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some((*r.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
        return buf.clone();
    }
    } else if is_print(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) || { let __v = (*graphicOnly.lock().unwrap().as_ref().unwrap()).clone(); __v } && is_in_graphic_list(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return unicode_utf8::append_rune(buf.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
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
        if !_matched && (!unicode_utf8::valid_rune(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))) || _fallthrough {
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

pub fn unhex(b: Arc<Mutex<Option<u8>>>) -> (i32, bool) {
    let mut v: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));

    let mut c = Arc::new(Mutex::new(Some((*b.lock().unwrap().as_ref().unwrap()) as i32)));
    if { let __tmp_x = ('0' as i32); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('9' as i32); __tmp_x <= __tmp_y } {
            return ({ let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x - __tmp_y }, true);
        } else if { let __tmp_x = ('a' as i32); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('f' as i32); __tmp_x <= __tmp_y } {
            return ({ let __tmp_x = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('a' as i32); __tmp_x - __tmp_y }; let __tmp_y = 10 as i32; __tmp_x + __tmp_y }, true);
        } else if { let __tmp_x = ('A' as i32); let __tmp_y = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('F' as i32); __tmp_x <= __tmp_y } {
            return ({ let __tmp_x = { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('A' as i32); __tmp_x - __tmp_y }; let __tmp_y = 10 as i32; __tmp_x + __tmp_y }, true);
        }
    return ((*v.lock().unwrap().as_ref().unwrap()), (*ok.lock().unwrap().as_ref().unwrap()));
}

/// UnquoteChar decodes the first character or byte in the escaped string
/// or character literal represented by the string s.
/// It returns four values:
///
///  1. value, the decoded Unicode code point or byte value;
///  2. multibyte, a boolean indicating whether the decoded character requires a multibyte UTF-8 representation;
///  3. tail, the remainder of the string after the character; and
///  4. an error that will be nil if the character is syntactically valid.
///
/// The second argument, quote, specifies the type of literal being parsed
/// and therefore which escaped quote character is permitted.
/// If set to a single quote, it permits the sequence \' and disallows unescaped '.
/// If set to a double quote, it permits \" and disallows unescaped ".
/// If set to zero, it does not permit either escape and allows both quote characters to appear unescaped.
pub fn unquote_char(mut s: Arc<Mutex<Option<String>>>, quote: Arc<Mutex<Option<u8>>>) -> (i32, bool, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut value: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut multibyte: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut tail: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // easy cases
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
    let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } && ({ let __tmp_x = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\'' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('"' as i32) as u8; __tmp_x == __tmp_y }) {
            { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
            return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
        } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = unicode_utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
            let (mut r, mut size) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
            return (r, true, Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (size) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
        } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\\' as i32) as u8; __tmp_x != __tmp_y } {
            return ((*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] } as i32))).lock().unwrap().as_ref().unwrap()), false, Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
        }

        // hard case: c is backslash
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x <= __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
    let mut c = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })));
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };

    '__go_switch_1: loop {
        { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('a' as i32) as u8) {
            { let new_val = ('\u{7}' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('b' as i32) as u8) {
            { let new_val = ('\u{8}' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('f' as i32) as u8) {
            { let new_val = ('\u{c}' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('n' as i32) as u8) {
            { let new_val = ('\n' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('r' as i32) as u8) {
            { let new_val = ('\r' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('t' as i32) as u8) {
            { let new_val = ('\t' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('v' as i32) as u8) {
            { let new_val = ('\u{b}' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('x' as i32) as u8) || _switch_val == (('u' as i32) as u8) || _switch_val == (('U' as i32) as u8) {
            let mut n = Arc::new(Mutex::new(Some(0)));
            { let _switch_val = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('x' as i32) as u8) {
            { let new_val = 2; *n.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('u' as i32) as u8) {
            { let new_val = 4; *n.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('U' as i32) as u8) {
            { let new_val = 8; *n.lock().unwrap() = Some(new_val); };
        }
    }
            let mut v: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
            let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } {
        let (mut x, mut ok) = unhex(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }))));
        if !ok {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x << __tmp_y }; let __tmp_y = x; __tmp_x | __tmp_y }; *v.lock().unwrap() = Some(new_val); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
            if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y } {
                // single-byte string, possibly not UTF-8
        { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *value.lock().unwrap() = Some(new_val); };
        break '__go_switch_1
    }
                        // single-byte string, possibly not UTF-8
            if !unicode_utf8::valid_rune(Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
            { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *value.lock().unwrap() = Some(new_val); };
            { let new_val = true; *multibyte.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('0' as i32) as u8) || _switch_val == (('1' as i32) as u8) || _switch_val == (('2' as i32) as u8) || _switch_val == (('3' as i32) as u8) || _switch_val == (('4' as i32) as u8) || _switch_val == (('5' as i32) as u8) || _switch_val == (('6' as i32) as u8) || _switch_val == (('7' as i32) as u8) {
            let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32); __tmp_x - __tmp_y })));
            if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
            let mut j = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = { let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        let mut x = Arc::new(Mutex::new(Some({ let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] } as i32))).lock().unwrap().as_ref().unwrap()); let __tmp_y = ('0' as i32); __tmp_x - __tmp_y })));
        if { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0 as i32; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 7 as i32; __tmp_x > __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
        { let new_val = { let __tmp_x = ({ let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x << __tmp_y }); let __tmp_y = { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y }; *v.lock().unwrap() = Some(new_val); };
        { let mut guard = j.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
            if { let __tmp_x = { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 255 as i32; __tmp_x > __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
            { let new_val = v.lock().unwrap().as_ref().unwrap().clone(); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('\\' as i32) as u8) {
            { let new_val = ('\\' as i32); *value.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('\'' as i32) as u8) || _switch_val == (('"' as i32) as u8) {
            if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
        { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
    }
            { let new_val = Arc::new(Mutex::new(Some((*c.lock().unwrap().as_ref().unwrap()) as i32))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *value.lock().unwrap() = __moved_val; };
        } else {
            { let __rhs_holder = ErrSyntax.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
            return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
        }
    };
        break;
    }
        // single-byte string, possibly not UTF-8
        // one digit already; two more
    { let new_val = s.lock().unwrap().as_ref().unwrap().clone(); *tail.lock().unwrap() = Some(new_val); };
    return ((*value.lock().unwrap().as_ref().unwrap()), (*multibyte.lock().unwrap().as_ref().unwrap()), tail.clone(), err.clone());
}

/// Unquote interprets s as a single-quoted, double-quoted,
/// or backquoted Go string literal, returning the string value
/// that s quotes.  (If s is single-quoted, it would be a Go
/// character literal; Unquote returns the corresponding
/// one-character string. For an empty character literal
/// Unquote returns the empty string.)
pub fn unquote(s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut out, mut rem, mut err) = unquote_1(Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));
    if { let __tmp_x = ((*rem.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), ErrSyntax.clone());
    }
    return ({ let __owned = out.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, err.clone());
}

/// unquote parses a quoted string at the start of the input,
/// returning the parsed prefix, the remaining suffix, and any parse errors.
/// If unescape is true, the parsed prefix is unescaped,
/// otherwise the input prefix is provided verbatim.
pub fn unquote_1(mut r#in: Arc<Mutex<Option<String>>>, unescape: Arc<Mutex<Option<bool>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut out: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut rem: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // Determine the quote form and optimistically find the terminating quote.
    if { let __tmp_x = ((*r#in.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ErrSyntax.clone());
    }
    let mut quote = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] })));
    let mut end = index(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(Some({ let __arg_holder = quote.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    if { let __tmp_x = end; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ErrSyntax.clone());
    }
    { let __rhs = 2; end = end + __rhs; };

    { let _switch_val = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('`' as i32) as u8) {
            if !{ let __v = (*unescape.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __high = (end) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };
        } else if !contains(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __high = (end) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some(('\r' as i32) as u8)))) {
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("`".len()) as usize; let __high = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };
        } else {
                        // Carriage return characters ('\r') inside raw string literals
                        // are discarded from the raw string value.
            let mut buf = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize))));
            let mut i = Arc::new(Mutex::new(Some("`".len() as i32)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y } as i32); __tmp_x < __tmp_y } {
        if { let __tmp_x = { let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('\r' as i32) as u8; __tmp_x != __tmp_y } {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }); __append_target.clone() }; buf = new_val; };
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            { let new_val = Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };
        }
                        // include quotes
                        // exclude quotes
                        // Carriage return characters ('\r') inside raw string literals
                        // are discarded from the raw string value.
                        // NOTE: Prior implementations did not verify that raw strings consist
                        // of valid UTF-8 characters and we continue to not verify it as such.
                        // The Go specification does not explicitly require valid UTF-8,
                        // but only mention that it is implicitly valid for Go source code
                        // (which must be valid UTF-8).
            return ({ let __owned = out.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = (end) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
        } else if _switch_val == (('"' as i32) as u8) || _switch_val == (('\'' as i32) as u8) {
                        // Handle quoted strings without any escape sequences.
            if !contains(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __high = (end) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some(('\\' as i32) as u8)))) && !contains(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __high = (end) as usize; __s[..__high].to_string() }))), Arc::new(Mutex::new(Some(('\n' as i32) as u8)))) {
        let mut valid: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        { let _switch_val = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (('"' as i32) as u8) {
            { let new_val = unicode_utf8::valid_string(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("\"".len()) as usize; let __high = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() })))); *valid.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('\'' as i32) as u8) {
            let (mut r, mut n) = unicode_utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("'".len()) as usize; let __high = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))));
            { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = 1; let __tmp_y = (n as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = (end as i32); __tmp_x == __tmp_y } && ({ let __tmp_x = r; let __tmp_y = unicode_utf8::RUNE_ERROR as i32; __tmp_x != __tmp_y } || { let __tmp_x = n; let __tmp_y = 1; __tmp_x != __tmp_y }); *valid.lock().unwrap() = Some(new_val); };
        }
    }
        if { let __v = (*valid.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __high = (end) as usize; __s[..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };
        if { let __v = (*unescape.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*out.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = ({ let __tmp_x = end; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };
    }
                // exclude quotes
        return ({ let __owned = out.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = (end) as usize; __s[__low..].to_string() }))), Arc::new(Mutex::new(None)));
    }
    }
                        // exclude quotes
                        // Handle quoted strings with escape sequences.
            let mut buf: Arc<Mutex<Option<Vec<u8>>>> = Arc::new(Mutex::new(None));
            let mut in0 = { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r#in.lock().unwrap() = __moved_val; };
            if { let __v = (*unescape.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        { let new_val = Arc::new(Mutex::new(Some(Vec::<u8>::with_capacity(({ let __tmp_x = { let __tmp_x = 3; let __tmp_y = end; __tmp_x * __tmp_y }; let __tmp_y = 2; __tmp_x / __tmp_y }) as usize)))); buf = new_val; };
    }
                        // try to avoid more allocations
            while { let __tmp_x = ((*r#in.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x != __tmp_y } {
                // Process the next character,
                // rejecting any unescaped newline characters which are invalid.
        let (mut r, mut multibyte, mut rem, mut err) = unquote_char(Arc::new(Mutex::new(Some({ let __arg_holder = r#in.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = quote.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        if { let __tmp_x = { let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('\n' as i32) as u8; __tmp_x == __tmp_y } || { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(Some("".to_string()))), { let __owned = in0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ErrSyntax.clone());
    }
        { let new_val = rem.lock().unwrap().as_ref().unwrap().clone(); *r#in.lock().unwrap() = Some(new_val); };

                // Append the character if unescaping the input.
        if { let __v = (*unescape.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = r; let __tmp_y = unicode_utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } || !multibyte {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(r as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
    } else {
        { let new_val = unicode_utf8::append_rune(buf.clone(), Arc::new(Mutex::new(Some(r)))); buf = new_val; };
    }
    }

                // Single quoted strings must be a single character.
        if { let __tmp_x = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('\'' as i32) as u8; __tmp_x == __tmp_y } {
        break
    }
    }
                        // Process the next character,
                        // rejecting any unescaped newline characters which are invalid.
                        // Append the character if unescaping the input.
                        // Single quoted strings must be a single character.
                        // Verify that the string ends with a terminating quote.
            if !({ let __tmp_x = ((*r#in.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = { let __v = (*quote.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y }) {
        return (Arc::new(Mutex::new(Some("".to_string()))), { let __owned = in0.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ErrSyntax.clone());
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *r#in.lock().unwrap() = __moved_val; };
            if { let __v = (*unescape.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some(String::from_utf8((*buf.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))), { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
    }
            return (Arc::new(Mutex::new(Some({ let __s = &((*in0.lock().unwrap().as_ref().unwrap()).clone()); let __high = ({ let __tmp_x = ((*in0.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = ((*r#in.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x - __tmp_y }) as usize; __s[..__high].to_string() }))), { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, Arc::new(Mutex::new(None)));
        } else {
            return (Arc::new(Mutex::new(Some("".to_string()))), { let __owned = r#in.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }, ErrSyntax.clone());
        }
    }
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