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
pub struct Lists(pub Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Vec<i64>>>>>>>>);

impl Display for Lists {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", format_map_nil(&self.0, "[]"))
    }
}


#[derive(Clone)]
pub struct Index {
    pub words: Arc<Mutex<Option<__ptrSlot_map_string_int>>>,
}

impl Index {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.words.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            words: __go_clone_0_0,
        }
    }
}


impl Default for Index {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(__ptrSlot_map_string_int::default())));
        Self {
            words: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Index {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.words.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Clone, Default)]
pub struct __ptrSlot_Lists {
    pub get: Arc<Mutex<Option<Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync>>>>,
    pub set: Arc<Mutex<Option<Arc<dyn Fn(Arc<Mutex<Option<Lists>>>) -> () + Send + Sync>>>>,
}

impl __ptrSlot_Lists {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.get.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.set.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            get: __go_clone_0_0,
            set: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for __ptrSlot_Lists {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", "<func>");
        let __go_fmt_1 = format!("{}", "<func>");
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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


pub fn size(p: Arc<Mutex<Option<__ptrSlot_map_string_int>>>) -> i64 {
    ({ let __len = (*{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64
}

pub fn add(p: Arc<Mutex<Option<__ptrSlot_map_string_int>>>, k: Arc<Mutex<Option<String>>>) {
    { let __map_holder = ({ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).clone(); let __key = ({ let __v = (*k.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
}

pub fn grow(p: Arc<Mutex<Option<__ptrSlot_Lists>>>) {
    { let __map_key = "g".to_string(); let __map_value = Arc::new(Mutex::new(Some(vec![1i64]))); (*{ let __named_map = (*({ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn keys(p: Arc<Mutex<Option<__ptrSlot_map_string_int>>>) -> Arc<Mutex<Option<Vec<String>>>> {
    let mut ks: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (k, _) in { let __range_holder = { let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = ks.clone(); let __append_value = k.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *ks.lock().unwrap() = __slice_header; };
    }
    sort::strings(ks.clone());
    return ks.clone();
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut p = { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> {
        let mut __slot_p1: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
        let mut __cell___slot_p1_1 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: __slot_p1.clone() })));
        let __cell___slot_p1_1_closure_clone = __cell___slot_p1_1.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_map_string_int {
            get: Arc::new(Mutex::new(Some({ let __cell___slot_p1_1_closure_clone_closure_clone = __cell___slot_p1_1_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell___slot_p1_1_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell___slot_p1_1_closure_clone_closure_clone = __cell___slot_p1_1_closure_clone.clone(); Arc::new(move |__slot_v2: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v2.clone(); (*__cell___slot_p1_1_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> + Send + Sync>))); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    println!("{}", format!("{}", size(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))))));
    add(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Arc::new(Mutex::new(Some("b".to_string()))));
    add(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Arc::new(Mutex::new(Some("a".to_string()))));
    println!("{} {} {} {}", format!("{}", ({ let __len = (*{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __map = { let __map_holder = ({ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"a".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __map = { let __map_holder = ({ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"b".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", format_slice(&keys(Arc::new(Mutex::new(Some({ let __arg_holder = p.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() })))))));
    let mut raw = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1i64))))]))));
    let mut __cell_raw_2 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: raw.clone() })));
    let __cell_raw_2_closure_clone = __cell_raw_2.clone(); let mut q = Arc::new(Mutex::new(Some(__ptrSlot_map_string_int {
        get: Arc::new(Mutex::new(Some({ let __cell_raw_2_closure_clone_closure_clone = __cell_raw_2_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell_raw_2_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_raw_2_closure_clone_closure_clone = __cell_raw_2_closure_clone.clone(); Arc::new(move |__slot_v3: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v3.clone(); (*__cell_raw_2_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
        ..Default::default()
    })));
    add(Arc::new(Mutex::new(Some({ let __arg_holder = q.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))), Arc::new(Mutex::new(Some("y".to_string()))));
    { let __map_handle = { let __f_holder = (*q.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.clone(); let __delete_key = &"x".to_string(); let mut __map_guard = __map_handle.lock().unwrap(); if let Some(__map) = __map_guard.as_mut() { __map.remove(__delete_key); } };
    let __cell_raw_2_closure_clone = __cell_raw_2.clone(); println!("{} {} {}", format!("{}", size(Arc::new(Mutex::new(Some({ let __arg_holder = q.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))))), format!("{}", ({ let __len = ({ let __len_target = { let __field = (*__cell_raw_2_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }); __len })), format!("{}", format_slice(&keys(Arc::new(Mutex::new(Some({ let __arg_value = __ptrSlot_map_string_int {
        get: Arc::new(Mutex::new(Some({ let __cell_raw_2_closure_clone_closure_clone = __cell_raw_2_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell_raw_2_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_raw_2_closure_clone_closure_clone = __cell_raw_2_closure_clone.clone(); Arc::new(move |__slot_v4: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v4.clone(); (*__cell_raw_2_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
        ..Default::default()
    }; __arg_value })))))));
    { let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("z".to_string(), Arc::new(Mutex::new(Some(9i64))))]))))) };
    println!("{} {}", format!("{}", format_map(&{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() })), format!("{}", ({ let __len = (*{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    { let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(None))) };
    println!("{} {}", format!("{}", { let __nil_result = (*{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).is_none(); __nil_result }), format!("{}", ({ let __len = (*{ let __f_holder = (*p.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
    let mut idx = Arc::new(Mutex::new(Some(Index { words: { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> {
        let mut __slot_p5: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))));
        let mut __cell___slot_p5_3 = Arc::new(Mutex::new(Some(AnonymousStruct1 { v: __slot_p5.clone() })));
        let __cell___slot_p5_3_closure_clone = __cell___slot_p5_3.clone(); return Arc::new(Mutex::new(Some(__ptrSlot_map_string_int {
            get: Arc::new(Mutex::new(Some({ let __cell___slot_p5_3_closure_clone_closure_clone = __cell___slot_p5_3_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> {
        return (*__cell___slot_p5_3_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> }))),
            set: Arc::new(Mutex::new(Some({ let __cell___slot_p5_3_closure_clone_closure_clone = __cell___slot_p5_3_closure_clone.clone(); Arc::new(move |__slot_v6: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>| {
        { let new_val = __slot_v6.clone(); (*__cell___slot_p5_3_closure_clone_closure_clone.lock().unwrap().as_mut().unwrap()).v = new_val; };
    }) as Arc<dyn Fn(Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>) -> () + Send + Sync> }))),
            ..Default::default()
        })));
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> + Send + Sync>))); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<__ptrSlot_map_string_int>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }, ..Default::default() })));
    let idx_closure_clone = idx.clone(); let mut count = Arc::new(Mutex::new(Some(Arc::new(move |w: Arc<Mutex<Option<String>>>| {
        { let __map_holder = ({ let __f_holder = (*(*idx_closure_clone.lock().unwrap().as_ref().unwrap()).words.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).clone(); let __key = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }); let mut __map_guard = __map_holder.lock().unwrap(); let __map = __map_guard.as_mut().unwrap(); let __entry = __map.entry(__key).or_insert_with(|| Arc::new(Mutex::new(Some(0)))); let mut __value = __entry.lock().unwrap(); * __value = Some((*__value.as_ref().unwrap()).clone() + 1); }
    }) as Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> () + Send + Sync>)));
    for w in &vec!["go".to_string(), "rust".to_string(), "go".to_string()] {
        {
            let __f: Arc<dyn Fn(Arc<Mutex<Option<String>>>) -> () + Send + Sync> = count.lock().unwrap().as_ref().unwrap().clone();
            (__f)(
                Arc::new(Mutex::new(Some(w.clone())))
            )
        };
    }
    println!("{} {}", format!("{}", format_map(&{ let __f_holder = (*(*idx.lock().unwrap().as_ref().unwrap()).words.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() })), format!("{}", format_slice(&keys(Arc::new(Mutex::new(Some({ let __selector_holder = (*idx.lock().unwrap().as_ref().unwrap()).words.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = __selector_guard.as_ref().unwrap().__go_value_clone(); std::mem::drop(__selector_guard); __cloned })))))));
    let mut l = Arc::new(Mutex::new(Some(Lists(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("a".to_string(), Arc::new(Mutex::new(Some(vec![1i64]))))]))))))));
    let mut __cell_l_4 = Arc::new(Mutex::new(Some(AnonymousStruct2 { v: l.clone() })));
    let __cell_l_4_closure_clone = __cell_l_4.clone(); let mut lp = Arc::new(Mutex::new(Some(__ptrSlot_Lists {
        get: Arc::new(Mutex::new(Some({ let __cell_l_4_closure_clone_closure_clone = __cell_l_4_closure_clone.clone(); Arc::new(move || -> Arc<Mutex<Option<Lists>>> {
        return (*__cell_l_4_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.clone();
    }) as Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync> }))),
        set: Arc::new(Mutex::new(Some({ let __cell_l_4_closure_clone_closure_clone = __cell_l_4_closure_clone.clone(); Arc::new(move |__slot_v7: Arc<Mutex<Option<Lists>>>| {
        { let new_val = __slot_v7.lock().unwrap().as_ref().unwrap().clone(); *(*__cell_l_4_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).v.lock().unwrap() = Some(new_val); };
    }) as Arc<dyn Fn(Arc<Mutex<Option<Lists>>>) -> () + Send + Sync> }))),
        ..Default::default()
    })));
    grow(Arc::new(Mutex::new(Some({ let __arg_holder = lp.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }))));
    println!("{} {} {}", format!("{}", { let __named_map_holder = { let __f_holder = (*lp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }), format!("{}", { let __named_map_holder = (*__cell_l_4.lock().unwrap().as_ref().unwrap()).v.clone(); let __named_map_guard = __named_map_holder.lock().unwrap(); let __map_holder = __named_map_guard.as_ref().map(|__v| __v.0.clone()); std::mem::drop(__named_map_guard); __map_holder.as_ref().map(|__map_holder| { let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }).unwrap_or(0) }), format!("{}", format_slice(&{ let __map = { let __map_holder = { let __named_map = (*({ let __f_holder = (*lp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }).lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"g".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) })));
    { let __f_holder = (*lp.lock().unwrap().as_ref().unwrap()).set.clone(); let __f: Arc<dyn Fn(Arc<Mutex<Option<Lists>>>) -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)(Arc::new(Mutex::new(Some(Lists(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<i64>>>>>::from([("z".to_string(), Arc::new(Mutex::new(Some(vec![9i64]))))]))))))))) };
    { let __go_print_arg_1 = { let __f_holder = (*lp.lock().unwrap().as_ref().unwrap()).get.clone(); let __f: Arc<dyn Fn() -> Arc<Mutex<Option<Lists>>> + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() }; let __go_print_value = println!("{} {}", format!("{}", format_map_nil(&{ let __named_map = (*(*__cell_l_4.lock().unwrap().as_ref().unwrap()).v.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }, "[]")), format!("{}", format_map_nil(&{ let __named_map = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).0.clone(); __named_map }, "[]"))); __go_print_value };
}

#[derive(Debug, Clone, Default)]
pub struct AnonymousStruct1 {
    pub v: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>,
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
        let __go_fmt_0 = format!("{}", format_map(&self.v));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub v: Arc<Mutex<Option<Lists>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.v.clone();
        Self {
            v: __go_clone_0_0,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Lists::default())));
        Self {
            v: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.v.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl GoValueClone for Index {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for __ptrSlot_Lists {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for __ptrSlot_map_string_int {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
