#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::collections::BTreeMap;
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
            __GO_RECOVER_PAYLOAD.with(|slot| *slot.borrow_mut() = Some(Box::new(go_runtime_error_text(*value)) as Box<dyn Any + Send + Sync>));
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

struct GoMutex {
    inner: std::sync::Arc<(std::sync::Mutex<bool>, std::sync::Condvar)>,
    is_nil: bool,
}

struct GoMutexGuard {
    mutex: GoMutex,
    active: bool,
}

impl GoMutex {
    fn new() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: false,
        }
    }

    fn nil() -> Self {
        GoMutex {
            inner: std::sync::Arc::new((std::sync::Mutex::new(false), std::sync::Condvar::new())),
            is_nil: true,
        }
    }

    fn go_ptr_eq(left: &Self, right: &Self) -> bool {
        if left.is_nil || right.is_nil {
            return left.is_nil && right.is_nil;
        }
        std::sync::Arc::ptr_eq(&left.inner, &right.inner)
    }

    fn lock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        while *locked {
            locked = ready.wait(locked).unwrap();
        }
        *locked = true;
    }

    fn unlock(&self) {
        if self.is_nil { panic!("nil *sync.Mutex"); }
        let (state, ready) = &*self.inner;
        let mut locked = state.lock().unwrap();
        if !*locked {
            panic!("sync.Mutex: unlock of unlocked mutex");
        }
        *locked = false;
        ready.notify_one();
    }

    fn guard(&self) -> GoMutexGuard {
        self.lock();
        GoMutexGuard {
            mutex: self.clone(),
            active: true,
        }
    }
}

impl Drop for GoMutexGuard {
    fn drop(&mut self) {
        if self.active {
            self.mutex.unlock();
            self.active = false;
        }
    }
}

impl Default for GoMutex {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for GoMutex {
    fn clone(&self) -> Self {
        GoMutex {
            inner: self.inner.clone(),
            is_nil: self.is_nil,
        }
    }
}

impl std::fmt::Debug for GoMutex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mutex")
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

pub trait Event: std::fmt::Display + Any {
    fn __go_clone_box_event(&self) -> Box<dyn Event + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_event(&self, other: &(dyn Event + Send + Sync)) -> bool;
    fn kind(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn Event + Send + Sync> {
    fn clone(&self) -> Self {
        Event::__go_clone_box_event(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Click {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}

impl Click {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_0_1 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_0_1,
        }
    }
}


impl Default for Click {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_0_1 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_0_1,
        }
    }
}

impl std::fmt::Display for Click {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
impl PartialEq for Click {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.x.lock().unwrap().as_ref().cloned() }; let __right = { other.x.lock().unwrap().as_ref().cloned() }; __left == __right }
                && { let __left = { self.y.lock().unwrap().as_ref().cloned() }; let __right = { other.y.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Debug, Clone)]
pub struct Key {
    pub code: Arc<Mutex<Option<i32>>>,
}

impl Key {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.code.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            code: __go_clone_0_0,
        }
    }
}


impl Default for Key {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            code: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.code.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        (
            { let __left = { self.code.lock().unwrap().as_ref().cloned() }; let __right = { other.code.lock().unwrap().as_ref().cloned() }; __left == __right }
        )
    }
}


#[derive(Clone)]
pub struct Bus {
    pub mu: GoMutex,
    pub handlers: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<GoChannel<Option<Box<dyn Event + Send + Sync>>>>>>>>>>>,
}

impl Bus {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.mu.clone();
        let __go_clone_1_0 = self.handlers.clone();
        Self {
            mu: __go_clone_0_0,
            handlers: __go_clone_1_0,
        }
    }
}


impl Default for Bus {
    fn default() -> Self {
        let __go_default_0_0 = GoMutex::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            mu: __go_default_0_0,
            handlers: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Bus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<map>");
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Click {
    pub fn kind(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some(format!("click({},{})", format!("{}", (*self.x.lock().unwrap().as_ref().unwrap())), format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()))))))
    }
}

impl Event for Click {
    fn kind(&self) -> Arc<Mutex<Option<String>>> {
        Click::kind(self)
    }
    fn __go_clone_box_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Event + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_event(&self, other: &(dyn Event + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Click>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct ClickPtr(pub Arc<Mutex<Option<Click>>>);

impl std::fmt::Display for ClickPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Event for ClickPtr {
    fn kind(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Click::kind(__recv)
    }
    fn __go_clone_box_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Event + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_event(&self, other: &(dyn Event + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<ClickPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Key {
    pub fn kind(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some(format!("{}{}", "key ".to_string(), (*Arc::new(Mutex::new(Some(char::from_u32(({ let __selector_holder = self.code.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())))));
    }
}

impl Event for Key {
    fn kind(&self) -> Arc<Mutex<Option<String>>> {
        Key::kind(self)
    }
    fn __go_clone_box_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Event + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_event(&self, other: &(dyn Event + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Key>() {
            self == __other
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct KeyPtr(pub Arc<Mutex<Option<Key>>>);

impl std::fmt::Display for KeyPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Event for KeyPtr {
    fn kind(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Key::kind(__recv)
    }
    fn __go_clone_box_event(&self) -> Box<dyn Event + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Event + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => if plus { format!("&{:#}", __v) } else { format!("&{}", __v) }, None => "<nil>".to_string() }
    }
    fn __go_eq_event(&self, other: &(dyn Event + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<KeyPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl Bus {
    pub fn subscribe(&mut self, topic: Arc<Mutex<Option<String>>>) -> GoChannel<Option<Box<dyn Event + Send + Sync>>> {
        let __mutex_guard_source_3 = self.mu.clone(); let __mutex_guard_3 = __mutex_guard_source_3.guard();
        // mu.Unlock() handled by RAII guard
        let mut ch = GoChannel::<Option<Box<dyn Event + Send + Sync>>>::new_buffered(4 as usize);
        { let __map_key = ({ let __v = (*topic.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = { let __slice = { let __map_holder = self.handlers.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&({ let __v = (*topic.lock().unwrap().as_ref().unwrap()).clone(); __v })).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_value = ch.clone(); let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) }; (*self.handlers.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        ch.clone()
    }

    pub fn publish(&self, topic: Arc<Mutex<Option<String>>>, e: Arc<Mutex<Option<Box<dyn Event + Send + Sync>>>>) {
        let __mutex_guard_source_3 = self.mu.clone(); let __mutex_guard_3 = __mutex_guard_source_3.guard();
        // mu.Unlock() handled by RAII guard
        { let __range_holder = { let __map = { let __map_holder = self.handlers.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*topic.lock().unwrap().as_ref().unwrap()).clone(); __v }))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for ch in __range_values.iter() {
        ch.send({ let __iface_handle = { let __iface_value = e.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
    } }
    }

    pub fn close(&self) {
        let __mutex_guard_source_3 = self.mu.clone(); let __mutex_guard_3 = __mutex_guard_source_3.guard();
        // mu.Unlock() handled by RAII guard
        for (_, chans) in { let __range_holder = self.handlers.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __range_holder = chans.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for ch in __range_values.iter() {
        ch.close();
    } }
    }
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut events = GoChannel::<Option<Box<dyn Event + Send + Sync>>>::new();
    let mut done = GoChannel::<Vec<String>>::new();
    let done_thread = done.clone(); let events_thread = events.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut seen: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));;
        for __iface_value in events_thread.clone() {
        let mut e = Arc::new(Mutex::new(__iface_value));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = seen.clone(); let __append_value = (*(*e.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *seen.lock().unwrap() = __slice_header; };
    };
        done_thread.send(seen.lock().unwrap().as_ref().unwrap().clone());;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    let mut wg = WaitGroup::new();
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        wg.add(1);
        let events_thread = events.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some({ let __arg_holder = i.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |n: Arc<Mutex<Option<i64>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            events_thread.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(Click { x: Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), y: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x * __tmp_y }))), ..Default::default() }) as Box<dyn Event + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });;
            events_thread.send({ let __iface_handle = Arc::new(Mutex::new(Some(Box::new(KeyPtr(Arc::new(Mutex::new(Some(Key { code: Arc::new(Mutex::new(Some((({ let __tmp_x = 97i64; let __tmp_y = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x + __tmp_y })) as i32))), ..Default::default() }))).clone())) as Box<dyn Event + Send + Sync>))); let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });;
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
    events.close();
    let mut seen = Arc::new(Mutex::new(Some(done.recv().unwrap_or_default())));
    let mut seen = { let __slice_header = GoSliceHeader::go_slice_header(&seen); Arc::new(Mutex::new(__slice_header)) };
    sort::strings(seen.clone());
    println!("{}", format!("{}", format_slice(&seen)));
    let mut bus = Arc::new(Mutex::new(Some(Bus { handlers: Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<GoChannel<Option<Box<dyn Event + Send + Sync>>>>>>>>::from([])))), ..Default::default() })));
    let mut a = { let __recv = bus.clone(); let __recv_ptr: *mut Bus = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Bus }; let __result = unsafe { &mut *__recv_ptr }.subscribe(Arc::new(Mutex::new(Some("ui".to_string())))); __result };
    let mut b = { let __recv = bus.clone(); let __recv_ptr: *mut Bus = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Bus }; let __result = unsafe { &mut *__recv_ptr }.subscribe(Arc::new(Mutex::new(Some("ui".to_string())))); __result };
    { let __recv = bus.clone(); let __recv_ptr: *const Bus = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Bus }; let __result = unsafe { &*__recv_ptr }.publish(
        Arc::new(Mutex::new(Some("ui".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(Click { x: Arc::new(Mutex::new(Some(1))), y: Arc::new(Mutex::new(Some(1))), ..Default::default() }) as Box<dyn Event + Send + Sync>))),
    ); __result };
    { let __recv = bus.clone(); let __recv_ptr: *const Bus = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Bus }; let __result = unsafe { &*__recv_ptr }.publish(
        Arc::new(Mutex::new(Some("ui".to_string()))),
        Arc::new(Mutex::new(Some(Box::new(KeyPtr(Arc::new(Mutex::new(Some(Key { code: Arc::new(Mutex::new(Some(('z' as i32) as i32))), ..Default::default() }))).clone())) as Box<dyn Event + Send + Sync>))),
    ); __result };
    { let __recv = bus.clone(); let __recv_ptr: *const Bus = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Bus }; let __result = unsafe { &*__recv_ptr }.close(); __result };
    for __iface_value in a.clone() {
        let mut e = Arc::new(Mutex::new(__iface_value));
        println!("{} {}", format!("{}", "a got".to_string()), format!("{}", (*(*e.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap())));
    }
    loop {
        let (mut e, mut ok) = match b.recv() { Some(v) => (Arc::new(Mutex::new(v)), Arc::new(Mutex::new(Some(true)))), None => (Arc::new(Mutex::new(None::<Box<dyn Event + Send + Sync>>)), Arc::new(Mutex::new(Some(false)))) };
        if !{ let __v = (*ok.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        break
    }
        println!("{} {}", format!("{}", "b got".to_string()), format!("{}", (*(*e.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap())));
    }
    let mut results = GoChannel::<Option<Box<dyn Event + Send + Sync>>>::new();
    let results_thread = results.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let results_defer_captured = results_thread.clone(); __defer_stack.push(Box::new(move || {
        results_defer_captured.close();
    }));;
        for e in &vec![Arc::new(Mutex::new(Some(Box::new(Click { x: Arc::new(Mutex::new(Some(3))), y: Arc::new(Mutex::new(Some(4))), ..Default::default() }) as Box<dyn Event + Send + Sync>))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(KeyPtr(Arc::new(Mutex::new(Some(Key { code: Arc::new(Mutex::new(Some(('q' as i32) as i32))), ..Default::default() }))).clone())) as Box<dyn Event + Send + Sync>)))] {
        results_thread.send({ let __iface_handle = { let __iface_value = (*e).lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) }; let __iface_value = (*__iface_handle.lock().unwrap()).clone(); __iface_value });
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
;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    for __iface_value in results.clone() {
        let mut e = Arc::new(Mutex::new(__iface_value));
        if { let __nil_result = (*e.lock().unwrap()).is_none(); __nil_result } {
        println!("{}", format!("{}", "nil event".to_string()));
        continue
    }
        {
        let (mut c, mut ok) = ({
        let val = e.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn Event + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<Click>() {
            (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(Default::default()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(Default::default()))), false)
        }
    });;
        if ok {
            println!("{} {} {}", format!("{}", "click at".to_string()), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap())));;
        } else {
            println!("{} {}", format!("{}", "other".to_string()), format!("{}", (*(*e.lock().unwrap().as_ref().unwrap()).kind().lock().unwrap().as_ref().unwrap())));;
        }
    }
    }
}

impl GoValueClone for Click {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Key {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Bus {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
