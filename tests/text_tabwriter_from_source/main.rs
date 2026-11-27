#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
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

thread_local! {
    static __GO_RECEIVER_HANDLES: std::cell::RefCell<std::vec::Vec<(usize, Box<dyn std::any::Any>)>> = std::cell::RefCell::new(std::vec::Vec::new());
}

fn go_set_receiver_handle<H: Clone + 'static>(handle: &H, address: usize) {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        if __handles.len() >= 16 {
            __handles.remove(0);
        }
        __handles.push((address, Box::new(handle.clone())));
    });
}

fn go_hint_receiver_mut<T: 'static>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *mut T {
    let __ptr = handle.lock().unwrap().as_mut().unwrap() as *mut T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_hint_receiver<T: 'static>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *const T {
    let __ptr = handle.lock().unwrap().as_ref().unwrap() as *const T;
    go_set_receiver_handle(handle, __ptr as *const () as usize);
    __ptr
}

fn go_receiver_ptr_mut<T>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *mut T {
    handle.lock().unwrap().as_mut().unwrap() as *mut T
}

fn go_receiver_ptr<T>(handle: &std::sync::Arc<std::sync::Mutex<Option<T>>>) -> *const T {
    handle.lock().unwrap().as_ref().unwrap() as *const T
}

fn go_take_receiver_handle<T: 'static>(address: usize) -> Option<std::sync::Arc<std::sync::Mutex<Option<T>>>> {
    __GO_RECEIVER_HANDLES.with(|__handles| {
        let mut __handles = __handles.borrow_mut();
        let __at = __handles.iter().rposition(|(__address, __handle)| *__address == address && __handle.is::<std::sync::Arc<std::sync::Mutex<Option<T>>>>())?;
        let (_, __handle) = __handles.remove(__at);
        __handle.downcast::<std::sync::Arc<std::sync::Mutex<Option<T>>>>().ok().map(|__handle| *__handle)
    })
}

fn main() {
    go_install_panic_exit();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::text_tabwriter::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut w = text_tabwriter::new_writer(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>; __arg_value }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(8))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some((' ' as u8) as u8))), Arc::new(Mutex::new(Some(0 as u64))));
    { let __s = format!("{}\n", format!("{}", "NAME\tAGE\tCITY".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "Ann\t30\tParis".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "Bartholomew\t5\tRome".to_string())); (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __recv = w.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    let mut tw = text_tabwriter::new_writer(Arc::new(Mutex::new(Some({ let __arg_value = Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>; __arg_value }))), Arc::new(Mutex::new(Some(6))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(2))), Arc::new(Mutex::new(Some(('.' as u8) as u8))), Arc::new(Mutex::new(Some(({ let __tmp_x = text_tabwriter::ALIGN_RIGHT as u64; let __tmp_y = text_tabwriter::DEBUG as u64; __tmp_x | __tmp_y }) as u64))));
    { let __s = format!("a\tbbb\tc\n"); (*tw.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("aaaa\tb\tccccc\n"); (*tw.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __recv = tw.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    let mut sb: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut t = text_tabwriter::new_writer(Arc::new(Mutex::new(Some(Box::new(strings::builder::BuilderPtr(sb.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(4))), Arc::new(Mutex::new(Some(1))), Arc::new(Mutex::new(Some((' ' as u8) as u8))), Arc::new(Mutex::new(Some(text_tabwriter::TAB_INDENT as u64))));
    let mut i = Arc::new(Mutex::new(Some(1i64)));
    while { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x <= __tmp_y } {
        { let __s = format!("{}\t{}\t{}\n", format!("{}", (*strings::repeat(Arc::new(Mutex::new(Some("x".to_string()))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 3; __tmp_x * __tmp_y })))).lock().unwrap().as_ref().unwrap())), format!("{}", { let __bin_i = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }); __bin_i * __bin_i }), format!("{}", { let __tmp_x = { let __tmp_x = { let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x % __tmp_y }; let __tmp_y = 0; __tmp_x == __tmp_y })); (*t.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __recv = t.clone(); let __recv_ptr = go_hint_receiver_mut(&__recv); let __result = unsafe { &mut *__recv_ptr }.flush(); __result };
    print!("{}", format!("{}", (*(*sb.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", strings::count((*sb.lock().unwrap().as_ref().unwrap()).string(), Arc::new(Mutex::new(Some("\n".to_string()))))));
}