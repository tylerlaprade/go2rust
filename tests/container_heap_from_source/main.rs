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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}

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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Task>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Item>() {
        format!("{}", v)
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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Task>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Item>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
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
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
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
    if val.is::<Arc<Mutex<Option<Task>>>>() { return "*main.Task" }
    if val.is::<Item>() { return "main.Item" }
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

/// IntHeap is the min-heap of container/heap's documentation.
#[derive(Debug, Clone, Default)]
pub struct IntHeap(pub Arc<Mutex<Option<Vec<i64>>>>);

impl Display for IntHeap {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


/// Task is an entry of a priority queue that tracks its own position, so
/// heap.Fix can move it after its priority changes.
#[derive(Debug, Clone)]
pub struct Task {
    pub name: Arc<Mutex<Option<String>>>,
    pub priority: Arc<Mutex<Option<i64>>>,
    pub index: Arc<Mutex<Option<i64>>>,
}

impl Task {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.priority.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.index.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            priority: __go_clone_1_0,
            index: __go_clone_2_0,
        }
    }
}


impl Default for Task {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            priority: __go_default_1_0,
            index: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.priority.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.index.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, Default)]
pub struct TaskQueue(pub Arc<Mutex<Option<Vec<Arc<Mutex<Option<Task>>>>>>>);

impl Display for TaskQueue {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice_wrapped(&self.0))
    }
}


/// Item is a value entry: the queue holds copies, not pointers.
#[derive(Debug, Clone)]
pub struct Item {
    pub node: Arc<Mutex<Option<i64>>>,
    pub dist: Arc<Mutex<Option<i64>>>,
}

impl Item {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.node.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.dist.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            node: __go_clone_0_0,
            dist: __go_clone_0_1,
        }
    }
}


impl Default for Item {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            node: __go_default_0_0,
            dist: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.node.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.dist.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone, Default)]
pub struct PQ(pub Arc<Mutex<Option<Vec<Item>>>>);

impl Display for PQ {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_slice(&self.0))
    }
}


#[derive(Debug, Clone)]
pub struct Edge {
    pub to: Arc<Mutex<Option<i64>>>,
    pub w: Arc<Mutex<Option<i64>>>,
}

impl Edge {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.to.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.w.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            to: __go_clone_0_0,
            w: __go_clone_0_1,
        }
    }
}


impl Default for Edge {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            to: __go_default_0_0,
            w: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.to.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl IntHeap {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_y = { let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
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

    pub fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<i64>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "int")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "int")
        }
    })); Arc::new(Mutex::new(Some(IntHeap(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut old = Arc::new(Mutex::new(Some((*self).clone())));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
        let mut x = Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() })));
        { let new_val = IntHeap(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) })))); *self = new_val; };
        return Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = x.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }) as Box<dyn Any + Send + Sync>)));
    }
}

impl sort::r#mod::Interface for IntHeap {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = { let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
            let __tmp_y = { let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() };
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
        if let Some(__other) = other.__go_as_any().downcast_ref::<IntHeap>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct IntHeapPtr(pub Arc<Mutex<Option<IntHeap>>>);

impl std::fmt::Display for IntHeapPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl container_heap::r#mod::Interface for IntHeapPtr {
    fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let __recv_ptr: *mut IntHeap = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut IntHeap };
        let __recv = unsafe { &mut *__recv_ptr };
        IntHeap::pop(__recv)
    }
    fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        let __recv_ptr: *mut IntHeap = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut IntHeap };
        let __recv = unsafe { &mut *__recv_ptr };
        IntHeap::push(__recv, x)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn container_heap::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn container_heap::r#mod::Interface + Send + Sync>
    }
    fn __go_eq_interface(&self, other: &(dyn container_heap::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IntHeapPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl sort::r#mod::Interface for IntHeapPtr {
    fn len(&self) -> i64 {
        let __recv_ptr: *const IntHeap = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap };
        let __recv = unsafe { &*__recv_ptr };
        IntHeap::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_ptr: *const IntHeap = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap };
        let __recv = unsafe { &*__recv_ptr };
        IntHeap::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_ptr: *const IntHeap = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap };
        let __recv = unsafe { &*__recv_ptr };
        IntHeap::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<IntHeapPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl TaskQueue {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    /// Less puts the highest priority first.
    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*(*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).priority.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).priority.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
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
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = j.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
    }

    pub fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        let mut task = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<Task>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.Task")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.Task")
        }
    }).clone();
        { let new_val = { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64; *(*task.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(task.clone()); Arc::new(Mutex::new(Some(TaskQueue(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut old = Arc::new(Mutex::new(Some((*self).clone())));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
        let mut task = { let __seq_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.clone();
        { let __idx = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __rhs = Default::default(); (*{ let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let new_val = -1; *(*task.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = TaskQueue(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__slice_guard); let __low = (0) as usize; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; __seq.reslice3(__low, __high, __max) })))); *self = new_val; };
        return Arc::new(Mutex::new(Some(Box::new(task.clone()) as Box<dyn Any + Send + Sync>)));
    }

    pub fn update(&self, task: Arc<Mutex<Option<Task>>>, priority: Arc<Mutex<Option<i64>>>) {
        { let new_val = priority.lock().unwrap().as_ref().unwrap().clone(); *(*task.lock().unwrap().as_ref().unwrap()).priority.lock().unwrap() = Some(new_val); };
        container_heap::fix(Arc::new(Mutex::new(Some(Box::new(TaskQueuePtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some({ let __selector_holder = (*task.lock().unwrap().as_ref().unwrap()).index.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))));
    }
}

impl sort::r#mod::Interface for TaskQueue {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    /// Less puts the highest priority first.
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*(*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).priority.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).priority.lock().unwrap().as_ref().unwrap());
            __tmp_x > __tmp_y
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
        { let new_val = i.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
        { let new_val = j.lock().unwrap().as_ref().unwrap().clone(); *(*{ let __seq_holder = __self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.lock().unwrap().as_ref().unwrap()).index.lock().unwrap() = Some(new_val); };
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TaskQueue>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TaskQueuePtr(pub Arc<Mutex<Option<TaskQueue>>>);

impl std::fmt::Display for TaskQueuePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl container_heap::r#mod::Interface for TaskQueuePtr {
    fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let __recv_ptr: *mut TaskQueue = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut TaskQueue };
        let __recv = unsafe { &mut *__recv_ptr };
        TaskQueue::pop(__recv)
    }
    fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        let __recv_ptr: *mut TaskQueue = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut TaskQueue };
        let __recv = unsafe { &mut *__recv_ptr };
        TaskQueue::push(__recv, x)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn container_heap::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn container_heap::r#mod::Interface + Send + Sync>
    }
    fn __go_eq_interface(&self, other: &(dyn container_heap::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TaskQueuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl sort::r#mod::Interface for TaskQueuePtr {
    fn len(&self) -> i64 {
        let __recv_ptr: *const TaskQueue = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const TaskQueue };
        let __recv = unsafe { &*__recv_ptr };
        TaskQueue::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_ptr: *const TaskQueue = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const TaskQueue };
        let __recv = unsafe { &*__recv_ptr };
        TaskQueue::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_ptr: *const TaskQueue = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const TaskQueue };
        let __recv = unsafe { &*__recv_ptr };
        TaskQueue::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TaskQueuePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl PQ {
    pub fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }

    pub fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.dist.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.dist.lock().unwrap().as_ref().unwrap());
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

    pub fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        { let new_val = { let __base = self.0.clone(); let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Item>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Item")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Item")
        }
    })); Arc::new(Mutex::new(Some(PQ(Arc::new(Mutex::new(Some(__values))))))) }; *self = new_val.lock().unwrap().take().unwrap_or_default(); };
    }

    pub fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let mut old = Arc::new(Mutex::new(Some((*self).clone())));
        let mut n = Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64)));
        let mut it = Arc::new(Mutex::new(Some({ let __seq_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.__go_value_clone())));
        { let new_val = PQ(Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*old.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); let __source_cap = __slice_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __slice_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__slice_guard); let __low = 0; let __high = ({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y }) as usize; let __max = __source_cap; __seq.reslice3(__low, __high, __max) })))); *self = new_val; };
        return Arc::new(Mutex::new(Some(Box::new({ let __arg_holder = it.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any + Send + Sync>)));
    }
}

impl sort::r#mod::Interface for PQ {
    fn len(&self) -> i64 {
        return { let __slice_holder = self.0.clone(); let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64;
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        return {
            let __tmp_x = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.dist.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*{ let __seq_holder = self.0.clone(); let __seq_index = ({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }.dist.lock().unwrap().as_ref().unwrap());
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
        if let Some(__other) = other.__go_as_any().downcast_ref::<PQ>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct PQPtr(pub Arc<Mutex<Option<PQ>>>);

impl std::fmt::Display for PQPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl container_heap::r#mod::Interface for PQPtr {
    fn pop(&mut self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> {
        let __recv_ptr: *mut PQ = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut PQ };
        let __recv = unsafe { &mut *__recv_ptr };
        PQ::pop(__recv)
    }
    fn push(&mut self, x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) {
        let __recv_ptr: *mut PQ = { let mut __recv_guard = self.0.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut PQ };
        let __recv = unsafe { &mut *__recv_ptr };
        PQ::push(__recv, x)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn container_heap::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn container_heap::r#mod::Interface + Send + Sync>
    }
    fn __go_eq_interface(&self, other: &(dyn container_heap::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PQPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl sort::r#mod::Interface for PQPtr {
    fn len(&self) -> i64 {
        let __recv_ptr: *const PQ = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const PQ };
        let __recv = unsafe { &*__recv_ptr };
        PQ::len(__recv)
    }
    fn less(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) -> bool {
        let __recv_ptr: *const PQ = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const PQ };
        let __recv = unsafe { &*__recv_ptr };
        PQ::less(__recv, i, j)
    }
    fn swap(&self, i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>) {
        let __recv_ptr: *const PQ = { let __recv_guard = self.0.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const PQ };
        let __recv = unsafe { &*__recv_ptr };
        PQ::swap(__recv, i, j)
    }
    fn __go_clone_box_interface(&self) -> Box<dyn sort::r#mod::Interface + Send + Sync> {
        Box::new(self.clone()) as Box<dyn sort::r#mod::Interface + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_interface(&self, other: &(dyn sort::r#mod::Interface + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<PQPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn dijkstra(g: Arc<Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<Vec<Edge>>>>>>>>, src: Arc<Mutex<Option<i64>>>, n: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Vec<i64>>>> {
    let mut dist = Arc::new(Mutex::new(Some(vec![0i64; ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize])));
    for i in 0..(({ let __range_holder = dist.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        { let __idx = (i) as usize; let __rhs = 1152921504606846976; (*dist.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    { let __idx = ({ let __v = (*src.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __rhs = 0; (*dist.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    let mut pq = Arc::new(Mutex::new(Some(PQ(Arc::new(Mutex::new(Some(vec![Item { node: Arc::new(Mutex::new(Some({ let __arg_holder = src.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), dist: Arc::new(Mutex::new(Some(0))), ..Default::default() }])))))));
    while { let __tmp_x = { let __recv = pq.clone(); let __recv_ptr: *const PQ = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const PQ }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut cur = Arc::new(Mutex::new(Some(({
        let val = container_heap::pop(Arc::new(Mutex::new(Some(Box::new(PQPtr(pq.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>)))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Item>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "main.Item")).__go_value_clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "main.Item")
        }
    }))));
        if {
            let __tmp_x = (*{ let __field = (*cur.lock().unwrap().as_ref().unwrap()).dist.clone(); __field }.lock().unwrap().as_ref().unwrap());
            let __tmp_y = { let __seq = { let __seq_holder = dist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[((*{ let __field = (*cur.lock().unwrap().as_ref().unwrap()).node.clone(); __field }.lock().unwrap().as_ref().unwrap())) as usize].clone() };
            __tmp_x > __tmp_y
        } {
        continue
    }
        { let __range_holder = { let __map = { let __map_holder = g.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&{ let __selector_holder = (*cur.lock().unwrap().as_ref().unwrap()).node.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for e in __range_values.iter() {
        let e = &e.__go_value_clone();
        {
        let mut nd = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*cur.lock().unwrap().as_ref().unwrap()).dist.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = (*e.w.lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y })));;
        if {
            let __tmp_x = { let __v = (*nd.lock().unwrap().as_ref().unwrap()).clone(); __v };
            let __tmp_y = { let __seq = { let __seq_holder = dist.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[((*e.to.lock().unwrap().as_ref().unwrap())) as usize].clone() };
            __tmp_x < __tmp_y
        } {
            { let __idx = ((*e.to.lock().unwrap().as_ref().unwrap())) as usize; let __rhs = { let __v = (*nd.lock().unwrap().as_ref().unwrap()).clone(); __v }; (*dist.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };;
            container_heap::push(Arc::new(Mutex::new(Some(Box::new(PQPtr(pq.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some({ let __arg_value = Box::new(Item { node: Arc::new(Mutex::new(Some({ let __selector_holder = e.to.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), dist: Arc::new(Mutex::new(Some({ let __arg_holder = nd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }) as Box<dyn Any + Send + Sync>; __arg_value }))));;
        }
    }
    } }
    }
    return dist.clone();
}

fn main() {
    go_install_panic_exit();
    ::container_heap::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut h = Arc::new(Mutex::new(Some(IntHeap(Arc::new(Mutex::new(Some(vec![5i64, 2i64, 8i64])))))));
    container_heap::init(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))));
    container_heap::push(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>))));
    { let __go_print_arg_1 = { let __recv = h.clone(); let __recv_ptr: *const IntHeap = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __go_print_value = print!("minimum: {}, len {}\n", format!("{}", { let __seq_holder = { let __named_slice = (*h.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __seq_index = (0) as usize; let __seq_guard = __seq_holder.lock().unwrap(); let __seq = __seq_guard.as_ref().unwrap(); __seq[__seq_index].clone() }), format!("{}", __go_print_arg_1)); __go_print_value };
    let mut popped: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    while { let __tmp_x = { let __recv = h.clone(); let __recv_ptr: *const IntHeap = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap }; let __result = unsafe { &*__recv_ptr }.len(); __result }; let __tmp_y = 0; __tmp_x > __tmp_y } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = popped.clone(); let __append_value = (*Arc::new(Mutex::new(Some(format!("{}", format!("{}", { let __text = match (container_heap::pop(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))).lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *popped.lock().unwrap() = __slice_header; };
    }
    println!("{} {}", format!("{}", "popped:".to_string()), format!("{}", (*strings::join(popped.clone(), Arc::new(Mutex::new(Some(" ".to_string())))).lock().unwrap().as_ref().unwrap())));
    let mut h2 = Arc::new(Mutex::new(Some(IntHeap(Arc::new(Mutex::new(Some(vec![9i64, 4i64, 7i64, 1i64, 6i64])))))));
    container_heap::init(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))));
    println!("{} {} {} {}", format!("{}", "removed:".to_string()), format!("{}", { let __text = match (container_heap::remove(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some(2))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", "left:".to_string()), format!("{}", { let __recv = h2.clone(); let __recv_ptr: *const IntHeap = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const IntHeap }; let __result = unsafe { &*__recv_ptr }.len(); __result }));
    { let __idx = (0) as usize; let __rhs = 10; (*{ let __named_slice = (*h2.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    container_heap::fix(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some(0))));
    println!("{} {} {} {} {}", format!("{}", "after fix:".to_string()), format!("{}", { let __text = match (container_heap::pop(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match (container_heap::pop(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match (container_heap::pop(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", { let __text = match (container_heap::pop(Arc::new(Mutex::new(Some(Box::new(IntHeapPtr(h2.clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))))).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    let mut tasks = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("write".to_string(), Arc::new(Mutex::new(Some(3i64)))), ("test".to_string(), Arc::new(Mutex::new(Some(2i64)))), ("ship".to_string(), Arc::new(Mutex::new(Some(4i64))))]))));
    let mut q = Arc::new(Mutex::new(Some(TaskQueue(Arc::new(Mutex::new(Some({ let mut v: Vec<Arc<Mutex<Option<Task>>>> = (0..(({ let __len = (*tasks.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| Arc::new(Mutex::new(None))).collect(); v.truncate((0) as usize); v })))))));
    for name in &vec!["write".to_string(), "test".to_string(), "ship".to_string()] {
        { let new_val = { let __base = { let __named_slice = (*q.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __base_guard = __base.lock().unwrap(); let mut __values = __base_guard.as_ref().cloned().unwrap_or_else(Vec::new); std::mem::drop(__base_guard); __values.push(Arc::new(Mutex::new(Some(Task { name: Arc::new(Mutex::new(Some((*name).clone()))), priority: Arc::new(Mutex::new(Some({ let __map = { let __map_holder = tasks.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(name)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }))), index: Arc::new(Mutex::new(Some({ let __slice_holder = { let __named_slice = (*q.lock().unwrap().as_ref().unwrap()).0.clone(); __named_slice }; let __slice_guard = __slice_holder.lock().unwrap(); __slice_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) } as i64))), ..Default::default() })))); Arc::new(Mutex::new(Some(TaskQueue(Arc::new(Mutex::new(Some(__values))))))) }; let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *q.lock().unwrap() = __moved_val; };
    }
    container_heap::init(Arc::new(Mutex::new(Some(Box::new(TaskQueuePtr(q.clone().clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))));
    let mut review = Arc::new(Mutex::new(Some(Task { name: Arc::new(Mutex::new(Some("review".to_string()))), priority: Arc::new(Mutex::new(Some(1))), ..Default::default() })));
    container_heap::push(Arc::new(Mutex::new(Some(Box::new(TaskQueuePtr(q.clone().clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(review.clone()) as Box<dyn Any + Send + Sync>))));
    (*q.lock().unwrap().as_ref().unwrap()).update(review.clone(), Arc::new(Mutex::new(Some(5))));
    while { let __tmp_x = (*q.lock().unwrap().as_ref().unwrap()).len(); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let mut task = ({
        let val = container_heap::pop(Arc::new(Mutex::new(Some(Box::new(TaskQueuePtr(q.clone().clone())) as Box<dyn container_heap::r#mod::Interface + Send + Sync>)))).clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            any_val.downcast_ref::<Arc<Mutex<Option<Task>>>>().unwrap_or_else(|| panic!("interface conversion: {} is {}, not {}", "interface {}", __go_type_name(&**any_val), "*main.Task")).clone()
        } else {
            panic!("interface conversion: {} is nil, not {}", "interface {}", "*main.Task")
        }
    }).clone();
        print!("{}:{}(index {}) ", format!("{}", (*{ let __field = (*task.lock().unwrap().as_ref().unwrap()).priority.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*task.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*task.lock().unwrap().as_ref().unwrap()).index.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    }
    println!();
    let mut g = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<Vec<Edge>>>>>::from([(0i64, Arc::new(Mutex::new(Some(vec![Edge { to: Arc::new(Mutex::new(Some(1))), w: Arc::new(Mutex::new(Some(4))), ..Default::default() }, Edge { to: Arc::new(Mutex::new(Some(2))), w: Arc::new(Mutex::new(Some(1))), ..Default::default() }])))), (2i64, Arc::new(Mutex::new(Some(vec![Edge { to: Arc::new(Mutex::new(Some(1))), w: Arc::new(Mutex::new(Some(2))), ..Default::default() }, Edge { to: Arc::new(Mutex::new(Some(3))), w: Arc::new(Mutex::new(Some(5))), ..Default::default() }])))), (1i64, Arc::new(Mutex::new(Some(vec![Edge { to: Arc::new(Mutex::new(Some(3))), w: Arc::new(Mutex::new(Some(1))), ..Default::default() }]))))]))));
    println!("{} {}", format!("{}", "dijkstra:".to_string()), format!("{}", format_slice(&dijkstra(g.clone(), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(4)))))));
}

impl GoValueClone for Task {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Item {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Edge {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
