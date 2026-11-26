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
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

fn format_map<K: Display + Ord + Clone, V>(map: &Arc<Mutex<Option<BTreeMap<K, Arc<Mutex<Option<V>>>>>>>) -> String
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
                    format!("{}:<nil>", k)
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

fn format_any(value: &(dyn Any + Send + Sync)) -> String {
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
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

/// The Go func value a func-typed expression holds, or None for a nil func.
/// A copy of a func value is a clone of the same closure.
trait GoFuncValue {
    type Func: ?Sized;
    fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>>;
}

macro_rules! impl_go_func_value {
    ($(($($arg:ident),*)),* $(,)?) => { $(
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { Some(self.clone()) }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.clone() }
        }
        impl<R: 'static, $($arg: 'static),*> GoFuncValue for std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn Fn($($arg),*) -> R + Send + Sync>>>> {
            type Func = dyn Fn($($arg),*) -> R + Send + Sync;
            fn go_func_value(&self) -> Option<std::sync::Arc<Self::Func>> { self.lock().unwrap().clone() }
        }
    )* };
}
impl_go_func_value!((), (A0), (A0, A1), (A0, A1, A2), (A0, A1, A2, A3), (A0, A1, A2, A3, A4), (A0, A1, A2, A3, A4, A5), (A0, A1, A2, A3, A4, A5, A6), (A0, A1, A2, A3, A4, A5, A6, A7), (A0, A1, A2, A3, A4, A5, A6, A7, A8), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10), (A0, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10, A11));

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
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any + Send + Sync>); }
    None
}

fn go_any_clone(value: &(dyn Any + Send + Sync)) -> Box<dyn Any + Send + Sync> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
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
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
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

#[derive(Debug, Clone)]
pub struct Node {
    pub v: Arc<Mutex<Option<i64>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub next: Arc<Mutex<Option<Node>>>,
    pub kids: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Node>>>>>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.v.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.next.clone();
        let __go_clone_3_0 = { let __guard = self.kids.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            v: __go_clone_0_0,
            name: __go_clone_1_0,
            next: __go_clone_2_0,
            kids: __go_clone_3_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        let __go_default_3_0 = Arc::new(Mutex::new(None));
        Self {
            v: __go_default_0_0,
            name: __go_default_1_0,
            next: __go_default_2_0,
            kids: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", { let __guard = self.next.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_3 = format!("{}", format_slice_wrapped(&self.kids));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}
impl GoComparable for Node {
    fn go_eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
    fn go_hash(&self, seed: usize) -> usize {
        let mut __hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&seed, &mut __hasher);
        std::hash::Hash::hash(&(self as *const Self as usize), &mut __hasher);
        std::hash::Hasher::finish(&__hasher) as usize
    }
}


pub fn walk(n: Arc<Mutex<Option<Node>>>, f: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync>>>>) {
    {
        let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
        (__f)(
            n.clone()
        )
    };
    { let __range_holder = (*n.lock().unwrap().as_ref().unwrap()).kids.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for k in __range_values.iter() {
        walk(k.clone(), Arc::new(Mutex::new(GoFuncValue::go_func_value(&(f)))));
    } }
}

/// find records the node a callback sees.
pub fn find(root: Arc<Mutex<Option<Node>>>, name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Node>>>, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
    let mut found: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
    let mut __cell_found_1 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: found.clone() })));
    let __cell_found_1_closure_clone = __cell_found_1.clone(); let name_closure_clone = name.clone(); walk(
        root.clone(),
        Arc::new(Mutex::new(Some(Arc::new(move |n: Arc<Mutex<Option<Node>>>| {
        if { let __tmp_x = { let __selector_holder = (*n.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned }; let __tmp_y = ({ let __v = (*name_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }); __tmp_x == __tmp_y } {
        { let new_val = n.clone(); (*__cell_found_1_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> () + Send + Sync>)))
    );
    if { let __nil_target = (*__cell_found_1.lock().unwrap().as_ref().unwrap()).v.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        return (
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(Arc::<dyn StdError + Send + Sync>::from(Box::<dyn StdError + Send + Sync>::from(format!("{}{}", "not found: ".to_string(), { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v }))))))
        );
    }
    return ({ let __return_value_0 = (*__cell_found_1.lock().unwrap().as_ref().unwrap()).v.clone(); __return_value_0 }, Arc::new(Mutex::new(None)));
}

/// lazy builds its map on the first call only.
pub fn lazy() -> Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync>>>> {
    let mut cache: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
    let mut __cell_cache_2 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: cache.clone() })));
    let __cell_cache_2_closure_clone = __cell_cache_2.clone(); return Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        if { let __nil_target = (*__cell_cache_2_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result } {
        println!("{}", format!("{}", "building".to_string()));
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); (*__cell_cache_2_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }
        return (*__cell_cache_2_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync>)));
}

/// recovered sets a pointer from a deferred closure.
pub fn recovered() -> Arc<Mutex<Option<Node>>> {
    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let mut n: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut last: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_last_3 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: last.clone() })));
        let __cell_last_3_defer_captured = __cell_last_3.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        {
        let mut r = go_recover();;
        if !{ let __nil_result = (*r.lock().unwrap()).as_ref().map_or(true, |__any| (**__any).is::<GoNilAny>()); __nil_result } {
            println!("{} {} {}", format!("{}", "recovered".to_string()), format!("{}", { let __text = match r.lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }), format!("{}", (*(*(*__cell_last_3_defer_captured.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));;
        }
    }
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    }));
        { let new_val = Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(41))), ..Default::default() }))).clone(); (*__cell_last_3.lock().unwrap().as_mut().unwrap()).v = new_val; };
        std::panic::panic_any(Box::new("boom".to_string()) as Box<dyn Any + Send + Sync>);
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
            n.clone()
        }
    }
}

pub fn nodes(vs: Arc<Mutex<Option<Vec<i64>>>>) -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    let mut vs = Arc::new(Mutex::new((*vs.lock().unwrap()).clone()));
    let vs_closure_clone = vs.clone(); return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>>>>| {
        { let __range_holder = vs_closure_clone.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for v in __range_values.iter().copied() {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(v))), ..Default::default() })))
            )
        } {
        return;
    }
    } }
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

pub fn maps() -> Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>>>> {
    return Arc::new(Mutex::new(Some(Arc::new(move |r#yield: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>>>>| {
        if !{
            let __f: Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))))
            )
        } {
        return;
    }
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync> = r#yield.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("b".to_string(), Arc::new(Mutex::new(Some(2i64))))]))))
            )
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync>)));
}

/// firstMap returns nil from the body of a range over a function.
pub fn first_map(stop: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
    let mut __rf_done_1: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
    let mut __rf_result_1_0: Arc<Mutex<Option<AnonymousStruct2>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut __rf_done_1_closure_clone = __rf_done_1.clone(); let __rf_result_1_0_closure_clone = __rf_result_1_0.clone(); let stop_closure_clone = stop.clone(); { let __f_holder = maps(); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_1_0: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| -> bool {
        let mut __rf_done_1_closure_clone = __rf_done_1_closure_clone.clone();
        let mut m = __rf_param_1_0.clone();
        if { let __v = (*stop_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
            { let new_val = Arc::new(Mutex::new(None)); (*__rf_result_1_0_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
            { let new_val = true; *__rf_done_1_closure_clone.lock().unwrap() = Some(new_val); };
            return false;
        }
    }
        {
    { let new_val = m.clone(); (*__rf_result_1_0_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    { let new_val = true; *__rf_done_1_closure_clone.lock().unwrap() = Some(new_val); };
    return false;
}
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>)))) };
    if { let __v = (*__rf_done_1.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return (*__rf_result_1_0.lock().unwrap().as_ref().unwrap()).v.clone();
    }
    Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))))
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut head: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_head_4 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: head.clone() })));
        let __cell_head_4_closure_clone = __cell_head_4.clone(); let mut push = Arc::new(Mutex::new(Some(Arc::new(move |v: Arc<Mutex<Option<i64>>>| {
        { let new_val = Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some({ let __arg_holder = v.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), next: { let __field = (*__cell_head_4_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone(); __field }, ..Default::default() }))).clone(); (*__cell_head_4_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync>)));
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = push.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(1)))
            )
        };
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = push.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(2)))
            )
        };
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<i64>>>) -> () + Send + Sync> = push.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(3)))
            )
        };
        let mut n = (*__cell_head_4.lock().unwrap().as_ref().unwrap()).v.clone();
    while { let __nil_result = (*n.lock().unwrap()).is_some(); __nil_result } {
        print!("{}{}", format!("{}", (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", " ".to_string()));
        { let new_val = (*n.lock().unwrap().as_ref().unwrap()).next.clone(); n = new_val; };
    }
        println!();
        let mut m: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
        let mut __cell_m_5 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: m.clone() })));
        let __cell_m_5_closure_clone = __cell_m_5.clone(); let mut initM = Arc::new(Mutex::new(Some(Arc::new(move || {
        { let new_val = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))])))); (*__cell_m_5_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn() -> () + Send + Sync>)));
        {
            let __f: Arc<dyn Fn() -> () + Send + Sync> = initM.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        };
        { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*(*__cell_m_5.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        println!("{} {}", format!("{}", format_map(&(*__cell_m_5.lock().unwrap().as_ref().unwrap()).v)), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*__cell_m_5.lock().unwrap().as_ref().unwrap()).v.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })));
        let mut p: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(None));
        let mut __cell_p_6 = Arc::new(Mutex::new(Some(AnonymousStruct3 { v: p.clone() })));
        let mut x = Arc::new(Mutex::new(Some(5i64)));
        let __cell_p_6_closure_clone = __cell_p_6.clone(); let x_closure_clone = x.clone(); { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        { let new_val = x_closure_clone.clone().clone(); (*__cell_p_6_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
        { let new_val = 6; *(*__cell_p_6.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
        println!("{} {}", format!("{}", { let __v = (*x.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*(*__cell_p_6.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        let mut ch: GoChannel<i64> = <GoChannel<i64>>::default();
        let mut __cell_ch_7 = Arc::new(Mutex::new(Some(AnonymousStruct4 { v: ch.clone() })));
        println!("{}", format!("{}", { let __channel = (*__cell_ch_7.lock().unwrap().as_ref().unwrap()).v.clone(); __channel }.is_nil()));
        let __cell_ch_7_closure_clone = __cell_ch_7.clone(); { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        (*__cell_ch_7_closure_clone.lock().unwrap().as_mut().unwrap()).v = GoChannel::<i64>::new_buffered(1 as usize);
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
        { let __channel = (*__cell_ch_7.lock().unwrap().as_ref().unwrap()).v.clone(); __channel }.send(1);
        println!("{} {}", format!("{}", { let __channel = (*__cell_ch_7.lock().unwrap().as_ref().unwrap()).v.clone(); __channel }.recv().unwrap_or_default()), format!("{}", { let __channel = (*__cell_ch_7.lock().unwrap().as_ref().unwrap()).v.clone(); __channel }.is_nil()));
        let mut root = Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("root".to_string()))), kids: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("a".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("b".to_string()))), kids: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Node { name: Arc::new(Mutex::new(Some("c".to_string()))), v: Arc::new(Mutex::new(Some(7))), ..Default::default() })))]))), ..Default::default() })))]))), ..Default::default() })));
        let (mut c, mut err) = find(root.clone(), Arc::new(Mutex::new(Some("c".to_string()))));
        println!("{} {}", format!("{}", (*{ let __field = (*c.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        { let (__tmp_0, __tmp_1) = find(root.clone(), Arc::new(Mutex::new(Some("zz".to_string())))); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        println!("{}", format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        let mut get = lazy();
        let mut get = Arc::new(Mutex::new(GoFuncValue::go_func_value(&get)));
        println!("{} {}", format!("{}", { let __map = { let __map_holder = {
            let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = get.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", ({ let __len = (*{
            let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = get.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
        let mut list = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(3))), ..Default::default() }))), Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(9))), ..Default::default() }))), Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(1))), ..Default::default() })))])));
        let mut best: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_best_8 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: best.clone() })));
        let __cell_best_8_closure_clone = __cell_best_8.clone(); let list_closure_clone = list.clone(); { let __sort_target = list_closure_clone.clone(); let __sort_less = Arc::new(Mutex::new(Some(Arc::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        if {
            let __go_cond_0 = { let __nil_target = (*__cell_best_8_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result };
            if __go_cond_0 {
                true
            } else {
                let __go_cond_1 = {
                    let __tmp_x = (*(*{ let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
                    let __tmp_y = (*(*(*__cell_best_8_closure_clone.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
                    __tmp_x > __tmp_y
                };
                __go_cond_1
            }
        } {
        { let new_val = { let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(); (*__cell_best_8_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }
        return {
            let __tmp_x = (*(*{ let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
            let __tmp_y = (*(*{ let __seq = { let __seq_holder = list_closure_clone.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
            __tmp_x < __tmp_y
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync>))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
        println!("{} {}", format!("{}", (*(*(*__cell_best_8.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*{ let __seq = { let __seq_holder = list.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));
        let mut cur: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_cur_9 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: cur.clone() })));
        let mut fs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
        let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x < __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y }))), ..Default::default() }))).clone(); (*__cell_cur_9.lock().unwrap().as_mut().unwrap()).v = new_val; };
        let __cell_cur_9_closure_clone = __cell_cur_9.clone(); { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = fs.clone(); let __append_value = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        return (*(*(*__cell_cur_9_closure_clone.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap());
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *fs.lock().unwrap() = __slice_header; };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let __range_holder = fs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter().cloned() {
        print!("{}{}", format!("{}", {
            let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }), format!("{}", " ".to_string()));
    } }
        println!();
        println!("{}", format!("{}", { let __nil_result = (*recovered().lock().unwrap()).is_none(); __nil_result }));
        let mut q: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_q_10 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: q.clone() })));
        let mut __rf_done_2: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let __cell_q_10_closure_clone = __cell_q_10.clone(); { let __f_holder = nodes(Arc::new(Mutex::new(Some(vec![1, 2, 3])))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_2_0: Arc<Mutex<Option<Node>>>| -> bool {
        { let new_val = __rf_param_2_0.clone(); (*__cell_q_10_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
        println!("{} {}", format!("{}", "bound".to_string()), format!("{}", (*(*(*__cell_q_10_closure_clone.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));
        { let new_val = Arc::new(Mutex::new(None)); (*__cell_q_10_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
        return false;
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>)))) };
        if { let __v = (*__rf_done_2.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        println!("{}", format!("{}", { let __nil_target = (*__cell_q_10.lock().unwrap().as_ref().unwrap()).v.clone(); let __nil_result = (*__nil_target.lock().unwrap()).is_none(); __nil_result }));
        let mut last: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_last_11 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: last.clone() })));
        let mut __rf_done_3: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let __cell_last_11_closure_clone = __cell_last_11.clone(); { let __f_holder = nodes(Arc::new(Mutex::new(Some(vec![7, 8, 9])))); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_3_0: Arc<Mutex<Option<Node>>>| -> bool {
        let mut n = __rf_param_3_0.clone();
        if { let __tmp_x = (*{ let __field = (*n.lock().unwrap().as_ref().unwrap()).v.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 7; __tmp_x > __tmp_y } {
        { let new_val = n.clone(); (*__cell_last_11_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<Node>>>) -> bool + Send + Sync>)))) };
        if { let __v = (*__rf_done_3.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        println!("{}", format!("{}", (*(*(*__cell_last_11.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));
        let mut lm: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(None));
        let mut __cell_lm_12 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: lm.clone() })));
        let mut __rf_done_4: Arc<Mutex<Option<bool>>> = Arc::new(Mutex::new(Some(false)));
        let __cell_lm_12_closure_clone = __cell_lm_12.clone(); { let __f_holder = maps(); let __f: Arc<dyn Fn(Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Arc::new(move |__rf_param_4_0: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| -> bool {
        { let new_val = __rf_param_4_0.clone(); (*__cell_lm_12_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
        true
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> bool + Send + Sync>)))) };
        if { let __v = (*__rf_done_4.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return;
    }
    }
        println!("{}", format!("{}", format_map(&(*__cell_lm_12.lock().unwrap().as_ref().unwrap()).v)));
        println!("{} {}", format!("{}", { let __nil_result = (*first_map(Arc::new(Mutex::new(Some(true)))).lock().unwrap()).is_none(); __nil_result }), format!("{}", format_map(&first_map(Arc::new(Mutex::new(Some(false)))))));
        let mut done: Arc<Mutex<Option<Node>>> = Arc::new(Mutex::new(None));
        let mut __cell_done_13 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: done.clone() })));
        let __cell_done_13_defer_captured = __cell_done_13.clone(); __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        println!("{} {}", format!("{}", "done".to_string()), format!("{}", (*(*(*__cell_done_13_defer_captured.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap())));
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    }));
        { let new_val = Arc::new(Mutex::new(Some(Node { v: Arc::new(Mutex::new(Some(42))), ..Default::default() }))).clone(); (*__cell_done_13.lock().unwrap().as_mut().unwrap()).v = new_val; };

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

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub v: Arc<Mutex<Option<Node>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct2 {
    pub v: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", format_map(&self.v));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct3 {
    pub v: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.v.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct4 {
    pub v: GoChannel<i64>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}


impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
