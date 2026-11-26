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
pub struct byLen(pub Arc<Mutex<Option<Vec<String>>>>);

impl Display for byLen {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


impl byLen {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            let __tmp_y = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            __tmp_x < __tmp_y
        };
    }

    pub fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
}

impl sort::r#mod::Interface for byLen {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            let __tmp_y = ({ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.len() as i64);
            __tmp_x < __tmp_y
        };
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let mut __self = self.clone();
        {
            let __go_lhs_sequence_0 = __self.0.clone().clone();
            let __go_lhs_index_0 = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __go_lhs_sequence_1 = __self.0.clone().clone();
            let __go_lhs_index_1 = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize;
            let __tmp_0 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_1 = { let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            (*__go_lhs_sequence_0.lock().unwrap().as_mut().unwrap())[__go_lhs_index_0] = __tmp_0;
            (*__go_lhs_sequence_1.lock().unwrap().as_mut().unwrap())[__go_lhs_index_1] = __tmp_1;
        };
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byLen>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct byLenPtr(pub Arc<Mutex<Option<byLen>>>);

impl std::fmt::Display for byLenPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl sort::r#mod::Interface for byLenPtr {
    fn len(&self) -> i64 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        byLen::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<byLenPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut nums = Arc::new(Mutex::new(Some(vec![5i64, 2i64, 8i64, 1i64])));
    sort::sort(sort::reverse(Arc::new(Mutex::new(Some(Box::new(sort::r#mod::IntSlice(nums.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());
    println!("{}", format!("{}", format_slice(&nums)));
    let mut words = Arc::new(Mutex::new(Some(vec!["ccc".to_string(), "a".to_string(), "bb".to_string()])));
    sort::sort(sort::reverse(Arc::new(Mutex::new(Some(Box::new(byLen(words.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());
    println!("{}", format!("{}", format_slice(&words)));
    let mut strs = Arc::new(Mutex::new(Some(vec!["b".to_string(), "c".to_string(), "a".to_string()])));
    sort::sort(Arc::new(Mutex::new(Some(Box::new(sort::r#mod::StringSlice(strs.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
    println!("{} {}", format!("{}", format_slice(&strs)), format!("{}", sort::is_sorted(Arc::new(Mutex::new(Some(Box::new(sort::r#mod::StringSlice(strs.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))))));
    let mut fs = Arc::new(Mutex::new(Some(sort::r#mod::Float64Slice(Arc::new(Mutex::new(Some(vec![2.5, 1.5, 2.0])))))));
    (*fs.lock().unwrap().as_ref().unwrap()).sort();
    println!("{} {} {} {}", format!("{}", format_slice(&{ let __named_slice = (*fs.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", (*fs.lock().unwrap().as_ref().unwrap()).len()), format!("{}", (*fs.lock().unwrap().as_ref().unwrap()).search(Arc::new(Mutex::new(Some(2.0))))), format!("{}", sort::search_ints(Arc::new(Mutex::new(Some(vec![1i64, 3i64, 5i64]))), Arc::new(Mutex::new(Some(4))))));
    let mut is = Arc::new(Mutex::new(Some(sort::r#mod::IntSlice(Arc::new(Mutex::new(Some(vec![3i64, 1i64, 2i64])))))));
    sort::stable(sort::reverse(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = is.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());
    println!("{} {}", format!("{}", format_slice(&{ let __named_slice = (*is.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice })), format!("{}", sort::is_sorted(Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = is.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn sort::r#mod::Interface + Send + Sync>))))));
    let mut ws = Arc::new(Mutex::new(Some(vec!["bb".to_string(), "a".to_string(), "cc".to_string(), "d".to_string(), "ee".to_string(), "f".to_string()])));
    sort::stable(Arc::new(Mutex::new(Some(Box::new(byLen(ws.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>))));
    println!("{}", format!("{}", format_slice(&ws)));
    let mut done = GoChannel::<Vec<i64>>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut v = Arc::new(Mutex::new(Some(vec![4i64, 9i64, 1i64])));;
        sort::sort(sort::reverse(Arc::new(Mutex::new(Some(Box::new(sort::r#mod::IntSlice(v.clone())) as Box<dyn sort::r#mod::Interface + Send + Sync>)))).clone());;
        done_thread.send(v.lock().unwrap().as_ref().unwrap().clone());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", format_slice_values(&done.recv().unwrap_or_default())));
}