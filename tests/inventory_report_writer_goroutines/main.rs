#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
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
                    format!("{}:{}", go_display_value(k), __nil_text)
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
    } else if let Some(v) = value.downcast_ref::<Inventory>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Inventory>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

thread_local! {
    static __GO_RECOVER_PAYLOAD: RefCell<Option<Box<dyn Any + Send + Sync>>> = RefCell::new(None);
}

fn go_recover() -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    __GO_RECOVER_PAYLOAD.with(|slot| Arc::new(Mutex::new(slot.borrow_mut().take())))
}

fn go_store_panic_payload(payload: Box<dyn Any + Send>) {
    let payload = match payload.downcast::<Box<dyn Any + Send + Sync>>() {
        Ok(boxed) => {
            let mut payload = *boxed;
            loop {
                match payload.downcast::<Box<dyn Any + Send + Sync>>() {
                    Ok(boxed) => {
                        payload = *boxed;
                    }
                    Err(payload) => {
                        __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(payload));
                        return;
                    }
                }
            }
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<String>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text_of_string(*value)) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<&'static str>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(value.to_string())) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i32>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let payload = match payload.downcast::<i64>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(payload) => payload,
    };
    let _payload = match payload.downcast::<bool>() {
        Ok(value) => {
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(*value) as Box<dyn Any + Send + Sync>));
            return;
        }
        Err(_payload) => _payload,
    };
    panic!("recover: unsupported Rust panic payload; emit panic_any with a Go any payload instead")
}

thread_local! {
    static __GO_PANIC_HOOK_SILENCE: std::cell::Cell<usize> = std::cell::Cell::new(0);
}

fn go_silence_panic_hook() -> usize {
    static __GO_PANIC_HOOK: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_HOOK.call_once(|| {
        let __previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |__info| {
            if __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.get()) == 0 {
                __previous(__info);
            }
        }));
    });
    __GO_PANIC_HOOK_SILENCE.with(|__depth| {
        let __previous = __depth.get();
        __depth.set(__previous + 1);
        __previous
    })
}

fn go_restore_panic_hook(__previous: usize) {
    __GO_PANIC_HOOK_SILENCE.with(|__depth| __depth.set(__previous));
}

fn go_resume_unrecovered_panic() {
    if let Some(payload) = __GO_RECOVER_PAYLOAD.with(|slot| slot.borrow_mut().take()) {
        std::panic::panic_any(payload);
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
    if val.is::<Arc<Mutex<Option<Vec<String>>>>>() { return "[]string" }
    if val.is::<Inventory>() { return "main.Inventory" }
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

// A nil interface value: the nil literal, or a nil interface value boxed as
// the program boxes one.
fn go_format_is_nil(value: &(dyn Any + Send + Sync)) -> bool {
    value.is::<GoFormatNil>()
}

// An argument as fmt names it when it does not fit the format: type=value.
// The value is written as %v writes it, with the width, the precision and
// the flags of the verb that did not fit.
fn go_format_typed_value(spec: &GoFormatSpec, value: &(dyn Any + Send + Sync)) -> String {
    if go_format_is_nil(value) {
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
        if go_format_is_nil(arg.as_ref()) {
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

#[derive(Debug, Clone, PartialEq)]
enum GoScanError {
    Eof,
    UnexpectedEof,
    Msg(String),
}

// Where the scanner reads runes from: fmt's io.RuneScanner. Ok(None) is
// the end of the input; Err is a read error, which the scan returns.
trait GoScanRunes {
    fn go_read_rune(&mut self) -> Result<Option<char>, String>;
    fn go_unread_rune(&mut self);
}

// fmt's readRune: UTF-8 from a byte source that cannot unread, with one
// rune of lookahead that is lost when the scan ends.
struct GoScanReadRune<F: FnMut() -> Result<Option<u8>, String>> {
    next_byte: F,
    pending: std::vec::Vec<u8>,
    peek: Option<char>,
    last: Option<char>,
}

impl<F: FnMut() -> Result<Option<u8>, String>> GoScanReadRune<F> {
    fn new(next_byte: F) -> Self {
        GoScanReadRune { next_byte, pending: std::vec::Vec::new(), peek: None, last: None }
    }

    fn read_byte(&mut self) -> Result<Option<u8>, String> {
        if !self.pending.is_empty() {
            return Ok(Some(self.pending.remove(0)));
        }
        (self.next_byte)()
    }
}

// utf8.FullRune of a prefix: whether more bytes can still make it one rune.
fn go_scan_wants_more(bytes: &[u8]) -> bool {
    let first = bytes[0];
    let want = match first {
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => 1,
    };
    if bytes.len() >= want {
        return false;
    }
    // A byte that cannot continue the rune ends it now.
    let (lo, hi) = match (first, bytes.len()) {
        (0xE0, 1) => (0xA0, 0xBF),
        (0xED, 1) => (0x80, 0x9F),
        (0xF0, 1) => (0x90, 0xBF),
        (0xF4, 1) => (0x80, 0x8F),
        _ => (0x80, 0xBF),
    };
    bytes[1..].iter().enumerate().all(|(k, &b)| if k == 0 { b >= lo && b <= hi } else { (0x80..=0xBF).contains(&b) })
}

impl<F: FnMut() -> Result<Option<u8>, String>> GoScanRunes for GoScanReadRune<F> {
    fn go_read_rune(&mut self) -> Result<Option<char>, String> {
        if let Some(r) = self.peek.take() {
            self.last = Some(r);
            return Ok(Some(r));
        }
        let first = match self.read_byte()? {
            None => return Ok(None),
            Some(b) => b,
        };
        if first < 0x80 {
            let r = first as char;
            self.last = Some(r);
            return Ok(Some(r));
        }
        // As readRune: read until the bytes are a full rune, or cannot be.
        let mut bytes: std::vec::Vec<u8> = std::vec![first];
        while go_scan_wants_more(&bytes) {
            match self.read_byte()? {
                Some(b) => bytes.push(b),
                None => break,
            }
        }
        let r = match std::str::from_utf8(&bytes) {
            Ok(text) => text.chars().next().unwrap_or('\u{FFFD}'),
            Err(_) => {
                // Bad UTF-8 is one U+FFFD for its first byte; the bytes
                // after it are read again, before any pending ones.
                let mut rest = bytes[1..].to_vec();
                rest.extend_from_slice(&self.pending);
                self.pending = rest;
                '\u{FFFD}'
            }
        };
        self.last = Some(r);
        Ok(Some(r))
    }

    fn go_unread_rune(&mut self) {
        if let Some(r) = self.last.take() {
            self.peek = Some(r);
        }
    }
}

// An io.RuneScanner of the program, as *bufio.Reader or *strings.Reader:
// its own ReadRune and UnreadRune.
struct GoScanRuneFns<'a> {
    read: Box<dyn FnMut() -> Result<Option<char>, String> + 'a>,
    unread: Box<dyn FnMut() + 'a>,
}

impl<'a> GoScanRunes for GoScanRuneFns<'a> {
    fn go_read_rune(&mut self) -> Result<Option<char>, String> {
        (self.read)()
    }

    fn go_unread_rune(&mut self) {
        (self.unread)()
    }
}

fn go_scan_string_source(text: String) -> GoScanReadRune<impl FnMut() -> Result<Option<u8>, String>> {
    let bytes = text.into_bytes();
    let mut at = 0usize;
    GoScanReadRune::new(move || {
        let b = bytes.get(at).copied();
        if b.is_some() {
            at += 1;
        }
        Ok(b)
    })
}

// The process's standard input, which os.Stdin reads too.
fn go_scan_stdin_source() -> GoScanReadRune<impl FnMut() -> Result<Option<u8>, String>> {
    GoScanReadRune::new(|| {
        use std::io::Read;
        let mut byte = [0u8; 1];
        loop {
            match std::io::stdin().lock().read(&mut byte) {
                Ok(0) => return Ok(None),
                Ok(_) => return Ok(Some(byte[0])),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(e) => return Err(format!("read /dev/stdin: {}", e)),
            }
        }
    })
}

// An operand: where the scanned value goes.
enum GoScanTarget<'a> {
    Int(u32, Box<dyn FnMut(i64) + 'a>),
    Uint(u32, Box<dyn FnMut(u64) + 'a>),
    Float(u32, Box<dyn FnMut(f64) + 'a>),
    Str(Box<dyn FnMut(String) + 'a>),
    Bytes(Box<dyn FnMut(std::vec::Vec<u8>) + 'a>),
    Bool(Box<dyn FnMut(bool) + 'a>),
}

const GO_SCAN_HUGE_WID: usize = 1 << 30;

struct GoScanState<'a> {
    rs: &'a mut dyn GoScanRunes,
    buf: String,
    count: usize,
    at_eof: bool,
    nl_is_end: bool,
    nl_is_space: bool,
    arg_limit: usize,
    limit: usize,
    max_wid: usize,
    // A read error ends the input; the scan returns it instead of the
    // operand it was reading, as fmt's getRune does.
    read_err: Option<String>,
}

fn go_scan_is_space(r: char) -> bool {
    let r = r as u32;
    if r >= 1 << 16 {
        return false;
    }
    const SPACE: [(u32, u32); 10] = [
        (0x0009, 0x000d),
        (0x0020, 0x0020),
        (0x0085, 0x0085),
        (0x00a0, 0x00a0),
        (0x1680, 0x1680),
        (0x2000, 0x200a),
        (0x2028, 0x2029),
        (0x202f, 0x202f),
        (0x205f, 0x205f),
        (0x3000, 0x3000),
    ];
    for (lo, hi) in SPACE {
        if r < lo {
            return false;
        }
        if r <= hi {
            return true;
        }
    }
    false
}

fn go_scan_quote(tok: &str) -> String {
    let mut out = String::from("\"");
    for c in tok.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            _ => out.push(c),
        }
    }
    out.push('"');
    out
}

// strconv's underscoreOK: underscores only between digits, or between a
// base prefix and a digit.
fn go_scan_underscore_ok(s: &str) -> bool {
    let b = s.as_bytes();
    let mut saw = b'^';
    let mut i = 0;
    let b = if !b.is_empty() && (b[0] == b'-' || b[0] == b'+') { &b[1..] } else { b };
    let mut hex = false;
    if b.len() >= 2 && b[0] == b'0' && matches!(b[1] | 0x20, b'b' | b'o' | b'x') {
        i = 2;
        saw = b'0';
        hex = (b[1] | 0x20) == b'x';
    }
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_digit() || (hex && (c | 0x20) >= b'a' && (c | 0x20) <= b'f') {
            saw = b'0';
        } else if c == b'_' {
            if saw != b'0' {
                return false;
            }
            saw = b'_';
        } else {
            if saw == b'_' {
                return false;
            }
            saw = b'!';
        }
        i += 1;
    }
    saw != b'_'
}

// strconv.ParseUint's digits: base 0 reads a prefix and allows
// underscores. Ok(None) is a value out of range.
fn go_scan_parse_digits(s: &str, base: u32) -> Result<Option<u64>, ()> {
    if s.is_empty() {
        return Err(());
    }
    let mut base = base;
    let mut digits = s;
    let base0 = base == 0;
    if base0 {
        base = 10;
        let b = s.as_bytes();
        if b[0] == b'0' {
            if b.len() >= 3 && (b[1] | 0x20) == b'b' {
                base = 2;
                digits = &s[2..];
            } else if b.len() >= 3 && (b[1] | 0x20) == b'o' {
                base = 8;
                digits = &s[2..];
            } else if b.len() >= 3 && (b[1] | 0x20) == b'x' {
                base = 16;
                digits = &s[2..];
            } else {
                base = 8;
                digits = &s[1..];
            }
        }
    }
    let mut value: u64 = 0;
    let mut underscores = false;
    let mut any = false;
    for c in digits.chars() {
        if c == '_' && base0 {
            underscores = true;
            continue;
        }
        let d = match c.to_digit(36) {
            Some(d) if d < base => d as u64,
            _ => return Err(()),
        };
        any = true;
        match value.checked_mul(base as u64).and_then(|v| v.checked_add(d)) {
            Some(v) => value = v,
            // strconv reports the range at the digit that overflows.
            None => return Ok(None),
        }
    }
    if !any && !(base0 && base == 8 && digits.is_empty()) {
        return Err(());
    }
    if underscores && !go_scan_underscore_ok(s) {
        return Err(());
    }
    Ok(Some(value))
}

impl<'a> GoScanState<'a> {
    fn new(rs: &'a mut dyn GoScanRunes, nl_is_space: bool, nl_is_end: bool) -> Self {
        GoScanState {
            rs,
            buf: String::new(),
            count: 0,
            at_eof: false,
            nl_is_end,
            nl_is_space,
            arg_limit: GO_SCAN_HUGE_WID,
            limit: GO_SCAN_HUGE_WID,
            max_wid: GO_SCAN_HUGE_WID,
            read_err: None,
        }
    }

    fn check_read(&mut self) -> Result<(), GoScanError> {
        match self.read_err.take() {
            Some(e) => Err(GoScanError::Msg(e)),
            None => Ok(()),
        }
    }

    fn read_rune(&mut self) -> Option<char> {
        if self.at_eof || self.count >= self.arg_limit {
            return None;
        }
        match self.rs.go_read_rune().unwrap_or_else(|e| {
            self.read_err = Some(e);
            None
        }) {
            Some(r) => {
                self.count += 1;
                if self.nl_is_end && r == '\n' {
                    self.at_eof = true;
                }
                Some(r)
            }
            None => {
                self.at_eof = true;
                None
            }
        }
    }

    fn get_rune(&mut self) -> Option<char> {
        self.read_rune()
    }

    fn must_read_rune(&mut self) -> Result<char, GoScanError> {
        self.get_rune().ok_or(GoScanError::UnexpectedEof)
    }

    fn unread_rune(&mut self) {
        self.rs.go_unread_rune();
        self.at_eof = false;
        self.count -= 1;
    }

    fn error_string(&self, text: &str) -> GoScanError {
        GoScanError::Msg(text.to_string())
    }

    fn skip_space(&mut self) -> Result<(), GoScanError> {
        loop {
            let r = match self.get_rune() {
                None => return Ok(()),
                Some(r) => r,
            };
            if r == '\r' && self.peek("\n") {
                continue;
            }
            if r == '\n' {
                if self.nl_is_space {
                    continue;
                }
                return Err(self.error_string("unexpected newline"));
            }
            if !go_scan_is_space(r) {
                self.unread_rune();
                return Ok(());
            }
        }
    }

    fn token(&mut self, skip_space: bool) -> Result<String, GoScanError> {
        if skip_space {
            self.skip_space()?;
        }
        loop {
            let r = match self.get_rune() {
                None => break,
                Some(r) => r,
            };
            if go_scan_is_space(r) {
                self.unread_rune();
                break;
            }
            self.buf.push(r);
        }
        Ok(self.buf.clone())
    }

    fn consume(&mut self, ok: &str, accept: bool) -> bool {
        let r = match self.get_rune() {
            None => return false,
            Some(r) => r,
        };
        if ok.contains(r) {
            if accept {
                self.buf.push(r);
            }
            return true;
        }
        if accept {
            self.unread_rune();
        }
        false
    }

    fn peek(&mut self, ok: &str) -> bool {
        match self.get_rune() {
            None => false,
            Some(r) => {
                self.unread_rune();
                ok.contains(r)
            }
        }
    }

    fn not_eof(&mut self) -> Result<(), GoScanError> {
        match self.get_rune() {
            None => Err(GoScanError::Eof),
            Some(_) => {
                self.unread_rune();
                Ok(())
            }
        }
    }

    fn accept(&mut self, ok: &str) -> bool {
        self.consume(ok, true)
    }

    fn ok_verb(&self, verb: char, ok_verbs: &str, typ: &str) -> Result<(), GoScanError> {
        if ok_verbs.contains(verb) {
            Ok(())
        } else {
            Err(GoScanError::Msg(format!("bad verb '%{}' for {}", verb, typ)))
        }
    }

    fn scan_bool(&mut self, verb: char) -> Result<bool, GoScanError> {
        self.skip_space()?;
        self.not_eof()?;
        self.ok_verb(verb, "tv", "boolean")?;
        let bool_error = || GoScanError::Msg("syntax error scanning boolean".to_string());
        match self.get_rune() {
            Some('0') => Ok(false),
            Some('1') => Ok(true),
            Some('t') | Some('T') => {
                if self.accept("rR") && (!self.accept("uU") || !self.accept("eE")) {
                    return Err(bool_error());
                }
                Ok(true)
            }
            Some('f') | Some('F') => {
                if self.accept("aA") && (!self.accept("lL") || !self.accept("sS") || !self.accept("eE")) {
                    return Err(bool_error());
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn get_base(&self, verb: char) -> Result<(u32, &'static str), GoScanError> {
        self.ok_verb(verb, "bdoUxXv", "integer")?;
        Ok(match verb {
            'b' => (2, "01"),
            'o' => (8, "01234567"),
            'x' | 'X' | 'U' => (16, "0123456789aAbBcCdDeEfF"),
            _ => (10, "0123456789"),
        })
    }

    fn scan_number(&mut self, digits: &str, have_digits: bool) -> Result<String, GoScanError> {
        if !have_digits {
            self.not_eof()?;
            if !self.accept(digits) {
                return Err(self.error_string("expected integer"));
            }
        }
        while self.accept(digits) {}
        Ok(self.buf.clone())
    }

    fn scan_rune(&mut self, bit_size: u32) -> Result<i64, GoScanError> {
        self.not_eof()?;
        let r = self.get_rune().map(|r| r as i64).unwrap_or(-1);
        let shift = 64 - bit_size;
        if (r << shift) >> shift != r {
            let text = char::from_u32(r as u32).map(|c| c.to_string()).unwrap_or_default();
            return Err(GoScanError::Msg(format!("overflow on character value {}", text)));
        }
        Ok(r)
    }

    fn scan_base_prefix(&mut self) -> (u32, &'static str, bool) {
        if !self.peek("0") {
            return (0, "0123456789_", false);
        }
        self.accept("0");
        if self.peek("bB") {
            self.consume("bB", true);
            (0, "01_", true)
        } else if self.peek("oO") {
            self.consume("oO", true);
            (0, "01234567_", true)
        } else if self.peek("xX") {
            self.consume("xX", true);
            (0, "0123456789aAbBcCdDeEfF_", true)
        } else {
            (0, "01234567_", true)
        }
    }

    fn scan_int(&mut self, verb: char, bit_size: u32) -> Result<i64, GoScanError> {
        if verb == 'c' {
            return self.scan_rune(bit_size);
        }
        self.skip_space()?;
        self.not_eof()?;
        let (mut base, mut digits) = self.get_base(verb)?;
        let mut have_digits = false;
        if verb == 'U' {
            if !self.consume("U", false) || !self.consume("+", false) {
                return Err(self.error_string("bad unicode format "));
            }
        } else {
            self.accept("+-");
            if verb == 'v' {
                let (b, d, h) = self.scan_base_prefix();
                base = b;
                digits = d;
                have_digits = h;
            }
        }
        let tok = self.scan_number(digits, have_digits)?;
        let (negative, unsigned) = match tok.as_bytes().first() {
            Some(b'-') => (true, &tok[1..]),
            Some(b'+') => (false, &tok[1..]),
            _ => (false, &tok[..]),
        };
        let syntax = || GoScanError::Msg(format!("strconv.ParseInt: parsing {}: invalid syntax", go_scan_quote(&tok)));
        let range = || GoScanError::Msg(format!("strconv.ParseInt: parsing {}: value out of range", go_scan_quote(&tok)));
        let magnitude = match go_scan_parse_digits(unsigned, base) {
            Err(()) => return Err(syntax()),
            Ok(None) => return Err(range()),
            Ok(Some(m)) => m,
        };
        let value = if negative {
            if magnitude > 1u64 << 63 {
                return Err(range());
            }
            (magnitude as i64).wrapping_neg()
        } else {
            if magnitude > i64::MAX as u64 {
                return Err(range());
            }
            magnitude as i64
        };
        let shift = 64 - bit_size;
        if (value << shift) >> shift != value {
            return Err(GoScanError::Msg(format!("integer overflow on token {}", tok)));
        }
        Ok(value)
    }

    fn scan_uint(&mut self, verb: char, bit_size: u32) -> Result<u64, GoScanError> {
        if verb == 'c' {
            return self.scan_rune(bit_size).map(|r| r as u64);
        }
        self.skip_space()?;
        self.not_eof()?;
        let (mut base, mut digits) = self.get_base(verb)?;
        let mut have_digits = false;
        if verb == 'U' {
            if !self.consume("U", false) || !self.consume("+", false) {
                return Err(self.error_string("bad unicode format "));
            }
        } else if verb == 'v' {
            let (b, d, h) = self.scan_base_prefix();
            base = b;
            digits = d;
            have_digits = h;
        }
        let tok = self.scan_number(digits, have_digits)?;
        let value = match go_scan_parse_digits(&tok, base) {
            Err(()) => return Err(GoScanError::Msg(format!("strconv.ParseUint: parsing {}: invalid syntax", go_scan_quote(&tok)))),
            Ok(None) => return Err(GoScanError::Msg(format!("strconv.ParseUint: parsing {}: value out of range", go_scan_quote(&tok)))),
            Ok(Some(v)) => v,
        };
        let shift = 64 - bit_size;
        if (value << shift) >> shift != value {
            return Err(GoScanError::Msg(format!("unsigned integer overflow on token {}", tok)));
        }
        Ok(value)
    }

    fn float_token(&mut self) -> String {
        self.buf.clear();
        if self.accept("nN") && self.accept("aA") && self.accept("nN") {
            return self.buf.clone();
        }
        self.accept("+-");
        if self.accept("iI") && self.accept("nN") && self.accept("fF") {
            return self.buf.clone();
        }
        let mut digits = "0123456789_";
        let mut exp = "eEpP";
        if self.accept("0") && self.accept("xX") {
            digits = "0123456789aAbBcCdDeEfF_";
            exp = "pP";
        }
        while self.accept(digits) {}
        if self.accept(".") {
            while self.accept(digits) {}
        }
        if self.accept(exp) {
            self.accept("+-");
            while self.accept("0123456789_") {}
        }
        self.buf.clone()
    }

    // strconv.ParseFloat for the decimal forms floatToken reads.
    fn parse_float(text: &str, bit_size: u32) -> Result<f64, GoScanError> {
        let syntax = || GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: invalid syntax", go_scan_quote(text)));
        let lower = text.to_ascii_lowercase();
        let body = lower.trim_start_matches(|c| c == '+' || c == '-');
        if body == "inf" || body == "infinity" || body == "nan" {
            let value: f64 = lower.parse().map_err(|_| syntax())?;
            return Ok(value);
        }
        if body.starts_with("0x") {
            return go_scan_parse_hex_float(&lower, bit_size).ok_or_else(syntax).and_then(|v| {
                if v.is_infinite() {
                    Err(GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: value out of range", go_scan_quote(text))))
                } else {
                    Ok(v)
                }
            });
        }
        if !body.chars().any(|c| c.is_ascii_digit()) {
            return Err(syntax());
        }
        let plain: String;
        let text_ref = if text.contains('_') {
            if !go_scan_underscore_ok(text) {
                return Err(syntax());
            }
            plain = text.replace('_', "");
            &plain
        } else {
            text
        };
        if text_ref.ends_with(|c: char| c == 'e' || c == 'E' || c == '+' || c == '-') {
            return Err(syntax());
        }
        let value = if bit_size == 32 {
            text_ref.parse::<f32>().map(|v| v as f64).map_err(|_| syntax())?
        } else {
            text_ref.parse::<f64>().map_err(|_| syntax())?
        };
        if value.is_infinite() {
            return Err(GoScanError::Msg(format!("strconv.ParseFloat: parsing {}: value out of range", go_scan_quote(text))));
        }
        Ok(value)
    }

    fn convert_float(&self, text: &str, bit_size: u32) -> Result<f64, GoScanError> {
        if let Some(p) = text.find('p') {
            if !text.contains(|c| c == 'x' || c == 'X') {
                // fmt's own decimal mantissa with a binary exponent, 1.5p3:
                // math.Ldexp of the two, with the exponent an int.
                let mantissa = Self::parse_float(&text[..p], bit_size).map_err(|e| match e {
                    GoScanError::Msg(m) => GoScanError::Msg(m.replacen(&go_scan_quote(&text[..p]), &go_scan_quote(text), 1)),
                    other => other,
                })?;
                let exp_text = &text[p + 1..];
                let exp: i64 = match exp_text.parse::<i64>() {
                    Ok(e) => e,
                    Err(_) if !exp_text.is_empty() && exp_text.trim_start_matches(|c| c == '+' || c == '-').chars().all(|c| c.is_ascii_digit()) && !exp_text.trim_start_matches(|c| c == '+' || c == '-').is_empty() => {
                        return Err(GoScanError::Msg(format!("strconv.Atoi: parsing {}: value out of range", go_scan_quote(text))));
                    }
                    Err(_) => return Err(GoScanError::Msg(format!("strconv.Atoi: parsing {}: invalid syntax", go_scan_quote(text)))),
                };
                return Ok(go_scan_ldexp(mantissa, exp));
            }
        }
        Self::parse_float(text, bit_size)
    }

    fn scan_float(&mut self, verb: char, bit_size: u32) -> Result<f64, GoScanError> {
        let typ = if bit_size == 32 { "float32" } else { "float64" };
        self.ok_verb(verb, "beEfFgGv", typ)?;
        self.skip_space()?;
        self.not_eof()?;
        let tok = self.float_token();
        self.convert_float(&tok, bit_size)
    }

    fn quoted_string(&mut self) -> Result<std::vec::Vec<u8>, GoScanError> {
        self.not_eof()?;
        let quote = self.get_rune();
        match quote {
            Some('`') => {
                loop {
                    let r = self.must_read_rune()?;
                    if r == '`' {
                        break;
                    }
                    self.buf.push(r);
                }
                Ok(self.buf.clone().into_bytes())
            }
            Some('"') => {
                self.buf.push('"');
                loop {
                    let r = self.must_read_rune()?;
                    self.buf.push(r);
                    if r == '\\' {
                        let next = self.must_read_rune()?;
                        self.buf.push(next);
                    } else if r == '"' {
                        break;
                    }
                }
                go_scan_unquote(&self.buf).ok_or_else(|| GoScanError::Msg("invalid syntax".to_string()))
            }
            _ => Err(self.error_string("expected quoted string")),
        }
    }

    fn hex_byte(&mut self) -> Result<Option<u8>, GoScanError> {
        let first = match self.get_rune() {
            None => return Ok(None),
            Some(r) => r,
        };
        let high = match first.to_digit(16) {
            Some(d) => d,
            None => {
                self.unread_rune();
                return Ok(None);
            }
        };
        let second = self.must_read_rune()?;
        match second.to_digit(16) {
            Some(low) => Ok(Some((high << 4 | low) as u8)),
            None => Err(self.error_string("illegal hex digit")),
        }
    }

    fn hex_bytes(&mut self) -> Result<std::vec::Vec<u8>, GoScanError> {
        self.not_eof()?;
        let mut bytes: std::vec::Vec<u8> = std::vec::Vec::new();
        while let Some(b) = self.hex_byte()? {
            bytes.push(b);
        }
        if bytes.is_empty() {
            return Err(self.error_string("no hex data for %x string"));
        }
        Ok(bytes)
    }

    fn convert_bytes(&mut self, verb: char) -> Result<std::vec::Vec<u8>, GoScanError> {
        self.ok_verb(verb, "svqxX", "string")?;
        self.skip_space()?;
        self.not_eof()?;
        match verb {
            'q' => self.quoted_string(),
            'x' | 'X' => self.hex_bytes(),
            _ => self.token(true).map(|s| s.into_bytes()),
        }
    }

    fn scan_percent(&mut self) -> Result<(), GoScanError> {
        self.skip_space()?;
        self.not_eof()?;
        if !self.accept("%") {
            return Err(self.error_string("missing literal %"));
        }
        Ok(())
    }

    fn scan_one(&mut self, verb: char, target: &mut GoScanTarget) -> Result<(), GoScanError> {
        self.buf.clear();
        // Each value is stored only if no read failed while it was read.
        match target {
            GoScanTarget::Bool(set) => {
                let v = self.scan_bool(verb);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Int(bits, set) => {
                let v = self.scan_int(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Uint(bits, set) => {
                let v = self.scan_uint(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Float(bits, set) => {
                let v = self.scan_float(verb, *bits);
                self.check_read()?;
                set(v?);
            }
            GoScanTarget::Str(set) => {
                let v = self.convert_bytes(verb);
                self.check_read()?;
                match String::from_utf8(v?) {
                    Ok(text) => set(text),
                    // A Go string holds any bytes, a Rust String only UTF-8.
                    Err(_) => panic!("fmt scan: a string that is not UTF-8, as %x or %q can give, is not supported; see docs/bridge_debt.md fmt-scan"),
                }
            }
            GoScanTarget::Bytes(set) => {
                let v = self.convert_bytes(verb);
                self.check_read()?;
                set(v?);
            }
        }
        Ok(())
    }

    fn do_scan(&mut self, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
        let mut n = 0i64;
        for target in targets.iter_mut() {
            if let Err(e) = self.scan_one('v', target) {
                return (n, Some(e));
            }
            n += 1;
        }
        if self.nl_is_end {
            loop {
                match self.get_rune() {
                    None | Some('\n') => break,
                    Some(r) if !go_scan_is_space(r) => return (n, Some(self.error_string("expected newline"))),
                    _ => {}
                }
            }
        }
        if let Err(e) = self.check_read() {
            return (n, Some(e));
        }
        (n, None)
    }

    // advance matches the format's literal text and spaces against the
    // input, and returns how many bytes of the format it used, or -1 when
    // the input does not match.
    fn advance(&mut self, format: &str) -> Result<i64, GoScanError> {
        let mut i = 0usize;
        while i < format.len() {
            let mut fmtc = format[i..].chars().next().unwrap();
            let mut w = fmtc.len_utf8();
            if go_scan_is_space(fmtc) {
                let mut newlines = 0;
                let mut trailing_space = false;
                while go_scan_is_space(fmtc) && i < format.len() {
                    if fmtc == '\n' {
                        newlines += 1;
                        trailing_space = false;
                    } else {
                        trailing_space = true;
                    }
                    i += w;
                    match format[i..].chars().next() {
                        Some(c) => {
                            fmtc = c;
                            w = c.len_utf8();
                        }
                        None => break,
                    }
                }
                for _ in 0..newlines {
                    let mut inputc = self.get_rune();
                    while matches!(inputc, Some(c) if go_scan_is_space(c) && c != '\n') {
                        inputc = self.get_rune();
                    }
                    if inputc != Some('\n') && inputc.is_some() {
                        return Err(self.error_string("newline in format does not match input"));
                    }
                }
                if trailing_space {
                    let mut inputc = self.get_rune();
                    if newlines == 0 {
                        if matches!(inputc, Some(c) if !go_scan_is_space(c)) {
                            return Err(self.error_string("expected space in input to match format"));
                        }
                        if inputc == Some('\n') {
                            return Err(self.error_string("newline in input does not match format"));
                        }
                    }
                    while matches!(inputc, Some(c) if go_scan_is_space(c) && c != '\n') {
                        inputc = self.get_rune();
                    }
                    if inputc.is_some() {
                        self.unread_rune();
                    }
                }
                continue;
            }
            if fmtc == '%' {
                if i + w == format.len() {
                    return Err(self.error_string("missing verb: % at end of format string"));
                }
                let nextc = format[i + w..].chars().next();
                if nextc != Some('%') {
                    return Ok(i as i64);
                }
                i += w;
            }
            let inputc = self.must_read_rune()?;
            if fmtc != inputc {
                self.unread_rune();
                return Ok(-1);
            }
            i += w;
        }
        Ok(i as i64)
    }

    fn do_scanf(&mut self, format: &str, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
        let mut n = 0usize;
        let result = (|| -> Result<(), GoScanError> {
            let bytes = format.as_bytes();
            let end = format.len() as i64 - 1;
            let mut i = 0usize;
            while (i as i64) <= end {
                let w = self.advance(&format[i..]);
                self.check_read()?;
                let w = w?;
                if w > 0 {
                    i += w as usize;
                    continue;
                }
                if bytes[i] != b'%' {
                    if w < 0 {
                        return Err(self.error_string("input does not match format"));
                    }
                    break;
                }
                i += 1;
                // The width, as fmt's parsenum reads it.
                let mut width = 0usize;
                let mut wid_present = false;
                if (i as i64) < end {
                    while (i as i64) < end && bytes[i].is_ascii_digit() {
                        // parsenum's tooLarge: a width over a million is
                        // dropped, and the verb is the format's last byte.
                        if width > 1_000_000 {
                            width = 0;
                            wid_present = false;
                            i = end as usize;
                            break;
                        }
                        width = width * 10 + (bytes[i] - b'0') as usize;
                        wid_present = true;
                        i += 1;
                    }
                } else {
                    i = end as usize;
                }
                self.max_wid = if wid_present { width } else { GO_SCAN_HUGE_WID };
                let c = format[i..].chars().next().unwrap_or('%');
                let cw = c.len_utf8();
                i += cw;
                if c != 'c' {
                    self.skip_space()?;
                }
                if c == '%' {
                    self.scan_percent()?;
                    continue;
                }
                self.arg_limit = self.limit;
                if self.count + self.max_wid < self.arg_limit {
                    self.arg_limit = self.count + self.max_wid;
                }
                if n >= targets.len() {
                    return Err(GoScanError::Msg(format!("too few operands for format '%{}'", &format[i - cw..])));
                }
                self.scan_one(c, &mut targets[n])?;
                n += 1;
                self.arg_limit = self.limit;
            }
            if n < targets.len() {
                return Err(self.error_string("too many operands"));
            }
            Ok(())
        })();
        (n as i64, result.err())
    }
}

// strconv.Unquote of a double-quoted string, as bytes: \xNN and octal
// escapes are bytes, \u and \U escapes and other text are UTF-8.
fn go_scan_unquote(text: &str) -> Option<std::vec::Vec<u8>> {
    let inner = text.strip_prefix('"')?.strip_suffix('"')?;
    let mut out: std::vec::Vec<u8> = std::vec::Vec::new();
    let mut chars = inner.chars();
    let mut utf8 = [0u8; 4];
    while let Some(c) = chars.next() {
        if c == '"' || c == '\n' {
            return None;
        }
        if c != '\\' {
            out.extend_from_slice(c.encode_utf8(&mut utf8).as_bytes());
            continue;
        }
        let escape = chars.next()?;
        let simple = match escape {
            'n' => Some('\n'),
            't' => Some('\t'),
            'r' => Some('\r'),
            'a' => Some('\u{7}'),
            'b' => Some('\u{8}'),
            'f' => Some('\u{c}'),
            'v' => Some('\u{b}'),
            '\\' => Some('\\'),
            '"' => Some('"'),
            _ => None,
        };
        if let Some(c) = simple {
            out.push(c as u8);
            continue;
        }
        match escape {
            'x' => {
                let hex: String = (0..2).filter_map(|_| chars.next()).collect();
                if hex.len() != 2 {
                    return None;
                }
                out.push(u8::from_str_radix(&hex, 16).ok()?);
            }
            'u' | 'U' => {
                let len = if escape == 'u' { 4 } else { 8 };
                let hex: String = (0..len).filter_map(|_| chars.next()).collect();
                if hex.len() != len {
                    return None;
                }
                let value = u32::from_str_radix(&hex, 16).ok()?;
                out.extend_from_slice(char::from_u32(value)?.encode_utf8(&mut utf8).as_bytes());
            }
            '0'..='7' => {
                let mut value = escape.to_digit(8)?;
                for _ in 0..2 {
                    value = value * 8 + chars.next()?.to_digit(8)?;
                }
                if value > 255 {
                    return None;
                }
                out.push(value as u8);
            }
            _ => return None,
        }
    }
    Some(out)
}

// math.Ldexp: frac times two to the exp, without overflowing on the way.
fn go_scan_ldexp(frac: f64, exp: i64) -> f64 {
    if frac == 0.0 || !frac.is_finite() {
        return frac;
    }
    let mut value = frac;
    let mut exp = exp.clamp(-2200, 2200) as i32;
    while exp > 0 {
        let step = exp.min(1000);
        value *= 2f64.powi(step);
        exp -= step;
    }
    while exp < 0 {
        let step = exp.max(-1000);
        value *= 2f64.powi(step);
        exp -= step;
    }
    value
}

// strconv.ParseFloat of a hexadecimal float: 0x, hex digits with an
// optional point, and a binary exponent p.
fn go_scan_parse_hex_float(lower: &str, bit_size: u32) -> Option<f64> {
    let (negative, rest) = match lower.as_bytes().first() {
        Some(b'-') => (true, &lower[1..]),
        Some(b'+') => (false, &lower[1..]),
        _ => (false, lower),
    };
    let rest = rest.strip_prefix("0x")?;
    let p = rest.find('p')?;
    let (digits, exp_text) = (&rest[..p], &rest[p + 1..]);
    let digits: String = digits.chars().filter(|&c| c != '_').collect();
    let mut mantissa: u128 = 0;
    let mut scale: i64 = 0;
    let mut seen_point = false;
    let mut any = false;
    for c in digits.chars() {
        if c == '.' {
            if seen_point {
                return None;
            }
            seen_point = true;
            continue;
        }
        let d = c.to_digit(16)? as u128;
        any = true;
        if mantissa < (1u128 << 120) {
            mantissa = mantissa * 16 + d;
            if seen_point {
                scale -= 4;
            }
        } else if !seen_point {
            scale += 4;
        }
    }
    if !any {
        return None;
    }
    let exp: i64 = exp_text.parse().ok()?;
    let value = go_scan_ldexp(mantissa as f64, exp + scale);
    let value = if bit_size == 32 { value as f32 as f64 } else { value };
    Some(if negative { -value } else { value })
}

// go_fmt_scan scans runes into targets as fmt.Fscan (mode 's'),
// fmt.Fscanln ('l') or fmt.Fscanf ('f', with format) does, and returns the
// number of operands filled and the error.
fn go_fmt_scan(rs: &mut dyn GoScanRunes, mode: char, format: &str, targets: &mut [GoScanTarget]) -> (i64, Option<GoScanError>) {
    let (nl_is_space, nl_is_end) = match mode {
        's' => (true, false),
        'l' => (false, true),
        _ => (false, false),
    };
    let mut state = GoScanState::new(rs, nl_is_space, nl_is_end);
    if mode == 'f' {
        state.do_scanf(format, targets)
    } else {
        state.do_scan(targets)
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: Arc<Mutex<Option<String>>>,
    pub qty: Arc<Mutex<Option<i64>>>,
    pub price: Arc<Mutex<Option<f64>>>,
    pub tags: Arc<Mutex<Option<Vec<String>>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.qty.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.price.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.tags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            qty: __go_clone_1_0,
            price: __go_clone_2_0,
            tags: __go_clone_3_0,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            qty: __go_default_1_0,
            price: __go_default_2_0,
            tags: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.qty.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = go_float_string(((*self.price.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_3 = format!("{}", format_slice(&self.tags));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Inventory(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Item>>>>>>>);

impl Display for Inventory {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map(&self.0))
    }
}


pub trait Reporter: std::fmt::Display + Any {
    fn __go_clone_box_reporter(&self) -> Box<dyn Reporter + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_reporter(&self, other: &(dyn Reporter + Send + Sync)) -> bool;
    fn report(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>;
}

impl Clone for Box<dyn Reporter + Send + Sync> {
    fn clone(&self) -> Self {
        Reporter::__go_clone_box_reporter(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct textReport {
    pub inv: Arc<Mutex<Option<Inventory>>>,
    pub title: Arc<Mutex<Option<String>>>,
}

impl textReport {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.inv.clone();
        let __go_clone_1_0 = { let __guard = self.title.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            inv: __go_clone_0_0,
            title: __go_clone_1_0,
        }
    }
}


impl Default for textReport {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Inventory::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            inv: __go_default_0_0,
            title: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for textReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.inv.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.title.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Inventory {
    pub fn add(&self, name: Arc<Mutex<Option<String>>>, qty: Arc<Mutex<Option<i64>>>, price: Arc<Mutex<Option<f64>>>, tags: Arc<Mutex<Option<Vec<String>>>>) {
        let mut __self = self.clone();
        {
        let (mut it, mut ok) = { let __map = { let __map_holder = __self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };;
        if ok {
            { let __target = (*it.lock().unwrap().as_ref().unwrap()).qty.clone(); let __rhs = (*qty.lock().unwrap().as_ref().unwrap()); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };;
            { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*it.lock().unwrap().as_ref().unwrap()).tags.clone(); let __append_values: Vec<_> = { let __slice_holder = tags.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); let __append_was_nil = (*__append_target.lock().unwrap()).is_none(); Arc::new(Mutex::new(if __append_was_nil && __appended.len() == 0 { None } else { Some(__appended) })) })); *(*it.lock().unwrap().as_ref().unwrap()).tags.lock().unwrap() = __slice_header; };;
            return;;
        }
    }
        { let __map_key = ({ let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some(Item { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), qty: Arc::new(Mutex::new(Some({ let __arg_holder = qty.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), price: Arc::new(Mutex::new(Some({ let __arg_holder = price.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), tags: { let __slice_header = GoSliceHeader::go_slice_header(&tags); Arc::new(Mutex::new(__slice_header)) }, ..Default::default() }))); (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn total(&self) -> f64 {
        let mut t = Arc::new(Mutex::new(Some(0.0)));
        for (_, it) in { let __range_holder = self.0.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __rhs = {
            let __tmp_x = (*Arc::new(Mutex::new(Some({ let __selector_holder = (*it.lock().unwrap().as_ref().unwrap()).qty.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned } as f64))).lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __field = (*it.lock().unwrap().as_ref().unwrap()).price.clone(); __field }.lock().unwrap().as_ref().unwrap());
            __tmp_x * __tmp_y
        }; let mut guard = t.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
        return { let __v = (*t.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

impl textReport {
    pub fn report(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        let mut names = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..({ let __named_map_holder = self.inv.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
        for (n, _) in { let __range_holder = { let __named_map_holder = self.inv.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); std::mem::drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = n.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };
    }
        let names_closure_clone = names.clone(); let mut r_closure_clone = (*self).clone(); { let __sort_target = names_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some({ let names_closure_clone_closure_clone = names_closure_clone.clone(); Arc::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        let mut r_closure_clone = r_closure_clone.clone();
        return {
            let __tmp_x = (*(*{ let __map = { let __map_holder = { let __named_map = (*r_closure_clone.inv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = names_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).qty.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*{ let __map = { let __map_holder = { let __named_map = (*r_closure_clone.inv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __seq = { let __seq_holder = names_closure_clone_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).qty.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> }))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
        { let __s = format!("== {} ==\n", (*self.title.lock().unwrap().as_ref().unwrap())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for n in __range_values.iter() {
        let mut it = { let __map = { let __map_holder = { let __named_map = (*self.inv.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(n)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        { let __go_print_arg_1 = strings::join({ let __field = (*it.lock().unwrap().as_ref().unwrap()).tags.clone(); __field }, Arc::new(Mutex::new(Some(",".to_string())))); let __go_print_value = { let __s = format!("{}", go_sprintf(&"%-6s %3d x %6.2f [%s]\n".to_string(), &[{ let __go_format_arg = Box::new({ let __selector_holder = (*it.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new({ let __selector_holder = (*it.lock().unwrap().as_ref().unwrap()).qty.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new({ let __selector_holder = (*it.lock().unwrap().as_ref().unwrap()).price.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new({ let __arg_holder = __go_print_arg_1.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>; __go_format_arg }], false)); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) }; __go_print_value };
    } }
        let (_, mut err) = { let __s = format!("total: {:.2}\n", (*self.inv.lock().unwrap().as_ref().unwrap()).total()); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        return { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) };
    }
}

impl Reporter for textReport {
    fn report(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        textReport::report(self, w)
    }
    fn __go_clone_box_reporter(&self) -> Box<dyn Reporter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reporter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reporter(&self, other: &(dyn Reporter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<textReport>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct textReportPtr(pub Arc<Mutex<Option<textReport>>>);

impl std::fmt::Display for textReportPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reporter for textReportPtr {
    fn report(&self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>> {
        let __recv_ptr: *const textReport = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const textReport };
        let __recv = unsafe { &*__recv_ptr };
        textReport::report(__recv, w)
    }
    fn __go_clone_box_reporter(&self) -> Box<dyn Reporter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reporter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_reporter(&self, other: &(dyn Reporter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<textReportPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    go_install_panic_exit();
    ::bufio::__go_init_all();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut started = GoChannel::<String>::new_buffered(1 as usize);
        let started_thread = started.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        started_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        started.recv().unwrap_or_default();
        let mut inv = Arc::new(Mutex::new(Some(Inventory(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Item>>>>::from([]))))))));
        (*inv.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("apple".to_string()))), Arc::new(Mutex::new(Some(3))), Arc::new(Mutex::new(Some(0.5))), Arc::new(Mutex::new(Some(vec!["fruit".to_string()]))));
        (*inv.lock().unwrap().as_ref().unwrap()).add(
            Arc::new(Mutex::new(Some("pear".to_string()))),
            Arc::new(Mutex::new(Some(1))),
            Arc::new(Mutex::new(Some(0.75))),
            Arc::new(Mutex::new(Some(vec![
                "fruit".to_string(),
                "green".to_string(),
            ]))),
        );
        (*inv.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("apple".to_string()))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(0.5))), Arc::new(Mutex::new(Some(vec!["red".to_string()]))));
        (*inv.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("nut".to_string()))), Arc::new(Mutex::new(Some(10))), Arc::new(Mutex::new(Some(0.1))), Arc::new(Mutex::new(Some(vec![]))));
        let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut rep: Arc<Mutex<Option<Box<dyn Reporter + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(textReport { inv: inv.clone(), title: Arc::new(Mutex::new(Some("stock".to_string()))), ..Default::default() }) as Box<dyn Reporter + Send + Sync>)));
        {
        let mut err = (*rep.lock().unwrap().as_ref().unwrap()).report(Arc::new(Mutex::new(Some(Box::new(strings::builder::BuilderPtr(sb.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "error".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
        }
    }
        let mut out = bufio::new_writer(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>; __arg_value }))));
        let out_defer_captured = out.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = out_defer_captured.clone(); let __recv_ptr: *mut bufio::r#mod::Writer = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut bufio::r#mod::Writer }; let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    }));
        { let __s = format!("{}", format!("{}", (*(*sb.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap()))); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        let mut a: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));let mut b: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
        let mut op: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(Some(String::new())));
        let (mut n, mut err) = { let mut __scan_src = go_scan_string_source("12 + 30".to_string()); let __scan_format: String = "%d %s %d".to_string(); let mut __scan_targets: std::vec::Vec<GoScanTarget> = std::vec![GoScanTarget::Int(64, { let __scan_h = a.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) }), GoScanTarget::Str({ let __scan_h = op.clone(); Box::new(move |__scan_v: String| { *__scan_h.lock().unwrap() = Some(__scan_v); }) }), GoScanTarget::Int(64, { let __scan_h = b.clone(); Box::new(move |__scan_v: i64| { *__scan_h.lock().unwrap() = Some(__scan_v as i64); }) })]; let (__scan_n, __scan_err) = go_fmt_scan(&mut __scan_src, 'f', &__scan_format, &mut __scan_targets); drop(__scan_targets); (__scan_n, match __scan_err { None => Arc::new(Mutex::new(None)), Some(GoScanError::Eof) => Arc::new(Mutex::new(Some((*io::EOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::UnexpectedEof) => Arc::new(Mutex::new(Some((*io::ErrUnexpectedEOF.lock().unwrap().as_ref().unwrap()).clone()))), Some(GoScanError::Msg(__scan_msg)) => Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(__scan_msg))))) }) };
        { let __s = format!("{} {} {} {} {} {}\n", format!("{}", n), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*op.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*b.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let __s = format!("{}", go_sprintf(&"%5.1f%%|%x|%08b|%q|%v|%T\n".to_string(), &[{ let __go_format_arg = Box::new(12.345) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(255i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(5i64) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(120i32) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new(Arc::new(Mutex::new(Some(vec!["a".to_string()]))).clone()) as Box<dyn Any + Send + Sync>; __go_format_arg }, { let __go_format_arg = Box::new((*inv.lock().unwrap()).clone().unwrap_or_default()) as Box<dyn Any + Send + Sync>; __go_format_arg }], false)); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };

        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
    }));
    match __go_panic_result {
        Ok(__go_value) => {
            go_restore_panic_hook(__go_previous_panic_hook);
            __go_value
        }
        Err(__go_panic_payload) => {
            go_store_panic_payload(__go_panic_payload);
            while let Some(f) = __defer_stack.pop() {
                if let Err(__go_defer_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
                    go_store_panic_payload(__go_defer_panic);
                }
            }
            go_restore_panic_hook(__go_previous_panic_hook);
            go_resume_unrecovered_panic();
            ()
        }
    }
}

impl GoValueClone for Item {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for textReport {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
