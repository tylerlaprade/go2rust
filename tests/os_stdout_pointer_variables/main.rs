#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
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

#[derive(Clone, Default)]
pub struct Logger {
    pub out: Arc<Mutex<Option<os_File>>>,
    pub w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl Logger {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.out.clone();
        let __go_clone_1_0 = self.w.clone();
        Self {
            out: __go_clone_0_0,
            w: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.out.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.w.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn pick(verbose: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<os_File>>> {
    if { let __v = (*verbose.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        return os::Stdout().clone();
    }
    os::Stderr().clone()
}

fn main() {
    go_install_panic_exit();
    ::io::__go_init_all();

    let mut out: Arc<Mutex<Option<os_File>>> = os::Stdout().clone();
    { let __s = format!("{}\n", format!("{}", "to stdout".to_string())); (*out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    println!("{} {} {} {}", format!("{}", {
        let __left = os::Stdout().clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stdout().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", {
        let __left = out.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        !__eq
    }));
    let mut l = Arc::new(Mutex::new(Some(Logger {
        out: pick(Arc::new(Mutex::new(Some(true)))).clone(),
        w: Arc::new(Mutex::new(Some(Box::new((*os::Stdout().lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn io::r#mod::Writer + Send + Sync>))),
        ..Default::default()
    })));
    { let __s = format!("logger {}\n", 1); (*(*l.lock().unwrap().as_ref().unwrap()).out.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __s = format!("{}\n", format!("{}", "writer".to_string())); (*(*l.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
    { let __go_print_arg_1 = Arc::new(Mutex::new(Some({
    let __left = pick(Arc::new(Mutex::new(Some(false))));
    let __right = os::Stderr().clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}))); let __go_print_arg_2 = Arc::new(Mutex::new(Some({
    let __left = pick(Arc::new(Mutex::new(Some(false))));
    let __right = out.clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}))); let __go_print_value = println!("{} {} {}", format!("{}", {
    let __left = (*l.lock().unwrap().as_ref().unwrap()).out.clone();
    let __right = os::Stdout().clone();
    let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
    let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
    __eq
}), format!("{}", { let __v = (*__go_print_arg_1.lock().unwrap().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_2.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let mut errOut: Arc<Mutex<Option<os_File>>> = Arc::new(Mutex::new(None));
    { let new_val = os::Stderr().clone(); errOut = new_val; };
    println!("{} {}", format!("{}", {
        let __left = errOut.clone();
        let __right = os::Stderr().clone();
        let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
        let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
        __eq
    }), format!("{}", { let __nil_result = (*errOut.lock().unwrap()).is_some(); __nil_result }));
    { let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.write_string(Arc::new(Mutex::new(Some(format!("{}{}", (*{ let __std_stream = (*os::Stdout().lock().unwrap().as_ref().unwrap()).clone(); __std_stream }.name().lock().unwrap().as_ref().unwrap()), "\n".to_string())))));
    {
        let (mut f, mut err) = os::open("/dev/null".to_string());;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            println!("{}", format!("{}", (*{ let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));;
            { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.close(); __result };;
        }
    }
    let mut files = Arc::new(Mutex::new(Some(vec![{ let __field = os::Stdout().clone(); __field }, { let __field = os::Stderr().clone(); __field }])));
    { let __range_holder = files.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for (i, f) in __range_values.iter().enumerate() {
        { let __go_print_arg_3 = { let __recv = f.clone(); let __recv_ptr: *mut os_File = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut os_File }; let __result = unsafe { &mut *__recv_ptr }.name(); __result }; let __go_print_value = println!("{} {} {}", format!("{}", i), format!("{}", {
            let __left = f.clone();
            let __right = os::Stdout().clone();
            let __both_nil = (*__left.lock().unwrap()).is_none() && (*__right.lock().unwrap()).is_none();
            let __eq = __both_nil || Arc::ptr_eq(&__left, &__right);
            __eq
        }), format!("{}", { let __v = (*__go_print_arg_3.lock().unwrap().as_ref().unwrap()).clone(); __v })); __go_print_value };
    } }
}

impl GoValueClone for Logger {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
