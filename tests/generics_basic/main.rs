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
pub struct List__int {
    pub head: Arc<Mutex<Option<element__int>>>,
    pub tail: Arc<Mutex<Option<element__int>>>,
}

impl List__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.head.clone();
        let __go_clone_0_1 = self.tail.clone();
        Self {
            head: __go_clone_0_0,
            tail: __go_clone_0_1,
        }
    }
}

impl std::fmt::Display for List__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.tail.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct element__int {
    pub next: Arc<Mutex<Option<element__int>>>,
    pub val: Arc<Mutex<Option<i64>>>,
}

impl element__int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.next.clone();
        let __go_clone_1_0 = { let __guard = self.val.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            next: __go_clone_0_0,
            val: __go_clone_1_0,
        }
    }
}


impl Default for element__int {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            next: __go_default_0_0,
            val: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for element__int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.val.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl List__int {
    pub fn push(&mut self, v: Arc<Mutex<Option<i64>>>) {
        if { let __nil_target = self.tail.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(element__int { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); self.head = new_val; };
        { let new_val = self.head.clone(); self.tail = new_val; };
    } else {
        { let new_val = Arc::new(Mutex::new(Some(element__int { val: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); (*self.tail.lock().unwrap().as_mut().unwrap()).next = new_val; };
        { let new_val = (*self.tail.lock().unwrap().as_ref().unwrap()).next.clone(); self.tail = new_val; };
    }
    }
}

pub fn map_keys__int__string(m: Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<String>>>>>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut r = Arc::new(Mutex::new(Some({ let mut v: Vec<i64> = (0..(({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| 0i64).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = r.clone(); let __append_value = k; let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *r.lock().unwrap() = __slice_header; };
    }
    return r.clone();
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<String>>>>::from([(1i64, Arc::new(Mutex::new(Some("2".to_string())))), (2i64, Arc::new(Mutex::new(Some("4".to_string())))), (4i64, Arc::new(Mutex::new(Some("8".to_string()))))]))));
    let mut keys = map_keys__int__string(m.clone());
    let mut keys = { let __slice_header = GoSliceHeader::go_slice_header(&keys); Arc::new(Mutex::new(__slice_header)) };
    sort::ints(keys.clone());
    println!("{} {}", format!("{}", "keys:".to_string()), format!("{}", format_slice(&keys)));
    let mut lst = Arc::new(Mutex::new(Some(List__int { head: Default::default(), tail: Default::default() })));
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(10))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(13))));
    (*lst.lock().unwrap().as_mut().unwrap()).push(Arc::new(Mutex::new(Some(23))));
}

impl GoValueClone for List__int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for element__int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
