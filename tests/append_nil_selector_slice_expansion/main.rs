#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
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
pub struct invocation {
    pub verb: Arc<Mutex<Option<String>>>,
    pub build_flags: Arc<Mutex<Option<Vec<String>>>>,
}

impl invocation {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.verb.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.build_flags.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            verb: __go_clone_0_0,
            build_flags: __go_clone_1_0,
        }
    }
}


impl Default for invocation {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            verb: __go_default_0_0,
            build_flags: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for invocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.verb.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice(&self.build_flags));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl invocation {
    pub fn run(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut goArgs = Arc::new(Mutex::new(Some(vec![{ let __selector_holder = self.verb.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }])));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = goArgs.clone(); let __append_values: Vec<_> = { let __slice_holder = self.build_flags.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.clone()).unwrap_or_default() }.iter().cloned().collect(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *goArgs.lock().unwrap() = __slice_header; };
        return goArgs.clone();
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut inv = Arc::new(Mutex::new(Some(invocation { verb: Arc::new(Mutex::new(Some("list".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", (*strings::join({ let __recv = inv.clone(); let __recv_ptr: *const invocation = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const invocation }; let __result = unsafe { &*__recv_ptr }.run(); __result }, Arc::new(Mutex::new(Some(",".to_string())))).lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for invocation {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
