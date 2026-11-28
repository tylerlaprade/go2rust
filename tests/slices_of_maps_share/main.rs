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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
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
        if __text == GO_UNPRINTABLE_PANIC_VALUE {
            return format_any(&**__inner);
        }
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
pub struct Table {
    pub rows: Arc<Mutex<Option<Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>>>>>,
}

impl Table {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.rows.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            rows: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __maps = (*self.rows.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl Table {
    pub fn add(&mut self, row: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>) {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = self.rows.clone(); let __append_value = row.clone(); let mut __appended: Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<String>>>>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *self.rows.lock().unwrap() = __slice_header; };
    }
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let __go_previous_panic_hook = go_silence_panic_hook();
    let __go_panic_result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut s = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("x".to_string(), Arc::new(Mutex::new(Some(1i64))))]))))])));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s.clone(); let __append_value = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("z".to_string(), Arc::new(Mutex::new(Some(3i64))))])))); let mut __appended: Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *s.lock().unwrap() = __slice_header; };
        { let __map_key = "w".to_string(); let __map_value = Arc::new(Mutex::new(Some(0))); (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        let mut m = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("k".to_string(), Arc::new(Mutex::new(Some(5i64))))]))));
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = s.clone(); let __append_value = m.clone(); let mut __appended: Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *s.lock().unwrap() = __slice_header; };
        { let __map_key = "k".to_string(); let __map_value = Arc::new(Mutex::new(Some(6))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        println!("{} {}", format!("{}", { let __maps = (*s.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", ({ let __len = (*{ let __seq = { let __seq_holder = s.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
        let mut ms = Arc::new(Mutex::new(Some((0..(3) as usize).map(|_| Arc::new(Mutex::new(None))).collect::<Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>>())));
        println!("{} {} {}", format!("{}", { let __maps = (*ms.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }), format!("{}", ({ let __len = (*{ let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
        { let __idx = (0) as usize; let __rhs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([])))); (*ms.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        { let __map_key = "a".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*{ let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        let mut alias = { let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() };
        { let __map_key = "b".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*alias.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        println!("{} {}", format!("{}", format_map(&{ let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() })), format!("{}", ({ let __len = (*alias.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
        for i in 0..(({ let __range_holder = ms.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        if { let __nil_result = (*{ let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(i) as usize].clone() }.lock().unwrap()).is_none(); __nil_result } {
        { let __idx = (i) as usize; let __rhs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("i".to_string(), Arc::new(Mutex::new(Some(i as i64))))])))); (*ms.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
    }
    }
        { let __range_holder = ms.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, m) in __range_values.iter().enumerate() {
        { let __map_key = "seen".to_string(); let __map_value = Arc::new(Mutex::new(Some(i as i64))); (*m.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    } }
        println!("{}", format!("{}", { let __maps = (*ms.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }));
        let mut cp = Arc::new(Mutex::new(Some((0..(2) as usize).map(|_| Arc::new(Mutex::new(None))).collect::<Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>>())));
        let mut n = {
            let _src = { let __copy_src_holder = ms.clone(); let __copy_src_guard = __copy_src_holder.lock().unwrap(); __copy_src_guard.as_ref().map(|__s| __s.to_vec()).unwrap_or_default() };
            let _n = std::cmp::min((*cp.lock().unwrap().as_ref().unwrap()).len(), _src.len());
            for _i in 0.._n {
                (*cp.lock().unwrap().as_mut().unwrap())[_i] = _src[_i].clone();
            }
            Arc::new(Mutex::new(Some(_n as i64)))
        };
        { let __map_key = "copied".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*{ let __seq = { let __seq_holder = cp.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        println!("{} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __map = { let __map_holder = { let __seq = { let __seq_holder = ms.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"copied".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| 0) }), format!("{}", { let __maps = (*cp.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }));
        let mut arr: Arc<Mutex<Option<[Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<bool>>>>>>>; 2]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Arc::new(Mutex::new(None::<BTreeMap<String, Arc<Mutex<Option<bool>>>>>))))));
        { let __idx = (0) as usize; let __rhs = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<bool>>>>::from([("t".to_string(), Arc::new(Mutex::new(Some(true))))])))); (*arr.lock().unwrap().as_mut().unwrap())[__idx] = __rhs; };
        let mut arr2 = { let __owned = arr.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        { let __map_key = "f".to_string(); let __map_value = Arc::new(Mutex::new(Some(false))); (*{ let __seq = { let __seq_holder = arr2.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        println!("{} {} {}", format!("{}", { let __maps = (*arr.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", { let __maps = (*arr2.lock().unwrap()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = arr.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }));
        let mut t = Arc::new(Mutex::new(Some(Table { rows: Default::default() })));
        let mut row = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("name".to_string(), Arc::new(Mutex::new(Some("x".to_string()))))]))));
        { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.add(row.clone()); __result };
        { let __recv = t.clone(); let __recv_ptr: *mut Table = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut Table }; let __result = unsafe { &mut *__recv_ptr }.add(Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<String>>>>::from([("name".to_string(), Arc::new(Mutex::new(Some("a".to_string()))))]))))); __result };
        { let __map_key = "extra".to_string(); let __map_value = Arc::new(Mutex::new(Some("y".to_string()))); (*row.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        let t_closure_clone = t.clone(); { let __sort_target = (*t_closure_clone.lock().unwrap().as_ref().unwrap()).rows.clone(); let __sort_less = Arc::new(Mutex::new(Some({ let t_closure_clone_closure_clone = t_closure_clone.clone(); Arc::new(move |i: Arc<Mutex<Option<i64>>>, j: Arc<Mutex<Option<i64>>>| -> bool {
        return {
            let __tmp_x = { let __map = { let __map_holder = { let __seq = { let __seq_holder = (*t_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"name".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) };
            let __tmp_y = { let __map = { let __map_holder = { let __seq = { let __seq_holder = (*t_closure_clone_closure_clone.lock().unwrap().as_ref().unwrap()).rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[({ let __v = (*j.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"name".to_string())).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) };
            __tmp_x < __tmp_y
        };
    }) as Arc<dyn Fn(Arc<Mutex<Option<i64>>>, Arc<Mutex<Option<i64>>>) -> bool + Send + Sync> }))); let __sort_len = { let __sort_guard = __sort_target.lock().unwrap(); __sort_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }; for __sort_i in 1..__sort_len { let mut __sort_j = __sort_i; while __sort_j > 0 { let __should_swap = { let mut __less_guard = __sort_less.lock().unwrap(); let __less = __less_guard.as_mut().expect("sort.Slice less function is nil"); __less(Arc::new(Mutex::new(Some(__sort_j as i64))), Arc::new(Mutex::new(Some((__sort_j - 1) as i64)))) }; if !__should_swap { break; } { let mut __sort_guard = __sort_target.lock().unwrap(); if let Some(__sort_values) = __sort_guard.as_mut() { __sort_values.swap(__sort_j, __sort_j - 1); } } __sort_j -= 1; } } };
        println!("{} {} {}", format!("{}", { let __maps = (*{ let __field = (*t.lock().unwrap().as_ref().unwrap()).rows.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone().clone(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", ({ let __len = (*{ let __seq = { let __seq_holder = (*t.lock().unwrap().as_ref().unwrap()).rows.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __v = (*t.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));
        let mut groups = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Vec<Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<i64>>>>>>>>>>>>::from([]))));
        { let __map_key = "g".to_string(); let __map_value = { let __slice = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); __map_guard.as_ref().unwrap().get(&"g".to_string()).cloned().unwrap_or_else(|| Arc::new(Mutex::new(None))) }; let __append_values = vec![
            Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("v".to_string(), Arc::new(Mutex::new(Some(1i64))))])))),
            Arc::new(Mutex::new(None::<BTreeMap<String, Arc<Mutex<Option<i64>>>>>)),
        ]; let mut __appended = (*__slice.lock().unwrap()).clone().unwrap_or_default(); __appended.extend(__append_values); Arc::new(Mutex::new(Some(__appended))) }; (*groups.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
        { let __map_key = "w".to_string(); let __map_value = Arc::new(Mutex::new(Some(2))); (*{ let __seq = { let __seq_holder = { let __map = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"g".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
        println!("{} {} {}", format!("{}", { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); match __map_guard.as_ref() { Some(__map) => format!("map[{}]", __map.iter().map(|(__k, __v)| { let __maps = (*__v.lock().unwrap()).clone().unwrap_or_default(); format!("{}:[{}]", go_display_value(__k), __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }).collect::<Vec<_>>().join(" ")), None => "map[]".to_string() } }), format!("{}", ({ let __len = (*{ let __map = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"g".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })), format!("{}", { let __nil_result = (*{ let __seq = { let __seq_holder = { let __map = { let __map_holder = groups.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&"g".to_string())).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap()).is_none(); __nil_result }));
        __defer_stack.push(Box::new(move || {
        { let __f_holder = Arc::new(Mutex::new(Some(Arc::new(move || {
        println!("{} {}", format!("{}", "recovered:".to_string()), format!("{}", { let __text = match (go_recover()).lock().unwrap().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }; __text }));
    }) as Arc<dyn Fn() -> () + Send + Sync>))); let __f: Arc<dyn Fn() -> () + Send + Sync> = __f_holder.lock().unwrap().as_ref().unwrap().clone(); (__f)() };
    }));
        let mut nilMaps = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None::<BTreeMap<String, Arc<Mutex<Option<i64>>>>>))])));
        { let __map_key = "boom".to_string(); let __map_value = Arc::new(Mutex::new(Some(1))); (*{ let __seq = { let __seq_holder = nilMaps.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };

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

impl GoValueClone for Table {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
