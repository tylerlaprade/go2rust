#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};


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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    std::any::type_name_of_val(val)
}

pub(crate) static memo: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<BTreeMap<i64, Arc<Mutex<Option<i64>>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *memo.lock().unwrap() = Some(BTreeMap::new());
    {
        let mut __go_map = BTreeMap::<i64, Arc<Mutex<Option<i64>>>>::new();
        *memo.lock().unwrap() = Some(__go_map);
    }
}


pub fn fib(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x < __tmp_y } {
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }
    {
        let (mut v, mut ok) = { let __map = { let __map_holder = memo.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; match __map.as_ref().and_then(|__map| __map.get(&{ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v })) { /* MAP_COMMA_OK */ Some(v) => (Arc::new(Mutex::new(Some(v.lock().unwrap().as_ref().unwrap().clone()))), true), None => (Arc::new(Mutex::new(Some(0))), false) } };;
        if ok {
            return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        }
    }
    let mut v = Arc::new(Mutex::new(Some({ let __tmp_x = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); let __tmp_y = fib(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x - __tmp_y })))); __tmp_x + __tmp_y })));
    { let __map_key = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __map_value = Arc::new(Mutex::new(Some((*v.lock().unwrap().as_ref().unwrap()).clone()))); (*memo.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
    return { let __v = (*v.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn factorial(n: Arc<Mutex<Option<i64>>>) -> i64 {
    if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x <= __tmp_y } {
        return 1;
    }
    return { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = factorial(Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x - __tmp_y })))); __tmp_x * __tmp_y };
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::unicode_utf8::__go_init_all();

    __go_init_all();
    println!("{} {}", format!("{}", fib(Arc::new(Mutex::new(Some(90))))), format!("{}", factorial(Arc::new(Mutex::new(Some(20))))));
    let mut big = Arc::new(Mutex::new(Some(1099511627776i64)));
    println!("{} {} {} {}", format!("{}", { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __tmp_x = { let __v = (*big.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y }), format!("{}", 9223372036854775807i64), format!("{}", i64::MIN));
    let mut total: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 100000; __tmp_x < __tmp_y } {
        { let __rhs = { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    println!("{}", format!("{}", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    let (mut n, _) = strconv::atoi(Arc::new(Mutex::new(Some("9000000000".to_string()))));
    println!("{} {}", format!("{}", { let __tmp_x = n; let __tmp_y = 1; __tmp_x + __tmp_y }), format!("{}", (*strconv::itoa(Arc::new(Mutex::new(Some({ let __tmp_x = n; let __tmp_y = 2; __tmp_x * __tmp_y })))).lock().unwrap().as_ref().unwrap())));
    let mut wrapped = Arc::new(Mutex::new(Some(9223372036854775807i64)));
    { let mut guard = wrapped.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    println!("{}", format!("{}", { let __tmp_x = { let __v = (*wrapped.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = i64::MIN; __tmp_x == __tmp_y }));
    let mut x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(7i64) as Box<dyn Any + Send + Sync>)));
    let (_, mut isInt) = ({
        let val = x.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<i64>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(0))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(0))), false)
        }
    });
    print!("{} {} {}\n", format!("{}", { let __type_name = match x.lock().unwrap().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" }; __type_name }), format!("{}", isInt), format!("{}", 5000000000i64));
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(('A' as i32))));
    print!("{} {} {}\n", format!("{}", "int32"), format!("{}", { let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", char::from_u32(({ let __v = (*r.lock().unwrap().as_ref().unwrap()).clone(); __v }) as i64 as u32).unwrap_or('\u{FFFD}')));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
