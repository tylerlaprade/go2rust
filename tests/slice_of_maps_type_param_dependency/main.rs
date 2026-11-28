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
use std::fmt::{Display};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display + Clone,
{
    format_map_nil(map, "<nil>")
}

// format_map_nil prints a value the map holds no value for as __nil_text:
// [] for a nil slice, as Go prints one.
fn format_map_nil<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>, __nil_text: &str) -> String
where
    V: Display + Clone,
{
    // The entries are copied out first, in key order, so a value's String
    // method that reads this map does not wait on the lock printing holds.
    let items: Option<Vec<(K, Rc<RefCell<Option<V>>>)>> = { let guard = map.borrow(); guard.as_ref().map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect()) };
    if let Some(items) = items {
        let formatted: Vec<String> = items
            .iter()
            .map(|(k, v)| {
                // The value is copied out too, so its String method may
                // read its own entry.
                let value = { let v_guard = v.borrow(); v_guard.clone() };
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

fn main() {
    go_install_panic_exit();
    ::example_com_slicemapsparam_dep::__go_init_all();

    let mut s = Rc::new(RefCell::new(Some(vec![Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("a".to_string(), Rc::new(RefCell::new(Some(1i64))))])))), Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("b".to_string(), Rc::new(RefCell::new(Some(2i64))))]))))])));
    let mut m = example_com_slicemapsparam_dep::first::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>(s.clone());
    { let __map_key = "c".to_string(); let __map_value = Rc::new(RefCell::new(Some(3))); (*m.borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    { let __map_key = "d".to_string(); let __map_value = Rc::new(RefCell::new(Some(4))); (*(*s.borrow().as_ref().unwrap())[(1) as usize].clone().borrow_mut().as_mut().unwrap_or_else(|| panic!("assignment to entry in nil map"))).insert(__map_key, __map_value); };
    { let __map_key = "e".to_string(); let __map_value = Rc::new(RefCell::new(Some(5))); (*example_com_slicemapsparam_dep::pick::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>, String>(s.clone(), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some("x".to_string())))).borrow_mut().as_mut().unwrap()).insert(__map_key, __map_value); };
    println!("{} {}", format!("{}", { let __maps = (*s.borrow()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", format_map(&m)));
    example_com_slicemapsparam_dep::mark::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>(s.clone(), Rc::new(RefCell::new(Some("k".to_string()))));
    println!("{}", format!("{}", { let __maps = (*s.borrow()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }));
    example_com_slicemapsparam_dep::swap::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>(s.clone());
    println!("{} {}", format!("{}", { let __maps = (*s.borrow()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", format_map(&m)));
    example_com_slicemapsparam_dep::fresh::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>(s.clone(), Rc::new(RefCell::new(Some(0))));
    { let __go_print_arg_1 = example_com_slicemapsparam_dep::count::<Vec<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>>(s.clone()); let __go_print_value = println!("{} {}", format!("{}", { let __maps = (*s.borrow()).as_ref().map(|__v| __v.to_vec()).unwrap_or_default(); format!("[{}]", __maps.iter().map(|__m| format_map(__m)).collect::<Vec<_>>().join(" ")) }), format!("{}", __go_print_arg_1)); __go_print_value };
}