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
    go_install_panic_exit();
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
    { let new_val = 20; let __deref_target = { let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); *__deref_target.lock().unwrap() = Some(new_val); };
    println!("{} {} {} {}", format!("{}", { let __v = (*{ let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&1)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*{ let __map = { let __map_holder = mp.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&2)).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*one.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*two.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let mut counts = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([]))));
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    for w in &vec!["x".to_string(), "y".to_string(), "x".to_string()] {
        {
            let __go_lhs_map_0 = counts.clone();
            let __go_lhs_key_0 = w.clone();
            let __tmp_0 = Arc::new(Mutex::new(Some({ let __tmp_x = { let __map = { let __map_holder = counts.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(w)).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }; let __tmp_y = 1; __tmp_x + __tmp_y })));
            let __tmp_1 = { let __tmp_x = { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x + __tmp_y };
            (*__go_lhs_map_0.lock().unwrap().as_mut().unwrap()).insert(__go_lhs_key_0, __tmp_0);
            *total.lock().unwrap() = Some(__tmp_1);
        };
    }
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
