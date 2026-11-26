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

#[derive(Debug, Clone, Default)]
pub struct Inventory {
    pub stock: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}

impl Inventory {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.stock.clone();
        Self {
            stock: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Inventory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.stock));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub fn keys(m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut out = Arc::new(Mutex::new(Some({ let mut v: Vec<String> = (0..(({ let __len = (*m.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })) as usize).map(|_| "".to_string()).collect(); v.truncate((0) as usize); v })));
    for (k, _) in { let __range_holder = m.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = out.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *out.lock().unwrap() = __slice_header; };
    }
    sort::strings(out.clone());
    return out.clone();
}

fn main() {
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut mm = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1i64])))), ("b".to_string(), Arc::new(Mutex::new(Some(vec![2i64]))))]))));
    {
        let __go_lhs_map_0 = mm.clone();
        let __go_lhs_key_0 = "a".to_string();
        let __go_lhs_map_1 = mm.clone();
        let __go_lhs_key_1 = "b".to_string();
        let __tmp_0 = { let __map = { let __map_holder = mm.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        let __tmp_1 = { let __map = { let __map_holder = mm.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    println!("{}", format!("{}", format_map_nil(&mm, "[]")));
    { let __idx = (0) as usize; let __rhs = 7; (*{ let __map = { let __map_holder = mm.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    println!("{}", format!("{}", format_map_nil(&mm, "[]")));
    let mut ms = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some("x".to_string())))), ("b".to_string(), Arc::new(Mutex::new(Some("y".to_string()))))]))));
    {
        let __go_lhs_map_0 = ms.clone();
        let __go_lhs_key_0 = "a".to_string();
        let __go_lhs_map_1 = ms.clone();
        let __go_lhs_key_1 = "b".to_string();
        let __tmp_0 = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = ms.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })));
        let __tmp_1 = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = ms.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })));
        (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    println!("{}", format!("{}", format_map(&ms)));
    let mut mp = Arc::new(Mutex::new(Some(BTreeMap::<i64, Arc<Mutex<Option<i64>>>>::from([]))));
    let (mut one, mut two) = (Arc::new(Mutex::new(Some(1i64))), Arc::new(Mutex::new(Some(2i64))));
    {
        let __go_lhs_map_0 = mp.clone();
        let __go_lhs_key_0 = 1;
        let __go_lhs_map_1 = mp.clone();
        let __go_lhs_key_1 = 2;
        let __tmp_0 = one.clone();
        let __tmp_1 = two.clone();
        (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    {
        let __go_lhs_map_0 = mp.clone();
        let __go_lhs_key_0 = 1;
        let __go_lhs_map_1 = mp.clone();
        let __go_lhs_key_1 = 2;
        let __tmp_0 = { let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&2)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        let __tmp_1 = { let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) };
        (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    { let new_val = 20; *{ let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", { let __v = (*{ let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*{ let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&2)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*one.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*two.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut counts = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))));
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    let mut mu = GoMutex::new();
    let mut wg = WaitGroup::new();
    for w in &vec!["x".to_string(), "y".to_string(), "x".to_string()] {
        wg.add(1);
        let counts_thread = counts.clone(); let mu_thread = mu.clone(); let mut total_thread = total.clone(); let wg_thread = wg.clone(); let __go_arg_0 = Arc::new(Mutex::new(Some((*w).clone()))); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();
        let mut __closure = move |w: Arc<Mutex<Option<String>>>| {
            let __go_previous_panic_hook = go_silence_panic_hook();
let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
let wg_defer_captured = wg_thread.clone(); __defer_stack.push(Box::new(move || {
        wg_defer_captured.done();
    }));;
            let __mutex_guard_source_566 = mu_thread.clone(); let __mutex_guard_566 = __mutex_guard_source_566.guard();;
            {
                let __go_lhs_map_0 = counts_thread.clone();
                let __go_lhs_key_0 = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v });
                let __tmp_0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __map = { let __map_holder = counts_thread.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = 1; __tmp_x + __tmp_y })));
                let __tmp_1 = { let __tmp_x = { let __v = (*total_thread.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
                (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
                *total_thread.lock().unwrap() = Some(__tmp_1);
            };;
            std::mem::drop(__mutex_guard_566);;
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
    }
    wg.wait();
    println!("{} {}", format!("{}", format_map(&counts)), format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut k = Arc::new(Mutex::new(Some("p".to_string())));
    let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("p".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    {
        let __go_lhs_map_1 = m.clone();
        let __go_lhs_key_1 = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v });
        let __tmp_0 = "q".to_string();
        let __tmp_1 = Arc::new(Mutex::new(Some(5)));
        *k.lock().unwrap() = Some(__tmp_0);
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    println!("{} {}", format!("{}", { let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", format_map(&m)));
    let mut old = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("apple".to_string(), Arc::new(Mutex::new(Some(3i64))))]))));
    let mut inv = Arc::new(Mutex::new(Some(Inventory { stock: old.clone(), ..Default::default() })));
    {
        let __go_lhs_map_1 = (*inv.lock().unwrap().as_ref().unwrap()).stock.clone();
        let __go_lhs_key_1 = "pear".to_string();
        let __tmp_0 = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))));
        let __tmp_1 = Arc::new(Mutex::new(Some(9)));
        (*inv.lock().unwrap().as_mut().unwrap()).stock = __tmp_0;
        (*__go_lhs_map_1.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_1, __tmp_1);
    };
    println!("{} {} {}", format!("{}", format_slice(&keys(old.clone()))), format!("{}", format_slice(&keys({ let __field = (*inv.lock().unwrap().as_ref().unwrap()).stock.clone(); __field }))), format!("{}", { let __map = { let __map_holder = old.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"pear".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }));
    let mut a = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    let mut b = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("b".to_string(), Arc::new(Mutex::new(Some(2i64))))]))));
    {
        let __tmp_0 = b.clone();
        let __tmp_1 = a.clone();
        a = __tmp_0;
        b = __tmp_1;
    };
    { let __map_key = "a2".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*a.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", format_slice(&keys(a.clone()))), format!("{}", format_slice(&keys(b.clone()))));
    {
        let __tmp_0 = a.clone();
        let __tmp_1 = { let __field = (*inv.lock().unwrap().as_ref().unwrap()).stock.clone(); __field };
        (*inv.lock().unwrap().as_mut().unwrap()).stock = __tmp_0;
        a = __tmp_1;
    };
    println!("{} {}", format!("{}", format_slice(&keys({ let __field = (*inv.lock().unwrap().as_ref().unwrap()).stock.clone(); __field }))), format!("{}", format_slice(&keys(a.clone()))));
    let mut c = a.clone();
    {
        let __tmp_0 = Arc::new(Mutex::new(None));
        let __tmp_1 = a.clone();
        a = __tmp_0;
        b = __tmp_1;
    };
    { let __go_print_arg_1 = keys(b.clone());
let mut __go_print_arg_1 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_1); Arc::new(Mutex::new(__slice_header)) }; let __go_print_arg_2 = keys(c.clone());
let mut __go_print_arg_2 = { let __slice_header = GoSliceHeader::go_slice_header(&__go_print_arg_2); Arc::new(Mutex::new(__slice_header)) }; let __go_print_arg_3 = Arc::new(Mutex::new(Some(({ let __len = (*a.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64))); let __go_print_value = println!("{} {} {} {}", format!("{}", { let __nil_result = (*a.lock().unwrap()).is_none(); __nil_result }), format!("{}", format_slice(&__go_print_arg_1)), format!("{}", format_slice(&__go_print_arg_2)), format!("{}", { let __v = (*__go_print_arg_3.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
}

impl GoValueClone for Inventory {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
