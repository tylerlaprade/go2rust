#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display};
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
pub struct Tally(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<i64>>>>>>>>);


impl Tally {
    pub fn add(&self, name: Arc<Mutex<Option<String>>>, value: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        { let __map_key = ({ let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = { let __slice = { let __map_holder = __self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&({ let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = ({ let __v = (*value.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    }

    pub fn replace(&self, old: Arc<Mutex<Option<String>>>, new: Arc<Mutex<Option<String>>>) {
        let mut __self = self.clone();
        {
        let (mut list, mut ok) = { let __map = { let __map_holder = __self.0.clone().clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (v.clone(), true), None => (Default::default(), false) } };
let mut list = { let __slice_header = GoSliceHeader::go_slice_header(&list); Arc::new(Mutex::new(__slice_header)) };;
        if ok {
            { let __map_handle = __self.0.clone(); let mut __map_guard = __map_handle.lock().unwrap(); __map_guard.as_mut().unwrap().remove(&({ let __v = (*old.lock().unwrap().as_ref().unwrap()).clone(); __v })); };;
            { let __map_key = ({ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = { let __slice = { let __map_holder = __self.0.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&({ let __v = (*new.lock().unwrap().as_ref().unwrap()).clone(); __v })).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_values: Vec<_> = { let __slice_holder = list.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) }; (*__self.0.clone().lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
        }
    }
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut t = Arc::new(Mutex::new(Some(Tally(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::new())))))));
    (*t.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some(1))));
    (*t.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some(2))));
    (*t.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some("b".to_string()))), Arc::new(Mutex::new(Some(3))));
    (*t.lock().unwrap().as_ref().unwrap()).replace(Arc::new(Mutex::new(Some("a".to_string()))), Arc::new(Mutex::new(Some("c".to_string()))));
    let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..({ let __named_map_holder = t.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = { let __named_map_holder = t.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()).unwrap_or_else(|| Arc::new(Mutex::new(None))); std::mem::drop(__named_map_guard); __map_holder }; let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    sort::strings(keys.clone());
    { let __range_holder = keys.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter() {
        println!("{} {}", format!("{}", k), format!("{}", format_slice(&{ let __map = { let __map_holder = { let __named_map = (*t.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(k)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    } }
}