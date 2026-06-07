use go2rust_stdlib_stubs::*;

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
        } else if { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = utf8::RUNE_SELF as u8; __tmp_x >= __tmp_y } {
            let (mut r, mut size) = utf8::decode_rune_in_string({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() });
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
            if !utf8::valid_rune({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) {
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
            { let new_val = utf8::valid_string(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("\"".len()) as usize; let __high = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() })))); *valid.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('\'' as i32) as u8) {
            let (mut r, mut n) = utf8::decode_rune_in_string(Arc::new(Mutex::new(Some({ let __s = &((*r#in.lock().unwrap().as_ref().unwrap()).clone()); let __low = ("'".len()) as usize; let __high = ({ let __tmp_x = (end as i32); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))));
            { let new_val = { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = 1; let __tmp_y = (n as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = (end as i32); __tmp_x == __tmp_y } && ({ let __tmp_x = r; let __tmp_y = utf8::RUNE_ERROR as i32; __tmp_x != __tmp_y } || { let __tmp_x = n; let __tmp_y = 1; __tmp_x != __tmp_y }); *valid.lock().unwrap() = Some(new_val); };
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
        if { let __tmp_x = r; let __tmp_y = utf8::RUNE_SELF as i32; __tmp_x < __tmp_y } || !multibyte {
        { let new_val = { let __append_target = buf.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(r as u8))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; buf = new_val; };
    } else {
        { let new_val = utf8::append_rune(buf.clone(), r); buf = new_val; };
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