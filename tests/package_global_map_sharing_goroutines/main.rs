#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
where
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Arc<Mutex<Option<V>>>)>> = { let guard = map.lock().unwrap(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.lock().unwrap(); v_guard.clone() };
                if let Some(ref val) = value {
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

#[derive(Debug, Clone, Default)]
pub struct Server {
    pub routes: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}

impl Server {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.routes.clone();
        Self {
            routes: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.routes));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_map_string_int {
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_map_string_int {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_map_string_int {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub(crate) struct GoGlobalMapSync<T> {
    slot: std::sync::Mutex<std::sync::Arc<std::sync::Mutex<Option<T>>>>,
}
// A lock of the map, which keeps the map's handle while it is held.
pub(crate) struct GoGlobalMapGuard<T: 'static> {
    guard: std::sync::MutexGuard<'static, Option<T>>,
    _handle: std::sync::Arc<std::sync::Mutex<Option<T>>>,
}
impl<T: 'static> std::ops::Deref for GoGlobalMapGuard<T> {
    type Target = Option<T>;
    fn deref(&self) -> &Option<T> { &self.guard }
}
impl<T: 'static> std::ops::DerefMut for GoGlobalMapGuard<T> {
    fn deref_mut(&mut self) -> &mut Option<T> { &mut self.guard }
}
impl<T: 'static> GoGlobalMapSync<T> {
    pub(crate) fn new() -> Self {
        Self { slot: std::sync::Mutex::new(std::sync::Arc::new(std::sync::Mutex::new(None))) }
    }
    pub(crate) fn clone(&self) -> std::sync::Arc<std::sync::Mutex<Option<T>>> {
        self.slot.lock().unwrap().clone()
    }
    // Assigning a map to the variable makes it that map's handle.
    pub(crate) fn set(&self, handle: std::sync::Arc<std::sync::Mutex<Option<T>>>) {
        *self.slot.lock().unwrap() = handle;
    }
    pub(crate) fn lock(&self) -> std::sync::LockResult<GoGlobalMapGuard<T>> {
        let handle = self.clone();
        // The guard keeps the handle, and is dropped before it.
        let mutex: &'static std::sync::Mutex<Option<T>> = unsafe { &*std::sync::Arc::as_ptr(&handle) };
        match mutex.lock() {
            Ok(guard) => Ok(GoGlobalMapGuard { guard, _handle: handle }),
            Err(poisoned) => Err(std::sync::PoisonError::new(GoGlobalMapGuard { guard: poisoned.into_inner(), _handle: handle })),
        }
    }
}

pub(crate) static routes: std::sync::LazyLock<GoGlobalMapSync<BTreeMap<String, Arc<Mutex<Option<i64>>>>>> = std::sync::LazyLock::new(GoGlobalMapSync::new);

pub(crate) static cache: std::sync::LazyLock<GoGlobalMapSync<BTreeMap<String, Arc<Mutex<Option<String>>>>>> = std::sync::LazyLock::new(GoGlobalMapSync::new);

pub(crate) static counts: std::sync::LazyLock<GoGlobalMapSync<BTreeMap<String, Arc<Mutex<Option<i64>>>>>> = std::sync::LazyLock::new(GoGlobalMapSync::new);

pub(crate) static loaded: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<i64>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *routes.lock().unwrap() = Some(BTreeMap::new());
    *cache.lock().unwrap() = Some(BTreeMap::new());
    *counts.lock().unwrap() = Some(BTreeMap::new());
    *loaded.lock().unwrap() = Some(0);
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<i64>>>>::new();
        *routes.lock().unwrap() = Some(__go_map);
    }
    {
        let mut __go_map = BTreeMap::<String, Arc<Mutex<Option<i64>>>>::new();
        __go_map.insert("a".to_string(), Arc::new(Mutex::new(Some(1))));
        __go_map.insert("b".to_string(), Arc::new(Mutex::new(Some(2))));
        *counts.lock().unwrap() = Some(__go_map);
    }
}


pub fn register(name: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i64>>>) {
    { let __map_key = ({ let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some((*n.lock().unwrap().as_ref().unwrap()).clone()))); (*routes.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn add(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, k: Arc<Mutex<Option<String>>>) {
    { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some(({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn load() -> (Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>, i64) {
    (
        Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("l".to_string(), Arc::new(Mutex::new(Some(9i64))))])))),
        2
    )
}

pub fn keys(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut out = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out.lock().unwrap() = __slice_header; };
    }
    sort::strings(out.clone());
    return out.clone();
}

fn __go_init_0() {
    { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::new()))).clone(); cache.set(new_val); };
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    __go_init_all();
    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        register(Arc::new(Mutex::new(Some("g".to_string()))), Arc::new(Mutex::new(Some(7))));;
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();
    { let __map_handle = routes.clone(); let __delete_key = &"g".to_string(); let mut __map_guard = __map_handle.lock().unwrap(); if let Some(__map) = __map_guard.as_mut() { __map.remove(__delete_key); } };
    register(Arc::new(Mutex::new(Some("x".to_string()))), Arc::new(Mutex::new(Some(1))));
    add(routes.clone(), Arc::new(Mutex::new(Some("w".to_string()))));
    let mut s = Arc::new(Mutex::new(Some(Server { routes: routes.clone(), ..Default::default() })));
    { let __map_key = "y".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*(*s.lock().unwrap().as_ref().unwrap()).routes.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", format_slice(&keys(routes.clone()))), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*s.lock().unwrap().as_ref().unwrap()).routes.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    { let __map_key = "k".to_string(); let __map_value = Arc::new(Mutex::new(Some("v".to_string()))); (*cache.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    let mut c = cache.clone();
    { let __map_key = "j".to_string(); let __map_value = Arc::new(Mutex::new(Some("w".to_string()))); (*c.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", ({ let __len = (*cache.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map = { let __map_holder = cache.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"j".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) }));
    let mut old = counts.clone();
    let mut local = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("m".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    { let new_val = local.clone(); counts.set(new_val); };
    { let __map_key = "n".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*local.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", format_slice(&keys(counts.clone()))), format!("{}", format_slice(&keys(old.clone()))));
    { let new_val = Arc::new(Mutex::new(None)); counts.set(new_val); };
    println!("{} {} {}", format!("{}", { let __nil_result = (*counts.lock().unwrap()).is_none(); __nil_result }), format!("{}", ({ let __len = (*local.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", ({ let __len = (*old.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))).clone(); counts.set(new_val); };
    { let __map_key = "z".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*counts.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", format_slice(&keys(counts.clone()))), format!("{}", format_slice(&keys(local.clone()))));
    let mut p = Arc::new(Mutex::new(Some(__ptrSlot_map_string_int {
        get: Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        routes.clone()
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync>))),
        set: Arc::new(Mutex::new(Some(Arc::new(move |__slot_v1: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v1.clone(); routes.set(new_val); };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync>))),
        ..Default::default()
    })));
    { let __map_key = "z".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*({ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", ({ let __len = (*routes.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map = { let __map_holder = routes.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"z".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    { let __map_handle = routes.clone(); let __delete_key = &"x".to_string(); let mut __map_guard = __map_handle.lock().unwrap(); if let Some(__map) = __map_guard.as_mut() { __map.remove(__delete_key); } };
    println!("{} {}", format!("{}", format_slice(&keys(routes.clone()))), format!("{}", format_slice(&keys({ let __field = (*s.lock().unwrap().as_ref().unwrap()).routes.clone(); __field }))));
    let mut before = counts.clone();
    {
        let __tmp_0 = Arc::new(Mutex::new(None));
        let __tmp_1 = 0;
        counts.set(__tmp_0);
        *loaded.lock().unwrap() = Some(__tmp_1);
    };
    println!("{} {} {}", format!("{}", format_slice(&keys(before.clone()))), format!("{}", { let __nil_result = (*counts.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __v = (*loaded.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    {
        let __tmp_0 = local.clone();
        let __tmp_1 = 1;
        counts.set(__tmp_0);
        *loaded.lock().unwrap() = Some(__tmp_1);
    };
    { let __map_key = "o".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*local.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", format_slice(&keys(counts.clone()))), format!("{}", { let __v = (*loaded.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    { let (__tmp_0, __tmp_1) = load(); counts.set(__tmp_0); *loaded.lock().unwrap() = Some(__tmp_1); };
    { let __map_key = "p".to_string(); let __map_value = Arc::new(Mutex::new(Some(4))); (*counts.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __go_print_arg_1 = keys(counts.clone());
let mut __go_print_arg_1 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_1); Arc::new(Mutex::new(__slice_header)) }; let __go_print_arg_2 = keys(local.clone());
let mut __go_print_arg_2 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_2); Arc::new(Mutex::new(__slice_header)) }; let __go_print_value = println!("{} {} {}", format!("{}", format_slice(&__go_print_arg_1)), format!("{}", { let __v = (*loaded.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&__go_print_arg_2))); __go_print_value };
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        self::__go_init_globals();
        self::__go_init_0();
    });
}


impl GoValueClone for Server {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for __ptrSlot_map_string_int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
