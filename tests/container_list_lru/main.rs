#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}
fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
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
    format_any_at(value, false)
}

fn format_any_at(value: &(dyn Any + Send + Sync), nested: bool) -> String {
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() {
        match (*v.lock().unwrap()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<entry>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

/// Runs an append, __store, for elements whose Go copy is not their Clone,
/// such as a struct of cells. When the append moves the slice to a new
/// backing array, the elements it carried over are replaced with __copy of
/// them, so the old array and the new one share nothing.
trait GoSliceAppendCopying<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T);
}

impl<T: Clone> GoSliceAppendCopying<T> for GoSlice<T> {
    fn go_append_copying(&mut self, __store: impl FnOnce(&mut Self), __copy: impl Fn(&T) -> T) {
        let __before = self.clone();
        let __len = self.len();
        __store(self);
        if !self.shares_backing_with(&__before) {
            for __i in 0..__len.min(self.len()) {
                let __value = __copy(&self.as_slice()[__i]);
                self.as_mut_slice()[__i] = __value;
            }
        }
    }
}

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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<entry>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
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
    if val.is::<Arc<Mutex<Option<entry>>>>() { return "*main.entry" }
    if val.is::<GoNilAny>() { return "<nil>" }
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
        || __value.is::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    None
}

// value as %+v prints it, when it is a struct.
fn go_format_fields_value(__value: &(dyn Any + Send + Sync)) -> Option<String> {
    None
}

// A nil interface value passed to a format known only at run time. The
// arguments are boxes, which cannot be nil themselves.
#[allow(dead_code)]
struct GoFormatNil;

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value, go_format_fields_value, GoFormatNil};
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

#[derive(Debug, Clone)]
pub struct entry {
    pub key: Arc<Mutex<Option<String>>>,
    pub value: Arc<Mutex<Option<i64>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for entry {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            key: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone)]
pub struct LRU {
    pub cap: Arc<Mutex<Option<i64>>>,
    pub ll: Arc<Mutex<Option<container_list::r#mod::List>>>,
    pub items: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<container_list::r#mod::Element>>>>>>>,
}

impl LRU {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cap.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.ll.clone();
        let __go_clone_2_0 = self.items.clone();
        Self {
            cap: __go_clone_0_0,
            ll: __go_clone_1_0,
            items: __go_clone_2_0,
        }
    }
}


impl Default for LRU {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            cap: __go_default_0_0,
            ll: __go_default_1_0,
            items: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for LRU {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.cap.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.ll.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", format_map(&self.items));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


pub static ErrMissing: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrMissing.lock().unwrap() = None;
    { let __rhs_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("missing key".to_string()))))).clone(); let new_val = (*__rhs_holder.lock().unwrap()).clone(); *ErrMissing.lock().unwrap() = new_val; }
}


impl LRU {
    pub fn get(&self, key: Arc<Mutex<Option<String>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        {
        let (mut el, mut ok) = { let __map = { let __map_holder = self.items.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*key.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            (*self.ll.lock().unwrap().as_mut().unwrap()).move_to_front(el.clone());;
            return ((*(*({
        let val = (*el.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<entry>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.entry")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.entry")
        }
    }).lock().unwrap().as_ref().unwrap()).value.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)));;
        }
    }
        (
            0,
            Arc::new(Mutex::new(Some(go_error_wrap(format!("{}", go_sprintf(&"get %q: %w".to_string(), &[{ let __go_format_arg = Box::new({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg: Box<dyn Any + Send + Sync> = if { let __nil_result = (*ErrMissing.lock().unwrap()).is_none(); __nil_result } { Box::new(GoFormatNil) } else { { let __err_holder = ErrMissing.clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } } }; __go_format_arg }], true)), vec![{ let __err_handle = { let __err_source = ErrMissing.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }]))))
        )
    }

    pub fn put(&mut self, key: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>) {
        {
        let (mut el, mut ok) = { let __map = { let __map_holder = self.items.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*key.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            { let new_val = value.lock().unwrap().as_ref().unwrap().clone(); *(*({
        let val = (*el.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<entry>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.entry")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.entry")
        }
    }).lock().unwrap().as_ref().unwrap()).value.lock().unwrap() = Some(new_val); };;
            (*self.ll.lock().unwrap().as_mut().unwrap()).move_to_front(el.clone());;
            return;;
        }
    }
        let mut el = (*self.ll.lock().unwrap().as_mut().unwrap()).push_front(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(entry { key: Arc::new(Mutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), value: Arc::new(Mutex::new(Some({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))));
        { let __map_key = ({ let __v = (*key.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = el.clone(); (*self.items.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        if {
            let __tmp_x = (*self.ll.lock().unwrap().as_ref().unwrap()).len();
            let __tmp_y = (*self.cap.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        } {
        let mut last = (*self.ll.lock().unwrap().as_ref().unwrap()).back();
        (*self.ll.lock().unwrap().as_mut().unwrap()).remove(last.clone());
        { let __map_handle = self.items.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&{ let __selector_holder = (*({
        let val = (*last.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<entry>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.entry")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.entry")
        }
    }).lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }); };
    }
    }

    pub fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut keys: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
        let mut el = (*self.ll.lock().unwrap().as_ref().unwrap()).front();
    while { let __nil_result = (*el.lock().unwrap()).is_some(); __nil_result } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = { let __selector_holder = (*({
        let val = (*el.lock().unwrap().as_ref().unwrap()).value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<entry>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.entry")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.entry")
        }
    }).lock().unwrap().as_ref().unwrap()).key.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
        { let new_val = { let __recv = el.clone(); let __recv_ptr: *const container_list::r#mod::Element = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const container_list::r#mod::Element }; let __result = unsafe { &*__recv_ptr }.next(); __result }.clone(); el = new_val; };
    }
        return keys.clone();
    }
}

pub fn new_l_r_u(capacity: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<LRU>>> {
    Arc::new(Mutex::new(Some(LRU {
        cap: Arc::new(Mutex::new(Some({ let __arg_holder = capacity.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
        ll: container_list::new().clone(),
        items: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<container_list::r#mod::Element>>>>::new()))),
        ..Default::default()
    })))
}

fn main() {
    ::container_list::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut c = new_l_r_u(Arc::new(Mutex::new(Some(2))));
    { let __recv = c.clone(); let __recv_ptr: *mut LRU = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut LRU }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some(1)))); __result };
    { let __recv = c.clone(); let __recv_ptr: *mut LRU = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut LRU }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("b".to_string()))), Arc::new(Mutex::new(Some(2)))); __result };
    let (mut v, _) = { let __recv = c.clone(); let __recv_ptr: *const LRU = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const LRU }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("a".to_string())))); __result };
    println!("{} {}", format!("{}", v), format!("{}", format_slice(&{ let __recv = c.clone(); let __recv_ptr: *const LRU = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const LRU }; let __result = unsafe { &*__recv_ptr }.keys(); __result })));
    { let __recv = c.clone(); let __recv_ptr: *mut LRU = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut LRU }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("c".to_string()))), Arc::new(Mutex::new(Some(3)))); __result };
    let (_, mut err) = { let __recv = c.clone(); let __recv_ptr: *const LRU = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const LRU }; let __result = unsafe { &*__recv_ptr }.get(Arc::new(Mutex::new(Some("b".to_string())))); __result };
    println!("{} {}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", (*Arc::new(Mutex::new(Some(go_errors_is({ let __err_handle = { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value }, { let __err_handle = { let __err_source = ErrMissing.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) }; let __err_value = (*__err_handle.lock().unwrap()).clone(); __err_value })))).lock().unwrap().as_ref().unwrap())));
    { let __recv = c.clone(); let __recv_ptr: *mut LRU = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut LRU }; let __result = unsafe { &mut *__recv_ptr }.put(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some(10)))); __result };
    println!("{} {}", format!("{}", format_slice(&{ let __recv = c.clone(); let __recv_ptr: *const LRU = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const LRU }; let __result = unsafe { &*__recv_ptr }.keys(); __result })), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*c.lock().unwrap().as_ref().unwrap()).items.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    let mut words = strings::fields(Arc::new(Mutex::new(Some("b c a b a a".to_string()))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
    let mut freq = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))));
    { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        { let __map_holder = freq.clone(); let __key = (*w).clone(); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    } }
    type kv = AnonymousStruct1;
    let mut kvs: Arc<Mutex<Option<Vec<kv>>>> = Arc::new(Mutex::new(None));
    for (k, n) in { let __range_holder = freq.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = kvs.clone(); let __append_value = kv { k: Arc::new(Mutex::new(Some(k.clone()))), n: Arc::new(Mutex::new(Some((*n.lock().unwrap().as_mut().unwrap())))), ..Default::default() }; let mut __appended: Vec<kv> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.go_append_copying(|__go_slice| { __go_slice.push(__append_value); }, |__go_elem| __go_elem.__go_value_clone()); Arc::new(Mutex::new(Some(__appended))) })); *kvs.lock().unwrap() = __slice_header; };
    }
    let kvs_closure_clone = kvs.clone(); { let __sort_target = kvs_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Box::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        if {
            let __tmp_x = (*{ let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.n.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.n.lock().unwrap().as_ref().unwrap());
            __tmp_x != __tmp_y
        } {
        return {
            let __tmp_x = (*{ let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.n.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.n.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        };
    }
        return {
            let __tmp_x = { let __selector_holder = { let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.k.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned };
            let __tmp_y = { let __selector_holder = { let __seq = { let __seq_holder = kvs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.k.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned };
            __tmp_x < __tmp_y
        };
    }) as Box<dyn FnMut(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
    println!("{}", format!("{}", format_slice(&kvs)));
    let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = kvs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, e) in __range_values.iter().enumerate() {
        let e = &e.__go_value_clone();
        if { let __tmp_x = i as i64; let __tmp_y = 0; __tmp_x > __tmp_y } {
        (*sb.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(", ".to_string()))));
    }
        (*sb.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("{}={}", format!("{}", (*e.k.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*e.n.lock().unwrap().as_ref().unwrap())))))));
    } }
    println!("{}", format!("{}", (*(*sb.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub k: Arc<Mutex<Option<String>>>,
    pub n: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.k.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            k: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            k: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.k.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for entry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for LRU {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
