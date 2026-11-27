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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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
pub struct Vec_ {
    pub x: Arc<Mutex<Option<f64>>>,
    pub y: Arc<Mutex<Option<f64>>>,
}

impl Vec_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Vec_ {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0.0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0.0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Vec_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = go_float_string(((*self.x.lock().unwrap().as_ref().unwrap())) as f64, false);
        let __go_fmt_1 = go_float_string(((*self.y.lock().unwrap().as_ref().unwrap())) as f64, false);
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_int {
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<i64>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Vec_ {
    pub fn add(&self, o: Arc<Mutex<Option<Vec_>>>) -> Arc<Mutex<Option<Vec_>>> {
        Arc::new(Mutex::new(Some(Vec_ { x: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.x.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*o.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))), y: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.y.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*{ let __field = (*o.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }))), ..Default::default() })))
    }

    pub fn scale(&self, k: Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Vec_>>> {
        Arc::new(Mutex::new(Some(Vec_ { x: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.x.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }))), y: Arc::new(Mutex::new(Some({ let __tmp_x = (*self.y.lock().unwrap().as_ref().unwrap()); let __tmp_y = { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x * __tmp_y }))), ..Default::default() })))
    }

    pub fn normalize(&mut self) {
        let mut l = math::sqrt(Arc::new(Mutex::new(Some({
            let __tmp_x = { let __tmp_x = (*self.x.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.x.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            let __tmp_y = { let __tmp_x = (*self.y.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*self.y.lock().unwrap().as_ref().unwrap()); __tmp_x * __tmp_y };
            __tmp_x + __tmp_y
        }))));
        { let __target = self.x.clone(); let __rhs = l; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
        { let __target = self.y.clone(); let __rhs = l; let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() / __rhs); };
    }
}

pub fn defer_order() -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut out: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        let i_defer_captured = Arc::new(Mutex::new((*i.lock().unwrap()).clone())); let mut out_defer_captured = out.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        let mut out_defer_captured = out_defer_captured.clone();
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out_defer_captured.clone(); let __append_value = ({ let __v = (*i_defer_captured.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out_defer_captured.lock().unwrap() = __slice_header; };
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    }));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        {
        { let new_val = Arc::new(Mutex::new(Some(vec![100i64]))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *out.lock().unwrap() = __moved_val; };;
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return out.clone();
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
            out.clone()
        }
    }
}

fn main() {
    go_install_panic_exit();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();

    let mut add = Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<Vec_>>>, __arg1: Arc<Mutex<Option<Vec_>>>| -> Arc<Mutex<Option<Vec_>>> { { let __recv = __arg0.clone(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).add(__arg1); __result } }) as Arc<dyn Fn(Arc<Mutex<Option<Vec_>>>, Arc<Mutex<Option<Vec_>>>) -> Arc<Mutex<Option<Vec_>>> + Send + Sync>)));
    let mut scale = Arc::new(Mutex::new(Some(Arc::new(move |__arg0: Arc<Mutex<Option<Vec_>>>, __arg1: Arc<Mutex<Option<f64>>>| -> Arc<Mutex<Option<Vec_>>> { { let __recv = __arg0.clone(); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).scale(__arg1); __result } }) as Arc<dyn Fn(Arc<Mutex<Option<Vec_>>>, Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Vec_>>> + Send + Sync>)));
    let mut v = {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec_>>>, Arc<Mutex<Option<Vec_>>>) -> Arc<Mutex<Option<Vec_>>> + Send + Sync> = add.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            Arc::new(Mutex::new(Some(Vec_ { x: Arc::new(Mutex::new(Some(1.0))), y: Arc::new(Mutex::new(Some(2.0))), ..Default::default() }))),
            Arc::new(Mutex::new(Some(Vec_ { x: Arc::new(Mutex::new(Some(3.0))), y: Arc::new(Mutex::new(Some(4.0))), ..Default::default() })))
        )
    };
    println!("{} {}", format!("{}", { let __v = (*v.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", (*{
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Vec_>>>, Arc<Mutex<Option<f64>>>) -> Arc<Mutex<Option<Vec_>>> + Send + Sync> = scale.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            v.clone(),
            Arc::new(Mutex::new(Some(0.5)))
        )
    }.lock().unwrap().as_ref().unwrap())));
    let mut p = Arc::new(Mutex::new(Some(Vec_ { x: Arc::new(Mutex::new(Some(3.0))), y: Arc::new(Mutex::new(Some(4.0))), ..Default::default() })));
    let mut norm = Arc::new(Mutex::new(Some({ let __recv = p.clone(); Arc::new(move || { (*__recv.lock().unwrap().as_mut().unwrap()).normalize() }) as Arc<dyn Fn() -> () + Send + Sync> })));
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = norm.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    print!("{} {}\n", format!("{:.2}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{:.2}", (*{ let __field = (*p.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", format_slice(&defer_order())));
    let mut pt = Arc::new(Mutex::new(Some(AnonymousStruct1 { name: Arc::new(Mutex::new(Some("a".to_string()))), pos: Arc::new(Mutex::new(Some([1i64, 2i64]))) })));
    let mut other = { let __owned = pt.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };
    { let __idx = (0) as usize; let __rhs = 9; (*(*other.lock().unwrap().as_ref().unwrap()).pos.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {} {}", format!("{}", { let __v = (*pt.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __v = (*other.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }), format!("{}", { let __tmp_x = ({ let __v = (*pt.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); let __tmp_y = ({ let __v = (*other.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }); __tmp_x == __tmp_y }));
    let mut u8: Arc<Mutex<Option<u8>>> = Arc::new(Mutex::new(Some(250)));
    { let __rhs = 10 as u8; let mut guard = u8.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    let mut i8: Arc<Mutex<Option<i8>>> = Arc::new(Mutex::new(Some(127)));
    { let mut guard = i8.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{} {} {} {} {} {} {} {} {} {} {}", format!("{}", { let __v = (*u8.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*i8.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = -(7); let __tmp_y = 2; __tmp_x / __tmp_y }), format!("{}", { let __tmp_x = -(7); let __tmp_y = 2; __tmp_x % __tmp_y }), format!("{}", (3i64)), format!("{}", (-4i64)), format!("{}", (1024i64)), format!("{}", { let __tmp_x = 0xF0; let __tmp_y = 0x3C; __tmp_x & __tmp_y }), format!("{}", { let __tmp_x = 0xF0; let __tmp_y = 0x0F; __tmp_x | __tmp_y }), format!("{}", { let __tmp_x = 0xF0; let __tmp_y = 0xFF; __tmp_x ^ __tmp_y }), format!("{}", { let __tmp_x = 6; let __tmp_y = 3; __tmp_x & ! __tmp_y }));
    let mut u: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(1)));
    println!("{} {} {} {} {} {}", format!("{}", { let __tmp_x = { let __v = (*u.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31; __tmp_x << __tmp_y }), format!("{}", !({ let __v = (*u.lock().unwrap().as_ref().unwrap()); __v })), format!("{}", 9223372036854775807i64), format!("{}", i64::MIN), format!("{}", 4294967295i64), { let __text = go_float_string((math::PI) as f64, false); __text });
    let mut x = Arc::new(Mutex::new(Some(10i64)));
    let mut ptr = x.clone();
    let mut __cell_ptr_1 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: ptr.clone() })));
    let __cell_ptr_1_closure_clone = __cell_ptr_1.clone(); let mut pp = Arc::new(Mutex::new(Some(__ptrSlot_int {
        get: Arc::new(Mutex::new(Some({ let __cell_ptr_1_closure_clone_closure_clone = __cell_ptr_1_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<i64>>> {
        return (*__cell_ptr_1_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<i64>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_ptr_1_closure_clone_closure_clone = __cell_ptr_1_closure_clone.clone(); Arc::new(move |__slot_v1: Arc<Mutex<Option<i64>>>| {
        { let new_val = __slot_v1.clone(); (*__cell_ptr_1_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> }))),
        ..Default::default()
    })));
    { let new_val = 20; let __deref_target = { let __f_holder = (*pp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<i64>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.clone(); *__deref_target.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*(*__cell_ptr_1.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 20; __tmp_x == __tmp_y }));
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub name: Arc<Mutex<Option<String>>>,
    pub pos: Arc<Mutex<Option<[i64; 2]>>>,
}
impl PartialEq for AnonymousStruct1 {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.name.lock().unwrap().as_ref().cloned() }; let __right = { other.name.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.pos.lock().unwrap().as_ref().cloned() }; let __right = { other.pos.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.pos.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            pos: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| 0))));
        Self {
            name: __go_default_0_0,
            pos: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.pos));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct2 {
    pub v: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl GoValueClone for Vec_ {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for __ptrSlot_int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
