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
// the same lock, as a *sync.RWMutex is. A writer waiting for the lock holds
// off new readers, as Go's does.
#[derive(Clone, Debug, Default)]
pub struct GoRWMutex {
    // (a writer holds it, how many readers hold it, how many writers wait
    // for it), and the waiters.
    inner: std::sync::Arc<(std::sync::Mutex<(bool, usize, usize)>, std::sync::Condvar)>,
}

impl GoRWMutex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        held.2 += 1;
        while held.0 || held.1 > 0 {
            held = ready.wait(held).unwrap();
        }
        held.2 -= 1;
        held.0 = true;
    }

    pub fn unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if !held.0 {
            panic!("sync: Unlock of unlocked RWMutex");
        }
        held.0 = false;
        ready.notify_all();
    }

    pub fn r_lock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        while held.0 || held.2 > 0 {
            held = ready.wait(held).unwrap();
        }
        held.1 += 1;
    }

    pub fn r_unlock(&self) {
        let (state, ready) = &*self.inner;
        let mut held = state.lock().unwrap();
        if held.1 == 0 {
            panic!("sync: RUnlock of unlocked RWMutex");
        }
        held.1 -= 1;
        if held.1 == 0 {
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


impl Table {
    pub fn read(&self, k: Arc<Mutex<Option<String>>>) -> i64 {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

        let __go_previous_panic_hook = go_silence_panic_hook();
        let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            self.r_lock();
            let mut t_defer_captured = self.clone(); __defer_stack.push(Box::new(move || {
        t_defer_captured.r_unlock();
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

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

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
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    { let __map_key = "c".to_string(); let __map_value = Arc::new(Mutex::new(Some(3))); (*(*t.lock().unwrap().as_ref().unwrap()).m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.r_lock(); __result };
    let mut keys = Arc::new(Mutex::new(Some(Vec::<String>::new())));
    for (k, _) in { let __range_holder = (*t.lock().unwrap().as_ref().unwrap()).m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.r_unlock(); __result };
    sort::strings(keys.clone());
    println!("{} {} {}", format!("{}", format_slice(&keys)), format!("{}", { let __recv = t.clone(); let __recv_ptr: *const Table = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Table }; let __result = unsafe { &*__recv_ptr }.read(Arc::new(Mutex::new(Some("b".to_string())))); __result }), format!("{}", { let __recv = t.clone(); let __recv_ptr: *const Table = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Table }; let __result = unsafe { &*__recv_ptr }.read(Arc::new(Mutex::new(Some("z".to_string())))); __result }));
    let mut r = Arc::new(Mutex::new(Some(Registry { table: t.clone(), name: Arc::new(Mutex::new(Some("reg".to_string()))), ..Default::default() })));
    (*r.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some("d".to_string()))), Arc::new(Mutex::new(Some(4))));
    (*r.lock().unwrap().as_mut().unwrap()).r_lock();
    println!("{} {}", format!("{}", (*{ let __field = (*r.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*r.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    (*r.lock().unwrap().as_mut().unwrap()).r_unlock();
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
            (*reg.lock().unwrap().as_mut().unwrap()).lock();;
            { let __map_key = { let __map_key_holder = Arc::new(Mutex::new(Some(format!("s{}", { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v })))).clone(); let __map_key_guard = __map_key_holder.lock().unwrap(); let __cloned = (*__map_key_guard.as_ref().unwrap()).clone(); std::mem::drop(__map_key_guard); __cloned }; let __map_value = Arc::new(Mutex::new(Some((*i.lock().unwrap().as_ref().unwrap()).clone()))); (*(*(*reg.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };;
            (*reg.lock().unwrap().as_mut().unwrap()).unlock();;
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
    { let __recv = both.clone(); let __recv_ptr: *mut Both = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Both }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.lock(); __result };
    { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
    { let __recv = both.clone(); let __recv_ptr: *mut Both = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Both }; let __result = unsafe { &mut *__recv_ptr }.unlock(); __result };
    { let __recv = both.clone(); let __recv_ptr: *mut Both = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Both }; let __result = unsafe { &mut *__recv_ptr }.r_lock(); __result };
    println!("{} {}", format!("{}", "both".to_string()), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*both.lock().unwrap().as_mut().unwrap()).table.lock().unwrap().as_mut().unwrap()).m.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
    { let __recv = both.clone(); let __recv_ptr: *mut Both = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Both }; let __result = unsafe { &mut *__recv_ptr }.r_unlock(); __result };
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
