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

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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

pub fn multiple_returns() -> (i64, Arc<Mutex<Option<String>>>, bool) {
    (42, Arc::new(Mutex::new(Some("hello".to_string()))), true)
}

pub fn process_slice(slice: Arc<Mutex<Option<Vec<i64>>>>) -> (i64, i64) {
    let mut sum: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut count: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));

    { let new_val = 0; *sum.lock().unwrap() = Some(new_val); };
    { let new_val = ({ let __len = (*slice.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64; *count.lock().unwrap() = Some(new_val); };
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter().copied() {
        { let __rhs = val; let mut guard = sum.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    } }
    return ((*sum.lock().unwrap().as_ref().unwrap()), (*count.lock().unwrap().as_ref().unwrap()));
}

fn main() {
    go_install_panic_exit();
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    println!("{}", format!("{}", "=== Ignoring return values ===".to_string()));
    let (mut num, _, _) = multiple_returns();
    print!("Only using first return: {}\n", num);
    let (_, mut str, _) = multiple_returns();
    print!("Only using middle return: {}\n", { let __v = (*str.lock().unwrap().as_ref().unwrap()).clone(); __v });
    let (_, _, mut flag) = multiple_returns();
    print!("Only using last return: {}\n", flag);
    println!("{}", format!("{}", "\n=== Ignoring in range loops ===".to_string()));
    let mut slice = Arc::new(Mutex::new(Some(vec![10i64, 20i64, 30i64, 40i64, 50i64])));
    println!("{}", format!("{}", "Values only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for val in __range_values.iter().copied() {
        print!("{} ", val);
    } }
    println!();
    println!("{}", format!("{}", "Indices only:".to_string()));
    { let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, _) in __range_values.iter().copied().enumerate() {
        print!("{} ", i);
    } }
    println!();
    println!("{}", format!("{}", "Indices (idiomatic):".to_string()));
    for i in 0..(({ let __range_holder = slice.clone(); let __range_guard = __range_holder.lock().unwrap(); __range_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) })) {
        print!("{} ", i);
    }
    println!();
    println!("{}", format!("{}", "\n=== Ignoring in map iteration ===".to_string()));
    let mut ages = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<i64>>>>::from([("Alice".to_string(), Arc::new(Mutex::new(Some(25i64)))), ("Bob".to_string(), Arc::new(Mutex::new(Some(30i64)))), ("Carol".to_string(), Arc::new(Mutex::new(Some(35i64))))]))));
    println!("{}", format!("{}", "Keys only:".to_string()));
    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (name, _) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = names.clone(); let __append_value = name.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *names.lock().unwrap() = __slice_header; };
    }
    sort::strings(names.clone());
    { let __range_holder = names.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for name in __range_values.iter() {
        print!("{} ", name);
    } }
    println!();
    println!("{}", format!("{}", "Values only:".to_string()));
    let mut sortedAges: Arc<Mutex<Option<Vec<i64>>>> = Arc::new(Mutex::new(None));
    for (_, age) in { let __range_holder = ages.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = sortedAges.clone(); let __append_value = (*age.lock().unwrap().as_ref().unwrap()).clone(); let mut __appended: Vec<i64> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *sortedAges.lock().unwrap() = __slice_header; };
    }
    sort::ints(sortedAges.clone());
    { let __range_holder = sortedAges.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for age in __range_values.iter().copied() {
        print!("{} ", age);
    } }
    println!();
    println!("{}", format!("{}", "\n=== Ignoring some return values in assignment ===".to_string()));
    let (mut sum, _) = process_slice(slice.clone());
    print!("Sum (ignoring count): {}\n", sum);
    let (_, mut count) = process_slice(slice.clone());
    print!("Count (ignoring sum): {}\n", count);
    println!("{}", format!("{}", "\n=== Blank identifier in declarations ===".to_string()));
    let _ = "This string is assigned but not used".to_string();
    let (mut a, _, mut c) = (Arc::new(Mutex::new(Some(1i64))), Arc::new(Mutex::new(Some(2i64))), Arc::new(Mutex::new(Some(3i64))));
    print!("a={}, c={} (middle value ignored)\n", format!("{}", { let __v = (*a.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*c.lock().unwrap().as_ref().unwrap()).clone(); __v }));
    println!("{}", format!("{}", "\n=== Blank identifier with type assertion ===".to_string()));
    let mut value: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new("hello world".to_string()) as Box<dyn Any + Send + Sync>)));
    {
        let (_, mut ok) = ({
        let val = value.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<std::string::String>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
            }
        } else {
            (Arc::new(Mutex::new(Some(std::string::String::new()))), false)
        }
    });;
        if ok {
            println!("{}", format!("{}", "Value is a string (but we ignored the actual value)".to_string()));;
        }
    }
    {
        let (_, mut ok) = ({
        let val = value.clone();
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
    });;
        if ok {
            println!("{}", format!("{}", "Value is an int".to_string()));;
        } else {
            println!("{}", format!("{}", "Value is not an int".to_string()));;
        }
    }
    println!("{}", format!("{}", "\n=== Blank identifier with channels ===".to_string()));
    let mut ch = GoChannel::<i64>::new_buffered(3 as usize);
    ch.send(1);
    ch.send(2);
    ch.send(3);
    ch.close();
    for _ in ch.clone() {
        println!("{}", format!("{}", "Received a value (but ignored it)".to_string()));
    }
    println!("{}", format!("{}", "\n=== Blank identifier in error handling ===".to_string()));
    let (mut result, _) = process_slice(Arc::new(Mutex::new(Some(vec![1i64, 2i64, 3i64, 4i64, 5i64]))));
    print!("Result (ignoring potential error): {}\n", result);
    println!("{}", format!("{}", "\n=== Complex example ===".to_string()));
    let mut data = Arc::new(Mutex::new(Some(vec![vec![1i64, 2i64, 3i64], vec![4i64, 5i64, 6i64], vec![7i64, 8i64, 9i64]])));
    let mut total = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = data.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for row in __range_values.iter() {
        for val in row.iter().copied() {
        { let __rhs = val; let mut guard = total.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    } }
    print!("Total of all values: {}\n", { let __v = (*total.lock().unwrap().as_ref().unwrap()).clone(); __v });
}