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

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut lines = Arc::new(Mutex::new(Some(vec!["a#".to_string(), "#b".to_string(), "\u{e9}#".to_string()])));
    let mut n = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (r, line) in __range_values.iter().enumerate() {
        for (c, ch) in (line).clone().char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x == __tmp_y } {
        { let __rhs = { let __tmp_x = { let __tmp_x = r as i64; let __tmp_y = 10; __tmp_x * __tmp_y }; let __tmp_y = c as i64; __tmp_x + __tmp_y }; let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + __rhs); };
    }
    }
    } }
    let mut count = Arc::new(Mutex::new(Some(0i64)));
    { let __range_holder = lines.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for line in __range_values.iter() {
        for (_, ch) in (line).clone().char_indices() {
        if { let __tmp_x = ch; let __tmp_y = '#'; __tmp_x != __tmp_y } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        for (i, _) in (line).clone().char_indices() {
        let _ = &i;
    }
    } }
    let mut words = strings::fields(Arc::new(Mutex::new(Some("go to rust".to_string()))));
    let mut words = { let __slice_header = GoSliceHeader::go_slice_header(&words); Arc::new(Mutex::new(__slice_header)) };
    let mut initials: Arc<Mutex<Option<Vec<i32>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = words.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for w in __range_values.iter() {
        for (_, ch) in (w).clone().char_indices() {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = initials.clone(); let __append_value = ch as i32; let mut __appended: Vec<i32> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *initials.lock().unwrap() = __slice_header; };
        break
    }
    } }
    println!("{} {} {}", format!("{}", { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", (*Arc::new(Mutex::new(Some((*initials.lock().unwrap().as_ref().unwrap()).iter().map(|&c| char::from_u32(c as u32).unwrap_or('\u{FFFD}')).collect::<String>()))).lock().unwrap().as_ref().unwrap())));
    let mut s = Arc::new(Mutex::new(Some("abc".to_string())));
    for (s, _) in { let __range_string = (*s.lock().unwrap().as_ref().unwrap()).clone(); __range_string }.char_indices() {
        print!("{}{}", format!("{}", s), format!("{}", " ".to_string()));
    }
    println!();
    for mut line in vec!["ab".to_string(), "cd".to_string()].iter().cloned() {
        for (_, ch) in (line).clone().char_indices() {
        { let new_val = "".to_string(); line = new_val; };
        print!("{}", format!("{}", (*Arc::new(Mutex::new(Some(char::from_u32(((ch as i32)) as u32).unwrap().to_string()))).lock().unwrap().as_ref().unwrap())));
    }
        print!("{}{}{}", format!("{}", "[".to_string()), format!("{}", line), format!("{}", "]".to_string()));
    }
    println!();
    let mut counts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    for w in &vec!["xy".to_string(), "zzz".to_string()] {
        let w_closure_clone = w.clone(); { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = counts.clone(); let __append_value = Arc::new(Mutex::new(Some(Arc::new(move || -> i64 {
        let mut n = Arc::new(Mutex::new(Some(0i64)));
        for _ in (w_closure_clone).clone().chars() {
        { let mut guard = n.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v };
    }) as Arc<dyn Fn() -> i64 + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Arc<dyn Fn() -> i64 + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *counts.lock().unwrap() = __slice_header; };
    }
    { let __range_holder = counts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for f in __range_values.iter().cloned() {
        print!("{}{}", format!("{}", {
            let __f: Arc<dyn Fn() -> i64 + Send + Sync> = f.lock().unwrap().as_ref().unwrap().clone();
            (__f)(

            )
        }), format!("{}", " ".to_string()));
    } }
    println!();
}