#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn format_slice<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Arc<Mutex<Option<C>>>) -> String
where
    C: AsRef<[Arc<Mutex<Option<T>>>]>,
    T: Display,
{
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.lock().unwrap();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        go_float_string(*v, false)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        go_float_string(*v as f64, true)
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Circle>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Rect>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

fn go_any_try_clone(value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Circle>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Rect>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Arc<Mutex<Option<Vec<String>>>>>() { return "[]string" }
    if val.is::<Arc<Mutex<Option<Vec<i64>>>>>() { return "[]int" }
    if val.is::<Circle>() { return "main.Circle" }
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<Rect>() { return "main.Rect" }
    std::any::type_name_of_val(val)
}

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

// An error that wraps others: fmt.Errorf with %w, or errors.Join. many is
// true when it wraps more than one, and then errors.Unwrap does not see it.
#[derive(Debug)]
struct GoWrapError {
    msg: String,
    wrapped: Vec<Arc<dyn StdError + Send + Sync>>,
    many: bool,
}

impl std::fmt::Display for GoWrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl StdError for GoWrapError {}

fn go_error_wrap(msg: String, wrapped: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Arc<dyn StdError + Send + Sync> {
    let many = wrapped.len() > 1;
    Arc::new(GoWrapError { msg, wrapped: wrapped.into_iter().flatten().collect(), many }) as Arc<dyn StdError + Send + Sync>
}

fn go_errors_join(errs: Vec<Option<Arc<dyn StdError + Send + Sync>>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let errs: Vec<Arc<dyn StdError + Send + Sync>> = errs.into_iter().flatten().collect();
    if errs.is_empty() {
        return None;
    }
    let msg = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>().join("\n");
    Some(Arc::new(GoWrapError { msg, wrapped: errs, many: true }) as Arc<dyn StdError + Send + Sync>)
}

// err and every error it wraps, depth first, in the order errors.Is and
// errors.As visit them.
fn go_error_chain(err: Option<Arc<dyn StdError + Send + Sync>>) -> Vec<Arc<dyn StdError + Send + Sync>> {
    let mut out = Vec::new();
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = err.into_iter().collect();
    while let Some(e) = stack.pop() {
        if let Some(w) = e.downcast_ref::<GoWrapError>() {
            stack.extend(w.wrapped.iter().rev().cloned());
        }
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
        out.push(e);
    }
    out
}

fn go_errors_unwrap(err: Option<Arc<dyn StdError + Send + Sync>>) -> Option<Arc<dyn StdError + Send + Sync>> {
    let err = err?;
    if let Some(w) = err.downcast_ref::<GoWrapError>() {
        return if w.many { None } else { w.wrapped.first().cloned() };
    }
    let e = err;
    let mut stack: Vec<Arc<dyn StdError + Send + Sync>> = Vec::new();
        if let Some(__v) = e.downcast_ref::<strconv::atoi::NumError>() { if let Some(__c) = (*__v.unwrap().lock().unwrap()).clone() { stack.push(__c); } }
    stack.pop()
}

// Two errors are the same error when they point to the same value, as ==
// on two errors is written.
fn go_errors_is(err: Option<Arc<dyn StdError + Send + Sync>>, target: Option<Arc<dyn StdError + Send + Sync>>) -> bool {
    match (err, target) {
        (None, None) => true,
        (Some(err), Some(target)) => go_error_chain(Some(err)).iter().any(|e| std::ptr::addr_eq(&**e, &*target)),
        _ => false,
    }
}

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &(dyn Any + Send + Sync)) -> bool {
    false
        || __value.is::<Circle>()
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    None
}

// value as %+v prints it, when it is a struct.
fn go_format_fields_value(__value: &(dyn Any + Send + Sync)) -> Option<String> {
    if let Some(__v) = __value.downcast_ref::<Rect>() {
        return Some(format!("{:#}", __v));
    }
    None
}

// The elements of value, boxed, when it is an array of basic values.
fn go_format_elements(__value: &(dyn Any + Send + Sync)) -> Option<Vec<Box<dyn Any + Send + Sync>>> {
    None
}

// A nil interface value passed to a format known only at run time. The
// arguments are boxes, which cannot be nil themselves.
#[allow(dead_code)]
struct GoFormatNil;

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value, go_format_fields_value, go_format_elements, GoFormatNil};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};
    #[allow(unused_imports)]
    use super::{GoSlice, Vec};

// A verb with its flags, as fmt reads it from a format.
#[derive(Default, Clone, Copy)]
struct GoFormatSpec {
    plus: bool,
    minus: bool,
    sharp: bool,
    space: bool,
    zero: bool,
    width: Option<usize>,
    precision: Option<usize>,
}

// Pads text to the width of spec. A number passes the length of its sign so
// that zero padding goes between the sign and the digits.
fn go_format_pad(spec: &GoFormatSpec, text: String, sign_len: usize, zero_allowed: bool) -> String {
    let width = match spec.width {
        Some(width) => width,
        None => return text,
    };
    let len = text.chars().count();
    if len >= width {
        return text;
    }
    let fill = width - len;
    if spec.minus {
        return text + &" ".repeat(fill);
    }
    if spec.zero && zero_allowed {
        let (sign, digits) = text.split_at(sign_len);
        return format!("{}{}{}", sign, "0".repeat(fill), digits);
    }
    " ".repeat(fill) + &text
}

fn go_format_sign(spec: &GoFormatSpec, negative: bool) -> &'static str {
    if negative {
        "-"
    } else if spec.plus {
        "+"
    } else if spec.space {
        " "
    } else {
        ""
    }
}

// Whether strconv.IsPrint prints c as it is: a letter, mark, number,
// punctuation, symbol or the ASCII space. Rust's standard library has no
// table of Unicode categories, so this knows the control characters, the
// other spaces, the format characters in common use (the soft hyphen, the
// zero-width and direction marks, the byte order mark) and the private-use
// areas. It does not know which code points are unassigned.
fn go_format_is_print(c: char) -> bool {
    if c == ' ' {
        return true;
    }
    if c.is_control() || c.is_whitespace() {
        return false;
    }
    !matches!(c as u32, 0xad | 0x600..=0x605 | 0x61c | 0x6dd | 0x70f | 0x8e2 | 0x180e | 0x200b..=0x200f | 0x202a..=0x202e | 0x2060..=0x206f | 0xfeff | 0xfff9..=0xfffb | 0xe000..=0xf8ff | 0xe0001 | 0xe0020..=0xe007f | 0xf0000..=0xffffd | 0x100000..=0x10fffd)
}

// Writes text the way strconv.Quote writes the inside of a quoted string,
// with quote as the delimiter. ascii is the + flag: every character outside
// ASCII is written as an escape.
fn go_format_escape(out: &mut String, text: &str, quote: char, ascii: bool) {
    for c in text.chars() {
        match c {
            '\u{7}' => out.push_str("\\a"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{b}' => out.push_str("\\v"),
            '\\' => out.push_str("\\\\"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if !go_format_is_print(c) || (ascii && !c.is_ascii()) => {
                if (c as u32) < 0x10000 {
                    out.push_str(&format!("\\u{:04x}", c as u32));
                } else {
                    out.push_str(&format!("\\U{:08x}", c as u32));
                }
            }
            c => out.push(c),
        }
    }
}

fn go_format_quote(text: &str, quote: char, ascii: bool) -> String {
    let mut out = String::new();
    out.push(quote);
    go_format_escape(&mut out, text, quote, ascii);
    out.push(quote);
    out
}

fn go_format_integer(spec: &GoFormatSpec, verb: char, value: i128) -> Option<String> {
    let negative = value < 0;
    let magnitude = value.unsigned_abs();
    let mut digits = match verb {
        'd' | 'v' => magnitude.to_string(),
        'b' => format!("{:b}", magnitude),
        'o' | 'O' => format!("{:o}", magnitude),
        'x' => format!("{:x}", magnitude),
        'X' => format!("{:X}", magnitude),
        'c' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, c.to_string(), 0, true));
        }
        'q' => {
            let c = u32::try_from(value).ok().and_then(char::from_u32).unwrap_or('\u{fffd}');
            return Some(go_format_pad(spec, go_format_quote(&c.to_string(), '\'', spec.plus), 0, true));
        }
        'U' => {
            // A precision above four asks for more digits.
            let digits = spec.precision.filter(|precision| *precision > 4).unwrap_or(4);
            let mut text = format!("U+{:0digits$X}", value as u64, digits = digits);
            if spec.sharp {
                if let Some(c) = u32::try_from(value).ok().and_then(char::from_u32).filter(|c| go_format_is_print(*c)) {
                    text.push_str(&format!(" '{}'", c));
                }
            }
            return Some(go_format_pad(spec, text, 0, false));
        }
        _ => return None,
    };
    let sign = go_format_sign(spec, negative);
    // A precision and zero padding both ask for a least number of digits.
    // The sign is counted in the width of zero padding; 0x, 0b and 0o are
    // not, they come on top.
    let least_digits = match spec.precision {
        // Zero with a precision of zero prints nothing but the padding.
        Some(0) if magnitude == 0 => return Some(go_format_pad(&GoFormatSpec { zero: false, ..*spec }, String::new(), 0, false)),
        Some(precision) => precision,
        None if spec.zero && !spec.minus => spec.width.unwrap_or(0).saturating_sub(sign.len()),
        None => 0,
    };
    if digits.len() < least_digits {
        digits = "0".repeat(least_digits - digits.len()) + &digits;
    }
    // The 0 that # puts before an octal number is only added to one that
    // does not start with a 0 already. %#O has it as well, after its 0o.
    let octal_zero = if spec.sharp && matches!(verb, 'o' | 'O') && !digits.starts_with('0') { "0" } else { "" };
    let prefix = match verb {
        'b' if spec.sharp => "0b",
        'O' => "0o",
        'x' if spec.sharp => "0x",
        'X' if spec.sharp => "0X",
        _ => "",
    };
    Some(go_format_pad(spec, format!("{}{}{}{}", sign, prefix, octal_zero, digits), 0, false))
}

// The digits of a float and the power of ten of the first one, from Rust's
// {:e} form, which rounds as strconv does.
fn go_format_float_digits(exponent_form: &str) -> (String, i32) {
    let (mantissa, exponent) = exponent_form.split_once('e').unwrap();
    let digits: String = mantissa.chars().filter(|c| c.is_ascii_digit()).collect();
    (digits, exponent.parse().unwrap())
}

fn go_format_float_exponent(digits: &str, exponent: i32, upper: bool) -> String {
    let mut out = String::new();
    out.push_str(&digits[..1]);
    if digits.len() > 1 {
        out.push('.');
        out.push_str(&digits[1..]);
    }
    out.push(if upper { 'E' } else { 'e' });
    out.push(if exponent < 0 { '-' } else { '+' });
    out.push_str(&format!("{:02}", exponent.abs()));
    out
}

// Formats the magnitude of a float as strconv.FormatFloat does for 'e', 'f'
// and 'g'. shortest is the float printed with as few digits as identify it.
fn go_format_float_magnitude(verb: char, precision: Option<usize>, shortest: &str, with_precision: impl Fn(usize) -> String, fixed: impl Fn(usize) -> String) -> String {
    let upper = verb.is_ascii_uppercase();
    match verb.to_ascii_lowercase() {
        'e' => {
            let (digits, exponent) = go_format_float_digits(&with_precision(precision.unwrap_or(6)));
            go_format_float_exponent(&digits, exponent, upper)
        }
        'f' => fixed(precision.unwrap_or(6)),
        _ => {
            let (digits, exponent, exponent_limit) = match precision {
                None => {
                    let (digits, exponent) = go_format_float_digits(shortest);
                    (digits, exponent, 6)
                }
                Some(precision) => {
                    let precision = precision.max(1);
                    let (digits, exponent) = go_format_float_digits(&with_precision(precision - 1));
                    let digits = digits.trim_end_matches('0');
                    let digits = if digits.is_empty() { "0".to_string() } else { digits.to_string() };
                    let mut limit = precision as i32;
                    if limit > digits.len() as i32 && digits.len() as i32 >= exponent + 1 {
                        limit = digits.len() as i32;
                    }
                    (digits, exponent, limit)
                }
            };
            if exponent < -4 || exponent >= exponent_limit {
                return go_format_float_exponent(&digits, exponent, upper);
            }
            if exponent < 0 {
                return format!("0.{}{}", "0".repeat((-exponent - 1) as usize), digits);
            }
            let whole = (exponent + 1) as usize;
            if digits.len() <= whole {
                return format!("{}{}", digits, "0".repeat(whole - digits.len()));
            }
            format!("{}.{}", &digits[..whole], &digits[whole..])
        }
    }
}

fn go_format_float(spec: &GoFormatSpec, verb: char, value: f64, single: bool) -> Option<String> {
    // fmt also writes a float in binary (%b) and in hex (%x). Neither is
    // done here.
    if matches!(verb, 'b' | 'x' | 'X') {
        panic!("go2rust: %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if !matches!(verb, 'v' | 'e' | 'E' | 'f' | 'F' | 'g' | 'G') {
        return None;
    }
    if spec.sharp {
        panic!("go2rust: the # flag on %{} of a float is not supported in a format that is only known at run time", verb);
    }
    if value.is_nan() {
        let sign = if spec.plus { "+" } else if spec.space { " " } else { "" };
        return Some(go_format_pad(spec, format!("{}NaN", sign), 0, false));
    }
    let negative = value.is_sign_negative();
    if value.is_infinite() {
        // +Inf keeps its sign, unless the space flag asks for a space there.
        let sign = if negative { "-" } else if spec.space && !spec.plus { " " } else { "+" };
        return Some(go_format_pad(spec, format!("{}Inf", sign), 0, false));
    }
    let magnitude = value.abs();
    // A float32 keeps the digits of the float32, not of the float64 it was
    // widened to.
    let narrow = magnitude as f32;
    let shortest = if single { format!("{:e}", narrow) } else { format!("{:e}", magnitude) };
    let with_precision = |precision: usize| if single { format!("{:.*e}", precision, narrow) } else { format!("{:.*e}", precision, magnitude) };
    let fixed = |precision: usize| if single { format!("{:.*}", precision, narrow) } else { format!("{:.*}", precision, magnitude) };
    let verb = if verb == 'v' { 'g' } else { verb };
    let digits = go_format_float_magnitude(verb, spec.precision, &shortest, with_precision, fixed);
    let sign = go_format_sign(spec, negative);
    Some(go_format_pad(spec, format!("{}{}", sign, digits), sign.len(), true))
}

// %x and %X of a string or a []byte: two digits for each byte. A precision
// counts bytes here, not characters.
fn go_format_hex(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> String {
    let bytes = match spec.precision {
        Some(precision) if precision < bytes.len() => &bytes[..precision],
        _ => bytes,
    };
    let prefix = if !spec.sharp { "" } else if verb == 'x' { "0x" } else { "0X" };
    let digits: Vec<String> = bytes.iter().map(|b| if verb == 'x' { format!("{:02x}", b) } else { format!("{:02X}", b) }).collect();
    let text = if spec.space {
        digits.iter().map(|d| format!("{}{}", prefix, d)).collect::<Vec<_>>().join(" ")
    } else if digits.is_empty() {
        String::new()
    } else {
        format!("{}{}", prefix, digits.concat())
    };
    go_format_pad(spec, text, 0, true)
}

fn go_format_string(spec: &GoFormatSpec, verb: char, text: &str) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, text.as_bytes()));
    }
    let truncated: String = match spec.precision {
        Some(precision) => text.chars().take(precision).collect(),
        None => text.to_string(),
    };
    let formatted = match verb {
        's' | 'v' => truncated,
        'q' => {
            // strconv.CanBackquote: no backquote, no ASCII control character
            // but tab, and no byte order mark.
            let raw = truncated.chars().all(|c| if c.is_ascii() { (c >= ' ' || c == '\t') && c != '\u{60}' && c != '\u{7f}' } else { c != '\u{feff}' });
            if spec.sharp && raw {
                format!("\u{60}{}\u{60}", truncated)
            } else {
                go_format_quote(&truncated, '"', spec.plus)
            }
        }
        _ => return None,
    };
    Some(go_format_pad(spec, formatted, 0, true))
}

// A []byte prints as text for the verbs that print a string.
fn go_format_bytes(spec: &GoFormatSpec, verb: char, bytes: &[u8]) -> Option<String> {
    if matches!(verb, 'x' | 'X') {
        return Some(go_format_hex(spec, verb, bytes));
    }
    if !matches!(verb, 's' | 'q') {
        return None;
    }
    // A precision counts characters, and a byte that is not UTF-8 counts as
    // one. It is applied first: what is cut off has no say in how the rest
    // is written.
    let mut kept = bytes.len();
    if let Some(precision) = spec.precision {
        let (mut count, mut end) = (0, 0);
        'chunks: for chunk in bytes.utf8_chunks() {
            for c in chunk.valid().chars() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += c.len_utf8();
            }
            for _ in chunk.invalid() {
                if count == precision {
                    break 'chunks;
                }
                count += 1;
                end += 1;
            }
        }
        kept = end;
    }
    let bytes = &bytes[..kept];
    let spec = &GoFormatSpec { precision: None, ..*spec };
    match std::str::from_utf8(bytes) {
        Ok(text) => go_format_string(spec, verb, text),
        // %q writes a byte that is not UTF-8 as an escape.
        Err(_) if verb == 'q' => {
            let mut quoted = String::from("\"");
            for chunk in bytes.utf8_chunks() {
                go_format_escape(&mut quoted, chunk.valid(), '"', spec.plus);
                for byte in chunk.invalid() {
                    quoted.push_str(&format!("\\x{:02x}", byte));
                }
            }
            quoted.push('"');
            Some(go_format_pad(spec, quoted, 0, true))
        }
        // %s would have to write the byte as it is, and the result is a Rust
        // String, which cannot hold it.
        Err(_) => panic!("go2rust: %s of a []byte that is not UTF-8 is not supported in a format that is only known at run time"),
    }
}

// Formats one argument for one verb. None means the verb does not apply to
// the argument's type, which fmt reports in the output.
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &(dyn Any + Send + Sync)) -> Option<String> {
    if verb == 'T' {
        return Some(go_format_pad(spec, __go_type_name(value).to_string(), 0, true));
    }
    if verb == 'p' {
        panic!("go2rust: %p is not supported in a format that is only known at run time");
    }
    // %+v and %#v ask for field names and Go syntax. A number or a bool has
    // neither, so it prints as %v prints it, with two exceptions for %#v: an
    // unsigned integer is written in hex and a string in quotes.
    let (plus_v, sharp_v) = (verb == 'v' && spec.plus, verb == 'v' && spec.sharp);
    let basic = GoFormatSpec { plus: spec.plus && !plus_v, sharp: spec.sharp && !sharp_v, ..*spec };
    macro_rules! signed {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    signed!(i8, i16, i32, i64, isize);
    // A rune taken from a range over a string can be a Rust char; it is a
    // Go rune, an int32.
    if let Some(v) = value.downcast_ref::<char>() {
        return go_format_integer(&basic, verb, *v as i128);
    }
    macro_rules! unsigned {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<$t>() {
                if sharp_v {
                    return go_format_integer(&GoFormatSpec { sharp: true, ..basic }, 'x', *v as i128);
                }
                return go_format_integer(&basic, verb, *v as i128);
            })*
        };
    }
    unsigned!(u8, u16, u32, u64, usize);
    if let Some(v) = value.downcast_ref::<f64>() {
        return go_format_float(&basic, verb, *v, false);
    }
    if let Some(v) = value.downcast_ref::<f32>() {
        return go_format_float(&basic, verb, *v as f64, true);
    }
    if let Some(v) = value.downcast_ref::<bool>() {
        return if matches!(verb, 't' | 'v') { Some(go_format_pad(&basic, v.to_string(), 0, true)) } else { None };
    }
    let string_verb = if sharp_v { 'q' } else { verb };
    if let Some(v) = value.downcast_ref::<String>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<&str>() {
        return go_format_string(&basic, string_verb, v);
    }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
        let guard = v.lock().unwrap();
        let bytes: &[u8] = guard.as_deref().unwrap_or(&[]);
        if let Some(text) = go_format_bytes(spec, verb, bytes) {
            return Some(text);
        }
        // Any other verb is applied to each byte, between brackets. A verb
        // that does not fit a number is reported for each byte. %#v writes
        // the slice as Go source, which is not done here.
        if !sharp_v {
            let items: Vec<String> = bytes.iter().map(|byte| go_format_integer(&basic, verb, *byte as i128).unwrap_or_else(|| format!("%!{}(uint8={})", verb, go_format_integer(&basic, 'd', *byte as i128).unwrap_or_default()))).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A slice of strings, numbers or bools: the verb, with its width and
    // flags, is applied to each element, between brackets, as fmt does
    // (%q of []string is ["a" "b"]). An element the verb does not fit is
    // reported in its place. A plain %v is left to Println's form below.
    let plain_v = verb == 'v' && spec.width.is_none() && spec.precision.is_none() && !spec.plus && !spec.sharp && !spec.space;
    macro_rules! basic_slice {
        ($($t:ty),*) => {
            $(if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<$t>>>>>() {
                if !plain_v && !sharp_v {
                    let guard = v.lock().unwrap();
                    let items: Vec<String> = guard.as_ref().map(|values| values.iter().map(|item| {
                        let item: &(dyn Any + Send + Sync) = item;
                        go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
                    }).collect()).unwrap_or_default();
                    return Some(format!("[{}]", items.join(" ")));
                }
            })*
        };
    }
    basic_slice!(String, i64, i32, i16, i8, u64, u32, u16, f64, f32, bool);
    // An array of them the same way. An array of bytes is the text or the
    // hex of those bytes for %s, %q, %x and %X, as a []byte is.
    if !plain_v && !sharp_v {
        if let Some(items) = go_format_elements(value) {
            let bytes: Option<Vec<u8>> = items.iter().map(|item| item.downcast_ref::<u8>().copied()).collect();
            if let Some(bytes) = bytes.filter(|bytes| !bytes.is_empty()) {
                if let Some(text) = go_format_bytes(spec, verb, &bytes) {
                    return Some(text);
                }
            }
            let items: Vec<String> = items.iter().map(|item| {
                let item: &(dyn Any + Send + Sync) = item.as_ref();
                go_format_argument(spec, verb, item).unwrap_or_else(|| format!("%!{}({})", verb, go_format_typed_value(spec, item)))
            }).collect();
            return Some(format!("[{}]", items.join(" ")));
        }
    }
    // A value with a String or an Error method is the text that method
    // returns, for the verbs that print a string.
    if go_format_is_text(value) && !sharp_v && matches!(verb, 'v' | 's' | 'q' | 'x' | 'X') {
        return go_format_string(&basic, verb, &format_any(value));
    }
    if let Some(inner) = go_format_named_basic_value(value) {
        return go_format_argument(spec, verb, inner.as_ref());
    }
    // Any other value prints as Println prints it, for a plain %v. fmt
    // applies a width, a precision, a flag or another verb to each field or
    // element, which is not done here. nil is one text.
    if plus_v && spec.width.is_none() && spec.precision.is_none() && !spec.space {
        if let Some(text) = go_format_fields_value(value) {
            return Some(text);
        }
    }
    if verb == 'v' && !plus_v && !sharp_v {
        let text = format_any(value);
        if text == "<nil>" {
            return Some(go_format_pad(spec, text, 0, true));
        }
        if spec.width.is_none() && spec.precision.is_none() && !spec.space {
            return Some(text);
        }
    }
    panic!("go2rust: %{}{}{} of a {} is not supported in a format that is only known at run time", if spec.plus { "+" } else { "" }, if spec.sharp { "#" } else { "" }, verb, __go_type_name(value));
}

// The number after % or after the dot: digits, or * for the next argument.
// Ok(None) is no number. Err is a * whose argument is not a usable integer.
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[Box<dyn Any + Send + Sync>], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &(dyn Any + Send + Sync) = match args.get(*next) {
            Some(arg) => arg.as_ref(),
            None => return Err(()),
        };
        *next += 1;
        // fmt takes any integer type here.
        let mut value: Option<i128> = None;
        macro_rules! integer {
            ($($t:ty),*) => {
                $(if let Some(v) = arg.downcast_ref::<$t>() {
                    value = Some(*v as i128);
                })*
            };
        }
        integer!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
        return match value {
            Some(value) if value.abs() <= 1_000_000 => Ok(Some(value as i64)),
            _ => Err(()),
        };
    }
    let mut value: Option<i64> = None;
    while let Some(digit) = chars.peek().and_then(|c| c.to_digit(10)) {
        value = Some(value.unwrap_or(0).saturating_mul(10).saturating_add(digit as i64));
        chars.next();
    }
    Ok(value.filter(|value| *value <= 1_000_000))
}

// Whether value is of a type that go_format_argument formats itself.
fn go_format_is_basic(value: &(dyn Any + Send + Sync)) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, Arc<Mutex<Option<Vec<u8>>>>)
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &(dyn Any + Send + Sync)) -> String {
    if value.is::<GoFormatNil>() {
        return "<nil>".to_string();
    }
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &(dyn Any + Send + Sync)) -> char {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    if is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize) {
        'd'
    } else if is_one_of!(f32, f64) {
        'g'
    } else if value.is::<bool>() {
        't'
    } else if is_one_of!(String, &str) {
        's'
    } else {
        'v'
    }
}

// fmt.Sprintf for a format that is only known when the program runs. A
// format written as a literal is expanded when the program is translated and
// does not come here. wraps is true for fmt.Errorf, where %w prints as %v.
pub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any + Send + Sync>], wraps: bool) -> String {
    let mut out = String::new();
    let mut next = 0;
    let mut chars = format.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '%' {
            out.push(c);
            continue;
        }
        let mut spec = GoFormatSpec::default();
        loop {
            match chars.peek() {
                Some('+') => spec.plus = true,
                Some('-') => spec.minus = true,
                Some('#') => spec.sharp = true,
                Some(' ') => spec.space = true,
                Some('0') => spec.zero = true,
                _ => break,
            }
            chars.next();
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        match go_format_number(&mut chars, args, &mut next) {
            // A negative width from an argument means left alignment.
            Ok(Some(width)) if width < 0 => {
                spec.minus = true;
                spec.width = Some((-width) as usize);
            }
            Ok(width) => spec.width = width.map(|width| width as usize),
            Err(()) => out.push_str("%!(BADWIDTH)"),
        }
        if chars.peek() == Some(&'.') {
            chars.next();
            match go_format_number(&mut chars, args, &mut next) {
                Ok(Some(precision)) if precision < 0 => out.push_str("%!(BADPREC)"),
                Ok(precision) => spec.precision = Some(precision.unwrap_or(0) as usize),
                Err(()) => out.push_str("%!(BADPREC)"),
            }
        }
        if chars.peek() == Some(&'[') {
            panic!("go2rust: an argument index such as %[1]d is not supported in a format that is only known at run time");
        }
        let verb = match chars.next() {
            Some(verb) => verb,
            None => {
                out.push_str("%!(NOVERB)");
                break;
            }
        };
        if verb == '%' {
            out.push('%');
            continue;
        }
        let arg = match args.get(next) {
            Some(arg) => arg,
            None => {
                out.push_str(&format!("%!{}(MISSING)", verb));
                continue;
            }
        };
        next += 1;
        // In fmt.Errorf %w prints an error as %v does. A number, a string, a
        // bool or a []byte is not an error, and fmt reports the verb: once
        // for a scalar, and for each byte of a []byte.
        // A nil interface value prints <nil> for %v and %T, and fmt reports
        // any other verb.
        if arg.is::<GoFormatNil>() {
            if matches!(verb, 'v' | 'T') {
                out.push_str(&go_format_pad(&spec, "<nil>".to_string(), 0, true));
            } else {
                out.push_str(&format!("%!{}(<nil>)", verb));
            }
            continue;
        }
        let verb = if verb == 'w' && wraps && !go_format_is_basic(arg.as_ref()) { 'v' } else { verb };
        match go_format_argument(&spec, verb, arg.as_ref()) {
            Some(text) => out.push_str(&text),
            None => out.push_str(&format!("%!{}({})", verb, go_format_typed_value(&spec, arg.as_ref()))),
        }
    }
    if next < args.len() {
        let extra: Vec<String> = args[next..].iter().map(|arg| go_format_typed_value(&GoFormatSpec::default(), arg.as_ref())).collect();
        out.push_str(&format!("%!(EXTRA {})", extra.join(", ")));
    }
    out
}
}
use __go_runtime_format::go_sprintf;

pub trait Shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool;
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

impl Clone for Box<dyn Shape + Send + Sync> {
    fn clone(&self) -> Self {
        Shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Rect {
    pub w: Arc<Mutex<Option<f64>>>,
    pub h: Arc<Mutex<Option<f64>>>,
}

impl Rect {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.h.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            w: __go_clone_0_0,
            h: __go_clone_0_1,
        }
    }
}


impl Default for Rect {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            w: __go_default_0_0,
            h: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = go_float_string(((*self.w.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_1 = go_float_string(((*self.h.lock().unwrap().as_ref().unwrap())) as f64, false);
        if __go_plus {
            return write!(f, "{{W:{} H:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.w.lock().unwrap().as_ref().cloned() }; let __right = { other.w.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.h.lock().unwrap().as_ref().cloned() }; let __right = { other.h.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Circle {
    pub r: Arc<Mutex<Option<f64>>>,
}

impl Circle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
        }
    }
}


impl Default for Circle {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            r: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Circle {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.r.lock().unwrap().as_ref().cloned() }; let __right = { other.r.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Animal {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Animal {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Animal {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = if __go_plus { format!("{:#}", (*self.name.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.name.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Name:{}}}", __go_fmt_0);
        }
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Dog {
    pub animal: Arc<Mutex<Option<Animal>>>,
    pub breed: Arc<Mutex<Option<String>>>,
}

impl Dog {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.animal.clone();
        let __go_clone_1_0 = { let __guard = self.breed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            animal: __go_clone_0_0,
            breed: __go_clone_1_0,
        }
    }
}


impl Default for Dog {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            animal: __go_default_0_0,
            breed: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_plus = f.alternate();
        let __go_fmt_0 = format!("{}", { let __guard = self.animal.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = if __go_plus { format!("{:#}", (*self.breed.lock().unwrap().as_ref().unwrap())) } else { format!("{}", (*self.breed.lock().unwrap().as_ref().unwrap())) };
        if __go_plus {
            return write!(f, "{{Animal:{} Breed:{}}}", __go_fmt_0, __go_fmt_1);
        }
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct NotFoundError {
    pub key: Arc<Mutex<Option<String>>>,
}

impl NotFoundError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for NotFoundError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for NotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}


impl Rect {
    pub fn area(&self) -> f64 {
        return { let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn perimeter(&self) -> f64 {
        return {
            let __tmp_x = 2.0;
            let __tmp_y = ({ let __tmp_x = (*self.w.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.h.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y });
            __tmp_x * __tmp_y
        };
    }
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        Rect::area(self)
    }
    fn perimeter(&self) -> f64 {
        Rect::perimeter(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Rect>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct RectPtr(pub Arc<Mutex<Option<Rect>>>);

impl std::fmt::Display for RectPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for RectPtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::area(__recv)
    }
    fn perimeter(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Rect::perimeter(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<RectPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Circle {
    pub fn area(&self) -> f64 {
        return {
            let __tmp_x = { let __tmp_x = 3.14159; let __tmp_y = (*self.r.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = (*self.r.lock().unwrap().as_ref().unwrap());
            __tmp_x * __tmp_y
        };
    }

    pub fn perimeter(&self) -> f64 {
        return { let __tmp_x = 6.28318; let __tmp_y = (*self.r.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("Circle({:.1})", (*self.r.lock().unwrap().as_ref().unwrap())))))
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        Circle::area(self)
    }
    fn perimeter(&self) -> f64 {
        Circle::perimeter(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Circle>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct CirclePtr(pub Arc<Mutex<Option<Circle>>>);

impl std::fmt::Display for CirclePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Shape for CirclePtr {
    fn area(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::area(__recv)
    }
    fn perimeter(&self) -> f64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Circle::perimeter(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn Shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn Shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<CirclePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Animal {
    pub fn speak(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*self.name.clone().lock().unwrap().as_ref().unwrap()), " makes a sound".to_string()))));
    }

    pub fn rename(&mut self, n: Arc<Mutex<Option<String>>>) {
        { let new_val = n.lock().unwrap().as_ref().unwrap().clone(); *self.name.lock().unwrap() = Some(new_val); };
    }
}

impl Dog {
    pub fn speak(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", (*(*self.animal.lock().unwrap().as_ref().unwrap()).name.clone().lock().unwrap().as_ref().unwrap()), " barks".to_string()))));
    }

    pub fn rename(&mut self, n: Arc<Mutex<Option<String>>>) {
        // Forward to embedded type's method
        let embedded = self.animal.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.rename(n)
    }
}

impl NotFoundError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "not found: ".to_string(), (*self.key.clone().lock().unwrap().as_ref().unwrap())))));
    }
}

impl StdError for NotFoundError {}


pub fn lookup(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, k: Arc<Mutex<Option<String>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    {
        let (mut v, mut ok) = { let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return ({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));;
        }
    }
    (
        0,
        Arc::new(Mutex::new(Some(go_error_wrap(format!("lookup failed: {}", match { let __err_handle = Arc::new(Mutex::new(Some(Arc::new(NotFoundError { key: Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>))); let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value } { Some(__wrapped) => format!("{}", __wrapped), None => "%!w(<nil>)".to_string() }), vec![{ let __err_handle = Arc::new(Mutex::new(Some(Arc::new(NotFoundError { key: Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Arc<dyn StdError + Send + Sync>))); let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }]))))
    )
}

pub fn sum(nums: Arc<Mutex<Option<Vec<i64>>>>) -> i64 {
    let mut t = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = nums.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for n in __range_values.iter().copied() {
        { let __rhs = n; let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn classify(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("integer {}", { let __text = match x.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", "string ".to_string(), (*strconv::quote(Arc::new(Mutex::new(Some({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).lock().unwrap().as_ref().unwrap())))));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut x = { let __slice_header = GoSliceHeader::go_slice_header(&x); Arc::new(Mutex::new(__slice_header)) };
        return Arc::new(Mutex::new(Some(format!("slice of {}", ({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })))));;
    } else if _ts_is_nil {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some("nil".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<fmt_Stringer>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Circle>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<strings::builder::Builder>>>>()).is_some() {
        let x: Arc<Mutex<Option<fmt_Stringer>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<fmt_Stringer>()) { Arc::new(Mutex::new(Some(typed_val.clone()))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Circle>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display(typed_val.clone())))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Circle>>>>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<strings::builder::Builder>>>>()) { Arc::new(Mutex::new(Some(fmt_Stringer::__go_from_display_cell(typed_val.clone())))) } else { Arc::new(Mutex::new(None)) };
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("{}{}", "stringer ".to_string(), (*(*x.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())))));;
    } else {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some(format!("other {}", { let __type_name = match x.lock().unwrap().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }; __type_name }))));;
    }
    }
    unreachable!()
}

pub fn grade(n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut _fallthrough = false;
        let mut _matched = false;
        if !_matched && ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 90; __tmp_x >= __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return Arc::new(Mutex::new(Some("A".to_string())));
        }
        if !_matched && ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 80; __tmp_x >= __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return Arc::new(Mutex::new(Some("B".to_string())));
        }
        if !_matched && ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 70; __tmp_x >= __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            _fallthrough = true;
        }
        if !_matched && ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 60; __tmp_x >= __tmp_y }) || _fallthrough {
            _matched = true;
            _fallthrough = false;
            return Arc::new(Mutex::new(Some("C/D".to_string())));
        }
    }
    Arc::new(Mutex::new(Some("F".to_string())))
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut shapes = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(Rect { w: Arc::new(Mutex::new(Some(3.0))), h: Arc::new(Mutex::new(Some(4.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(Circle { r: Arc::new(Mutex::new(Some(1.0))), ..Default::default() }) as Box<dyn Shape + Send + Sync>)))])));
    let mut total = Arc::new(Mutex::new(Some(0.0)));
    { let __range_holder = shapes.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for s in __range_values.iter() {
        { let __rhs = (*s.lock().unwrap().as_ref().unwrap()).area(); let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        print!("{}", go_sprintf(&"%-8T %8.3f %v\n".to_string(), &[{ let __go_format_arg: Box<dyn Any + Send + Sync> = if { let __nil_result = (*s.lock().unwrap()).is_none(); __nil_result } { Box::new(GoFormatNil) } else { { let __iface_holder = s.clone(); let __iface_guard = __iface_holder.lock().unwrap(); let __iface: Box<dyn Shape + Send + Sync> = __iface_guard.as_ref().unwrap().__go_clone_box_shape(); __iface as Box<dyn Any + Send + Sync> } }; __go_format_arg }, { let __go_format_arg = Box::new((*s.lock().unwrap().as_ref().unwrap()).perimeter()) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg: Box<dyn Any + Send + Sync> = if { let __nil_result = (*s.lock().unwrap()).is_none(); __nil_result } { Box::new(GoFormatNil) } else { { let __iface_holder = s.clone(); let __iface_guard = __iface_holder.lock().unwrap(); let __iface: Box<dyn Shape + Send + Sync> = __iface_guard.as_ref().unwrap().__go_clone_box_shape(); __iface as Box<dyn Any + Send + Sync> } }; __go_format_arg }], false));
    } }
    print!("total={:.2}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let mut d = Arc::new(Mutex::new(Some(Dog {
        animal: Arc::new(Mutex::new(Some(Animal { name: Arc::new(Mutex::new(Some("Rex".to_string()))), ..Default::default() }))).clone(),
        breed: Arc::new(Mutex::new(Some("lab".to_string()))),
        ..Default::default()
    })));
    println!("{} {} {}", format!("{}", (*(*d.lock().unwrap().as_ref().unwrap()).speak().lock().unwrap().as_ref().unwrap())), format!("{}", "|".to_string()), format!("{}", (*(*(*d.lock().unwrap().as_ref().unwrap()).animal.lock().unwrap().as_ref().unwrap()).speak().lock().unwrap().as_ref().unwrap())));
    (*d.lock().unwrap().as_mut().unwrap()).rename(Arc::new(Mutex::new(Some("Max".to_string()))));
    println!("{} {}", format!("{}", (*(*(*d.lock().unwrap().as_ref().unwrap()).animal.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*d.lock().unwrap().as_ref().unwrap()).breed.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    {
        let (_, mut err) = lookup(m.clone(), Arc::new(Mutex::new(Some("z".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            let mut nf: Arc<Mutex<Option<NotFoundError>>> = Arc::new(Mutex::new(None));;
            { let __go_print_arg_1 = Arc::new(Mutex::new(Some({ let mut __as_found = None; for __as_err in go_error_chain({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }) { if let Some(__as_value) = __as_err.downcast_ref::<NotFoundError>() { __as_found = Some(__as_value.clone()); break; } } match __as_found { Some(__as_value) => { let new_val = Arc::new(Mutex::new(Some(__as_value))); nf = new_val; true } None => false } }))); let __go_print_value = println!("{} {} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*{ let __field = (*nf.lock().unwrap().as_ref().unwrap()).key.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone())); __go_print_value };;
        }
    }
    let mut nums = Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64])));
    println!("{} {} {}", format!("{}", sum(Arc::new(Mutex::new(Some(vec![]))))), format!("{}", sum(Arc::new(Mutex::new(Some(vec![1, 2]))))), format!("{}", sum(nums.clone())));
    for v in &vec![
        Box::new(42i64) as Box<dyn Any + Send + Sync>,
        Box::new("hi".to_string()) as Box<dyn Any + Send + Sync>,
        Box::new(Arc::new(Mutex::new(Some(vec![1i64]))).clone()) as Box<dyn Any + Send + Sync>,
        Box::new(GoNilAny) as Box<dyn Any + Send + Sync>,
        Box::new(Circle { r: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
        Box::new(3.5) as Box<dyn Any + Send + Sync>,
    ] {
        println!("{}", format!("{}", (*classify(Arc::new(Mutex::new(Some(go_any_clone(v.as_ref()))))).lock().unwrap().as_ref().unwrap())));
    }
    for n in vec![95i64, 85i64, 75i64, 65i64, 10i64].iter().copied() {
        print!("{}{}", format!("{}", (*grade(Arc::new(Mutex::new(Some(n.clone())))).lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()));
    }
    println!();
    let mut parts = strings::split(Arc::new(Mutex::new(Some("a,b,,c".to_string()))), Arc::new(Mutex::new(Some(",".to_string()))));
    let mut parts = { let __slice_header = GoSliceHeader::go_slice_header(&parts); Arc::new(Mutex::new(__slice_header)) };
    print!("{}", go_sprintf(&"%q %d\n".to_string(), &[{ let __go_format_arg = Box::new(parts.clone()) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(({ let __len = (*parts.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as Box<dyn Any + Send + Sync>; __go_format_arg }], false));
    println!("{} {} {}", format!("{}", (*strings::join(parts.clone(), Arc::new(Mutex::new(Some("|".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("ab".to_string()))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*strings::to_upper(Arc::new(Mutex::new(Some("h\u{e9}llo".to_string())))).lock().unwrap().as_ref().unwrap())));
    let (mut n, mut err) = strconv::atoi(Arc::new(Mutex::new(Some("12x".to_string()))));
    println!("{} {}", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
    let mut f = Arc::new(Mutex::new(Some({ let mut __recv = ({ let __v = (*d.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); Arc::new(move || -> Arc<Mutex<Option<String>>> { let mut __recv = __recv.clone(); __recv.speak() }) as Arc<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync> })));
    let mut g = Arc::new(Mutex::new(Some(Arc::new(move |
        __arg0: Arc<Mutex<Option<Animal>>>
    | -> Arc<Mutex<Option<String>>> {
        {
            let __recv = __arg0.clone();
            let __recv_ptr: *const Animal = {
                let __recv_guard = __recv.lock().unwrap();
                __recv_guard.as_ref().unwrap() as *const Animal
            };
            let __result = unsafe { &*__recv_ptr }.speak();
            __result
        }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Animal>>>) -> Arc<Mutex<Option<String>>> + Send + Sync>)));
    println!("{} {}", format!("{}", (*{
        let __f: Arc<dyn Fn() -> Arc<Mutex<Option<String>>> + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Animal>>>) -> Arc<Mutex<Option<String>>> + Send + Sync> = g.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            { let __field = (*d.lock().unwrap().as_ref().unwrap()).animal.clone(); __field }
        )
    }.lock().unwrap().as_ref().unwrap())));
    let mut x: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(250)));
    { let __rhs = 10 as u8; let mut guard = x.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    println!("{} {} {} {} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (1024i64)), format!("{}", { let __tmp_x = 0xff; let __tmp_y = 0x0f; __tmp_x & ! __tmp_y }), format!("{}", { let __tmp_x = -(7); let __tmp_y = 2; __tmp_x / __tmp_y }), format!("{}", { let __tmp_x = -(7); let __tmp_y = 2; __tmp_x % __tmp_y }));
    eprintln!("{}", format!("{}", "to stderr".to_string()));
    let mut b = Arc::new(Mutex::new(Some(("hello".to_string()).as_bytes().to_go_slice())));
    { let __idx = (0) as usize; let __rhs = ('H' as u8) as u8; (*b.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*b.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap())), format!("{}", format_slice(&Arc::new(Mutex::new(Some({
        let __seq_holder = b.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (1) as usize;
        let __high = (3) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))));
    for (i, r) in "h\u{e9}y".to_string().char_indices() {
        print!("{}{}{}{}", format!("{}", i), format!("{}", ":".to_string()), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32(((r as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()));
    }
    println!();
    print!("{}", go_sprintf(&"%5d|%-5d|%05d|%x|%X|%o|%b|%e|%g|%8.2f|%+v\n".to_string(), &[{ let __go_format_arg = Box::new(42i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(42i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(42i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(255i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(255i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(8i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(5i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(1234.5678) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(0.000012) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(3.14159) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(Rect { w: Arc::new(Mutex::new(Some(1.0))), h: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }) as Box<dyn Any + Send + Sync>; __go_format_arg }], false));
}

impl GoValueClone for Rect {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Circle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Animal {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Dog {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for NotFoundError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
