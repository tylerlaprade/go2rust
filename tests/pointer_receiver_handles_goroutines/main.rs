#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display,
{
    let guard = map.lock().unwrap();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.lock().unwrap();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:{}", go_display_value(k), __nil_text)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
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

struct GoWriteBack<F: FnMut()>(F);

impl<F: FnMut()> Drop for GoWriteBack<F> {
    fn drop(&mut self) {
        (self.0)()
    }
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

pub struct GoLocalPtrKey<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for GoLocalPtrKey<T> {
    fn clone(&self) -> Self { GoLocalPtrKey(self.0.clone()) }
}

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

#[derive(Clone)]
pub struct Srv {
    pub seen: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
    pub wg: WaitGroup,
}

impl Srv {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.seen.clone();
        let __go_clone_1_0 = self.wg.clone();
        Self {
            seen: __go_clone_0_0,
            wg: __go_clone_1_0,
        }
    }
}


impl Default for Srv {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = WaitGroup::new();
        Self {
            seen: __go_default_0_0,
            wg: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Srv {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.seen));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct Node {
    pub name: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Node>>>,
    pub tags: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        let __go_clone_2_0 = self.tags.clone();
        Self {
            name: __go_clone_0_0,
            next: __go_clone_1_0,
            tags: __go_clone_2_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            next: __go_default_1_0,
            tags: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_2 = format!("{}", format_map(&self.tags));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub parts: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
    pub next: Arc<Mutex<Option<Builder>>>,
}

impl Builder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.parts.clone();
        let __go_clone_1_0 = self.next.clone();
        Self {
            parts: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.parts));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct Dept {
    pub name: Arc<Mutex<Option<String>>>,
    pub staff: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Employee>>>>>>>,
    pub lead: Arc<Mutex<Option<Employee>>>,
}

impl Dept {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.staff.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.lead.clone();
        Self {
            name: __go_clone_0_0,
            staff: __go_clone_1_0,
            lead: __go_clone_2_0,
        }
    }
}


impl Default for Dept {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            staff: __go_default_1_0,
            lead: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for Dept {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_slice_wrapped(&self.staff));
        let __go_fmt_2 = format!("{}", { let __guard = self.lead.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct Employee {
    pub name: Arc<Mutex<Option<String>>>,
    pub dept: Arc<Mutex<Option<Dept>>>,
}

impl Employee {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.dept.clone();
        Self {
            name: __go_clone_0_0,
            dept: __go_clone_1_0,
        }
    }
}


impl Default for Employee {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            name: __go_default_0_0,
            dept: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Employee {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.dept.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub trait Registrar: std::fmt::Display + Any {
    fn __go_clone_box_registrar(&self) -> Box<dyn Registrar + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_registrar(&self, other: &(dyn Registrar + Send + Sync)) -> bool;
    fn register(&self);
}

impl Clone for Box<dyn Registrar + Send + Sync> {
    fn clone(&self) -> Self {
        Registrar::__go_clone_box_registrar(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Team {
    pub dept: Arc<Mutex<Option<Dept>>>,
    pub size: Arc<Mutex<Option<i64>>>,
}

impl Team {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.dept.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = { let __guard = self.size.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            dept: __go_clone_0_0,
            size: __go_clone_1_0,
        }
    }
}


impl Default for Team {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Dept::default())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            dept: __go_default_0_0,
            size: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.dept.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.size.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) static registry_1: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Vec<Arc<Mutex<Option<Dept>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *registry_1.lock().unwrap() = Some(Vec::nil());
}


impl Srv {
    pub fn r#loop(&mut self) {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let mut s_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        s_defer_captured.wg.done();
    }));
            { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); self.seen = new_val; };

            // Execute deferred functions
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
    }

    pub fn reset(&mut self) {
        { let new_val = Arc::new(Mutex::new(None)); self.seen = new_val; };
    }

    pub fn start(&mut self) {
        let (mut s, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        { let __sync_value = (*s.lock().unwrap().as_ref().unwrap()).wg.clone(); __sync_value }.add(1);
        let s_thread = s.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        { let __recv = s_thread.clone(); let __recv_ptr = go_receiver_ptr_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.r#loop(); __result };
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let __sync_value = (*s.lock().unwrap().as_ref().unwrap()).wg.clone(); __sync_value }.wait();
    }

    pub fn stop(&mut self) {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

        let (mut s, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let s_defer_captured = s.clone(); __defer_stack.push(Box::new(move || {
        { let __recv = s_defer_captured.clone(); let __recv_ptr = go_receiver_ptr_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.reset(); __result };
    }));

            // Execute deferred functions
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
    }
}

impl Node {
    /// Last walks the list from n and tags the node it ends on.
    pub fn last(&mut self) -> Arc<Mutex<Option<Node>>> {
        let (mut n, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        while { let __nil_target = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); n = new_val; };
    }
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("last".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); (*n.lock().unwrap().as_mut().unwrap()).tags = new_val; };
        return n.clone();
    }

    /// Link appends a node after n and returns n, for chaining.
    pub fn link(&mut self, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Node>>> {
        let (mut n, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        { let new_val = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() }))).clone(); (*n.lock().unwrap().as_mut().unwrap()).next = new_val; };
        return n.clone();
    }
}

impl Builder {
    pub fn add(&mut self, k: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Builder>>> {
        let (mut b, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        if { let __nil_target = (*b.lock().unwrap().as_ref().unwrap()).parts.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))); (*b.lock().unwrap().as_mut().unwrap()).parts = new_val; };
    }
        { let __map_holder = (*b.lock().unwrap().as_ref().unwrap()).parts.clone(); let __key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
        return b.clone();
    }

    pub fn chain(&mut self) -> Arc<Mutex<Option<Builder>>> {
        { let new_val = Arc::new(Mutex::new(Some(Builder { parts: Default::default(), next: Default::default() }))).clone(); self.next = new_val; };
        self.next.clone()
    }
}

impl Dept {
    pub fn hire(&mut self, e: Arc<Mutex<Option<Employee>>>) {
        let (mut d, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        { let new_val = d.clone(); (*e.lock().unwrap().as_mut().unwrap()).dept = new_val; };
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = (*d.lock().unwrap().as_ref().unwrap()).staff.clone(); let __append_value = e.clone(); let mut __appended: Vec<Arc<Mutex<Option<Employee>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *(*d.lock().unwrap().as_ref().unwrap()).staff.lock().unwrap() = __slice_header; };
    }

    pub fn promote(&mut self) -> Arc<Mutex<Option<Dept>>> {
        let (mut d, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        let __self_ptr = self as *mut Self;
        let __self_write_back = GoWriteBack(move || if let Some(__copy) = &__self_copy { unsafe { *__self_ptr = __copy.lock().unwrap().as_ref().unwrap().clone(); } });
        { let new_val = { let __seq = { let __seq_holder = (*d.lock().unwrap().as_ref().unwrap()).staff.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone(); (*d.lock().unwrap().as_mut().unwrap()).lead = new_val; };
        return d.clone();
    }

    pub fn register(&self) {
        let (mut d, __self_copy) = match go_take_receiver_handle::<Self>(self as *const Self as usize) {
            Some(__handle) => (__handle, None),
            None => {
                let __copy = Arc::new(Mutex::new(Some(self.clone())));
                (__copy.clone(), Some(__copy))
            }
        };
        { let new_val = { let __collection_holder = { let __append_target = registry_1.clone(); let __append_value = d.clone(); let mut __appended: Vec<Arc<Mutex<Option<Dept>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }.clone(); let __collection_guard = __collection_holder.lock().unwrap(); (*__collection_guard).clone() }; *registry_1.lock().unwrap() = new_val; };
    }
}

impl Registrar for Dept {
    fn register(&self) {
        Dept::register(self)
    }
    fn __go_clone_box_registrar(&self) -> Box<dyn Registrar + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Registrar + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_registrar(&self, other: &(dyn Registrar + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Dept>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct DeptPtr(pub Arc<Mutex<Option<Dept>>>);

impl std::fmt::Display for DeptPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Registrar for DeptPtr {
    fn register(&self) {
        let __recv_ptr = go_hint_receiver(&self.0);
        let __recv = unsafe { &*__recv_ptr };
        Dept::register(__recv)
    }
    fn __go_clone_box_registrar(&self) -> Box<dyn Registrar + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Registrar + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_registrar(&self, other: &(dyn Registrar + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<DeptPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Team {
    pub fn hire(&mut self, e: Arc<Mutex<Option<Employee>>>) {
        // Forward to embedded type's method
        let embedded = self.dept.clone();
        let __recv_ptr = go_hint_receiver_mut(&embedded);
        let embedded_ref = unsafe { &mut *__recv_ptr };
        embedded_ref.hire(e)
    }

    pub fn promote(&mut self) -> Arc<Mutex<Option<Dept>>> {
        // Forward to embedded type's method
        let embedded = self.dept.clone();
        let __recv_ptr = go_hint_receiver_mut(&embedded);
        let embedded_ref = unsafe { &mut *__recv_ptr };
        embedded_ref.promote()
    }

    pub fn register(&self) {
        // Forward to embedded type's method
        let embedded = self.dept.clone();
        let __recv_ptr = go_hint_receiver(&embedded);
        let embedded_ref = unsafe { &*__recv_ptr };
        embedded_ref.register()
    }
}

impl Registrar for Team {
    fn register(&self) {
        Team::register(self)
    }
    fn __go_clone_box_registrar(&self) -> Box<dyn Registrar + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Registrar + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_registrar(&self, other: &(dyn Registrar + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Team>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct TeamPtr(pub Arc<Mutex<Option<Team>>>);

impl std::fmt::Display for TeamPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Registrar for TeamPtr {
    fn register(&self) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Team::register(__recv)
    }
    fn __go_clone_box_registrar(&self) -> Box<dyn Registrar + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Registrar + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_registrar(&self, other: &(dyn Registrar + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<TeamPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    __go_init_all();
    let mut srv = Arc::new(Mutex::new(Some(Srv { seen: Default::default(), wg: WaitGroup::new() })));
    { let __recv = srv.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.start(); __result };
    println!("{} {}", format!("{}", ({ let __len = ({ let __len_target = { let __field = (*srv.lock().unwrap().as_ref().unwrap()).seen.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })), format!("{}", { let __map = { let __map_holder = (*srv.lock().unwrap().as_ref().unwrap()).seen.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    { let __recv = srv.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.stop(); __result };
    println!("{}", format!("{}", { let __nil_target = (*srv.lock().unwrap().as_ref().unwrap()).seen.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));
    let mut list = Arc::new(Mutex::new(Some(Node {
        name: Arc::new(Mutex::new(Some("a".to_string()))),
        next: Arc::new(Mutex::new(Some(Node {
            name: Arc::new(Mutex::new(Some("b".to_string()))),
            next: Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("c".to_string()))), ..Default::default() }))).clone(),
            ..Default::default()
        }))).clone(),
        ..Default::default()
    })));
    let mut last = { let __recv = list.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.last(); __result };
    println!("{} {} {} {}", format!("{}", (*{ let __field = (*last.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __map = { let __map_holder = (*(*(*list.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).tags.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"last".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", {
        let __left = last.clone();
        let __right = (*(*list.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).next.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id };
        __eq
    }), format!("{}", (*(*(*list.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()));
    let mut head: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __recv = { let __recv = head.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.link(Arc::new(Mutex::new(Some("x".to_string())))); __result }; let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.link(Arc::new(Mutex::new(Some("y".to_string())))); __result };
    println!("{}", format!("{}", (*(*(*head.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()));
    let mut bld: Arc<Mutex<Option<Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let __recv = { let __recv = { let __recv = bld.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some("x".to_string())))); __result }; let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some("y".to_string())))); __result }; let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some("x".to_string())))); __result };
    let mut keys = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    for (k, _) in { let __range_holder = (*bld.lock().unwrap().as_ref().unwrap()).parts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    sort::strings(keys.clone());
    println!("{} {}", format!("{}", format_slice(&keys)), format!("{}", { let __map = { let __map_holder = (*bld.lock().unwrap().as_ref().unwrap()).parts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"x".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    { let __recv = (*bld.lock().unwrap().as_mut().unwrap()).chain(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some("deep".to_string())))); __result };
    println!("{}", format!("{}", {
        let __go_cond_0 = { let __nil_target = (*bld.lock().unwrap().as_ref().unwrap()).next.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result };
        if __go_cond_0 {
            let __go_cond_1 = {
                let __tmp_x = { let __map = { let __map_holder = (*(*bld.lock().unwrap().as_ref().unwrap()).next.lock().unwrap().as_ref().unwrap()).parts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"deep".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
                let __tmp_y = 1;
                __tmp_x == __tmp_y
            };
            __go_cond_1
        } else {
            false
        }
    }));
    let mut eng = Arc::new(Mutex::new(Some(Dept { name: Arc::new(Mutex::new(Some("eng".to_string()))), ..Default::default() })));
    for e in &vec![Arc::new(Mutex::new(Some(Employee { name: Arc::new(Mutex::new(Some("ann".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(Employee { name: Arc::new(Mutex::new(Some("bob".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(Employee { name: Arc::new(Mutex::new(Some("cat".to_string()))), ..Default::default() })))] {
        { let __recv = eng.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.hire(e.clone()); __result };
    }
    let mut byDept = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Dept>, Arc<Mutex<Option<Vec<String>>>>>::from([]))));
    { let __range_holder = (*eng.lock().unwrap().as_ref().unwrap()).staff.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for e in __range_values.iter() {
        { let __map_key = GoLocalPtrKey::new((*e.lock().unwrap().as_ref().unwrap()).dept.clone()); let __map_value = { let __slice = { let __map_holder = byDept.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&GoLocalPtrKey::new((*e.lock().unwrap().as_ref().unwrap()).dept.clone())).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = { let __selector_holder = (*e.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*byDept.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
    println!("{} {}", format!("{}", format_slice(&{ let __map = { let __map_holder = byDept.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(eng.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })), format!("{}", ({ let __len = (*byDept.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    println!("{} {}", format!("{}", (*(*(*{ let __recv = eng.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.promote(); __result }.lock().unwrap().as_ref().unwrap()).lead.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", { let __nil_target = (*eng.lock().unwrap().as_ref().unwrap()).lead.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_some(); __nil_result }));
    let mut r: Arc<Mutex<Option<Box<dyn Registrar + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(DeptPtr(eng.clone())) as Box<dyn Registrar + Send + Sync>)));
    (*r.lock().unwrap().as_ref().unwrap()).register();
    let mut register = Arc::new(Mutex::new(Some({ let __recv = eng.clone(); Arc::new(move || { let __recv_ptr = go_hint_receiver_mut(&__recv); unsafe { &mut *__recv_ptr }.register() }) as Arc<dyn Fn() -> () + Send + Sync> })));
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = register.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    println!("{} {} {}", format!("{}", ({ let __len = (*registry_1.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", {
        let __left = { let __seq = { let __seq_holder = registry_1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        let __right = eng.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id };
        __eq
    }), format!("{}", {
        let __left = { let __seq = { let __seq_holder = registry_1.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() };
        let __right = eng.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id };
        __eq
    }));
    let mut registered = Arc::new(Mutex::new(Some(BTreeMap::<GoLocalPtrKey<Dept>, Arc<Mutex<Option<i64>>>>::from([]))));
    { let __range_holder = registry_1.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for d in __range_values.iter() {
        { let __map_holder = registered.clone(); let __key = GoLocalPtrKey::new(d.clone()); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    } }
    println!("{} {}", format!("{}", ({ let __len = (*registered.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map = { let __map_holder = registered.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoLocalPtrKey::new(eng.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut team = {
        let __owner = Arc::new(Mutex::new(Some(Team { dept: Arc::new(Mutex::new(Some(Dept { name: Arc::new(Mutex::new(Some("ops".to_string()))), ..Default::default() }))), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().dept.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = team.clone(); let __recv_ptr: *mut Team = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Team }; let __result = unsafe { &mut *__recv_ptr }.hire(Arc::new(Mutex::new(Some(Employee { name: Arc::new(Mutex::new(Some("dan".to_string()))), ..Default::default() })))); __result };
    println!("{} {}", format!("{}", (*(*(*{ let __recv = team.clone(); let __recv_ptr: *mut Team = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Team }; let __result = unsafe { &mut *__recv_ptr }.promote(); __result }.lock().unwrap().as_ref().unwrap()).lead.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", {
        let __left = (*(*(*team.lock().unwrap().as_mut().unwrap()).dept.lock().unwrap().as_mut().unwrap()).lead.lock().unwrap().as_ref().unwrap()).dept.clone();
        let __right = (*team.lock().unwrap().as_ref().unwrap()).dept.clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right) || { let __left_id = __left.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); let __right_id = __right.lock().unwrap().as_ref().map(|__v| Arc::as_ptr(&__v.name) as *const () as usize); __left_id.is_some() && __left_id == __right_id };
        __eq
    }));
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
    });
}


impl GoValueClone for Srv {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Builder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Dept {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Employee {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Team {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
