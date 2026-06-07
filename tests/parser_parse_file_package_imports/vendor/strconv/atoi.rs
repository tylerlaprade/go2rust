use go2rust_stdlib_stubs::*;

use crate::{GoInteger, go_integer_add_one, go_integer_cast, go_integer_from_i128, go_integer_sub_one};

use crate::atob::*;
use crate::atoc::*;
use crate::atof::*;
use crate::bytealg::*;
use crate::ctoa::*;
use crate::decimal::*;
use crate::eisel_lemire::*;
use crate::ftoa::*;
use crate::ftoaryu::*;
use crate::isprint::*;
use crate::itoa::*;
use crate::quote::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const INT_SIZE_1: i32 = 32 << (!(0 as u64) >> 63);


pub const INT_SIZE: i32 = INT_SIZE_1;


pub(crate) const MAX_UINT64: u128 = (1 << 64) - 1;


/// A NumError records a failed conversion.
#[derive(Clone)]
pub struct NumError {
    pub func: Arc<Mutex<Option<String>>>,
    pub num: Arc<Mutex<Option<String>>>,
    pub err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
}

impl NumError {
    pub fn __go_value_clone(&self) -> Self {
        Self { func: { let __guard = self.func.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, num: { let __guard = self.num.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, err: self.err.clone() }
    }
}


impl Default for NumError {
    fn default() -> Self {
        Self { func: Arc::new(Mutex::new(Some(String::new()))), num: Arc::new(Mutex::new(Some(String::new()))), err: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for NumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for NumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for NumError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Func") {
            out.func = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Num") {
            out.num = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static ErrRange: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrSyntax: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("value out of range".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrRange.lock().unwrap() = new_val; }
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid syntax".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrSyntax.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrRange.lock().unwrap() = None;
    *ErrSyntax.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("value out of range".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrRange.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_5() {
    { let __rhs_holder = Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from("invalid syntax".to_string())))).clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrSyntax.lock().unwrap() = new_val; }
}


impl NumError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", "strconv.".to_string())); __s.push_str(&format!("{}", (*self.func.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ": ".to_string())); __s.push_str(&format!("{}", "parsing ".to_string())); __s.push_str(&format!("{}", (*quote(Arc::new(Mutex::new(Some({ let __selector_holder = self.num.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })))).lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", ": ".to_string())); __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", self.err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap()))); __s })));
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.err.clone()
    }
}

impl StdError for NumError {}


/// lower(c) is a lower-case letter if and only if
/// c is either that lower-case letter or the equivalent upper-case letter.
/// Instead of writing c == 'x' || c == 'X' one can write lower(c) == 'x'.
/// Note that lower of non-letters can produce other non-letters.
pub fn lower(c: Arc<Mutex<Option<u8>>>) -> u8 {
    return { let __tmp_x = { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ({ let __tmp_x = ('x' as i32); let __tmp_y = ('X' as i32); __tmp_x - __tmp_y }) as u8; __tmp_x | __tmp_y };
}

pub fn syntax_error(r#fn: Arc<Mutex<Option<String>>>, str: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<NumError>>> {
    Arc::new(Mutex::new(Some(NumError { func: Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), num: internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), err: ErrSyntax.clone(), ..Default::default() })))
}

pub fn range_error(r#fn: Arc<Mutex<Option<String>>>, str: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<NumError>>> {
    Arc::new(Mutex::new(Some(NumError { func: Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), num: internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), err: ErrRange.clone(), ..Default::default() })))
}

pub fn base_error(r#fn: Arc<Mutex<Option<String>>>, str: Arc<Mutex<Option<String>>>, base: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<NumError>>> {
    Arc::new(Mutex::new(Some(NumError { func: Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), num: internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), err: Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(format!("{}{}", "invalid base ".to_string(), (*itoa(Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())))))), ..Default::default() })))
}

pub fn bit_size_error(r#fn: Arc<Mutex<Option<String>>>, str: Arc<Mutex<Option<String>>>, bitSize: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<NumError>>> {
    Arc::new(Mutex::new(Some(NumError { func: Arc::new(Mutex::new(Some({ let __arg_holder = r#fn.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), num: internal_stringslite::clone(Arc::new(Mutex::new(Some({ let __arg_holder = str.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))), err: Arc::new(Mutex::new(Some(Box::<dyn std::error::Error + Send + Sync>::from(format!("{}{}", "invalid bit size ".to_string(), (*itoa(Arc::new(Mutex::new(Some({ let __arg_holder = bitSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())))))), ..Default::default() })))
}

/// ParseUint is like [ParseInt] but for unsigned numbers.
///
/// A sign prefix is not permitted.
pub fn parse_uint(mut s: Arc<Mutex<Option<String>>>, mut base: Arc<Mutex<Option<i32>>>, mut bitSize: Arc<Mutex<Option<i32>>>) -> (u64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    const fnParseUint: &'static str = "ParseUint";


    if { let __tmp_x = (*s.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x == __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::new((*syntax_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }

    let mut base0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y })));

    let mut s0 = { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
    if { let __tmp_x = 2; let __tmp_y = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 36; __tmp_x <= __tmp_y } {
        } else if { let __tmp_x = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
                        // Look for octal, hex prefix.
            { let new_val = 10; *base.lock().unwrap() = Some(new_val); };
            if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } {
        if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('b' as i32) as u8; __tmp_x == __tmp_y } {
            { let new_val = 2; *base.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        } else if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('o' as i32) as u8; __tmp_x == __tmp_y } {
            { let new_val = 8; *base.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        } else if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 3; __tmp_x >= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y } {
            { let new_val = 16; *base.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (2) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        } else {
            { let new_val = 8; *base.lock().unwrap() = Some(new_val); };
            { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
        }
    }
        } else {
            return (0, Arc::new(Mutex::new(Some(Box::new((*base_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = base.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
        }

        // valid base; nothing to do
        // Look for octal, hex prefix.
    if { let __tmp_x = { let __v = (*bitSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let new_val = 64; *bitSize.lock().unwrap() = Some(new_val); };
    } else if { let __tmp_x = { let __v = (*bitSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*bitSize.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 64; __tmp_x > __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::new((*bit_size_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = bitSize.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }

        // Cutoff is the smallest number such that cutoff*base > maxUint64.
        // Use compile-time constants for common cases.
    let mut cutoff: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    { let _switch_val = { let __v = (*base.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (10) {
            { let new_val = (((MAX_UINT64 as u64) / (10 as u64)) + (1 as u64)) as u64; *cutoff.lock().unwrap() = Some(new_val); };
        } else if _switch_val == (16) {
            { let new_val = (((MAX_UINT64 as u64) / (16 as u64)) + (1 as u64)) as u64; *cutoff.lock().unwrap() = Some(new_val); };
        } else {
            { let new_val = { let __tmp_x = { let __tmp_x = MAX_UINT64 as u64; let __tmp_y = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x / __tmp_y }; let __tmp_y = 1 as u64; __tmp_x + __tmp_y }; *cutoff.lock().unwrap() = Some(new_val); };
        }
    }

    let mut maxVal = Arc::new(Mutex::new(Some({ let __tmp_x = { let __tmp_x = (1 as u64); let __tmp_y = (*Arc::new(Mutex::new(Some((*bitSize.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y }; let __tmp_y = 1 as u64; __tmp_x - __tmp_y })));

    let mut underscores = Arc::new(Mutex::new(Some(false)));
    let mut n: Arc<Mutex<Option<u64>>> = Arc::new(Mutex::new(Some(0)));
    { let __range_holder = Arc::new(Mutex::new(Some(({ let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }).as_bytes().to_vec()))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for c in __range_values.iter().copied() {
        let mut d: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(0)));
        if { let __tmp_x = c; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } && { let __v = (*base0.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            { let new_val = true; *underscores.lock().unwrap() = Some(new_val); };
            continue
        } else if { let __tmp_x = ('0' as i32) as u8; let __tmp_y = c; __tmp_x <= __tmp_y } && { let __tmp_x = c; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } {
            { let new_val = { let __tmp_x = c; let __tmp_y = ('0' as i32) as u8; __tmp_x - __tmp_y }; *d.lock().unwrap() = Some(new_val); };
        } else if { let __tmp_x = ('a' as i32) as u8; let __tmp_y = lower(Arc::new(Mutex::new(Some(c.clone())))); __tmp_x <= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some(c.clone())))); let __tmp_y = ('z' as i32) as u8; __tmp_x <= __tmp_y } {
            { let new_val = { let __tmp_x = { let __tmp_x = lower(Arc::new(Mutex::new(Some(c.clone())))); let __tmp_y = ('a' as i32) as u8; __tmp_x - __tmp_y }; let __tmp_y = 10 as u8; __tmp_x + __tmp_y }; *d.lock().unwrap() = Some(new_val); };
        } else {
            return (0, Arc::new(Mutex::new(Some(Box::new((*syntax_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
        }
        if { let __tmp_x = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u8))).lock().unwrap().as_ref().unwrap()); __tmp_x >= __tmp_y } {
        return (0, Arc::new(Mutex::new(Some(Box::new((*syntax_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*cutoff.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x >= __tmp_y } {
                // n*base overflows
        return ({ let __v = (*maxVal.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(Box::new((*range_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
                // n*base overflows
        { let __rhs = (*Arc::new(Mutex::new(Some((*base.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() * __rhs); };
        let mut n1 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some((*d.lock().unwrap().as_ref().unwrap()) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));
        if { let __tmp_x = { let __v = (*n1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n1.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*maxVal.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x > __tmp_y } {
                // n+d overflows
        return ({ let __v = (*maxVal.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(Some(Box::new((*range_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }
                // n+d overflows
        { let new_val = n1.lock().unwrap().as_ref().unwrap().clone(); *n.lock().unwrap() = Some(new_val); };
    } }

        // n*base overflows
        // n+d overflows
    if { let __v = (*underscores.lock().unwrap().as_ref().unwrap()).clone(); __v } && !underscore_o_k(Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))) {
        return (0, Arc::new(Mutex::new(Some(Box::new((*syntax_error(Arc::new(Mutex::new(Some(fnParseUint.to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = s0.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))));
    }

    return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
}

/// underscoreOK reports whether the underscores in s are allowed.
/// Checking them in this one function lets all the parsers skip over them simply.
/// Underscore must appear only between digits or between a base prefix and a digit.
pub fn underscore_o_k(mut s: Arc<Mutex<Option<String>>>) -> bool {
        // saw tracks the last character (class) we saw:
        // ^ for beginning of number,
        // 0 for a digit or base prefix,
        // _ for an underscore,
        // ! for none of the above.
    let mut saw = Arc::new(Mutex::new(Some(('^' as i32))));
    let mut i = Arc::new(Mutex::new(Some(0)));

        // Optional sign.
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 1; __tmp_x >= __tmp_y } && ({ let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('-' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('+' as i32) as u8; __tmp_x == __tmp_y }) {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; __s[__low..].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *s.lock().unwrap() = __moved_val; };
    }

        // Optional base prefix.
    let mut hex = Arc::new(Mutex::new(Some(false)));
    if { let __tmp_x = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = 2; __tmp_x >= __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }; let __tmp_y = ('0' as i32) as u8; __tmp_x == __tmp_y } && ({ let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('b' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('o' as i32) as u8; __tmp_x == __tmp_y } || { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y }) {
        { let new_val = 2; *i.lock().unwrap() = Some(new_val); };
        { let new_val = ('0' as i32); *saw.lock().unwrap() = Some(new_val); };
        { let new_val = { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(1) as usize] })))); let __tmp_y = ('x' as i32) as u8; __tmp_x == __tmp_y }; *hex.lock().unwrap() = Some(new_val); };
    }

        // base prefix counts as a digit for "underscore as digit separator"
        // Number proper.
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*s.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
                // Digits are always okay.
        if { let __tmp_x = ('0' as i32) as u8; let __tmp_y = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; __tmp_x <= __tmp_y } && { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('9' as i32) as u8; __tmp_x <= __tmp_y } || { let __v = (*hex.lock().unwrap().as_ref().unwrap()).clone(); __v } && { let __tmp_x = ('a' as i32) as u8; let __tmp_y = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))); __tmp_x <= __tmp_y } && { let __tmp_x = lower(Arc::new(Mutex::new(Some({ let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] })))); let __tmp_y = ('f' as i32) as u8; __tmp_x <= __tmp_y } {
        { let new_val = ('0' as i32); *saw.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Underscore must follow digit.
        if { let __tmp_x = { let __s = &((*s.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] }; let __tmp_y = ('_' as i32) as u8; __tmp_x == __tmp_y } {
        if { let __tmp_x = { let __v = (*saw.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('0' as i32); __tmp_x != __tmp_y } {
        return false;
    }
        { let new_val = ('_' as i32); *saw.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }; continue
    }

                // Underscore must also be followed by digit.
        if { let __tmp_x = { let __v = (*saw.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x == __tmp_y } {
        return false;
    }

                // Saw non-digit, non-underscore.
        { let new_val = ('!' as i32); *saw.lock().unwrap() = Some(new_val); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        // Digits are always okay.
        // Underscore must follow digit.
        // Underscore must also be followed by digit.
        // Saw non-digit, non-underscore.
    return { let __tmp_x = { let __v = (*saw.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = ('_' as i32); __tmp_x != __tmp_y };
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for NumError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
