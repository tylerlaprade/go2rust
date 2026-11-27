#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};


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

pub fn atoi(s: Arc<Mutex<Option<String>>>) -> i64 {
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    for (_, c) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        { let new_val = { let __tmp_x = { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = (*Arc::new(Mutex::new(Some(((((c as i32) - ('0' as i32)))) as i64))).lock().unwrap().as_ref().unwrap()); __tmp_x + __tmp_y }; *n.lock().unwrap() = Some(new_val); };
    }
    return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

pub fn caesar(s: Arc<Mutex<Option<String>>>, k: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<String>>> {
    let mut b: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    for (_, c) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        if { let __tmp_x = c; let __tmp_y = 'a'; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = 'z'; __tmp_x <= __tmp_y } {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(('a' as i32) + ((c as i32) - ('a' as i32) + ((*Arc::new(Mutex::new(Some(({ let __v = (*k.lock().unwrap().as_ref().unwrap()); __v }) as i32))).lock().unwrap().as_ref().unwrap()))) % (26 as i32)))));
        } else if { let __tmp_x = c; let __tmp_y = 'A'; __tmp_x >= __tmp_y } && { let __tmp_x = c; let __tmp_y = 'Z'; __tmp_x <= __tmp_y } {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(('A' as i32) + ((c as i32) - ('A' as i32) + ((*Arc::new(Mutex::new(Some(({ let __v = (*k.lock().unwrap().as_ref().unwrap()); __v }) as i32))).lock().unwrap().as_ref().unwrap()))) % (26 as i32)))));
        } else {
            (*b.lock().unwrap().as_mut().unwrap()).write_rune(Arc::new(Mutex::new(Some(c as i32))));
        }
    }
    return (*b.lock().unwrap().as_ref().unwrap()).string();
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    println!("{}", format!("{}", { let __tmp_x = atoi(Arc::new(Mutex::new(Some("4096".to_string())))); let __tmp_y = 1; __tmp_x + __tmp_y }));
    println!("{} {}", format!("{}", (*caesar(Arc::new(Mutex::new(Some("Hello, World!".to_string()))), Arc::new(Mutex::new(Some(3)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*caesar(Arc::new(Mutex::new(Some("xyz".to_string()))), Arc::new(Mutex::new(Some(2)))).lock().unwrap().as_ref().unwrap())));
    let mut counts = Arc::new(Mutex::new(Some([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])));
    for (_, c) in "banana".to_string().char_indices() {
        { let __idx = ((c as i32) - ('a' as i32)) as usize; let mut __seq_guard = counts.lock().unwrap(); let __seq = __seq_guard.as_mut().unwrap(); __seq[__idx] = __seq[__idx] + 1; }
    }
    println!("{} {} {}", format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = counts.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(13) as usize].clone() }));
    for (i, c) in "az".to_string().char_indices() {
        let mut up = Arc::new(Mutex::new(Some((c as i32) - ('a' as i32) + ('A' as i32))));
        let mut next = Arc::new(Mutex::new(Some((c as i32) + (1 as i32))));
        println!("{} {} {} {} {} {}", format!("{}", i), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*up.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32((({ let __v = (*next.lock().unwrap().as_ref().unwrap()); __v })) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())), format!("{}", (c as i32) * (2 as i32)), format!("{}", (c as i32) | (0x20 as i32)), format!("{}", (c as i32) & ! (0x20 as i32)));
    }
}