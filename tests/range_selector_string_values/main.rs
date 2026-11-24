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

#[derive(Debug, Clone, Default)]
pub struct Command {
    pub args: Arc<Mutex<Option<Vec<String>>>>,
}

impl Command {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.args.clone();
        Self {
            args: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_slice(&self.args));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn quote(s: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    return Arc::new(Mutex::new(Some({
        let mut __s = String::new();
        __s.push_str(&format!("{}", "<".to_string()));
        __s.push_str(&format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        __s.push_str(&format!("{}", ">".to_string()));
        __s
    })));
}

pub fn debug(cmd: Arc<Mutex<Option<Command>>>) -> Arc<Mutex<Option<String>>> {
    let mut args: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = (*cmd.lock().unwrap().as_ref().unwrap()).args.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for arg in __range_values.iter() {
        let mut quoted = quote(Arc::new(Mutex::new(Some(arg.clone()))));
        if { let __tmp_x = (*Arc::new(Mutex::new(Some({ let __s = &((*quoted.lock().unwrap().as_ref().unwrap()).clone()); let __low = (1) as usize; let __high = ({ let __tmp_x = (({ let __len = (*quoted.lock().unwrap().as_ref().unwrap()).len(); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; __s[__low..__high].to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = (*arg).clone(); __tmp_x != __tmp_y } || strings::contains(Arc::new(Mutex::new(Some((*arg).clone()))), Arc::new(Mutex::new(Some(" ".to_string())))) {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = args.clone(); let __append_value = ({ let __v = (*quoted.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *args.lock().unwrap() = __slice_header; };
    } else {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = args.clone(); let __append_value = (*arg).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *args.lock().unwrap() = __slice_header; };
    }
    } }
    return strings::join(args.clone(), Arc::new(Mutex::new(Some(" ".to_string()))));
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", (*debug(Arc::new(Mutex::new(Some(Command { args: Arc::new(Mutex::new(Some(vec!["go".to_string(), "list ./...".to_string()]))), ..Default::default() })))).lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for Command {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
