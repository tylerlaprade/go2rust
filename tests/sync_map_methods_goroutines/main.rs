#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

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

struct WaitGroup {
    count: std::sync::Arc<(std::sync::Mutex<i64>, std::sync::Condvar)>,
}

impl WaitGroup {
    fn new() -> Self {
        WaitGroup {
            count: std::sync::Arc::new((std::sync::Mutex::new(0), std::sync::Condvar::new())),
        }
    }

    fn add(&self, n: i64) {
        let (lock, _) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count += n;
    }

    fn done(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        *count -= 1;
        if *count <= 0 {
            cvar.notify_all();
        }
    }

    fn wait(&self) {
        let (lock, cvar) = &*self.count;
        let mut count = lock.lock().unwrap();
        while *count > 0 {
            count = cvar.wait(count).unwrap();
        }
    }
}

impl Clone for WaitGroup {
    fn clone(&self) -> Self {
        WaitGroup {
            count: self.count.clone(),
        }
    }
}

impl Default for WaitGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for WaitGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WaitGroup")
    }
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


#[derive(Debug, Clone)]
pub struct node {
    pub name: Arc<Mutex<Option<String>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut m: Arc<Mutex<Option<sync_Map>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut wg = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        wg.add(1);
        let m_thread = m.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __sync_value = (*m_thread.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.store(
                { let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() },
                { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i },
            );;
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    let (mut v, mut ok) = { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load((3 as i64));
    let mut n = Arc::new(Mutex::new(Some(({
        let val = v.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }))));
    println!("{} {}", format!("{}", { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", ok));
    let (mut prev, mut loaded) = { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.swap((3 as i64), "three".to_string());
    println!("{} {} {}", format!("{}", { let __text = match prev.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", loaded), format!("{}", { let __text = match (must_load(m.clone(), Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    println!("{} {} {}", format!("{}", { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.compare_and_swap((3 as i64), "four".to_string(), (4 as i64))), format!("{}", { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.compare_and_swap((3 as i64), "three".to_string(), (4 as i64))), format!("{}", { let __text = match (must_load(m.clone(), Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    println!("{} {}", format!("{}", { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.compare_and_delete((4 as i64), (15 as i64))), format!("{}", { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.compare_and_delete((4 as i64), (16 as i64))));
    let mut keys: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    let m_closure_clone = m.clone(); { let __sync_value = (*m_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.range(
        Arc::new(Mutex::new(Some({ let m_closure_clone_closure_clone = m_closure_clone.clone(); Arc::new(move |k: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| -> bool {
        {
        let mut n = Arc::new(Mutex::new(Some(({
        let val = k.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }))));;
        if { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y } {
            { let __sync_value = (*m_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.delete(k.clone());;
        }
    }
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool + Send + Sync> }))),
    );
    let mut keys_closure_clone = keys.clone(); { let __sync_value = (*m.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.range(
        Arc::new(Mutex::new(Some(Arc::new(move |k: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| -> bool {
        let mut keys_closure_clone = keys_closure_clone.clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys_closure_clone.clone(); let __append_value = ({
        let val = k.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    }); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys_closure_clone.lock().unwrap() = __slice_header; };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool + Send + Sync>))),
    );
    sort::ints(keys.clone());
    println!("{}", format!("{}", format_slice(&keys)));
    let mut pm: Arc<Mutex<Option<sync_Map>>> = Arc::new(Mutex::new(Some(Default::default())));
    let (mut a, mut b) = (Arc::new(Mutex::new(Some(node { name: Arc::new(Mutex::new(Some("a".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(node { name: Arc::new(Mutex::new(Some("a".to_string()))), ..Default::default() }))));
    { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.store(a.clone(), (1 as i64));
    let (_, mut okA) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load(a.clone());
    { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.store(
        point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() },
        "p".to_string(),
    );
    let (mut pv, mut okP) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load(point { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(2))), ..Default::default() });
    let (_, mut okQ) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load(point { x: Arc::new(Mutex::new(Some(2))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() });
    println!("{} {} {} {}", format!("{}", okA), format!("{}", { let __text = match pv.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", okP), format!("{}", okQ));
    let _ = &b;
    let mut key: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("k".to_string()) as Box<dyn Any + Send + Sync>)));
    let mut val: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(42i64) as Box<dyn Any + Send + Sync>)));
    { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.store(key.clone(), val.clone());
    let (mut got, _) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load("k".to_string());
    println!("{}", format!("{}", ({
        let val = got.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    })));
    { let new_val = Box::new(7i64) as Box<dyn Any + Send + Sync>; *val.lock().unwrap() = Some(new_val); };
    { let (__tmp_0, __tmp_1) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load(key.clone()); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *got.lock().unwrap() = __moved_tmp_0; };
    println!("{}", format!("{}", { let __text = match got.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut stop = Arc::new(Mutex::new(Some(0i64)));
    let mut stop_closure_clone = stop.clone(); { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.range(
        Arc::new(Mutex::new(Some(Arc::new(move |k: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>| -> bool {
        let mut stop_closure_clone = stop_closure_clone.clone();
        { let mut guard = stop_closure_clone.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        false
    }) as Arc<dyn Fn(Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> bool + Send + Sync>))),
    );
    println!("{}", format!("{}", { let __v = (*stop.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.clear();
    { let (__tmp_0, __tmp_1) = { let __sync_value = (*pm.lock().unwrap().as_ref().unwrap()).clone(); __sync_value }.load("k".to_string()); ok = __tmp_1; };
    println!("{}", format!("{}", ok));
}

pub fn must_load(m: Arc<Mutex<Option<sync_Map>>>, k: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
    let (mut v, _) = { let __recv = m.clone(); let __recv_ptr: *mut sync_Map = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut sync_Map }; let __result = unsafe { &mut *__recv_ptr }.load(k.clone()); __result };
    return go_any_handle_copy(&v);
}

impl GoValueClone for point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
