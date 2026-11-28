#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use unicode_utf8::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};


/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>>>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.lock().unwrap().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    go_clone_registered_any(value)
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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

#[derive(Debug, Clone)]
pub struct Utf8Map {
    pub r: Arc<Mutex<Option<i32>>>,
    pub str: Arc<Mutex<Option<String>>>,
}

impl Utf8Map {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.str.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            str: __go_clone_1_0,
        }
    }
}


impl Default for Utf8Map {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            r: __go_default_0_0,
            str: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Utf8Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.str.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct RuneLenTest {
    pub r: Arc<Mutex<Option<i32>>>,
    pub size: Arc<Mutex<Option<i64>>>,
}

impl RuneLenTest {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for RuneLenTest {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for RuneLenTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct ValidRuneTest {
    pub r: Arc<Mutex<Option<i32>>>,
    pub ok: Arc<Mutex<Option<bool>>>,
}

impl ValidRuneTest {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.ok.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r: __go_clone_0_0,
            ok: __go_clone_1_0,
        }
    }
}


impl Default for ValidRuneTest {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            r: __go_default_0_0,
            ok: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for ValidRuneTest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.ok.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static utf8map: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Utf8Map>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static runelentests: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<RuneLenTest>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub(crate) static validrunetests: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<ValidRuneTest>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *utf8map.lock().unwrap() = Some(Vec::nil());
    *runelentests.lock().unwrap() = Some(Vec::nil());
    *validrunetests.lock().unwrap() = Some(Vec::nil());
    {
        let mut __go_slice = Vec::<Utf8Map>::with_capacity(32);
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0000 as i32))), str: Arc::new(Mutex::new(Some("\u{0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0001 as i32))), str: Arc::new(Mutex::new(Some("\u{1}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x007e as i32))), str: Arc::new(Mutex::new(Some("~".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x007f as i32))), str: Arc::new(Mutex::new(Some("\u{7f}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0080 as i32))), str: Arc::new(Mutex::new(Some("\u{80}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0081 as i32))), str: Arc::new(Mutex::new(Some("\u{81}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00bf as i32))), str: Arc::new(Mutex::new(Some("\u{bf}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00c0 as i32))), str: Arc::new(Mutex::new(Some("\u{c0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00c1 as i32))), str: Arc::new(Mutex::new(Some("\u{c1}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00c8 as i32))), str: Arc::new(Mutex::new(Some("\u{c8}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00d0 as i32))), str: Arc::new(Mutex::new(Some("\u{d0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00e0 as i32))), str: Arc::new(Mutex::new(Some("\u{e0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00f0 as i32))), str: Arc::new(Mutex::new(Some("\u{f0}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00f8 as i32))), str: Arc::new(Mutex::new(Some("\u{f8}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x00ff as i32))), str: Arc::new(Mutex::new(Some("\u{ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0100 as i32))), str: Arc::new(Mutex::new(Some("\u{100}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x07ff as i32))), str: Arc::new(Mutex::new(Some("\u{7ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0400 as i32))), str: Arc::new(Mutex::new(Some("\u{400}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0800 as i32))), str: Arc::new(Mutex::new(Some("\u{800}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x0801 as i32))), str: Arc::new(Mutex::new(Some("\u{801}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x1000 as i32))), str: Arc::new(Mutex::new(Some("\u{1000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xd000 as i32))), str: Arc::new(Mutex::new(Some("\u{d000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xd7ff as i32))), str: Arc::new(Mutex::new(Some("\u{d7ff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xe000 as i32))), str: Arc::new(Mutex::new(Some("\u{e000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xfffe as i32))), str: Arc::new(Mutex::new(Some("\u{fffe}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xffff as i32))), str: Arc::new(Mutex::new(Some("\u{ffff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x10000 as i32))), str: Arc::new(Mutex::new(Some("\u{10000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x10001 as i32))), str: Arc::new(Mutex::new(Some("\u{10001}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x40000 as i32))), str: Arc::new(Mutex::new(Some("\u{40000}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x10fffe as i32))), str: Arc::new(Mutex::new(Some("\u{10fffe}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0x10ffff as i32))), str: Arc::new(Mutex::new(Some("\u{10ffff}".to_string()))), ..Default::default() });
        __go_slice.push(Utf8Map { r: Arc::new(Mutex::new(Some(0xFFFD as i32))), str: Arc::new(Mutex::new(Some("\u{fffd}".to_string()))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *utf8map.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<RuneLenTest>::with_capacity(10);
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(0 as i32))), size: Arc::new(Mutex::new(Some(1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(('e' as i32)))), size: Arc::new(Mutex::new(Some(1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(('\u{e9}' as i32)))), size: Arc::new(Mutex::new(Some(2))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(('\u{263a}' as i32)))), size: Arc::new(Mutex::new(Some(3))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(RUNE_ERROR as i32))), size: Arc::new(Mutex::new(Some(3))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(MAX_RUNE as i32))), size: Arc::new(Mutex::new(Some(4))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(0xD800 as i32))), size: Arc::new(Mutex::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(0xDFFF as i32))), size: Arc::new(Mutex::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some({ let __tmp_x = MAX_RUNE; let __tmp_y = 1; __tmp_x + __tmp_y } as i32))), size: Arc::new(Mutex::new(Some(-1))), ..Default::default() });
        __go_slice.push(RuneLenTest { r: Arc::new(Mutex::new(Some(-1 as i32))), size: Arc::new(Mutex::new(Some(-1))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *runelentests.lock().unwrap() = Some(__go_slice);
    }
    {
        let mut __go_slice = Vec::<ValidRuneTest>::with_capacity(12);
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(0 as i32))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(('e' as i32)))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(('\u{e9}' as i32)))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(('\u{263a}' as i32)))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(RUNE_ERROR as i32))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(MAX_RUNE as i32))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(0xD7FF as i32))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(0xD800 as i32))), ok: Arc::new(Mutex::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(0xDFFF as i32))), ok: Arc::new(Mutex::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(0xE000 as i32))), ok: Arc::new(Mutex::new(Some(true))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some({ let __tmp_x = MAX_RUNE; let __tmp_y = 1; __tmp_x + __tmp_y } as i32))), ok: Arc::new(Mutex::new(Some(false))), ..Default::default() });
        __go_slice.push(ValidRuneTest { r: Arc::new(Mutex::new(Some(-1 as i32))), ok: Arc::new(Mutex::new(Some(false))), ..Default::default() });
        let __go_slice = Vec::from(__go_slice.into_boxed_slice().into_vec());
        *validrunetests.lock().unwrap() = Some(__go_slice);
    }
}


/// TestConstants is copied verbatim from unicode/utf8's own utf8_test.go.
/// Keeping the package test body unchanged makes this fixture an oracle for the
/// source-transpiled package rather than a hand-written approximation of it.
pub fn test_constants(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>) {
    if false {
        { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("utf8.MaxRune is wrong: %x should be %x".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(1114111i32) as Box<dyn Any + Send + Sync>, Box::new(1114111i32) as Box<dyn Any + Send + Sync>])))); __result };
    }
    if false {
        { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("utf8.RuneError is wrong: %x should be %x".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(65533i32) as Box<dyn Any + Send + Sync>, Box::new(65533i32) as Box<dyn Any + Send + Sync>])))); __result };
    }
}

pub fn test_append_rune(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = utf8map.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for m in __range_values.iter() {
        let m = &m.__go_value_clone();
        {
        let mut buf = append_rune(Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));
let mut buf = { let __slice_header = GoSliceHeader::go_slice_header(&buf); Arc::new(Mutex::new(__slice_header)) };;
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*buf.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = { let __selector_holder = m.str.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; __tmp_x != __tmp_y } {
            { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("AppendRune(nil, %#04x) = %s, want %s".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(buf.clone()) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = m.str.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
        }
    }
        {
        let mut buf = append_rune(
            Arc::new(Mutex::new(Some(("init".to_string()).as_bytes().to_go_slice()))),
            Arc::new(Mutex::new(Some({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })))
        );
let mut buf = { let __slice_header = GoSliceHeader::go_slice_header(&buf); Arc::new(Mutex::new(__slice_header)) };;
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8(std::vec::Vec::from((*buf.lock().unwrap().as_ref().unwrap()).clone())).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = format!("{}{}", "init".to_string(), (*m.str.lock().unwrap().as_ref().unwrap()).clone()); __tmp_x != __tmp_y } {
            { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(
                Arc::new(Mutex::new(Some("AppendRune(init, %#04x) = %s, want %s".to_string()))),
                Arc::new(Mutex::new(Some(vec![
                    Box::new({ let __selector_holder = m.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>,
                    Box::new(buf.clone()) as Box<dyn Any + Send + Sync>,
                    Box::new(format!("{}{}", "init".to_string(), (*m.str.lock().unwrap().as_ref().unwrap()).clone())) as Box<dyn Any + Send + Sync>,
                ]))),
            ); __result };;
        }
    }
    } }
}

/// Check that negative runes encode as U+FFFD.
pub fn test_negative_rune(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>) {
    let mut errorbuf = Arc::new(Mutex::new(Some(vec![0u8; (U_T_F_MAX) as usize])));
    { let __slice_header = GoSliceHeader::go_slice_header(&(Arc::new(Mutex::new(Some({
        let __seq_holder = errorbuf.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (0) as usize;
        let __high = (encode_rune(errorbuf.clone(), Arc::new(Mutex::new(Some(RUNE_ERROR as i32))))) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))); *errorbuf.lock().unwrap() = __slice_header; };
    let mut buf = Arc::new(Mutex::new(Some(vec![0u8; (U_T_F_MAX) as usize])));
    { let __slice_header = GoSliceHeader::go_slice_header(&(Arc::new(Mutex::new(Some({
        let __seq_holder = buf.clone();
        let __seq_guard = __seq_holder.lock().unwrap();
        let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
        let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
        std::mem::drop(__seq_guard);
        let __low = (0) as usize;
        let __high = (encode_rune(buf.clone(), Arc::new(Mutex::new(Some(-1 as i32))))) as usize;
        let __max = __source_cap;
        __seq.reslice3_zeroed(__low, __high, __max)
    }))))); *buf.lock().unwrap() = __slice_header; };
    if !bytes::equal(buf.clone(), errorbuf.clone()) {
        { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("incorrect encoding [% x] for -1; expected [% x]".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new(buf.clone()) as Box<dyn Any + Send + Sync>, Box::new(errorbuf.clone()) as Box<dyn Any + Send + Sync>])))); __result };
    }
}

pub fn test_rune_len(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = runelentests.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for tt in __range_values.iter() {
        let tt = &tt.__go_value_clone();
        {
        let mut size = rune_len(Arc::new(Mutex::new(Some({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = size; let __tmp_y = (*tt.size.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
            { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("RuneLen(%#U) = %d, want %d".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(size) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = tt.size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
        }
    }
    } }
}

pub fn test_valid_rune(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>) {
    { let __range_holder = validrunetests.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for tt in __range_values.iter() {
        let tt = &tt.__go_value_clone();
        {
        let mut ok = valid_rune(Arc::new(Mutex::new(Some({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));;
        if { let __tmp_x = ok; let __tmp_y = (*tt.ok.lock().unwrap().as_ref().unwrap()); __tmp_x != __tmp_y } {
            { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.errorf(Arc::new(Mutex::new(Some("ValidRune(%#U) = %t, want %t".to_string()))), Arc::new(Mutex::new(Some(vec![Box::new({ let __selector_holder = tt.r.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>, Box::new(ok) as Box<dyn Any + Send + Sync>, Box::new({ let __selector_holder = tt.ok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as Box<dyn Any + Send + Sync>])))); __result };;
        }
    }
    } }
}

pub fn run(t: Arc<Mutex<Option<oracle_testing::r#mod::T>>>, name: Arc<Mutex<Option<String>>>, test: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>>>>) -> bool {
    { let __recv = t.clone(); let __recv_ptr: *mut oracle_testing::r#mod::T = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut oracle_testing::r#mod::T }; let __result = unsafe { &mut *__recv_ptr }.reset(); __result };
    {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync> = test.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            t.clone()
        )
    };
    if { let __recv = t.clone(); let __recv_ptr: *const oracle_testing::r#mod::T = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const oracle_testing::r#mod::T }; let __result = unsafe { &*__recv_ptr }.failed(); __result } {
        {
            let __go_print_arg_0 = format!("{}", format!("{}{}", "FAIL ".to_string(), { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            eprintln!("{}", __go_print_arg_0)
        };
        return false;
    }
    {
            let __go_print_arg_0 = format!("{}", format!("{}{}", "PASS ".to_string(), { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }));
            eprintln!("{}", __go_print_arg_0)
        };
    true
}

fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::oracle_testing::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    let mut t = Arc::new(Mutex::new(Some(oracle_testing::r#mod::T { ..Default::default() })));
    let mut passed = Arc::new(Mutex::new(Some(0i64)));
    let mut failed = Arc::new(Mutex::new(Some(0i64)));
    for tt in &vec![
        AnonymousStruct1 {
            name: Arc::new(Mutex::new(Some("TestConstants".to_string()))),
            test: Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<oracle_testing::r#mod::T>>>| { test_constants(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>))),
            ..Default::default()
        },
        AnonymousStruct1 {
            name: Arc::new(Mutex::new(Some("TestAppendRune".to_string()))),
            test: Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<oracle_testing::r#mod::T>>>| { test_append_rune(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>))),
            ..Default::default()
        },
        AnonymousStruct1 {
            name: Arc::new(Mutex::new(Some("TestNegativeRune".to_string()))),
            test: Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<oracle_testing::r#mod::T>>>| { test_negative_rune(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>))),
            ..Default::default()
        },
        AnonymousStruct1 {
            name: Arc::new(Mutex::new(Some("TestRuneLen".to_string()))),
            test: Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<oracle_testing::r#mod::T>>>| { test_rune_len(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>))),
            ..Default::default()
        },
        AnonymousStruct1 {
            name: Arc::new(Mutex::new(Some("TestValidRune".to_string()))),
            test: Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<oracle_testing::r#mod::T>>>| { test_valid_rune(__arg0) }) as Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>))),
            ..Default::default()
        },
    ] {
        let tt = &tt.__go_value_clone();
        if run(t.clone(), Arc::new(Mutex::new(Some({ let __selector_holder = tt.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), Arc::new(Mutex::new({ let __func_value = (*tt.test.lock().unwrap()).clone(); __func_value }))) {
        { let mut guard = passed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    } else {
        { let mut guard = failed.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
    {
            let __go_print_arg_0 = format!("{}", "summary".to_string());
            let __go_print_arg_1 = format!("{}", { let __v = (*passed.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_2 = format!("{}", "passed,".to_string());
            let __go_print_arg_3 = format!("{}", { let __v = (*failed.lock().unwrap().as_ref().unwrap()).clone(); __v });
            let __go_print_arg_4 = format!("{}", "failed".to_string());
            eprintln!("{} {} {} {} {}", __go_print_arg_0, __go_print_arg_1, __go_print_arg_2, __go_print_arg_3, __go_print_arg_4)
        };
}

#[derive(Clone)]
pub struct AnonymousStruct1 {
    pub name: Arc<Mutex<Option<String>>>,
    pub test: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<oracle_testing::r#mod::T>>>) -> () + Send + Sync>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.test.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            test: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            test: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}


impl GoValueClone for Utf8Map {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for RuneLenTest {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for ValidRuneTest {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
