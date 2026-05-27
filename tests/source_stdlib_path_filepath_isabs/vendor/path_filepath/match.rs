use go2rust_stdlib_stubs::*;

use crate::path::*;
use crate::path_unix::*;
use crate::symlink::*;
use crate::symlink_unix::*;

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub static ErrBadPattern: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrBadPattern.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("syntax error in pattern".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrBadPattern.lock().unwrap() = new_val; }
}


/// Match reports whether name matches the shell file name pattern.
/// The pattern syntax is:
///
///	pattern:
///		{ term }
///	term:
///		'*'         matches any sequence of non-Separator characters
///		'?'         matches any single non-Separator character
///		'[' [ '^' ] { character-range } ']'
///		            character class (must be non-empty)
///		c           matches character c (c != '*', '?', '\\', '[')
///		'\\' c      matches character c
///
///	character-range:
///		c           matches character c (c != '\\', '-', ']')
///		'\\' c      matches character c
///		lo '-' hi   matches character c for lo <= c <= hi
///
/// Match requires pattern to match all of name, not just a substring.
/// The only possible returned error is [ErrBadPattern], when pattern
/// is malformed.
///
/// On Windows, escaping is disabled. Instead, '\\' is treated as
/// path separator.
pub fn r#match(mut pattern: Arc<Mutex<Option<String>>>, mut name: Arc<Mutex<Option<String>>>) -> (bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut matched: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    'pattern: while { let __tmp_x = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } {
        let mut star: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let mut chunk: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        { let (__tmp_0, __tmp_1, __tmp_2) = scan_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *star.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *pattern.lock().unwrap() = __moved_tmp_2; };
        if { let __v = (*star.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = (*chunk.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ""; __tmp_x == __tmp_y } {
                // Trailing * matches rest of string unless it has a /.
        return (!((*Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __arg = Arc::new(Mutex::new(Some(char::from_u32(((*Separator.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); __s.contains(&__arg) }))).lock().unwrap().as_ref().unwrap())), Arc::new(Mutex::new(None)));
    }

                // Trailing * matches rest of string unless it has a /.
                // Look for match at current position.
        let (mut t, mut ok, mut err) = match_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = chunk.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));

                // if we're the last chunk, make sure we've exhausted the name
                // otherwise we'll give a false result even if we could still match
                // using the star
        if ok && ({ let __tmp_x = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } || { let __tmp_x = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y }) {
        { let new_val = t.lock().unwrap().as_ref().unwrap().clone(); *name.lock().unwrap() = Some(new_val); };
        continue
    }
        if (*err.lock().unwrap()).is_some() {
        return (false, err.clone());
    }
        if { let __v = (*star.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // Look for match skipping i+1 bytes.
                // Cannot skip /.
        let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } && { let __tmp_x = { let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x != __tmp_y } {
        let (mut t, mut ok, mut err) = match_chunk(Arc::new(Mutex::new(Some({ let __arg_holder = chunk.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __s = &((*name.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }) as usize..].to_string() }))));
        if ok {
                // if we're the last chunk, make sure we exhausted the name
        if { let __tmp_x = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } && { let __tmp_x = ((*t.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }
        { let new_val = t.lock().unwrap().as_ref().unwrap().clone(); *name.lock().unwrap() = Some(new_val); };
        continue 'pattern
    }
                // if we're the last chunk, make sure we exhausted the name
        if (*err.lock().unwrap()).is_some() {
        return (false, err.clone());
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
                // Look for match skipping i+1 bytes.
                // Cannot skip /.
                // if we're the last chunk, make sure we exhausted the name
        return (false, Arc::new(Mutex::new(None)));
    }
        // Trailing * matches rest of string unless it has a /.
        // Look for match at current position.
        // if we're the last chunk, make sure we've exhausted the name
        // otherwise we'll give a false result even if we could still match
        // using the star
        // Look for match skipping i+1 bytes.
        // Cannot skip /.
        // if we're the last chunk, make sure we exhausted the name
    ({ let __tmp_x = ((*name.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y }, Arc::new(Mutex::new(None)))
}

/// scanChunk gets the next segment of pattern, which is a non-star string
/// possibly preceded by a star.
pub fn scan_chunk(mut pattern: Arc<Mutex<Option<String>>>) -> (bool, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<String>>>) {
    let mut star: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut chunk: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    while { let __tmp_x = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*pattern.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('*' as u8); __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*pattern.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *pattern.lock().unwrap() = __moved_val; };
        { let new_val = true; *star.lock().unwrap() = Some(new_val); };
    }
    let mut inrange = Arc::new(Mutex::new(Some(false)));
    let mut i: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    { let new_val = 0; *i.lock().unwrap() = Some(new_val); };
    'scan: while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        { let _switch_val = { let __s = &((*pattern.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] };
    if _switch_val == (('\\' as i32) as u8) {
            if { let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x != __tmp_y } {
                // error check handled in matchChunk: bad pattern.
        if { let __tmp_x = ({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*pattern.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        } else if _switch_val == (('[' as i32) as u8) {
            { let new_val = true; *inrange.lock().unwrap() = Some(new_val); };
        } else if _switch_val == ((']' as i32) as u8) {
            { let new_val = false; *inrange.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (('*' as i32) as u8) {
            if !{ let __v = (*inrange.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break 'scan
    }
        }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // error check handled in matchChunk: bad pattern.
    return ({ let __v = (*star.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some({ let __s = &((*pattern.lock().unwrap().as_ref().unwrap()).clone()); __s[(0) as usize..({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].to_string() }))), Arc::new(Mutex::new(Some({ let __s = &((*pattern.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() }))));
}

/// matchChunk checks whether chunk matches the beginning of s.
/// If so, it returns the remainder of s (after the match).
/// Chunk is all single-character operators: literals, char classes, and ?.
pub fn match_chunk(mut chunk: Arc<Mutex<Option<String>>>, mut s: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<String>>>, bool, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut rest: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut ok: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // failed records whether the match has failed.
        // After the match fails, the loop continues on processing chunk,
        // checking that the pattern is well-formed but no longer reading s.
    let mut failed = Arc::new(Mutex::new(Some(false)));
    while { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } {
        if !{ let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } {
        { let new_val = true; *failed.lock().unwrap() = Some(new_val); };
    }
        {
        let _switch_val = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] };
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && (_switch_val == ('[' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
                        // character class
            let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
            if !{ let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        { let (__tmp_0, __tmp_1) = utf8::decode_rune_in_string(s.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *r.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *n.lock().unwrap() = __moved_tmp_1; };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
                        // possibly negated
            let mut negated = Arc::new(Mutex::new(Some(false)));
            if { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('^' as u8); __tmp_x == __tmp_y } {
        { let new_val = true; *negated.lock().unwrap() = Some(new_val); };
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
    }
                        // parse all ranges
            let mut r#match = Arc::new(Mutex::new(Some(false)));
            let mut nrange = Arc::new(Mutex::new(Some(0)));
            loop {
        if { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x > __tmp_y } && { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (']' as u8); __tmp_x == __tmp_y } && { let __tmp_x = { let __v = (*nrange.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
        break
    }
        let mut lo: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut hi: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
        {
        { let (__tmp_0, __tmp_1, __tmp_2) = get_esc(Arc::new(Mutex::new(Some({ let __arg_holder = chunk.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *lo.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(Some("".to_string()))), false, err.clone());;
        }
    }
        { let new_val = lo.lock().unwrap().as_ref().unwrap().clone(); *hi.lock().unwrap() = Some(new_val); };
        if { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as u8); __tmp_x == __tmp_y } {
        {
        { let (__tmp_0, __tmp_1, __tmp_2) = get_esc(Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() })))); *hi.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_2; };;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(Some("".to_string()))), false, err.clone());;
        }
    }
    }
        if { let __tmp_x = { let __v = (*lo.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*hi.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } {
        { let new_val = true; *r#match.lock().unwrap() = Some(new_val); };
    }
        { let mut guard = nrange.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            if { let __tmp_x = { let __v = (*r#match.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*negated.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x == __tmp_y } {
        { let new_val = true; *failed.lock().unwrap() = Some(new_val); };
    }
        }
        if !_matched && (_switch_val == ('?' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if !{ let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = SEPARATOR as u8; __tmp_x == __tmp_y } {
        { let new_val = true; *failed.lock().unwrap() = Some(new_val); };
    }
        let (_, mut n) = utf8::decode_rune_in_string(s.clone());
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s[(n) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
        }
        if !_matched && (_switch_val == ('\\' as i32) as u8) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if { let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(Some("".to_string()))), false, ErrBadPattern.clone());
    }
    }
            _fallthrough = true;
        }
        if !_matched || _fallthrough {
            _matched = true;
            _fallthrough = false;
            if !{ let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        if { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; __tmp_x != __tmp_y } {
        { let new_val = true; *failed.lock().unwrap() = Some(new_val); };
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
        }
    }
    }
        // character class
        // possibly negated
        // parse all ranges
    if { let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (Arc::new(Mutex::new(Some("".to_string()))), false, Arc::new(Mutex::new(None)));
    }
    (Arc::new(Mutex::new(Some(s.lock().unwrap().as_ref().unwrap().clone()))), true, Arc::new(Mutex::new(None)))
}

/// getEsc gets a possibly-escaped character from chunk, for a character class.
pub fn get_esc(mut chunk: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut nchunk: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    if { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as u8); __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = (']' as u8); __tmp_x == __tmp_y } {
        { let __rhs_holder = ErrBadPattern.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*r.lock().unwrap().as_ref().unwrap()), nchunk, err);
    }
    if { let __tmp_x = { let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('\\' as u8); __tmp_x == __tmp_y } && { let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x != __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(1) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *chunk.lock().unwrap() = __moved_val; };
        if { let __tmp_x = ((*chunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } {
        { let __rhs_holder = ErrBadPattern.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
        return ((*r.lock().unwrap().as_ref().unwrap()), nchunk, err);
    }
    }
    let (mut r, mut n) = utf8::decode_rune_in_string(chunk.clone());
    if { let __tmp_x = r; let __tmp_y = utf8::RUNE_ERROR as i32; __tmp_x == __tmp_y } && { let __tmp_x = n; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let __rhs_holder = ErrBadPattern.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*chunk.lock().unwrap().as_ref().unwrap()).clone()); __s[(n) as usize..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *nchunk.lock().unwrap() = __moved_val; };
    if { let __tmp_x = ((*nchunk.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (0 as i32); __tmp_x == __tmp_y } {
        { let __rhs_holder = ErrBadPattern.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *err.lock().unwrap() = new_val; };
    }
    return ((*r.lock().unwrap().as_ref().unwrap()), nchunk, err);
}

/// Glob returns the names of all files matching pattern or nil
/// if there is no matching file. The syntax of patterns is the same
/// as in [Match]. The pattern may describe hierarchical names such as
/// /usr/*/bin/ed (assuming the [Separator] is '/').
///
/// Glob ignores file system errors such as I/O errors reading directories.
/// The only possible returned error is [ErrBadPattern], when pattern
/// is malformed.
pub fn glob(pattern: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut matches: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    glob_with_limit(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(0))))
}

pub fn glob_with_limit(pattern: Arc<Mutex<Option<String>>>, depth: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut matches: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // This limit is used prevent stack exhaustion issues. See CVE-2022-30632.
    const pathSeparatorsLimit: i32 = 10000;

    if { let __tmp_x = { let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = pathSeparatorsLimit; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), ErrBadPattern.clone());
    }

        // Check pattern is well-formed.
    {
        let (_, mut err) = r#match(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("".to_string()))));;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(None)), err.clone());;
        }
    }
    if !(*has_meta(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()) {
        {
        { let (__tmp_0, __tmp_1) = os::lstat(pattern.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };;
        if (*err.lock().unwrap()).is_some() {
            return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)));;
        }
    }
        return (Arc::new(Mutex::new(Some(vec![(*pattern.lock().unwrap().as_ref().unwrap()).clone()]))), Arc::new(Mutex::new(None)));
    }

    let (mut dir, mut file) = split(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let mut volumeLen = Arc::new(Mutex::new(Some(0)));
    if { let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x == __tmp_y } {
        { let (__tmp_0, __tmp_1) = clean_glob_path_windows(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *volumeLen.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_tmp_1; };
    } else {
        { let new_val = clean_glob_path(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *dir.lock().unwrap() = __moved_val; };
    }

    if !(*has_meta(Arc::new(Mutex::new(Some({ let __s = &((*dir.lock().unwrap().as_ref().unwrap()).clone()); __s[({ let __v = (*volumeLen.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize..].to_string() })))).lock().unwrap().as_ref().unwrap()) {
        return glob_1(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(None)));
    }

        // Prevent infinite recursion. See issue 15879.
    if { let __tmp_x = (*dir.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*pattern.lock().unwrap().as_ref().unwrap()).clone(); __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), ErrBadPattern.clone());
    }

    let mut m: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    { let (__tmp_0, __tmp_1) = glob_with_limit(Arc::new(Mutex::new(Some({ let __arg_holder = dir.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*depth.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y })))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *m.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
    if (*err.lock().unwrap()).is_some() {
        return (matches, err);
    }
    { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d in __range_values.iter() {
        { let (__tmp_0, __tmp_1) = glob_1(Arc::new(Mutex::new(Some((*d).clone()))), Arc::new(Mutex::new(Some({ let __arg_holder = file.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), matches.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *matches.lock().unwrap() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        if (*err.lock().unwrap()).is_some() {
        return (matches, err);
    }
    } }
    (matches, err)
}

/// cleanGlobPath prepares path for glob matching.
pub fn clean_glob_path(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    { let _switch_val = (*path.lock().unwrap().as_ref().unwrap()).clone();
    if _switch_val == ("".to_string()) {
            Arc::new(Mutex::new(Some(".".to_string())))
        } else if _switch_val == ({ let __v = Arc::new(Mutex::new(Some(char::from_u32(((*Separator.lock().unwrap().as_ref().unwrap())) as u32).unwrap().to_string()))); let __owned = (*__v.lock().unwrap().as_ref().unwrap()).clone(); __owned }) {
                        // do nothing to the path
            Arc::new(Mutex::new(Some(path.lock().unwrap().as_ref().unwrap().clone())))
        } else {
            Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[(0) as usize..({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }) as usize].to_string() })))
        }
    }
}

/// cleanGlobPathWindows is windows version of cleanGlobPath.
pub fn clean_glob_path_windows(path: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<String>>>) {
    let mut prefixLen: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut cleaned: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));

    let mut vollen = filepathlite::volume_name_len(path.clone());
    if { let __tmp_x = (*path.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ""; __tmp_x == __tmp_y } {
            (0, Arc::new(Mutex::new(Some(".".to_string()))))
        } else if { let __tmp_x = ({ let __tmp_x = { let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x == __tmp_y } && (*os::is_path_separator({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }) as usize] }).lock().unwrap().as_ref().unwrap()) {
                        // do nothing to the path
            ({ let __tmp_x = { let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }, Arc::new(Mutex::new(Some(path.lock().unwrap().as_ref().unwrap().clone()))))
        } else if { let __tmp_x = ({ let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x == __tmp_y } && { let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (2 as i32); __tmp_x == __tmp_y } {
            ({ let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(format!("{}{}", { let __v = (*path.lock().unwrap().as_ref().unwrap()).clone(); __v }, ".".to_string())))))
        } else {
            if { let __tmp_x = ({ let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x >= __tmp_y } {
        { let new_val = { let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }; *vollen.lock().unwrap() = Some(new_val); };
    }
            ({ let __v = (*vollen.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some({ let __s = &((*path.lock().unwrap().as_ref().unwrap()).clone()); __s[(0) as usize..({ let __tmp_x = ((*path.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (1 as i32); __tmp_x - __tmp_y }) as usize].to_string() }))))
        }
}

/// glob searches for files matching pattern in the directory dir
/// and appends them to matches. If the directory cannot be
/// opened, it returns the existing matches. New matches are
/// added in lexicographical order.
pub fn glob_1(dir: Arc<Mutex<Option<String>>>, pattern: Arc<Mutex<Option<String>>>, matches: Arc<Mutex<Option<Vec<String>>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut m: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut e: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

    { let new_val = matches.clone(); m = new_val; };
    let (mut fi, mut err) = os::stat(dir.clone());
    if (*err.lock().unwrap()).is_some() {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (m, e)
    }
    }
        // ignore I/O error
    if !(*(*fi.lock().unwrap().as_ref().unwrap()).is_dir().lock().unwrap().as_ref().unwrap()) {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (m, e)
    }
    }
        // ignore I/O error
    let (mut d, mut err) = os::open(dir.clone());
    if (*err.lock().unwrap()).is_some() {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (m, e)
    }
    }
        // ignore I/O error
    let d_defer_captured = d.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = d_defer_captured.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };
    }));

    let (mut names, _) = { let __recv = d.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.readdirnames(Arc::new(Mutex::new(Some(-1)))); __result };
    { let mut __sort_guard = names.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.sort(); } };

    { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for n in __range_values.iter() {
        let (mut matched, mut err) = r#match(Arc::new(Mutex::new(Some({ let __arg_holder = pattern.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(n.clone()))));
        if (*err.lock().unwrap()).is_some() {
        {
        { let new_val = err.lock().unwrap().as_ref().unwrap().clone(); *e.lock().unwrap() = Some(new_val); };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (m, e)
    }
    }
        if matched {
        { let new_val = { let __append_target = m.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*join(Arc::new(Mutex::new(Some(vec![{ let __v = (*dir.lock().unwrap().as_ref().unwrap()).clone(); __v }, n])))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; m = new_val; };
    }
    } }
    {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return (m, e)
    }
}

/// hasMeta reports whether path contains any of the magic characters
/// recognized by Match.
pub fn has_meta(path: Arc<Mutex<Option<String>>>) -> bool {
    let mut magicChars = Arc::new(Mutex::new(Some("*?[".to_string())));
    if { let __tmp_x = runtime::G_O_O_S; let __tmp_y = "windows"; __tmp_x != __tmp_y } {
        { let new_val = "*?[\\".to_string(); *magicChars.lock().unwrap() = Some(new_val); };
    }
    (*strings::contains_any(path.clone(), magicChars.clone()).lock().unwrap().as_ref().unwrap())
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
