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


// Go expands a pointer to a struct only when the pointer is the value being
// printed. Inside a struct or a slice it prints the address. That is also
// what keeps a value that points back to itself from printing forever.
fn go_format_any_pointer(nested: bool, address: *const (), is_nil: impl FnOnce() -> bool, expand: impl FnOnce() -> String) -> String {
    if !nested {
        return expand();
    }
    if is_nil() { "<nil>".to_string() } else { format!("{:p}", address) }
}

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, false)
}

// Prints a value held inside the one being printed: a struct's interface
// field, an element of a []interface{}.
fn format_any_nested(value: &(dyn Any + Send + Sync)) -> String {
    format_any_at(value, true)
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
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Node>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Point>>>>() {
        go_format_any_pointer(nested, Arc::as_ptr(v) as *const (), || v.try_lock().map(|__guard| __guard.is_none()).unwrap_or(false), || match (*v.lock().unwrap()).as_ref() { Some(__pointee) => format!("&{}", __pointee), None => "<nil>".to_string() })
    } else if let Some(v) = value.downcast_ref::<Label>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Node>() {
        format!("{}", v)
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

fn format_any_slice_values(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>, format_value: fn(&(dyn Any + Send + Sync)) -> String) -> String {
    let guard = slice.lock().unwrap();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.iter().map(|v| format_value(v.as_ref())).collect();
        formatted.join(" ")
    } else {
        String::new()
    }
}

fn format_any_slice(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format!("[{}]", format_any_slice_values(slice, format_any_nested))
}

fn format_any_variadic(slice: &Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>>) -> String {
    format_any_slice_values(slice, format_any)
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
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Node>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Arc<Mutex<Option<Point>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Label>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    if let Some(v) = value.downcast_ref::<Node>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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

/// Node can point at itself through its interface field.
#[derive(Clone)]
pub struct Node {
    pub name: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", match self.next.lock().unwrap().as_ref() { Some(__v) => format_any_nested(__v.as_ref()), None => "<nil>".to_string() });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Point {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// Label prints itself by printing another value. That is a print call of its
/// own, so the pointer it prints is expanded wherever the Label sits.
#[derive(Debug, Clone)]
pub struct Label {
    pub x: Arc<Mutex<Option<i64>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
        }
    }
}


impl Default for Label {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl Label {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        render(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some({ let __selector_holder = self.x.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>))))
    }
}

pub fn render(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{}", { let __text = match v.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }))))
}

/// shape describes text around the addresses in it, which change from run to
/// run: whether it starts with prefix, and how many addresses it holds.
pub fn shape(text: Arc<Mutex<Option<String>>>, prefix: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    Arc::new(Mutex::new(Some(format!("{} {}", format!("{}", strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = prefix.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))), format!("{}", { let __tmp_x = (({ let __len = (*strings::split(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("0x".to_string())))).lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64); let __tmp_y = 1; __tmp_x - __tmp_y })))))
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut done = GoChannel::<String>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send("started".to_string());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
    let mut r#loop = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("loop".to_string()))), ..Default::default() })));
    { let new_val = Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>; *(*r#loop.lock().unwrap().as_ref().unwrap()).next.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", "pointer, boxed:".to_string()), format!("{}", (*shape(
        render(Arc::new(Mutex::new(Some(Box::new(r#loop.clone()) as Box<dyn Any + Send + Sync>)))),
        Arc::new(Mutex::new(Some("&{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "pointer, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", { let __text = match r#loop.lock().unwrap().as_ref() { Some(__value) => format!("&{}", __value), None => "<nil>".to_string() }; __text })))),
        Arc::new(Mutex::new(Some("&{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, boxed:".to_string()), format!("{}", (*shape(
        render(Arc::new(Mutex::new(Some(Box::new({ let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).clone(); __v }) as Box<dyn Any + Send + Sync>)))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "struct value, typed:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", { let __v = (*r#loop.lock().unwrap().as_ref().unwrap()).clone(); __v })))),
        Arc::new(Mutex::new(Some("{loop 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    let mut tail = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("tail".to_string()))), ..Default::default() })));
    let mut text = render(Arc::new(Mutex::new(Some(Box::new(Node { name: Arc::new(Mutex::new(Some("holder".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(tail.clone()) as Box<dyn Any + Send + Sync>))), ..Default::default() }) as Box<dyn Any + Send + Sync>))));
    println!("{} {} {}", format!("{}", "struct holding a pointer:".to_string()), format!("{}", (*shape(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("{holder 0x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = text.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("tail".to_string()))))));
    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("end".to_string()))), ..Default::default() }));
    println!("{}", format!("{}", Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }));
    println!("{}", format!("{}", (*render(Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("row".to_string()))), next: Arc::new(Mutex::new(Some(Box::new(Label { x: Arc::new(Mutex::new(Some(7))), ..Default::default() }) as Box<dyn Any + Send + Sync>))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut labels: Arc<Mutex<Option<Vec<Box<dyn Any + Send + Sync>>>>> = Arc::new(Mutex::new(None));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = labels.clone(); let __append_values = vec![
        Box::new(Label { x: Arc::new(Mutex::new(Some(8))), ..Default::default() }) as Box<dyn Any + Send + Sync>,
        Box::new(Arc::new(Mutex::new(Some(Point { x: Arc::new(Mutex::new(Some(2))), ..Default::default() }))).clone()) as Box<dyn Any + Send + Sync>,
        Box::new(1i64) as Box<dyn Any + Send + Sync>,
    ]; let mut __appended: Vec<Box<dyn Any + Send + Sync>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) })); *labels.lock().unwrap() = __slice_header; };
    println!("{} {}", format!("{}", "slice:".to_string()), format!("{}", (*shape(
        Arc::new(Mutex::new(Some(format!("{}", format_any_slice(&labels))))),
        Arc::new(Mutex::new(Some("[&{8} 0x".to_string())))
    ).lock().unwrap().as_ref().unwrap())));
    println!("{}", format_any_variadic(&labels));
}

impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Label {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
