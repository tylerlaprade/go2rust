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

#[derive(Clone, Debug)]
struct GoOnce {
    done: std::sync::Arc<std::sync::atomic::AtomicBool>,
    m: std::sync::Arc<std::sync::Mutex<()>>,
}

impl GoOnce {
    fn new() -> Self {
        GoOnce {
            done: std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false)),
            m: std::sync::Arc::new(std::sync::Mutex::new(())),
        }
    }

    // Go's Once: the first caller runs f while it holds m, so no call
    // returns before f has finished, and f counts as done even if it
    // panics.
    fn r#do<F>(&self, mut f: F)
    where
        F: FnMut(),
    {
        if self.done.load(std::sync::atomic::Ordering::Acquire) {
            return;
        }
        let _running = self.m.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        if !self.done.load(std::sync::atomic::Ordering::Acquire) {
            struct MarkDone<'a>(&'a std::sync::atomic::AtomicBool);
            impl Drop for MarkDone<'_> {
                fn drop(&mut self) {
                    self.0.store(true, std::sync::atomic::Ordering::Release);
                }
            }
            let _mark = MarkDone(&self.done);
            f();
        }
    }
}

impl Default for GoOnce {
    fn default() -> Self {
        Self::new()
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
pub struct Base {
    pub once: GoOnce,
    pub index: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}

impl Base {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.once.clone();
        let __go_clone_1_0 = self.index.clone();
        Self {
            once: __go_clone_0_0,
            index: __go_clone_1_0,
        }
    }
}


impl Default for Base {
    fn default() -> Self {
        let __go_default_0_0 = GoOnce::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            once: __go_default_0_0,
            index: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.index));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Cache__string {
    pub once: GoOnce,
    pub items: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>,
}

impl Cache__string {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.once.clone();
        let __go_clone_1_0 = self.items.clone();
        Self {
            once: __go_clone_0_0,
            items: __go_clone_1_0,
        }
    }
}


impl Default for Cache__string {
    fn default() -> Self {
        let __go_default_0_0 = GoOnce::new();
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            once: __go_default_0_0,
            items: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Cache__string {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.items));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Names {
    pub cache__string: Arc<Mutex<Option<Cache__string>>>,
}

impl Names {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.cache__string.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            cache__string: __go_clone_0_0,
        }
    }
}


impl Default for Names {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Cache__string::default())));
        Self {
            cache__string: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Names {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.cache__string.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone)]
pub struct Stats {
    pub base: Arc<Mutex<Option<Base>>>,
    pub once: GoOnce,
    pub mu: GoMutex,
    pub counts: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
    pub head: Arc<Mutex<Option<node>>>,
}

impl Stats {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.base.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        let __go_clone_1_0 = self.once.clone();
        let __go_clone_2_0 = self.mu.clone();
        let __go_clone_3_0 = self.counts.clone();
        let __go_clone_4_0 = self.head.clone();
        Self {
            base: __go_clone_0_0,
            once: __go_clone_1_0,
            mu: __go_clone_2_0,
            counts: __go_clone_3_0,
            head: __go_clone_4_0,
        }
    }
}


impl Default for Stats {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Base::default())));
        let __go_default_1_0 = GoOnce::new();
        let __go_default_2_0 = GoMutex::new();
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        let __go_default_4_0 = Arc::new(Mutex::new(None));
        Self {
            base: __go_default_0_0,
            once: __go_default_1_0,
            mu: __go_default_2_0,
            counts: __go_default_3_0,
            head: __go_default_4_0,
        }
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.base.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", format_map(&self.counts));
        let __go_fmt_2 = format!("{}", { let __guard = self.head.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct node {
    pub key: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<node>>>,
}

impl node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = self.next.clone();
        Self {
            key: __go_clone_0_0,
            next: __go_clone_1_0,
        }
    }
}


impl Default for node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(None));
        Self {
            key: __go_default_0_0,
            next: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl Base {
    pub fn build(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("base".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); self.index = new_val; };
    }

    pub fn lookup(&mut self, k: Arc<Mutex<Option<String>>>) -> i64 {
        { let __once = self.once.clone(); __once.r#do(|| { self.build(); }); };
        { let __map = { let __map_holder = self.index.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }
    }
}

impl Cache__string {
    pub fn fill(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([])))); self.items = new_val; };
    }
}

impl Names {
    /// Get reaches fill, a method of the instantiated Cache[string], through
    /// Names.
    pub fn get(&mut self, k: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
        { let __once = (*self.cache__string.lock().unwrap().as_ref().unwrap()).once.clone(); __once.r#do(|| { { let __promoted_recv = self.cache__string.clone(); let mut __promoted_guard = __promoted_recv.lock().unwrap(); let __promoted_ref = __promoted_guard.as_mut().unwrap(); let __result = __promoted_ref.fill(); __result }; }); };
        {
        let (mut v, mut ok) = { let __map = { let __map_holder = (*self.cache__string.lock().unwrap().as_ref().unwrap()).items.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(String::new()))), false) } };;
        if ok {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
        { let __map_key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __map_value = Arc::new(Mutex::new(Some(format!("{}{}", "new:".to_string(), { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v })))); (*(*self.cache__string.lock().unwrap().as_ref().unwrap()).items.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*self.cache__string.lock().unwrap().as_ref().unwrap()).items.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })))
    }

    pub fn fill(&mut self) {
        // Forward to embedded type's method
        let embedded = self.cache__string.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.fill()
    }
}

impl Stats {
    pub fn init(&mut self) {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))); self.counts = new_val; };
        { let new_val = Arc::new(Mutex::new(Some(node { key: Arc::new(Mutex::new(Some("start".to_string()))), ..Default::default() }))).clone(); self.head = new_val; };
    }

    pub fn add(&mut self, k: Arc<Mutex<Option<String>>>) {
        { let __once = self.once.clone(); __once.r#do(|| { self.init(); }); };
        let __mutex_guard_source_22 = self.mu.clone(); let __mutex_guard_22 = __mutex_guard_source_22.guard();
        { let __map_holder = self.counts.clone(); let __key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
        { let new_val = Arc::new(Mutex::new(Some(node { key: Arc::new(Mutex::new(Some({ let __arg_holder = k.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), next: { let __field = self.head.clone(); __field }, ..Default::default() }))).clone(); self.head = new_val; };
        std::mem::drop(__mutex_guard_22);
    }

    pub fn keys(&self) -> Arc<Mutex<Option<Vec<String>>>> {
        let mut keys = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = ({ let __len_target = { let __field = self.counts.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
        for (k, _) in { let __range_holder = self.counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = keys.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *keys.lock().unwrap() = __slice_header; };
    }
        sort::strings(keys.clone());
        return keys.clone();
    }

    pub fn lookup(&mut self, k: Arc<Mutex<Option<String>>>) -> i64 {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.lookup(k)
    }

    pub fn build(&mut self) {
        // Forward to embedded type's method
        let embedded = self.base.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.build()
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut s = {
        let __owner = Arc::new(Mutex::new(Some(Stats { ..Default::default() })));
        let __embedded = { let __owner_guard = __owner.lock().unwrap(); __owner_guard.as_ref().unwrap().base.clone() };
        let __embedded_key = { let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) };
        go_register_embedded_owner(__embedded_key, __owner.clone());
        __owner
    };
    for k in &vec!["a".to_string(), "b".to_string(), "a".to_string()] {
        { let __recv = s.clone(); let __recv_ptr: *mut Stats = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Stats }; let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some((*k).clone())))); __result };
    }
    println!("{} {} {}", format!("{}", format_slice(&{ let __recv = s.clone(); let __recv_ptr: *const Stats = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Stats }; let __result = unsafe { &*__recv_ptr }.keys(); __result })), format!("{}", { let __map = { let __map_holder = (*s.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = (*s.lock().unwrap().as_ref().unwrap()).counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    let mut p = (*s.lock().unwrap().as_ref().unwrap()).head.clone();
    while { let __nil_result = (*p.lock().unwrap()).is_some(); __nil_result } {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        { let new_val = (*p.lock().unwrap().as_ref().unwrap()).next.clone(); p = new_val; };
    }
    println!("{} {} {}", format!("{}", "nodes:".to_string()), format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*(*{ let __base = (*s.lock().unwrap().as_ref().unwrap()).head.clone(); __base }.lock().unwrap().as_ref().unwrap()).key.lock().unwrap().as_ref().unwrap()).clone()));
    println!("{} {}", format!("{}", { let __recv = s.clone(); let __recv_ptr: *mut Stats = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Stats }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some("base".to_string())))); __result }), format!("{}", { let __recv = s.clone(); let __recv_ptr: *mut Stats = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Stats }; let __result = unsafe { &mut *__recv_ptr }.lookup(Arc::new(Mutex::new(Some("missing".to_string())))); __result }));
    let mut names: Arc<Mutex<Option<Names>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{} {} {}", format!("{}", (*(*names.lock().unwrap().as_mut().unwrap()).get(Arc::new(Mutex::new(Some("x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", (*(*names.lock().unwrap().as_mut().unwrap()).get(Arc::new(Mutex::new(Some("x".to_string())))).lock().unwrap().as_ref().unwrap())), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*(*names.lock().unwrap().as_mut().unwrap()).cache__string.lock().unwrap().as_mut().unwrap()).items.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
}

impl GoValueClone for Base {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Cache__string {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Names {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for Stats {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
