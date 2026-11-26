#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};
use std::thread;


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

pub trait positioner: std::fmt::Display + Any {
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool;
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>>;
}

impl Clone for Box<dyn positioner + Send + Sync> {
    fn clone(&self) -> Self {
        positioner::__go_clone_box_positioner(self.as_ref())
    }
}

pub trait named: std::fmt::Display + Any {
    fn __go_clone_box_named(&self) -> Box<dyn named + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_named(&self, other: &(dyn named + Send + Sync)) -> bool;
    fn string(&self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn named + Send + Sync> {
    fn clone(&self) -> Self {
        named::__go_clone_box_named(self.as_ref())
    }
}

impl named for go_ast::r#mod::IdentPtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ident::string(__recv)
        }
    }
    fn __go_clone_box_named(&self) -> Box<dyn named + Send + Sync> {
        Box::new(self.clone()) as Box<dyn named + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_named(&self, other: &(dyn named + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::BasicLitPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::BasicLit::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::BasicLitPtr>() {
            false
        } else {
            false
        }
    }
}

impl positioner for go_ast::r#mod::IdentPtr {
    fn pos(&self) -> Arc<Mutex<Option<go_token::position::Pos>>> {
        {
            let __recv_guard = self.0.lock().unwrap();
            let __recv = __recv_guard.as_ref().unwrap();
            go_ast::r#mod::Ident::pos(__recv)
        }
    }
    fn __go_clone_box_positioner(&self) -> Box<dyn positioner + Send + Sync> {
        Box::new(self.clone()) as Box<dyn positioner + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_positioner(&self, other: &(dyn positioner + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<go_ast::r#mod::IdentPtr>() {
            false
        } else {
            false
        }
    }
}

pub fn at(p: Arc<Mutex<Option<Box<dyn positioner + Send + Sync>>>>) -> Arc<Mutex<Option<go_token::position::Pos>>> {
    (*p.lock().unwrap().as_ref().unwrap()).pos()
}

pub fn describe(n: Arc<Mutex<Option<Box<dyn named + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    (*n.lock().unwrap().as_ref().unwrap()).string()
}

fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_token::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
    if let Err(__go_goroutine_panic) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
        done_thread.send(true);;;
    })) {
        let __go_panic_text = if let Some(__text) = __go_goroutine_panic.downcast_ref::<String>() { __text.clone() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<&'static str>() { __text.to_string() } else if let Some(__text) = __go_goroutine_panic.downcast_ref::<Box<dyn std::any::Any + Send + Sync>>().and_then(|__value| __value.downcast_ref::<String>()) { __text.clone() } else { String::from("goroutine panicked") };
        eprintln!("panic: {}", __go_panic_text);
        std::process::exit(2);
    }
    });
    done.recv().unwrap_or_default();
    let mut id = go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string()))));
    println!("{} {}", format!("{}", {
        let __tmp_x = (*at(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn positioner + Send + Sync>)))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = (*{ let __recv = id.clone(); let __recv_ptr: *const go_ast::r#mod::Ident = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::Ident }; let __result = unsafe { &*__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()).clone();
        __tmp_x == __tmp_y
    }), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut held: Arc<Mutex<Option<Box<dyn named + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)));
    { let new_val = "y".to_string(); *(*id.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = Some(new_val); };
    println!("{} {}", format!("{}", (*(*held.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())), format!("{}", (*describe(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(id.clone())) as Box<dyn named + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
    let mut lit = Arc::new(Mutex::new(Some(go_ast::r#mod::BasicLit { kind: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::I_N_T as i64))))))), value: Arc::new(Mutex::new(Some("1".to_string()))), ..Default::default() })));
    println!("{}", format!("{}", {
        let __tmp_x = (*at(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::BasicLitPtr(lit.clone())) as Box<dyn positioner + Send + Sync>)))).lock().unwrap().as_ref().unwrap()).clone();
        let __tmp_y = (*{ let __recv = lit.clone(); let __recv_ptr: *const go_ast::r#mod::BasicLit = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_ast::r#mod::BasicLit }; let __result = unsafe { &*__recv_ptr }.pos(); __result }.lock().unwrap().as_ref().unwrap()).clone();
        __tmp_x == __tmp_y
    }));
}