#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
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

#[derive(Debug, Clone, Default)]
pub struct Stack(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct counter {
    pub n: Arc<Mutex<Option<i64>>>,
}

impl counter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            n: __go_clone_0_0,
        }
    }
}


impl Default for counter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            n: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for counter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Matrix(pub Arc<Mutex<Option<Vec<Vec<f64>>>>>);


impl Stack {
    pub fn push(&mut self, v: Arc<Mutex<Option<i64>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(({ let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v })); Arc::new(Mutex::new(Some(Stack(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> i64 {
        let mut old = Arc::new(Mutex::new(Some((*self).clone())));
        let mut v = Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })));
        { let new_val = Stack(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__slice_guard); let __low = 0; let __high = ({ let __tmp_x = ({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))); *self = new_val; };
        return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
}

impl counter {
    pub fn next(&mut self) -> i64 {
        { let __target = self.n.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        return (*self.n.lock().unwrap().as_ref().unwrap());
    }
}

impl Matrix {
    pub fn same(&self, o: Arc<Mutex<Option<Matrix>>>) -> bool {
        return { let __tmp_x = ({ let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); let __tmp_y = ({ let __slice_holder = { let __named_slice = (*o.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64); __tmp_x == __tmp_y };
    }
}

pub fn eval_r_p_n(expr: Arc<Mutex<Option<String>>>) -> i64 {
    let mut st: Arc<Mutex<Option<Stack>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __range_holder = strings::fields(Arc::new(Mutex::new(Some({ let __arg_holder = expr.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for t in __range_values.iter() {
        { let _switch_val = (*t).clone();
    if _switch_val == ("+".to_string()) || _switch_val == ("*".to_string()) {
            let (mut b, mut a) = ({ let __tuple_value = (*st.lock().unwrap().as_mut().unwrap()).pop(); __tuple_value }, { let __tuple_value = (*st.lock().unwrap().as_mut().unwrap()).pop(); __tuple_value });
            if { let __tmp_x = (*t).clone(); let __tmp_y = "+".to_string(); __tmp_x == __tmp_y } {
        (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some({ let __tmp_x = a; let __tmp_y = b; __tmp_x + __tmp_y }))));
    } else {
        (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some({ let __tmp_x = a; let __tmp_y = b; __tmp_x * __tmp_y }))));
    }
        } else {
            let (mut n, _) = strconv::atoi(Arc::new(Mutex::new(Some((*t).clone()))));
            (*st.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(n))));
        }
    }
    } }
    return (*st.lock().unwrap().as_mut().unwrap()).pop();
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

    println!("{}", format!("{}", eval_r_p_n(Arc::new(Mutex::new(Some("3 4 + 2 *".to_string()))))));
    let mut c = Arc::new(Mutex::new(Some(counter { n: Arc::new(Mutex::new(Some(0))) })));
    let (mut x, mut y, mut z) = ({ let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value }, { let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value }, { let __tuple_value = { let __recv = c.clone(); let __recv_ptr: *mut counter = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut counter }; let __result = unsafe { &mut *__recv_ptr }.next(); __result }; __tuple_value });
    println!("{} {} {}", format!("{}", x), format!("{}", y), format!("{}", z));
    let mut v = Arc::new(Mutex::new(Some(7i64)));
    let (mut p, mut q) = ({ let __tuple_value = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value }, { let __tuple_value = { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) }; __tuple_value });
    println!("{} {}", format!("{}", { let __v = (*p.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*q.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut byLen = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<String>>>>>::from([]))));
    { let __range_holder = strings::fields(Arc::new(Mutex::new(Some("go is fun to use".to_string())))).clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        { let __map_key = w.len() as i64; let __map_value = { let __slice = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&(w.len() as i64)).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = (*w).clone(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*byLen.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{} {}", format!("{}", format_slice(&{ let __map = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&2)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", format_slice(&{ let __map = { let __map_holder = byLen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&3)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    println!("{}", format!("{}", {
        let __recv = Matrix(Arc::new(Mutex::new(Some(vec![vec![1.0]]))));
        let __result = __recv.same(
            Arc::new(Mutex::new(Some(Matrix(Arc::new(Mutex::new(Some(vec![vec![2.0]]))))))),
        );
        __result
    }));
}

impl GoValueClone for counter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
