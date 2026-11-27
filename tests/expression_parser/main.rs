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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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
    } else if let Some(v) = value.downcast_ref::<BinPtr>() {
        { let __guard = v.0.lock().unwrap(); match __guard.as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }
    } else if let Some(v) = value.downcast_ref::<NumPtr>() {
        { let __guard = v.0.lock().unwrap(); match __guard.as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }
    } else if let Some(v) = value.downcast_ref::<VarPtr>() {
        { let __guard = v.0.lock().unwrap(); match __guard.as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn go_install_panic_exit() {
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
    }
    go_panic_value_text(__payload)
}

fn go_panic_value_text(__payload: &dyn Any) -> String {
    // A boxed payload holds the Go value itself, so its string is printed
    // as it is.
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return __text.clone();
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return __text.to_string();
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_value_text(&**__inner);
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_value_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    // Go prints a float in a panic as +1.500000e+000; a float32 is widened
    // to float64 first, as Go's print does.
    let __float = __payload
        .downcast_ref::<f64>()
        .copied()
        .or_else(|| __payload.downcast_ref::<f32>().map(|__value| *__value as f64));
    if let Some(__value) = __float {
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if __value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
    }
    String::from(GO_UNPRINTABLE_PANIC_VALUE)
}

const GO_UNPRINTABLE_PANIC_VALUE: &str = "(unprintable panic value)";

fn go_runtime_error_text(__text: String) -> String {
    if __text == "attempt to divide by zero" || __text == "attempt to calculate the remainder with a divisor of zero" {
        return "runtime error: integer divide by zero".to_string();
    }
    if let Some(__rest) = __text.strip_prefix("index out of bounds: the len is ") {
        if let Some((__len, __index)) = __rest.split_once(" but the index is ") {
            return format!("runtime error: index out of range [{}] with length {}", __index, __len);
        }
    }
    // A nil pointer's cell is empty, so reading through it unwraps None.
    if __text == "called `Option::unwrap()` on a `None` value" || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return "runtime error: invalid memory address or nil pointer dereference".to_string();
    }
    __text
}

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
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
    if val.is::<BinPtr>() { return "*main.Bin" }
    if val.is::<NumPtr>() { return "*main.Num" }
    if val.is::<VarPtr>() { return "*main.Var" }
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

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &(dyn Any + Send + Sync)) -> bool {
    false
        || __value.is::<BinPtr>()
        || __value.is::<NumPtr>()
        || __value.is::<VarPtr>()
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &(dyn Any + Send + Sync)) -> Option<Box<dyn Any + Send + Sync>> {
    None
}

// value as %+v prints it, when it is a struct.
fn go_format_fields_value(__value: &(dyn Any + Send + Sync)) -> Option<String> {
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

pub trait Node: std::fmt::Display + Any {
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool;
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn Node + Send + Sync> {
    fn clone(&self) -> Self {
        Node::__go_clone_box_node(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Num {
    pub v: Arc<Mutex<Option<f64>>>,
}

impl Num {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl Default for Num {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            v: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.v.lock().unwrap().as_ref().cloned() }; let __right = { other.v.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Var {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Var {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Var {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}
impl PartialEq for Var {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone)]
pub struct Bin {
    pub op: Arc<Mutex<Option<u8>>>,
    pub l: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>,
    pub r: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>,
}

impl Bin {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.op.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.l.clone();
        let __go_clone_1_1 = self.r.clone();
        Self {
            op: __go_clone_0_0,
            l: __go_clone_1_0,
            r: __go_clone_1_1,
        }
    }
}


impl Default for Bin {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_1_1 = Arc::new(Mutex::new(None));
        Self {
            op: __go_default_0_0,
            l: __go_default_1_0,
            r: __go_default_1_1,
        }
    }
}

impl std::fmt::Display for Bin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct parser {
    pub src: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<i64>>>,
}

impl parser {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.src.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            src: __go_clone_0_0,
            pos: __go_clone_1_0,
        }
    }
}


impl Default for parser {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            src: __go_default_0_0,
            pos: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for parser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.src.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.pos.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static ops_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<u8, Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ops_1.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<u8, Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>>::new();
        __go_map.insert(('+' as u8), Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>| -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        return (
            { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y },
            Arc::new(Mutex::new(None))
        );
    }) as Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>))));
        __go_map.insert(('-' as u8), Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>| -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        return (
            { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x - __tmp_y },
            Arc::new(Mutex::new(None))
        );
    }) as Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>))));
        __go_map.insert(('*' as u8), Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>| -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        return (
            { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y },
            Arc::new(Mutex::new(None))
        );
    }) as Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>))));
        __go_map.insert(('/' as u8), Arc::new(Mutex::new(Some(Arc::new(move |a: Arc<Mutex<Option<f64>>>, b: Arc<Mutex<Option<f64>>>| -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        if { let __tmp_x = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0.0; __tmp_x == __tmp_y } {
        return (
            0.0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("division by zero".to_string())))))
        );
    }
        return (
            { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x / __tmp_y },
            Arc::new(Mutex::new(None))
        );
    }) as Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync>))));
        *ops_1.lock().unwrap() = Some(__go_map);
    }
}


impl Num {
    pub fn eval(&self, __arg0: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        return ((*self.v.lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)));
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        strconv::format_float(Arc::new(Mutex::new(Some({ let __selector_holder = self.v.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new(Some(('g' as u8) as u8))), Arc::new(Mutex::new(Some(-1))), Arc::new(Mutex::new(Some(64))))
    }
}

impl Node for Num {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        Num::eval(self, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Num>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct NumPtr(pub Arc<Mutex<Option<Num>>>);

impl std::fmt::Display for NumPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for NumPtr {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Num::eval(__recv, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NumPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Var {
    pub fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut x, mut ok) = { let __map = { let __map_holder = env.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0.0))), false) } };
        if !ok {
        return (
            0.0,
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("undefined: {}", (*self.name.lock().unwrap().as_ref().unwrap())))))))
        );
    }
        return ({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }, Arc::new(Mutex::new(None)));
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }
}

impl Node for Var {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        Var::eval(self, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Var>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct VarPtr(pub Arc<Mutex<Option<Var>>>);

impl std::fmt::Display for VarPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for VarPtr {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Var::eval(__recv, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<VarPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Bin {
    pub fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut l, mut err) = (*self.l.lock().unwrap().as_ref().unwrap()).eval(env.clone());
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0.0, { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        let (mut r, __tmp_1) = (*self.r.lock().unwrap().as_ref().unwrap()).eval(env.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0.0, { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        { let __f_holder = { let __map = { let __map_holder = ops_1.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = self.op.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }; let __f: Arc<dyn Fn(Arc<Mutex<Option<f64>>>, Arc<Mutex<Option<f64>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(l))), Arc::new(Mutex::new(Some(r)))) }
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("({} {} {})", format!("{}", match self.l.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }), format!("{}", char::from_u32(((*self.op.lock().unwrap().as_ref().unwrap())) as i64 as u32).unwrap_or('\u{FFFD}')), format!("{}", match self.r.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() })))))
    }
}

impl Node for Bin {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        Bin::eval(self, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Bin>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct BinPtr(pub Arc<Mutex<Option<Bin>>>);

impl std::fmt::Display for BinPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Node for BinPtr {
    fn eval(&self, env: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<f64>>>>>>>) -> (f64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Bin::eval(__recv, env)
    }
    fn __go_clone_box_node(&self) -> Box<dyn Node + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Node + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_node(&self, other: &(dyn Node + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<BinPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl parser {
    pub fn peek(&mut self) -> u8 {
        while {
            let __go_cond_0 = {
                let __tmp_x = ((*self.pos.clone().lock().unwrap().as_ref().unwrap()) as i64);
                let __tmp_y = (({ let __len = (*self.src.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64);
                __tmp_x < __tmp_y
            };
            if __go_cond_0 {
                let __go_cond_1 = {
                    let __tmp_x = { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] };
                    let __tmp_y = (' ' as u8) as u8;
                    __tmp_x == __tmp_y
                };
                __go_cond_1
            } else {
                false
            }
        } {
        { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        if {
            let __tmp_x = ((*self.pos.clone().lock().unwrap().as_ref().unwrap()) as i64);
            let __tmp_y = (({ let __len = (*self.src.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64);
            __tmp_x >= __tmp_y
        } {
        return 0;
    }
        { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] }
    }

    pub fn expr(&mut self) -> (Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut left, mut err) = self.term();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        loop {
        let mut c = self.peek();
        if { let __tmp_x = c; let __tmp_y = ('+' as u8) as u8; __tmp_x != __tmp_y } && { let __tmp_x = c; let __tmp_y = ('-' as u8) as u8; __tmp_x != __tmp_y } {
        return (left.clone(), Arc::new(Mutex::new(None)));
    }
        { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let (mut right, mut err) = self.term();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(BinPtr(Arc::new(Mutex::new(Some(Bin { op: Arc::new(Mutex::new(Some(c))), l: { let __iface_value = left.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }, r: { let __iface_value = right.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }, ..Default::default() }))).clone())) as Box<dyn Node + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *left.lock().unwrap() = __iface_value; };
    }
    }

    pub fn term(&mut self) -> (Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut left, mut err) = self.factor();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        loop {
        let mut c = self.peek();
        if { let __tmp_x = c; let __tmp_y = ('*' as u8) as u8; __tmp_x != __tmp_y } && { let __tmp_x = c; let __tmp_y = ('/' as u8) as u8; __tmp_x != __tmp_y } {
        return (left.clone(), Arc::new(Mutex::new(None)));
    }
        { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        let (mut right, mut err) = self.factor();
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
        { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(BinPtr(Arc::new(Mutex::new(Some(Bin { op: Arc::new(Mutex::new(Some(c))), l: { let __iface_value = left.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }, r: { let __iface_value = right.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }, ..Default::default() }))).clone())) as Box<dyn Node + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *left.lock().unwrap() = __iface_value; };
    }
    }

    pub fn factor(&mut self) -> (Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let mut c = self.peek();
        if { let __tmp_x = c; let __tmp_y = ('(' as u8) as u8; __tmp_x == __tmp_y } {
            { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            let (mut n, mut err) = self.expr();
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
            if { let __tmp_x = self.peek(); let __tmp_y = (')' as u8) as u8; __tmp_x != __tmp_y } {
        return (
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("expected ) at {}", (*self.pos.lock().unwrap().as_ref().unwrap())))))))
        );
    }
            { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
            return (n.clone(), Arc::new(Mutex::new(None)));
        } else if { let __tmp_x = c; let __tmp_y = ('0' as u8) as u8; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = ('9' as u8) as u8; __tmp_x <= __tmp_y } {
            let mut start = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
            while {
                let __go_cond_0 = {
                    let __tmp_x = ((*self.pos.clone().lock().unwrap().as_ref().unwrap()) as i64);
                    let __tmp_y = (({ let __len = (*self.src.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64);
                    __tmp_x < __tmp_y
                };
                if __go_cond_0 {
                    let __go_cond_1 = {
                        let __go_cond_2 = {
                            let __go_cond_3 = {
                                let __tmp_x = { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] };
                                let __tmp_y = ('0' as u8) as u8;
                                __tmp_x >= __tmp_y
                            };
                            if __go_cond_3 {
                                let __go_cond_4 = {
                                    let __tmp_x = { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] };
                                    let __tmp_y = ('9' as u8) as u8;
                                    __tmp_x <= __tmp_y
                                };
                                __go_cond_4
                            } else {
                                false
                            }
                        };
                        if __go_cond_2 {
                            true
                        } else {
                            let __go_cond_5 = {
                                let __tmp_x = { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] };
                                let __tmp_y = ('.' as u8) as u8;
                                __tmp_x == __tmp_y
                            };
                            __go_cond_5
                        }
                    };
                    __go_cond_1
                } else {
                    false
                }
            } {
        { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            let (mut v, mut err) = strconv::parse_float(Arc::new(Mutex::new(Some({ let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = (*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[__low..__high].to_string() }))), Arc::new(Mutex::new(Some(64))));
            if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
            return (
                Arc::new(Mutex::new(Some(Box::new(NumPtr(Arc::new(Mutex::new(Some(Num { v: Arc::new(Mutex::new(Some(v))), ..Default::default() }))).clone())) as Box<dyn Node + Send + Sync>))),
                Arc::new(Mutex::new(None))
            );
        } else if unicode::is_letter(Arc::new(Mutex::new(Some(c as i32)))) {
            let mut start = Arc::new(Mutex::new(Some({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
            while {
                let __go_cond_0 = {
                    let __tmp_x = ((*self.pos.clone().lock().unwrap().as_ref().unwrap()) as i64);
                    let __tmp_y = (({ let __len = (*self.src.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64);
                    __tmp_x < __tmp_y
                };
                if __go_cond_0 {
                    let __go_cond_1 = unicode::is_letter(Arc::new(Mutex::new(Some({ let __arg_value = { let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); __s.as_bytes()[(*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize] } as i32; __arg_value }))));
                    __go_cond_1
                } else {
                    false
                }
            } {
        { let __target = self.pos.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
            return (
                Arc::new(Mutex::new(Some(Box::new(VarPtr(Arc::new(Mutex::new(Some(Var { name: Arc::new(Mutex::new(Some((*Arc::new(Mutex::new(Some({ let __s = &((*self.src.lock().unwrap().as_ref().unwrap()).clone()); let __low = ({ let __v = (*start.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __high = (*self.pos.clone().lock().unwrap().as_ref().unwrap()) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone()))), ..Default::default() }))).clone())) as Box<dyn Node + Send + Sync>))),
                Arc::new(Mutex::new(None))
            );
        }
        return (
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("{}", go_sprintf(&"unexpected %q at %d".to_string(), &[{ let __go_format_arg = Box::new(c) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new({ let __selector_holder = self.pos.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>; __go_format_arg }], true)))))))
        );
    }
}

pub fn parse(src: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    let mut p = Arc::new(Mutex::new(Some(parser { src: Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    let (mut n, mut err) = { let __recv = p.clone(); let __recv_ptr: *mut parser = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut parser }; let __result = unsafe { &mut *__recv_ptr }.expr(); __result };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
    if { let __tmp_x = { let __recv = p.clone(); let __recv_ptr: *mut parser = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut parser }; let __result = unsafe { &mut *__recv_ptr }.peek(); __result }; let __tmp_y = 0 as u8; __tmp_x != __tmp_y } {
        return (
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("trailing input at {}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).pos.clone(); __field }.lock().unwrap().as_ref().unwrap())))))))
        );
    }
    return (n.clone(), Arc::new(Mutex::new(None)));
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut env = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<f64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(3.0)))), ("y".to_string(), Arc::new(Mutex::new(Some(4.0))))]))));
    for src in &vec!["1 + 2 * 3".to_string(), "(1 + 2) * 3".to_string(), "x * x + y * y".to_string(), "10 / (y - 4)".to_string(), "2 * z".to_string(), "(1 + 2".to_string(), "7 / 2".to_string()] {
        let (mut n, mut err) = parse(Arc::new(Mutex::new(Some(src.clone()))));
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {} {} {}", format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("-".to_string()))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap())), format!("{}", src), format!("{}", "parse error:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        continue
    }
        let (mut v, __tmp_1) = (*n.lock().unwrap().as_ref().unwrap()).eval(env.clone()); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        print!("{} => error: {}\n", format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        continue
    }
        print!("{}", go_sprintf(&"%s => %g\n".to_string(), &[{ let __go_format_arg: Box<dyn Any + Send + Sync> = if { let __nil_result = (*n.lock().unwrap()).is_none(); __nil_result } { Box::new(GoFormatNil) } else { { let __iface_holder = n.clone(); let __iface_guard = __iface_holder.lock().unwrap(); let __iface: Box<dyn Node + Send + Sync> = __iface_guard.as_ref().unwrap().__go_clone_box_node(); __iface as Box<dyn Any + Send + Sync> } }; __go_format_arg }, { let __go_format_arg = Box::new(v) as Box<dyn Any + Send + Sync>; __go_format_arg }], false));
    }
    let mut n: Arc<Mutex<Option<Box<dyn Node + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(NumPtr(Arc::new(Mutex::new(Some(Num { v: Arc::new(Mutex::new(Some(2.5))), ..Default::default() }))).clone())) as Box<dyn Node + Send + Sync>)));
    println!("{}", format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }));
    print!("{} {}\n", format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }), format!("{}", match n.lock().unwrap().as_ref() { Some(__iface) => __iface.as_ref().__go_print(false), None => "<nil>".to_string() }));
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}


impl GoValueClone for Num {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Var {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Bin {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for parser {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
