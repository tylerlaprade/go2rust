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
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Arc<Mutex<Option<V>>>)>> = { let guard = map.lock().unwrap(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
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

// A readers-writer lock: any number of readers, or one writer. A clone is
// the same lock, as a *sync.RWMutex is. As in Go, a writer waiting for the
// lock holds off readers that come after it, and the readers that were
// waiting when a writer unlocks are admitted before the next writer.
#[derive(Clone, Debug, Default)]
pub struct GoRWMutex {
    inner: std::sync::Arc<(std::sync::Mutex<GoRWMutexState>, std::sync::Condvar)>,
}

#[derive(Debug, Default)]
pub struct GoRWMutexState {
    writer: bool,
    readers: usize,
    waiting_writers: usize,
    // A reader that has to wait takes a ticket. A writer's unlock admits
    // every ticket issued so far, and the next writer waits until the
    // admitted readers have taken the lock.
    waiting_readers: usize,
    next_ticket: u64,
    admit_below: u64,
    admitted_waiting: usize,
}

impl GoRWMutex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        held.waiting_writers += 1;
        while held.writer || held.readers > 0 || held.admitted_waiting > 0 {
            held = ready.wait(held).unwrap();
        }
        held.waiting_writers -= 1;
        held.writer = true;
    }

    pub fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if !held.writer {
            panic!("sync: Unlock of unlocked RWMutex");
        }
        held.writer = false;
        held.admit_below = held.next_ticket;
        held.admitted_waiting = held.waiting_readers;
        ready.notify_all();
    }

    pub fn r_lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if !held.writer && held.waiting_writers == 0 {
            held.readers += 1;
            return;
        }
        let ticket = held.next_ticket;
        held.next_ticket += 1;
        held.waiting_readers += 1;
        while held.writer || (held.waiting_writers > 0 && ticket >= held.admit_below) {
            held = ready.wait(held).unwrap();
        }
        held.waiting_readers -= 1;
        if ticket < held.admit_below {
            held.admitted_waiting -= 1;
        }
        held.readers += 1;
    }

    pub fn r_unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if held.readers == 0 {
            panic!("sync: RUnlock of unlocked RWMutex");
        }
        held.readers -= 1;
        if held.readers == 0 {
            ready.notify_all();
        }
    }
}

impl std::fmt::Display for GoRWMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RWMutex")
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

#[derive(Clone)]
pub struct Table {
    pub r_w_mutex: GoRWMutex,
    pub m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}

impl Table {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r_w_mutex.clone();
        let __go_clone_1_0 = self.m.clone();
        Self {
            r_w_mutex: __go_clone_0_0,
            m: __go_clone_1_0,
        }
    }
}


impl Default for Table {
    fn default() -> Self {
        let __go_default_0_0 = GoRWMutex::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            r_w_mutex: __go_default_0_0,
            m: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.m));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Registry {
    pub table: Arc<Mutex<Option<Table>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl Registry {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.table.clone();
        let __go_clone_1_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            table: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for Registry {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            table: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.table.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Both embeds Table, which has its own mutex, and a mutex of its own: Go
/// picks the shallower one, Both's own.
#[derive(Clone)]
pub struct Both {
    pub table: Arc<Mutex<Option<Table>>>,
    pub r_w_mutex: GoRWMutex,
}

impl Both {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.table.clone();
        let __go_clone_1_0 = self.r_w_mutex.clone();
        Self {
            table: __go_clone_0_0,
            r_w_mutex: __go_clone_1_0,
        }
    }
}


impl Default for Both {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = GoRWMutex::new();
        Self {
            table: __go_default_0_0,
            r_w_mutex: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Both {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.table.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


/// Deep's mutex is two levels below Outer and Shallow's is one, so Go
/// promotes Outer.Lock from Shallow.
#[derive(Clone)]
pub struct Deep {
    pub r_w_mutex: GoRWMutex,
}

impl Deep {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r_w_mutex.clone();
        Self {
            r_w_mutex: __go_clone_0_0,
        }
    }
}


impl Default for Deep {
    fn default() -> Self {
        let __go_default_0_0 = GoRWMutex::new();
        Self {
            r_w_mutex: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Deep {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


#[derive(Clone, Default)]
pub struct Middle {
    pub deep: Arc<Mutex<Option<Deep>>>,
}

impl Middle {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.deep.clone();
        Self {
            deep: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Middle {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.deep.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Shallow {
    pub r_w_mutex: GoRWMutex,
    pub n: Arc<Mutex<Option<i64>>>,
}

impl Shallow {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r_w_mutex.clone();
        let __go_clone_1_0 = { let __guard = self.n.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r_w_mutex: __go_clone_0_0,
            n: __go_clone_1_0,
        }
    }
}


impl Default for Shallow {
    fn default() -> Self {
        let __go_default_0_0 = GoRWMutex::new();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r_w_mutex: __go_default_0_0,
            n: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Shallow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.n.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct Outer {
    pub middle: Arc<Mutex<Option<Middle>>>,
    pub shallow: Arc<Mutex<Option<Shallow>>>,
}

impl Outer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.middle.clone();
        let __go_clone_1_0 = self.shallow.clone();
        Self {
            middle: __go_clone_0_0,
            shallow: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Outer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.middle.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", { let __guard = self.shallow.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


/// Picky's own mutex is one level below it and Custom's Lock two, so Go
/// promotes Picky.Lock from the mutex, though *Deeper comes first.
#[derive(Debug, Clone)]
pub struct Custom {
    pub calls: Arc<Mutex<Option<i64>>>,
}

impl Custom {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.calls.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            calls: __go_clone_0_0,
        }
    }
}


impl Default for Custom {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            calls: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Custom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.calls.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct Deeper {
    pub custom: Arc<Mutex<Option<Custom>>>,
}

impl Deeper {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.custom.clone();
        Self {
            custom: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Deeper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.custom.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Picky {
    pub deeper: Arc<Mutex<Option<Deeper>>>,
    pub r_w_mutex: GoRWMutex,
}

impl Picky {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.deeper.clone();
        let __go_clone_1_0 = self.r_w_mutex.clone();
        Self {
            deeper: __go_clone_0_0,
            r_w_mutex: __go_clone_1_0,
        }
    }
}


impl Default for Picky {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = GoRWMutex::new();
        Self {
            deeper: __go_default_0_0,
            r_w_mutex: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Picky {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.deeper.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Table {
    pub fn read(&self, k: Arc<Mutex<Option<String>>>) -> i64 {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.r_lock();
            let __defer_mutex = { let __recv = self.clone(); __recv.__go_rw_mutex() }; __defer_stack.push(Box::new(move || {
        __defer_mutex.r_unlock();
    }));
            {
        let __return_value = { let __map = { let __map_holder = self.m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) };
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return __return_value;
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
                0 as i64
            }
        }
    }

    pub fn write(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<i64>>>) {
        self.lock();
        { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*self.m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        self.unlock();
    }

    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        self.r_w_mutex.clone()
    }

    pub fn lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.lock()
    }

    pub fn r_lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_lock()
    }

    pub fn r_unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_unlock()
    }

    pub fn unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.unlock()
    }
}

impl Registry {
    /// Swap replaces r's Table while holding the old one's lock: the deferred
    /// Unlock unlocks the Table that was locked, as Go follows r.Table to the
    /// mutex when the defer runs.
    pub fn swap(&mut self) {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.lock();
            let __defer_mutex = { let __recv = self.clone(); __recv.__go_rw_mutex() }; __defer_stack.push(Box::new(move || {
        __defer_mutex.unlock();
    }));
            { let new_val = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("swapped".to_string(), Arc::new(Mutex::new(Some(1i64))))])))), ..Default::default() }))).clone(); self.table = new_val; };

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

    /// Bump locks through Table's mutex from a method of Registry.
    pub fn bump(&mut self, k: Arc<Mutex<Option<String>>>) {
        self.lock();
        { let __map_holder = (*self.table.lock().unwrap().as_ref().unwrap()).m.clone(); let __key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
        self.unlock();
    }

    pub fn read(&self, k: Arc<Mutex<Option<String>>>) -> i64 {
        // Forward to embedded type's method
        let embedded = self.table.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(k)
    }

    pub fn write(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<i64>>>) {
        // Forward to embedded type's method
        let embedded = self.table.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write(k, v)
    }

    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        let embedded = self.table.clone();
        let guard = embedded.lock().unwrap();
        guard.as_ref().unwrap().__go_rw_mutex()
    }

    pub fn lock(&self) {
        self.__go_rw_mutex().lock()
    }

    pub fn r_lock(&self) {
        self.__go_rw_mutex().r_lock()
    }

    pub fn r_unlock(&self) {
        self.__go_rw_mutex().r_unlock()
    }

    pub fn unlock(&self) {
        self.__go_rw_mutex().unlock()
    }
}

impl Custom {
    pub fn lock(&mut self) {
        { let __target = self.calls.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }

    pub fn unlock(&mut self) {
        { let __target = self.calls.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() - 1); }
    }
}

impl Picky {
    /// Hold locks Picky's own mutex from a method of Picky.
    pub fn hold(&self) {
        self.lock();
        self.r_w_mutex.unlock();
    }

    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        self.r_w_mutex.clone()
    }

    pub fn lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.lock()
    }

    pub fn r_lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_lock()
    }

    pub fn r_unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_unlock()
    }

    pub fn unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.unlock()
    }
}

impl Both {
    pub fn read(&self, k: Arc<Mutex<Option<String>>>) -> i64 {
        // Forward to embedded type's method
        let embedded = self.table.clone();
        let guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(k)
    }

    pub fn write(&mut self, k: Arc<Mutex<Option<String>>>, v: Arc<Mutex<Option<i64>>>) {
        // Forward to embedded type's method
        let embedded = self.table.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write(k, v)
    }

    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        self.r_w_mutex.clone()
    }

    pub fn lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.lock()
    }

    pub fn r_lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_lock()
    }

    pub fn r_unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_unlock()
    }

    pub fn unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.unlock()
    }
}

impl Deep {
    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        self.r_w_mutex.clone()
    }

    pub fn lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.lock()
    }

    pub fn r_lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_lock()
    }

    pub fn r_unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_unlock()
    }

    pub fn unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.unlock()
    }
}

impl Deeper {
    pub fn lock(&mut self) {
        // Forward to embedded type's method
        let embedded = self.custom.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.lock()
    }

    pub fn unlock(&mut self) {
        // Forward to embedded type's method
        let embedded = self.custom.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.unlock()
    }
}

impl Middle {
    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        let embedded = self.deep.clone();
        let guard = embedded.lock().unwrap();
        guard.as_ref().unwrap().__go_rw_mutex()
    }

    pub fn lock(&self) {
        self.__go_rw_mutex().lock()
    }

    pub fn r_lock(&self) {
        self.__go_rw_mutex().r_lock()
    }

    pub fn r_unlock(&self) {
        self.__go_rw_mutex().r_unlock()
    }

    pub fn unlock(&self) {
        self.__go_rw_mutex().unlock()
    }
}

impl Outer {
    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        let embedded = self.shallow.clone();
        let guard = embedded.lock().unwrap();
        guard.as_ref().unwrap().__go_rw_mutex()
    }

    pub fn lock(&self) {
        self.__go_rw_mutex().lock()
    }

    pub fn r_lock(&self) {
        self.__go_rw_mutex().r_lock()
    }

    pub fn r_unlock(&self) {
        self.__go_rw_mutex().r_unlock()
    }

    pub fn unlock(&self) {
        self.__go_rw_mutex().unlock()
    }
}

impl Shallow {
    pub fn __go_rw_mutex(&self) -> GoRWMutex {
        self.r_w_mutex.clone()
    }

    pub fn lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.lock()
    }

    pub fn r_lock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_lock()
    }

    pub fn r_unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.r_unlock()
    }

    pub fn unlock(&self) {
        let embedded_ref = &self.r_w_mutex;
        embedded_ref.unlock()
    }
}

fn main() {
    go_install_panic_exit();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();
    ::time::__go_init_all();

    let mut t = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))])))), ..Default::default() })));
    let mut wg = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        wg.add(1);
        let t_thread = t.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            { let __recv = t_thread.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.write(
                Arc::new(Mutex::new(Some(format!("w{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))),
                Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))),
            ); __result };;
            let _ = { let __recv = t_thread.clone(); let __recv_ptr: *const Table = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Table }; let __result = unsafe { &*__recv_ptr }.read(Arc::new(Mutex::new(Some("a".to_string())))); __result };;
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg.wait();
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.write(Arc::new(Mutex::new(Some("b".to_string()))), Arc::new(Mutex::new(Some(2)))); __result };
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __map_key = "c".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*(*t.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_lock();
    let mut keys = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    for (k, _) in { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_unlock();
    sort::strings(keys.clone());
    println!("{} {} {}", format!("{}", format_slice(&keys)), format!("{}", { let __recv = t.clone(); let __recv_ptr: *const Table = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Table }; let __result = unsafe { &*__recv_ptr }.read(Arc::new(Mutex::new(Some("b".to_string())))); __result }), format!("{}", { let __recv = t.clone(); let __recv_ptr: *const Table = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Table }; let __result = unsafe { &*__recv_ptr }.read(Arc::new(Mutex::new(Some("z".to_string())))); __result }));
    let mut r = Arc::new(Mutex::new(Some(Registry { table: t.clone(), name: Arc::new(Mutex::new(Some("reg".to_string()))), ..Default::default() })));
    (*r.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some("d".to_string()))), Arc::new(Mutex::new(Some(4))));
    { let __sync_value = (*r.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_lock();
    println!("{} {}", format!("{}", (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*r.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    { let __sync_value = (*r.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_unlock();
    let mut r2 = Arc::new(Mutex::new(Some(Registry { table: t.clone(), name: Arc::new(Mutex::new(Some("reg2".to_string()))), ..Default::default() })));
    let mut wg2 = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 8; __tmp_x < __tmp_y } {
        wg2.add(1);
        let r_thread = r.clone(); let r2_thread = r2.clone(); let wg2_thread = wg2.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg2_defer_captured = wg2_thread.clone(); __defer_stack.push(Box::new(move || {
        wg2_defer_captured.done();
    }));;
            let mut reg = { let __owned = r_thread.lock().unwrap().as_ref().unwrap().__go_value_clone(); Arc::new(Mutex::new(Some(__owned))) };;
            if { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 1; __tmp_x == __tmp_y } {
        { let new_val = r2_thread.lock().unwrap().as_ref().unwrap().__go_value_clone(); *reg.lock().unwrap() = Some(new_val); };
    };
            { let __sync_value = (*reg.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();;
            { let __map_key = { let __map_key_holder = Arc::new(Mutex::new(Some(format!("s{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); (*(*(*reg.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            { let __sync_value = (*reg.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();;
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    wg2.wait();
    println!("{}", format!("{}", ({ let __len = ({ let __len_target = { let __field = (*t.lock().unwrap().as_ref().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    (*r.lock().unwrap().as_mut().unwrap()).bump(Arc::new(Mutex::new(Some("d".to_string()))));
    println!("{}", format!("{}", { let __map = { let __map_holder = (*(*r.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"d".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut both = {
        let __owner = Arc::new(Mutex::new(Some(Both { table: t.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().table.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __sync_value = (*both.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*t.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*both.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*both.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_lock();
    println!("{} {}", format!("{}", "both".to_string()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*both.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    { let __sync_value = (*both.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_unlock();
    let mut o = Arc::new(Mutex::new(Some(Outer {
        middle: {
            let __owner = Arc::new(Mutex::new(Some(Middle {
                deep: Arc::new(Mutex::new(Some(Deep { r_w_mutex: GoRWMutex::new() }))).clone(),
                ..Default::default()
            })));
            let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().deep.clone() };
            let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
            go_register_embedded_owner(__embedded_key, __owner.clone());
            __owner
        }.clone(),
        shallow: Arc::new(Mutex::new(Some(Shallow { n: Arc::new(Mutex::new(Some(7))), ..Default::default() }))).clone(),
        ..Default::default()
    })));
    { let __sync_value = (*o.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*(*o.lock().unwrap().as_ref().unwrap()).shallow.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*(*(*o.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*(*(*o.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*o.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_lock();
    println!("{} {}", format!("{}", "outer".to_string()), format!("{}", (*(*(*o.lock().unwrap().as_ref().unwrap()).shallow.lock().unwrap().as_ref().unwrap()).n.lock().unwrap().as_ref().unwrap())));
    { let __sync_value = (*o.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_unlock();
    let mut shared = Arc::new(Mutex::new(Some(Registry {
        table: Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() }))).clone(),
        name: Arc::new(Mutex::new(Some("shared".to_string()))),
        ..Default::default()
    })));
    { let __sync_value = (*shared.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    let mut waiting = GoChannel::<bool>::new();
    let mut wg3 = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        wg3.add(1);
        let shared_thread = shared.clone(); let waiting_thread = waiting.clone(); let wg3_thread = wg3.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg3_defer_captured = wg3_thread.clone(); __defer_stack.push(Box::new(move || {
        wg3_defer_captured.done();
    }));;
            waiting_thread.send(true);;
            { let __sync_value = (*shared_thread.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();;
            { let __map_key = { let __map_key_holder = Arc::new(Mutex::new(Some(format!("{}{}", format!("{}", "k".to_string()), format!("{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }))))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); (*(*(*shared_thread.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            { let __sync_value = (*shared_thread.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();;
            { let __sync_value = (*shared_thread.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_lock();;
            let _ = ({ let __len = ({ let __len_target = { let __field = (*(*shared_thread.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len });;
            { let __sync_value = (*shared_thread.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.r_unlock();;
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 4; __tmp_x < __tmp_y } {
        waiting.recv().unwrap_or_default();
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((20 as i64 * time::MILLISECOND) as i64))))))));
    { let __sync_value = (*shared.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    wg3.wait();
    println!("{} {}", format!("{}", (*{ let __field = (*shared.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*shared.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    let mut readerFirst = Arc::new(Mutex::new(Some(false)));
    let (mut round, mut wait) = (Arc::new(Mutex::new(Some(0i64))), Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((10 as i64 * time::MILLISECOND) as i64))))))));
    while { let __tmp_x = { let __v = (*round.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 5; __tmp_x < __tmp_y } && !{ let __v = (*readerFirst.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        let mut mu = GoRWMutex::new();
        mu.lock();
        let mut order = GoChannel::<String>::new_buffered(2 as usize);
        let mut finished = GoChannel::<bool>::new();
        let finished_thread = finished.clone(); let mu_thread = mu.clone(); let order_thread = order.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        mu_thread.r_lock();;
        order_thread.send("reader".to_string());;
        mu_thread.r_unlock();;
        finished_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        time::sleep(Arc::new(Mutex::new(Some({ let __arg_holder = wait.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        let finished_thread = finished.clone(); let mu_thread = mu.clone(); let order_thread = order.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        mu_thread.lock();;
        order_thread.send("writer".to_string());;
        mu_thread.unlock();;
        finished_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        time::sleep(Arc::new(Mutex::new(Some({ let __arg_holder = wait.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
        mu.unlock();
        finished.recv().unwrap_or_default();
        finished.recv().unwrap_or_default();
        { let new_val = { let __tmp_x = order.recv().unwrap_or_default(); let __tmp_y = "reader".to_string(); __tmp_x == __tmp_y }; *readerFirst.lock().unwrap() = Some(new_val); };
        {
            let __tmp_0 = { let __tmp_x = { let __v = (*round.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
            let __tmp_1 = { let __tmp_x = ({ let __v = (*wait.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __tmp_y = time::r#mod::Duration(Arc::new(Mutex::new(Some(2 as i64)))); __tmp_x * __tmp_y };
            *round.lock().unwrap() = Some(__tmp_0);
            *wait.lock().unwrap() = Some(__tmp_1);
        };
    }
    println!("{} {}", format!("{}", "waiting reader before the next writer:".to_string()), format!("{}", { let __v = (*readerFirst.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut deep = Arc::new(Mutex::new(Some(Outer {
        middle: {
            let __owner = Arc::new(Mutex::new(Some(Middle {
                deep: Arc::new(Mutex::new(Some(Deep { r_w_mutex: GoRWMutex::new() }))).clone(),
                ..Default::default()
            })));
            let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().deep.clone() };
            let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
            go_register_embedded_owner(__embedded_key, __owner.clone());
            __owner
        }.clone(),
        shallow: Arc::new(Mutex::new(Some(Shallow { r_w_mutex: GoRWMutex::new(), n: Arc::new(Mutex::new(Some(0))) }))).clone(),
        ..Default::default()
    })));
    { let __sync_value = (*(*(*deep.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    let mut started = GoChannel::<bool>::new();
    let mut wg4 = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        wg4.add(1);
        let deep_thread = deep.clone(); let started_thread = started.clone(); let wg4_thread = wg4.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg4_defer_captured = wg4_thread.clone(); __defer_stack.push(Box::new(move || {
        wg4_defer_captured.done();
    }));;
        started_thread.send(true);;
        { let __sync_value = (*(*(*deep_thread.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();;
        { let __sync_value = (*(*(*deep_thread.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();;
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
;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    started.recv().unwrap_or_default();
    started.recv().unwrap_or_default();
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((20 as i64 * time::MILLISECOND) as i64))))))));
    { let __sync_value = (*(*(*deep.lock().unwrap().as_mut().unwrap()).middle.lock().unwrap().as_mut().unwrap()).deep.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    wg4.wait();
    println!("{}", format!("{}", "deep shared".to_string()));
    let mut p = {
        let __owner = Arc::new(Mutex::new(Some(Picky {
            deeper: {
                let __owner = Arc::new(Mutex::new(Some(Deeper {
                    custom: Arc::new(Mutex::new(Some(Custom { calls: Arc::new(Mutex::new(Some(0))) }))).clone(),
                    ..Default::default()
                })));
                let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().custom.clone() };
                let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
                go_register_embedded_owner(__embedded_key, __owner.clone());
                __owner
            }.clone(),
            ..Default::default()
        })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().deeper.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = p.clone(); let __recv_ptr: *const Picky = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Picky }; let __result = unsafe { &*__recv_ptr }.hold(); __result };
    { let __sync_value = (*p.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    println!("{} {}", format!("{}", "picky locked".to_string()), format!("{}", (*(*(*p.lock().unwrap().as_ref().unwrap()).deeper.lock().unwrap().as_ref().unwrap().custom.lock().unwrap().as_ref().unwrap()).calls.lock().unwrap().as_ref().unwrap())));
    { let __sync_value = (*p.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    println!("{} {}", format!("{}", "picky".to_string()), format!("{}", (*(*(*p.lock().unwrap().as_ref().unwrap()).deeper.lock().unwrap().as_ref().unwrap().custom.lock().unwrap().as_ref().unwrap()).calls.lock().unwrap().as_ref().unwrap())));
    let mut held = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() })));
    let mut reg = {
        let __owner = Arc::new(Mutex::new(Some(Registry { table: held.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().table.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __sync_value = (*held.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    let mut unlock = Arc::new(Mutex::new(Some({ let __recv = reg.clone(); let __recv = { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }; Arc::new(move || { __recv.unlock() }) as Arc<dyn Fn() -> () + Send + Sync> })));
    { let new_val = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1i64))))])))), ..Default::default() }))).clone(); (*reg.lock().unwrap().as_mut().unwrap()).table = new_val; };
    {
        let __f: Arc<dyn Fn() -> () + Send + Sync> = unlock.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    };
    { let __sync_value = (*held.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*held.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    println!("{} {}", format!("{}", "method value".to_string()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*reg.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    let mut before = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() })));
    let mut swapped = {
        let __owner = Arc::new(Mutex::new(Some(Registry { table: before.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().table.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __recv = swapped.clone(); let __recv_ptr: *mut Registry = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Registry }; let __result = unsafe { &mut *__recv_ptr }.swap(); __result };
    { let __sync_value = (*before.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*before.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    { let __sync_value = (*swapped.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*swapped.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    println!("{} {}", format!("{}", "deferred unlock".to_string()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*swapped.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    let mut gheld = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() })));
    let mut greg = {
        let __owner = Arc::new(Mutex::new(Some(Registry { table: gheld.clone(), ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().table.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    { let __sync_value = (*greg.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    let greg_thread = greg.clone(); let __go_mutex = { let __recv = greg.clone(); { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }; std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        __go_mutex.unlock();
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    { let new_val = Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() }))).clone(); (*greg.lock().unwrap().as_mut().unwrap()).table = new_val; };
    { let __sync_value = (*gheld.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.lock();
    { let __sync_value = (*gheld.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value }.unlock();
    println!("{}", format!("{}", "go unlock".to_string()));
    let mut regs = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Registry {
        table: Arc::new(Mutex::new(Some(Table { m: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))), ..Default::default() }))).clone(),
        ..Default::default()
    })))])));
    let regs_closure_clone = regs.clone(); let mut get = Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<Registry>>> {
        return {
            let __seq = { let __seq_holder = regs_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned };
            __seq[(0) as usize].clone()
        };
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Registry>>> + Send + Sync>)));
    { let __recv = {
        let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Registry>>> + Send + Sync> = get.lock().unwrap().as_ref().unwrap().clone();
        (__f)(

        )
    }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.lock();
    let mut ready = GoChannel::<bool>::new();
    let mut wg5 = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        wg5.add(1);
        let get_thread = get.clone(); let ready_thread = ready.clone(); let regs_thread = regs.clone(); let wg5_thread = wg5.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |i: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg5_defer_captured = wg5_thread.clone(); __defer_stack.push(Box::new(move || {
        wg5_defer_captured.done();
    }));;
            ready_thread.send(true);;
            if { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        { let __recv = {
            let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Registry>>> + Send + Sync> = get_thread.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.lock();
        { let __recv = {
            let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Registry>>> + Send + Sync> = get_thread.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.unlock();
    } else {
        { let __recv = { let __seq = { let __seq_holder = regs_thread.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.lock();
        { let __recv = { let __seq = { let __seq_holder = regs_thread.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.unlock();
    };
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

        };
        __closure(__go_arg_0);
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    ready.recv().unwrap_or_default();
    ready.recv().unwrap_or_default();
    time::sleep(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some((20 as i64 * time::MILLISECOND) as i64))))))));
    { let __recv = { let __seq = { let __seq_holder = regs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }; { let __sync_value = (*__recv.lock().unwrap().as_ref().unwrap()).__go_rw_mutex(); __sync_value } }.unlock();
    wg5.wait();
    println!("{}", format!("{}", "call and index receivers".to_string()));
}

impl GoValueClone for Table {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Registry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Both {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Deep {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Middle {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Shallow {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Outer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Custom {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Deeper {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Picky {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
