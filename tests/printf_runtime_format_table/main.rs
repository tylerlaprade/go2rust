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
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
        format_slice(v)
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
    if val.is::<Rc<RefCell<Option<Vec<u8>>>>>() { return "[]uint8" }
    std::any::type_name_of_val(val)
}

// Whether value has a String or an Error method. fmt prints such a value as
// the one text the method returns.
fn go_format_is_text(__value: &dyn Any) -> bool {
    false
}

#[allow(unused_imports)]
mod __go_runtime_format {
    use super::{format_any, __go_type_name, go_format_is_text};
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

// go_sprintf for f(format, args...), where the arguments are a slice.
pub(super) fn go_sprintf_spread(format: &str, args: &Rc<RefCell<Option<Vec<Box<dyn Any>>>>>, wraps: bool) -> String {
    let guard = args.borrow();
    go_sprintf(format, guard.as_deref().unwrap_or(&[]), wraps)
}
}
use __go_runtime_format::{go_sprintf, go_sprintf_spread};

/// sprintf receives its format as a value, so the format is only known when
/// the program runs.
pub fn sprintf(format: Rc<RefCell<Option<String>>>, args: Rc<RefCell<Option<Vec<Box<dyn Any>>>>>) -> Rc<RefCell<Option<String>>> {
    Rc::new(RefCell::new(Some(format!("{}", go_sprintf_spread(&(*format.borrow().as_ref().unwrap()).clone(), &args, false)))))
}

pub fn show(format: Rc<RefCell<Option<String>>>, text: Rc<RefCell<Option<String>>>) {
    println!("{} {} {}", format!("{}", { let __v = (*format.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "=>".to_string()), format!("{}", {
        let mut __s = String::new();
        __s.push_str(&format!("{}", "[".to_string()));
        __s.push_str(&format!("{}", (*text.borrow().as_ref().unwrap())));
        __s.push_str(&format!("{}", "]".to_string()));
        __s
    }));
}

fn main() {
    let mut intFormats = Rc::new(RefCell::new(Some(vec!["%d".to_string(), "%5d".to_string(), "%-5d|".to_string(), "%05d".to_string(), "%+d".to_string(), "% d".to_string(), "%x".to_string(), "%X".to_string(), "%#x".to_string(), "%o".to_string(), "%#o".to_string(), "%O".to_string(), "%b".to_string(), "%c".to_string(), "%q".to_string(), "%U".to_string(), "%#U".to_string(), "%v".to_string(), "%3v".to_string(), "%.3d".to_string(), "%8.3d".to_string(), "%08.3d".to_string(), "%-8.3d|".to_string(), "%+05d".to_string(), "%.0d".to_string(), "%s".to_string(), "%t".to_string(), "%f".to_string(), "%e".to_string(), "%T".to_string()])));
    let mut ints = Rc::new(RefCell::new(Some(vec![0, 7, -42, 65, 255, 123456789])));
    { let __range_holder = intFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        { let __range_holder = ints.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for value in __range_values.iter().copied() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(value) as Box<dyn Any>])))));
    } }
    } }
    show(Rc::new(RefCell::new(Some("%d %d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d %d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(-(9000000000) as i64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new({ let __v = Rc::new(RefCell::new(Some(200 as u8))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%x %X %#x".to_string()))), sprintf(Rc::new(RefCell::new(Some("%x %X %#x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(-(255) as i64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new({ let __v = Rc::new(RefCell::new(Some(200 as u8))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new({ let __v = Rc::new(RefCell::new(Some(4096 as i64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));

    let mut floatFormats = Rc::new(RefCell::new(Some(vec!["%v".to_string(), "%g".to_string(), "%G".to_string(), "%e".to_string(), "%E".to_string(), "%f".to_string(), "%F".to_string(), "%.2f".to_string(), "%8.2f|".to_string(), "%-8.2f|".to_string(), "%08.2f".to_string(), "%+.2f".to_string(), "% .2f".to_string(), "%.0f".to_string(), "%.3e".to_string(), "%.3g".to_string(), "%.10g".to_string(), "%.1g".to_string(), "%10.3g|".to_string(), "%6.2f".to_string(), "%d".to_string(), "%s".to_string(), "%T".to_string(), "%5v|".to_string(), "%.0e".to_string(), "%.0g".to_string()])));
    let mut floats = Rc::new(RefCell::new(Some(vec![0.0, 1.0, 2.5, -(2.5), 0.1, 100000.0, 1000000.0, 1234567.0, 123456789.0, 1e20, 1e21, 1e-4, 1e-5, 0.000123456, 3.141592653589793, 1e100, 123.456, 0.5, 1.5, 2.675, 99.995])));
    { let __range_holder = floatFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        { let __range_holder = floats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for value in __range_values.iter().copied() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(value) as Box<dyn Any>])))));
    } }
    } }
    let mut narrowFormats = Rc::new(RefCell::new(Some(vec!["%v".to_string(), "%g".to_string(), "%f".to_string(), "%.2f".to_string(), "%e".to_string(), "%T".to_string()])));
    { let __range_holder = narrowFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(0.1 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(2.5 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(16777216.0 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(3.1415927 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(1e10 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(1e-5 as f32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
    } }

    let mut stringFormats = Rc::new(RefCell::new(Some(vec!["%v".to_string(), "%s".to_string(), "%q".to_string(), "%10s|".to_string(), "%-10s|".to_string(), "%.2s".to_string(), "%10.2s|".to_string(), "%x".to_string(), "%X".to_string(), "% x".to_string(), "%#x".to_string(), "%d".to_string(), "%t".to_string(), "%T".to_string(), "%05s".to_string(), "%#q".to_string(), "%c".to_string()])));
    let mut texts = Rc::new(RefCell::new(Some(vec!["hi".to_string(), "".to_string(), "h\u{e9}llo w\u{f6}rld".to_string(), "tab\there".to_string(), "quote\"d".to_string(), "back\\slash".to_string(), "new\nline".to_string(), "bell\u{7}".to_string(), "`tick`".to_string()])));
    { let __range_holder = stringFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        let mut i = Rc::new(RefCell::new(Some(0)));
    while ((*i.borrow().as_ref().unwrap()) as i32) < ((*texts.borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32) {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new((*texts.borrow().as_ref().unwrap())[((*i.borrow().as_ref().unwrap())) as usize].clone()) as Box<dyn Any>])))));
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    } }

    let mut boolFormats = Rc::new(RefCell::new(Some(vec!["%v".to_string(), "%t".to_string(), "%5t|".to_string(), "%-6t|".to_string(), "%d".to_string(), "%s".to_string(), "%T".to_string(), "%q".to_string()])));
    { let __range_holder = boolFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(true) as Box<dyn Any>])))));
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(false) as Box<dyn Any>])))));
    } }

    show(Rc::new(RefCell::new(Some("plain text".to_string()))), sprintf(Rc::new(RefCell::new(Some("plain text".to_string()))), Rc::new(RefCell::new(Some(vec![])))));
    show(Rc::new(RefCell::new(Some("100%%".to_string()))), sprintf(Rc::new(RefCell::new(Some("100%%".to_string()))), Rc::new(RefCell::new(Some(vec![])))));
    show(Rc::new(RefCell::new(Some("%d%%".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d%%".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(50) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%d %s %v %t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d %s %v %t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new("two".to_string()) as Box<dyn Any>, Box::new(3.5) as Box<dyn Any>, Box::new(true) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%d %d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d %d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new(2) as Box<dyn Any>, Box::new("x".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%!".to_string()))), sprintf(Rc::new(RefCell::new(Some("%!".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%z".to_string()))), sprintf(Rc::new(RefCell::new(Some("%z".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("trailing %".to_string()))), sprintf(Rc::new(RefCell::new(Some("trailing %".to_string()))), Rc::new(RefCell::new(Some(vec![])))));
    show(Rc::new(RefCell::new(Some("%5".to_string()))), sprintf(Rc::new(RefCell::new(Some("%5".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(6) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%-*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%-*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(6) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.*f".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.*f".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(2) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*.*f|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*.*f|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(8) as Box<dyn Any>, Box::new(3) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("x".to_string()) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%w".to_string()))), sprintf(Rc::new(RefCell::new(Some("%w".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("wrapped".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%v %v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%v %v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%-5d|%5s|%05.1f".to_string()))), sprintf(Rc::new(RefCell::new(Some("%-5d|%5s|%05.1f".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(3) as Box<dyn Any>, Box::new("ab".to_string()) as Box<dyn Any>, Box::new(2.25) as Box<dyn Any>])))));

        // A negative width from an argument means left alignment. A negative
        // precision is reported and then ignored.
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(-(6)) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*s|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*s|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(-(6)) as Box<dyn Any>, Box::new("ab".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%-*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%-*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(-(6)) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.*f|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.*f|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(-(2)) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>])))));

        // Precision counts bytes for %x and characters for %s.
    show(Rc::new(RefCell::new(Some("%.1x".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.1x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{e9}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.3x".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.3x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("h\u{e9}llo".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.2X".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.2X".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("h\u{e9}llo".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.3s".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.3s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("h\u{e9}llo".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("% .2x".to_string()))), sprintf(Rc::new(RefCell::new(Some("% .2x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("h\u{e9}llo".to_string()) as Box<dyn Any>])))));

        // %#v writes a value the way Go source does. %+v changes nothing for
        // a number or a string.
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("hello".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("new\nline".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(-(5) as i64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(200 as u8))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(2.5) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1000000.0) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(true) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#10v|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#10v|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("hi".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("hello".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(42) as Box<dyn Any>])))));

        // Zero padding and a precision count digits, and 0x, 0b or 0o comes on
        // top of them. The 0 that # puts before an octal number is not added to
        // a number that already starts with one. A precision of zero prints
        // nothing for zero.
    let mut prefixFormats = Rc::new(RefCell::new(Some(vec!["%#08x".to_string(), "%#08X".to_string(), "%#08b".to_string(), "%#08o".to_string(), "%08O".to_string(), "%#8x|".to_string(), "%#-8x|".to_string(), "%#-08x|".to_string(), "%#.3o".to_string(), "%#.1o".to_string(), "%#.3x".to_string(), "%#8.3x|".to_string(), "%+08d".to_string(), "% 08d".to_string(), "%+.0d|".to_string(), "%#.0x|".to_string(), "%5.0d|".to_string(), "%#o".to_string(), "%#b".to_string(), "%O".to_string(), "%#O".to_string(), "%#.3O".to_string(), "%#08O".to_string()])));
    let mut prefixValues = Rc::new(RefCell::new(Some(vec![0, 7, 8, -42, 255])));
    { let __range_holder = prefixFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        { let __range_holder = prefixValues.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for value in __range_values.iter().copied() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(value) as Box<dyn Any>])))));
    } }
    } }

        // A width or a precision from an argument can be any integer type.
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(6 as u32))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(-(6) as i8))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(6 as i64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(6 as u64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(6 as i16))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.*f".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.*f".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(2 as u8))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.*f".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.*f".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new({ let __v = Rc::new(RefCell::new(Some(3 as u64))); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>, Box::new(3.14159) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%*d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%*d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(2.5) as Box<dyn Any>, Box::new(42) as Box<dyn Any>])))));

        // %+q writes characters outside ASCII as escapes.
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{e9}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("a\tb\u{263a}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{1f600}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("plain".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\u{e9}' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('x' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\u{1f600}' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{e9}\u{263a}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\'' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\\' as i32)) as Box<dyn Any>])))));

        // Zero padding applies to a character, a quoted character, a bool and
        // the name of a type as it does to a string.
    show(Rc::new(RefCell::new(Some("%05c|%05q|%-05c|%05U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%05c|%05q|%-05c|%05U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%05c|%05q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%05c|%05q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\u{e9}' as i32)) as Box<dyn Any>, Box::new(('\u{e9}' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%05T|%05t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%05T|%05t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(65) as Box<dyn Any>, Box::new(true) as Box<dyn Any>])))));

        // %q writes a character that cannot be seen as an escape.
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("zero\u{200b}width".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("soft\u{ad}hyphen".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{feff}mark".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("no\u{a0}break".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\u{200b}' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("zero\u{200b}width".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(('\u{200b}' as i32)) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new((' ' as i32)) as Box<dyn Any>])))));

        // %q writes a byte that is not UTF-8 as an escape too.
    let mut broken = Rc::new(RefCell::new(Some(vec![0xff as u8, 0x41 as u8, 0xc3 as u8, 0xa9 as u8])));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%x".to_string()))), sprintf(Rc::new(RefCell::new(Some("%x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.1q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.1q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.3q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.3q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%12q|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%12q|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(broken.clone()) as Box<dyn Any>])))));

        // A precision is applied before %#q chooses its quotes, so a byte that
        // is cut off has no say in them.
    show(Rc::new(RefCell::new(Some("%#.1q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#.1q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![0x41 as u8, 0xff as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#.1q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#.1q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![0x41 as u8, 0x42 as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#.2q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#.2q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![0x41 as u8, 0xff as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.2s|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.2s|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![0x41 as u8, 0x42 as u8, 0xff as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.3q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.3q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![0xe2 as u8, 0x82 as u8, 0x41 as u8, 0x42 as u8]))).clone()) as Box<dyn Any>])))));

        // %#q writes a raw string unless the text has a backquote, an ASCII
        // control character other than tab, or a byte order mark.
    show(Rc::new(RefCell::new(Some("%#q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("\u{feff}mark".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("next\u{85}line".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("del\u{7f}".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%#q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%#q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("zero\u{200b}width".to_string()) as Box<dyn Any>])))));

        // A precision above four gives %U more digits.
    show(Rc::new(RefCell::new(Some("%.8U|%.2U|%#.6U|%10.6U|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.8U|%.2U|%#.6U|%10.6U|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>, Box::new(65) as Box<dyn Any>])))));

        // Infinity keeps its sign and is never padded with zeros. NaN has no
        // sign unless a flag asks for one.
    let mut zero: Rc<RefCell<Option<f64>>> = Rc::new(RefCell::new(Some(0.0)));
    let mut edges = Rc::new(RefCell::new(Some(vec![1.0 / (*zero.borrow().as_ref().unwrap()), -1.0 / (*zero.borrow().as_ref().unwrap()), { let __bin_zero = (*zero.borrow().as_ref().unwrap()).clone(); __bin_zero / __bin_zero }])));
    let mut edgeFormats = Rc::new(RefCell::new(Some(vec!["%f".to_string(), "%+f".to_string(), "% f".to_string(), "%+ f".to_string(), "%8.2f|".to_string(), "%-8.2f|".to_string(), "%08.2f|".to_string(), "% 08.2f|".to_string(), "%v".to_string(), "%+v".to_string(), "%g".to_string(), "%e".to_string(), "%E".to_string(), "%G".to_string(), "%.0f".to_string()])));
    { let __range_holder = edgeFormats.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for format in __range_values.iter() {
        { let __range_holder = edges.clone(); let __range_guard = __range_holder.borrow(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for value in __range_values.iter().copied() {
        show(Rc::new(RefCell::new(Some((*format).clone()))), sprintf(Rc::new(RefCell::new(Some((*format).clone()))), Rc::new(RefCell::new(Some(vec![Box::new(value) as Box<dyn Any>])))));
    } }
    } }

        // A []byte prints as text with %s, %q and %x, and as numbers with %v.
    let mut data = Rc::new(RefCell::new(Some(("Hello".to_string()).as_bytes().to_vec())));
    show(Rc::new(RefCell::new(Some("%s".to_string()))), sprintf(Rc::new(RefCell::new(Some("%s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%q".to_string()))), sprintf(Rc::new(RefCell::new(Some("%q".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%x".to_string()))), sprintf(Rc::new(RefCell::new(Some("%x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%X".to_string()))), sprintf(Rc::new(RefCell::new(Some("%X".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("% x".to_string()))), sprintf(Rc::new(RefCell::new(Some("% x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%s".to_string()))), sprintf(Rc::new(RefCell::new(Some("%s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(("direct".to_string()).as_bytes().to_vec()))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%10s|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%10s|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.2s".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.2s".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));

        // A verb that does not print a []byte as text is applied to each byte.
    let mut pair = Rc::new(RefCell::new(Some(vec![1 as u8, 2 as u8])));
    show(Rc::new(RefCell::new(Some("%d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%c".to_string()))), sprintf(Rc::new(RefCell::new(Some("%c".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(data.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%5v".to_string()))), sprintf(Rc::new(RefCell::new(Some("%5v".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(pair.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%03d|%-4d|%+d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%03d|%-4d|%+d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(pair.clone()) as Box<dyn Any>, Box::new(pair.clone()) as Box<dyn Any>, Box::new(pair.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%o %b %U".to_string()))), sprintf(Rc::new(RefCell::new(Some("%o %b %U".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![8 as u8]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![5 as u8]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![65 as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(pair.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%w".to_string()))), sprintf(Rc::new(RefCell::new(Some("%w".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(pair.clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%v|%d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%v|%d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(Vec::<u8>::new()))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(Vec::<u8>::new()))).clone()) as Box<dyn Any>])))));

        // When a verb does not fit, fmt writes the value as %v would, with the
        // width, the precision and the flags of the verb.
    show(Rc::new(RefCell::new(Some("%05t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%05t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1 as u8, 42 as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.3t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.3t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1 as u8, 42 as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+t|%-4t|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+t|%-4t|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![1 as u8, 42 as u8]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1 as u8, 42 as u8]))).clone()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%05t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%05t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(42) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%+t".to_string()))), sprintf(Rc::new(RefCell::new(Some("%+t".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(5) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%5s|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%5s|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(5) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%-5d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%-5d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("x".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%.1d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%.1d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new("abc".to_string()) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%8.2d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%8.2d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(2.5) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%6d|".to_string()))), sprintf(Rc::new(RefCell::new(Some("%6d|".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(true) as Box<dyn Any>])))));
    show(Rc::new(RefCell::new(Some("%5d".to_string()))), sprintf(Rc::new(RefCell::new(Some("%5d".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1) as Box<dyn Any>, Box::new(2) as Box<dyn Any>])))));
}