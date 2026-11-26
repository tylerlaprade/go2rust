#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<point>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<square>() {
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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<point>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<square>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

#[allow(dead_code)]
fn go_any_handle_copy(handle: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let __copy = match handle.lock().unwrap().as_ref() {
        None => None,
        Some(value) => match go_any_try_clone(&**value) {
            Some(copy) => Some(copy),
            None => return handle.clone(),
        },
    };
    Arc::new(Mutex::new(__copy))
}

/// Stores Go's copy of source's value in target's cell. A source that no
/// one else holds gives up its value; a value whose type has no copy comes
/// back for the caller to share instead.
#[allow(dead_code)]
fn go_any_store(target: &Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, source: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Option<Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>> {
    if Arc::ptr_eq(target, &source) {
        return None;
    }
    let __value = if Arc::strong_count(&source) == 1 {
        source.lock().unwrap().take()
    } else {
        let __copy = match source.lock().unwrap().as_ref() {
            None => None,
            Some(value) => match go_any_try_clone(&**value) {
                Some(copy) => Some(copy),
                None => return Some(source.clone()),
            },
        };
        __copy
    };
    *target.lock().unwrap() = __value;
    None
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
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
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
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_payload_text(&**__inner);
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_payload_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_payload_text(&**__inner);
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
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
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

struct GoPointerError<T> {
    handle: Arc<Mutex<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Arc<Mutex<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.lock().unwrap();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + Send + Sync + 'static> std::error::Error for GoPointerError<T> {}

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
    if val.is::<Arc<Mutex<Option<point>>>>() { return "*main.point" }
    if val.is::<GoNilAny>() { return "<nil>" }
    if val.is::<point>() { return "main.point" }
    if val.is::<square>() { return "main.square" }
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

#[derive(Debug, Clone)]
pub struct point {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait shape: std::fmt::Display + Any {
    fn __go_clone_box_shape(&self) -> Box<dyn shape + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_shape(&self, other: &(dyn shape + Send + Sync)) -> bool;
    fn area(&self) -> i64;
}

impl Clone for Box<dyn shape + Send + Sync> {
    fn clone(&self) -> Self {
        shape::__go_clone_box_shape(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct square {
    pub side: Arc<Mutex<Option<i64>>>,
}

impl square {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.side.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            side: __go_clone_0_0,
        }
    }
}


impl Default for square {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            side: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for square {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.side.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for square {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.side.lock().unwrap().as_ref().cloned() }; let __right = { other.side.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone, Default)]
pub struct r#box {
    pub r#in: GoChannel<Option<Box<dyn Any + Send + Sync>>>,
}

impl r#box {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r#in.clone();
        Self {
            r#in: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for r#box {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl square {
    pub fn area(&self) -> i64 {
        return { let __tmp_x = (*self.side.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.side.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
    }
}

impl shape for square {
    fn area(&self) -> i64 {
        square::area(self)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_shape(&self, other: &(dyn shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<square>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct squarePtr(pub Arc<Mutex<Option<square>>>);

impl std::fmt::Display for squarePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl shape for squarePtr {
    fn area(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        square::area(__recv)
    }
    fn __go_clone_box_shape(&self) -> Box<dyn shape + Send + Sync> {
        Box::new(self.clone()) as Box<dyn shape + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_shape(&self, other: &(dyn shape + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<squarePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn produce(out: GoChannel<Option<Box<dyn Any + Send + Sync>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
    out.send({ let __iface_handle = go_any_handle_copy(&v); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
}

pub fn next(r#in: GoChannel<Option<Box<dyn Any + Send + Sync>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    Arc::new(Mutex::new(r#in.recv().unwrap_or_default())).clone()
}

fn main() {
    go_install_panic_exit();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    first_half();
    second_half();
}

pub fn first_half() {
    let mut ch = GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new_buffered(8 as usize);
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new("old".to_string()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    println!("{}", format!("{}", ({
        let val = (Arc::new(Mutex::new(ch.recv().unwrap_or_default()))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<String>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "string")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "string")
        }
    })));
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(42i64) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(3.5) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(true) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() }) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send(None::<Box<dyn Any + Send + Sync>>);
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some({ let __err_holder = Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from("boom".to_string()))))).clone(); let __err_guard = __err_holder.lock().unwrap(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any + Send + Sync>, Some(__err) => Box::new(Arc::new(Mutex::new(Some(__err.clone())))) as Box<dyn Any + Send + Sync> } }))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(square { side: Arc::new(Mutex::new(Some(3))), ..Default::default() }) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        let mut v = Arc::new(Mutex::new(ch.recv().unwrap_or_default()));
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
    if _ts_val.and_then(|__v| __v.downcast_ref::<i64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<i64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        println!("{} {}", format!("{}", "int".to_string()), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<f64>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<f64>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        println!("{} {}", format!("{}", "float".to_string()), { let __text = go_float_string(({ let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }) as f64, false); __text });;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<bool>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<bool>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        println!("{} {}", format!("{}", "bool".to_string()), format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<point>()).is_some() {
        let x = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<point>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        println!("{} {} {}", format!("{}", "point".to_string()), format!("{}", (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*x.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).is_some() {
        let x = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Vec<i64>>>>>()).unwrap().clone();
        std::mem::drop(_ts_guard);
        let mut x = { let __slice_header = GoSliceHeader::go_slice_header(&x); Arc::new(Mutex::new(__slice_header)) };
        println!("{} {}", format!("{}", "slice".to_string()), format!("{}", ({ let __len = (*x.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));;
    } else if _ts_is_nil {
        let x = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        println!("{}", format!("{}", "nil".to_string()));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapErrors>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<time::format::ParseError>>>>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<time::zoneinfo_read::fileSizeError>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<time::zoneinfo_read::fileSizeError>>>>()).is_some() {
        let x = if let Some(__err_handle) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>>()) { { let __err_source = __err_handle; let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) } } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_errorString>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<errors_joinError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<fmt_wrapErrors>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<time::format::ParseError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<time::zoneinfo_read::fileSizeError>()) { Arc::new(Mutex::new(Some(Arc::new(typed_val.clone()) as Arc<dyn StdError + Send + Sync>))) } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<Arc<Mutex<Option<time::zoneinfo_read::fileSizeError>>>>()) { Arc::new(Mutex::new(Some(Arc::new(GoPointerError::new(typed_val.clone())) as Arc<dyn StdError + Send + Sync>))) } else { unreachable!("type switch error case matched no error payload") };
        std::mem::drop(_ts_guard);
        println!("{} {}", format!("{}", "error".to_string()), format!("{}", { let __text = match x.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<square>()).is_some() {
        let x: Arc<Mutex<Option<Box<dyn shape + Send + Sync>>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<square>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn shape + Send + Sync>)))
        } else {
            panic!("type switch interface case condition matched no concrete implementor")
        };
        std::mem::drop(_ts_guard);
        println!("{} {}", format!("{}", "shape".to_string()), format!("{}", (*x.lock().unwrap().as_ref().unwrap()).area()));;
    }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut s = Arc::new(Mutex::new(Some("kept".to_string())));
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    println!("{} {}", format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __text = match (Arc::new(Mutex::new(ch.recv().unwrap_or_default()))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut anyVal: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>)));
    ch.send({ let __iface_handle = go_any_handle_copy(&anyVal); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    let (mut got, mut ok) = match ch.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
    println!("{} {}", format!("{}", { let __text = match got.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut results = GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new();
    let results_thread = results.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        results_thread.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new("from goroutine".to_string()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });;
        results_thread.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(99i64) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });;
        results_thread.close();;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    for __iface_value in results.clone() {
        let mut r = Arc::new(Mutex::new(__iface_value));
        println!("{}", format!("{}", { let __text = match r.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    }
    let (mut v, mut ok) = match results.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
    println!("{} {}", format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v }));
}

pub fn second_half() {
    let mut ch = GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new_buffered(4 as usize);
    let mut p = Arc::new(Mutex::new(Some(point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() })));
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(p.clone()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    let mut got = ({
        let val = (Arc::new(Mutex::new(ch.recv().unwrap_or_default()))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<point>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.point")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.point")
        }
    }).clone();
    { let new_val = 10; *(*got.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    println!("{}", format!("{}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut b = Arc::new(Mutex::new(Some(r#box { r#in: GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new_buffered(1 as usize), ..Default::default() })));
    { let __channel = (*b.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new("field".to_string()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    println!("{}", format!("{}", { let __text = match (Arc::new(Mutex::new({ let __channel = (*b.lock().unwrap().as_ref().unwrap()).r#in.clone(); __channel }.recv().unwrap_or_default()))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    produce(ch.clone(), Arc::new(Mutex::new(Some(Box::new(5i64) as Box<dyn Any + Send + Sync>))));
    println!("{}", format!("{}", { let __text = match (next(ch.clone())).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("start".to_string()) as Box<dyn Any + Send + Sync>)));
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(6i64) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    { let new_val = ch.recv().unwrap_or_default(); *v.lock().unwrap() = new_val; };
    println!("{}", format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>>::from([]))));
    ch.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    { let __map_key = "k".to_string(); let __map_value = Arc::new(Mutex::new(ch.recv().unwrap_or_default())); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{}", format!("{}", { let __text = match ({ let __map = { let __map_holder = m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"k".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut sel = GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new_buffered(1 as usize);
    {
    let __go_select_channel_0 = sel.clone();
    let mut __go_select_value_0 = Some({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new("selected".to_string()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    loop {
        if __go_select_channel_0.try_send_select(&mut __go_select_value_0) {
            break;
        }
        println!("{}", format!("{}", "full".to_string()));
        break;
    }
    }
    {
    let __go_select_channel_0 = sel.clone();
    let __go_select_channel_1 = time::after(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::SECOND as i64)))))))).clone();
    loop {
        if let Some(__select_recv) = __go_select_channel_0.try_recv_select() {
            let __select_ok = __select_recv.is_some();
            let x = match __select_recv { Some(__v) => __v, None => Default::default() };
            let mut x = Arc::new(Mutex::new(x));
            println!("{} {}", format!("{}", "got".to_string()), format!("{}", { let __text = match x.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
            break;
        }
        if let Some(_) = __go_select_channel_1.try_recv_select() {
            println!("{}", format!("{}", "timeout".to_string()));
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    }
    let mut done = GoChannel::<AnonymousStruct1>::new();
    let mut work = GoChannel::<Option<Box<dyn Any + Send + Sync>>>::new();
    let done_thread = done.clone(); let work_thread = work.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let done_defer_captured = done_thread.clone(); __defer_stack.push(Box::new(move || {
        done_defer_captured.close();
    }));;
        for __iface_value in work_thread.clone() {
        let mut w = Arc::new(Mutex::new(__iface_value));
        {
        let (mut n, mut ok) = ({
        let val = w.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "n".to_string()), format!("{}", { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }));;
        } else {
            println!("{} {}", format!("{}", "other".to_string()), format!("{}", { let __text = match w.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));;
        }
    }
    };
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
;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    work.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(21i64) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    work.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new("text".to_string()) as Box<dyn Any + Send + Sync>))); let __iface_value = __iface_handle.lock().unwrap().take(); __iface_value });
    work.send(None::<Box<dyn Any + Send + Sync>>);
    work.close();
    done.recv().unwrap_or_default();
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl GoValueClone for point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for square {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for r#box {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
