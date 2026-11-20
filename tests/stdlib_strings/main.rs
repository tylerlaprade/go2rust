use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
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

fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f32>() {
        v.to_string()
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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i32>() { return "int" }
    if val.is::<i64>() { return "int64" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u32>() { return "uint" }
    if val.is::<u64>() { return "uint64" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i32>>() { return "[]int" }
    if val.is::<Vec<i64>>() { return "[]int64" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
}

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &dyn Any) -> bool {
    false
}

// The basic value a newtype such as "type Level int" holds, boxed.
fn go_format_named_basic_value(__value: &dyn Any) -> Option<Box<dyn Any>> {
    None
}

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text, go_format_named_basic_value};
    use std::any::Any;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

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
fn go_format_argument(spec: &GoFormatSpec, verb: char, value: &dyn Any) -> Option<String> {
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
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
        let guard = v.borrow();
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
fn go_format_number(chars: &mut std::iter::Peekable<std::str::Chars>, args: &[Box<dyn Any>], next: &mut usize) -> Result<Option<i64>, ()> {
    if chars.peek() == Some(&'*') {
        chars.next();
        let arg: &dyn Any = match args.get(*next) {
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
fn go_format_is_basic(value: &dyn Any) -> bool {
    macro_rules! is_one_of {
        ($($t:ty),*) => { false $(|| value.is::<$t>())* };
    }
    is_one_of!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64, bool, String, &str, Rc<RefCell<Option<Vec<u8>>>>)
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &dyn Any) -> String {
    let text = go_format_argument(spec, go_format_plain_verb(value), value).unwrap_or_default();
    format!("{}={}", __go_type_name(value), text)
}

// The verb that writes value as %v does and leaves the flags as they are.
// %v itself would read + and # as %+v and %#v.
fn go_format_plain_verb(value: &dyn Any) -> char {
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
pub(super) fn go_sprintf(format: &str, args: &[Box<dyn Any>], wraps: bool) -> String {
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

fn main() {
        // Basic string operations
    println!("{}", format!("{}", "=== Basic string operations ===".to_string()));

    let mut str = Rc::new(RefCell::new(Some("Hello, World!".to_string())));
    print!("Original string: {}\n", { let __v = (*str.borrow().as_ref().unwrap()).clone(); __v });
    print!("Length: {}\n", (*str.borrow().as_ref().unwrap()).len());

        // String indexing and slicing
    print!("First character: {}\n", ({ let __s = &((*str.borrow().as_ref().unwrap()).clone()); __s.as_bytes()[(0) as usize] }) as u8 as char);
    print!("Last character: {}\n", ({ let __s = &((*str.borrow().as_ref().unwrap()).clone()); __s.as_bytes()[(((*str.borrow().as_ref().unwrap()).len() as i32) - (1 as i32)) as usize] }) as u8 as char);
    print!("Substring [0:5]: {}\n", (*Rc::new(RefCell::new(Some({ let __s = &((*str.borrow().as_ref().unwrap()).clone()); let __low = (0) as usize; let __high = (5) as usize; __s[__low..__high].to_string() }))).borrow().as_ref().unwrap()));
    print!("Substring [7:]: {}\n", (*Rc::new(RefCell::new(Some({ let __s = &((*str.borrow().as_ref().unwrap()).clone()); let __low = (7) as usize; __s[__low..].to_string() }))).borrow().as_ref().unwrap()));

        // String concatenation
    println!("{}", format!("{}", "\n=== String concatenation ===".to_string()));
    let mut first = Rc::new(RefCell::new(Some("Hello".to_string())));
    let mut second = Rc::new(RefCell::new(Some("World".to_string())));
    let mut combined = Rc::new(RefCell::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", (*first.borrow().as_ref().unwrap())));
        __s.push_str(&format!("{}", ", ".to_string()));
        __s.push_str(&format!("{}", (*second.borrow().as_ref().unwrap())));
        __s.push_str(&format!("{}", "!".to_string()));
        __s
    })));
    print!("Concatenated: {}\n", { let __v = (*combined.borrow().as_ref().unwrap()).clone(); __v });

        // Using strings package
    println!("{}", format!("{}", "\n=== strings package functions ===".to_string()));

    let mut text = Rc::new(RefCell::new(Some("  Go is awesome for systems programming  ".to_string())));
    print!("Original: '{}'\n", { let __v = (*text.borrow().as_ref().unwrap()).clone(); __v });

        // Trimming
    let mut trimmed = Rc::new(RefCell::new(Some({ let __s = (*text.borrow().as_ref().unwrap()).clone(); __s.trim().to_string() })));
    print!("Trimmed: '{}'\n", { let __v = (*trimmed.borrow().as_ref().unwrap()).clone(); __v });

        // Case conversion
    print!("Upper: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*trimmed.borrow().as_ref().unwrap()).clone(); __s.to_uppercase() }))).borrow().as_ref().unwrap()));
    print!("Lower: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*trimmed.borrow().as_ref().unwrap()).clone(); __s.to_lowercase() }))).borrow().as_ref().unwrap()));
    print!("Title: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*trimmed.borrow().as_ref().unwrap()).clone(); let mut __out = String::new(); let mut __new_word = true; for __ch in __s.chars() { if __ch.is_alphanumeric() { if __new_word { for __upper in __ch.to_uppercase() { __out.push(__upper); } } else { __out.push(__ch); } __new_word = false; } else { __out.push(__ch); __new_word = true; } } __out }))).borrow().as_ref().unwrap()));

        // String searching
    println!("{}", format!("{}", "\n=== String searching ===".to_string()));
    let mut searchText = Rc::new(RefCell::new(Some("The quick brown fox jumps over the lazy dog".to_string())));
    print!("Text: {}\n", { let __v = (*searchText.borrow().as_ref().unwrap()).clone(); __v });

    print!("Contains 'fox': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __arg = "fox".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap()));
    print!("Contains 'cat': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __arg = "cat".to_string(); __s.contains(&__arg) }))).borrow().as_ref().unwrap()));

    print!("Index of 'fox': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __substr = "fox".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).borrow().as_ref().unwrap()));
    print!("Index of 'cat': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __substr = "cat".to_string(); __s.find(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).borrow().as_ref().unwrap()));
    print!("IndexAny of vowels: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __chars = "aeiou".to_string(); __s.char_indices().find(|(_, __ch)| __chars.contains(*__ch)).map(|(__i, _)| __i as i32).unwrap_or(-1) }))).borrow().as_ref().unwrap()));

    print!("Last index of 'the': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __substr = "the".to_string(); __s.rfind(&__substr).map(|__i| __i as i32).unwrap_or(-1) }))).borrow().as_ref().unwrap()));

    print!("Count of 'the': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*searchText.borrow().as_ref().unwrap()).clone(); let __substr = "the".to_string(); if __substr.is_empty() { __s.chars().count() as i32 + 1 } else { __s.matches(&__substr).count() as i32 } }))).borrow().as_ref().unwrap()));

        // String prefixes and suffixes
    println!("{}", format!("{}", "\n=== Prefixes and suffixes ===".to_string()));
    let mut filename = Rc::new(RefCell::new(Some("document.txt".to_string())));
    print!("Filename: {}\n", { let __v = (*filename.borrow().as_ref().unwrap()).clone(); __v });
    print!("Has .txt suffix: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*filename.borrow().as_ref().unwrap()).clone(); let __arg = ".txt".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap()));
    print!("Has .pdf suffix: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*filename.borrow().as_ref().unwrap()).clone(); let __arg = ".pdf".to_string(); __s.ends_with(&__arg) }))).borrow().as_ref().unwrap()));
    print!("Has 'doc' prefix: {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*filename.borrow().as_ref().unwrap()).clone(); let __arg = "doc".to_string(); __s.starts_with(&__arg) }))).borrow().as_ref().unwrap()));

        // String splitting and joining
    println!("{}", format!("{}", "\n=== Splitting and joining ===".to_string()));
    let mut csv = Rc::new(RefCell::new(Some("apple,banana,cherry,date".to_string())));
    print!("CSV: {}\n", { let __v = (*csv.borrow().as_ref().unwrap()).clone(); __v });

    let mut fruits = Rc::new(RefCell::new(Some({ let __s = (*csv.borrow().as_ref().unwrap()).clone(); let __sep = ",".to_string(); __s.split(&__sep).map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    print!("Split result: {}\n", format_slice(&fruits));

    let mut rejoined = Rc::new(RefCell::new(Some({ let __parts = (*fruits.borrow()).as_ref().cloned().unwrap_or_default(); let __sep = " | ".to_string(); __parts.join(&__sep) })));
    print!("Rejoined: {}\n", { let __v = (*rejoined.borrow().as_ref().unwrap()).clone(); __v });
    let (mut before, mut after, mut found) = { let __s = (*csv.borrow().as_ref().unwrap()).clone(); let __sep = ",".to_string(); if let Some(__idx) = __s.find(&__sep) { let __before = __s[..__idx].to_string(); let __after = __s[__idx + __sep.len()..].to_string(); (Rc::new(RefCell::new(Some(__before))), Rc::new(RefCell::new(Some(__after))), true) } else { (Rc::new(RefCell::new(Some(__s))), Rc::new(RefCell::new(Some(String::new()))), false) } };
    print!("Cut first comma: {} | {} | {}\n", { let __v = (*before.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*after.borrow().as_ref().unwrap()).clone(); __v }, found);
    { let (__tmp_0, __tmp_1, __tmp_2) = { let __s = (*csv.borrow().as_ref().unwrap()).clone(); let __sep = ";".to_string(); if let Some(__idx) = __s.find(&__sep) { let __before = __s[..__idx].to_string(); let __after = __s[__idx + __sep.len()..].to_string(); (Rc::new(RefCell::new(Some(__before))), Rc::new(RefCell::new(Some(__after))), true) } else { (Rc::new(RefCell::new(Some(__s))), Rc::new(RefCell::new(Some(String::new()))), false) } }; let __moved_tmp_0 = { let mut __guard = __tmp_0.borrow_mut(); __guard.take() }; *before.borrow_mut() = __moved_tmp_0; let __moved_tmp_1 = { let mut __guard = __tmp_1.borrow_mut(); __guard.take() }; *after.borrow_mut() = __moved_tmp_1; found = __tmp_2; };
    print!("Cut semicolon: {} | {} | {}\n", { let __v = (*before.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*after.borrow().as_ref().unwrap()).clone(); __v }, found);

        // Fields (split on whitespace)
    let mut sentence = Rc::new(RefCell::new(Some("The quick brown fox".to_string())));
    let mut words = Rc::new(RefCell::new(Some({ let __s = (*sentence.borrow().as_ref().unwrap()).clone(); __s.split_whitespace().map(|__part| __part.to_string()).collect::<Vec<String>>() })));
    print!("Words: {}\n", format_slice(&words));

        // String replacement
    println!("{}", format!("{}", "\n=== String replacement ===".to_string()));
    let mut original = Rc::new(RefCell::new(Some("I like cats and cats like me".to_string())));
    print!("Original: {}\n", { let __v = (*original.borrow().as_ref().unwrap()).clone(); __v });

    let mut replaced = Rc::new(RefCell::new(Some({ let __s = (*original.borrow().as_ref().unwrap()).clone(); let __old = "cats".to_string(); let __new = "dogs".to_string(); let __n = 1; if __n < 0 { __s.replace(&__old, &__new) } else { __s.replacen(&__old, &__new, __n as usize) } })));
    print!("Replace first 'cats': {}\n", { let __v = (*replaced.borrow().as_ref().unwrap()).clone(); __v });

    let mut replacedAll = Rc::new(RefCell::new(Some({ let __s = (*original.borrow().as_ref().unwrap()).clone(); let __old = "cats".to_string(); let __new = "dogs".to_string(); __s.replace(&__old, &__new) })));
    print!("Replace all 'cats': {}\n", { let __v = (*replacedAll.borrow().as_ref().unwrap()).clone(); __v });

        // String repetition
    println!("{}", format!("{}", "\n=== String repetition ===".to_string()));
    let mut pattern = Rc::new(RefCell::new(Some("Go! ".to_string())));
    let mut repeated = Rc::new(RefCell::new(Some({ let __s = (*pattern.borrow().as_ref().unwrap()).clone(); let __count = 3; __s.repeat(__count as usize) })));
    print!("Repeated: {}\n", { let __v = (*repeated.borrow().as_ref().unwrap()).clone(); __v });

        // String comparison
    println!("{}", format!("{}", "\n=== String comparison ===".to_string()));
    let mut str1 = Rc::new(RefCell::new(Some("apple".to_string())));
    let mut str2 = Rc::new(RefCell::new(Some("banana".to_string())));
    let mut str3 = Rc::new(RefCell::new(Some("apple".to_string())));

    print!("'{}' == '{}': {}\n", { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str2.borrow().as_ref().unwrap()).clone(); __v }, (*str1.borrow().as_ref().unwrap()) == (*str2.borrow().as_ref().unwrap()));
    print!("'{}' == '{}': {}\n", { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str3.borrow().as_ref().unwrap()).clone(); __v }, (*str1.borrow().as_ref().unwrap()) == (*str3.borrow().as_ref().unwrap()));
    print!("'{}' < '{}': {}\n", { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str2.borrow().as_ref().unwrap()).clone(); __v }, (*str1.borrow().as_ref().unwrap()) < (*str2.borrow().as_ref().unwrap()));
    print!("Compare('{}', '{}'): {}\n", { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str2.borrow().as_ref().unwrap()).clone(); __v }, (*Rc::new(RefCell::new(Some({ let __a = (*str1.borrow().as_ref().unwrap()).clone(); let __b = (*str2.borrow().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).borrow().as_ref().unwrap()));
    print!("Compare('{}', '{}'): {}\n", { let __v = (*str2.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, (*Rc::new(RefCell::new(Some({ let __a = (*str2.borrow().as_ref().unwrap()).clone(); let __b = (*str1.borrow().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).borrow().as_ref().unwrap()));
    print!("Compare('{}', '{}'): {}\n", { let __v = (*str1.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*str3.borrow().as_ref().unwrap()).clone(); __v }, (*Rc::new(RefCell::new(Some({ let __a = (*str1.borrow().as_ref().unwrap()).clone(); let __b = (*str3.borrow().as_ref().unwrap()).clone(); match __a.cmp(&__b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1 } }))).borrow().as_ref().unwrap()));

        // Case-insensitive comparison
    print!("EqualFold('Apple', 'APPLE'): {}\n", (*Rc::new(RefCell::new(Some({ let __a = "Apple".to_string(); let __b = "APPLE".to_string(); __a.to_lowercase() == __b.to_lowercase() }))).borrow().as_ref().unwrap()));

        // String building with strings.Builder
    println!("{}", format!("{}", "\n=== String building ===".to_string()));
    let mut builder: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(Some(Default::default())));

    (*builder.borrow_mut().as_mut().unwrap()).push_str("Building ");
    (*builder.borrow_mut().as_mut().unwrap()).push_str("a ");
    (*builder.borrow_mut().as_mut().unwrap()).push_str("string ");
    (*builder.borrow_mut().as_mut().unwrap()).push_str("efficiently");

    let mut built = Rc::new(RefCell::new(Some({ let __builder = builder.clone(); let __guard = __builder.borrow(); let __value = (*__guard.as_ref().unwrap()).clone(); drop(__guard); __value })));
    print!("Built string: {}\n", { let __v = (*built.borrow().as_ref().unwrap()).clone(); __v });
    print!("Builder length: {}\n", (*builder.borrow().as_ref().unwrap()).len() as i32);

        // Rune iteration (Unicode support)
    println!("{}", format!("{}", "\n=== Unicode and runes ===".to_string()));
    let mut unicode = Rc::new(RefCell::new(Some("Hello, \u{4e16}\u{754c}! \u{1f30d}".to_string())));
    print!("Unicode string: {}\n", { let __v = (*unicode.borrow().as_ref().unwrap()).clone(); __v });
    print!("Byte length: {}\n", (*unicode.borrow().as_ref().unwrap()).len());

    let mut runeCount = Rc::new(RefCell::new(Some(0)));
    for (_, r) in (*unicode.borrow().as_ref().unwrap()).char_indices() {
        { let mut guard = runeCount.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        print!("Rune: {} (U+{})\n", r, format!("{:04X}", r as u32));
    }
    print!("Rune count: {}\n", { let __v = (*runeCount.borrow().as_ref().unwrap()).clone(); __v });

        // String trimming variations
    println!("{}", format!("{}", "\n=== String trimming variations ===".to_string()));
    let mut messy = Rc::new(RefCell::new(Some("!!!Hello World!!!".to_string())));
    print!("Original: {}\n", { let __v = (*messy.borrow().as_ref().unwrap()).clone(); __v });
    print!("TrimLeft '!': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*messy.borrow().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_start_matches(|__ch| __cutset.contains(__ch)).to_string() }))).borrow().as_ref().unwrap()));
    print!("TrimRight '!': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*messy.borrow().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_end_matches(|__ch| __cutset.contains(__ch)).to_string() }))).borrow().as_ref().unwrap()));
    print!("Trim '!': {}\n", (*Rc::new(RefCell::new(Some({ let __s = (*messy.borrow().as_ref().unwrap()).clone(); let __cutset = "!".to_string(); __s.trim_matches(|__ch| __cutset.contains(__ch)).to_string() }))).borrow().as_ref().unwrap()));

        // String formatting with different verbs
    println!("{}", format!("{}", "\n=== String formatting ===".to_string()));
    let mut name = Rc::new(RefCell::new(Some("Alice".to_string())));
    let mut age = Rc::new(RefCell::new(Some(30)));
    let mut height = Rc::new(RefCell::new(Some(5.6)));

    print!("Name: {}, Age: {}, Height: {:.1}\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*age.borrow().as_ref().unwrap()).clone(); __v }, { let __v = (*height.borrow().as_ref().unwrap()).clone(); __v });
    print!("{}", go_sprintf(&"Quoted string: %q\n".to_string(), &[Box::new({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any>], false));
    print!("String with width: '{:>10}'\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });
    print!("Left-aligned: '{:<10}'\n", { let __v = (*name.borrow().as_ref().unwrap()).clone(); __v });
}