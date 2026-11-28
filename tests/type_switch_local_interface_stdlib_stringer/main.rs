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


/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

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

pub trait stringer: std::fmt::Display + Any {
    fn __go_clone_box_stringer(&self) -> Box<dyn stringer + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_stringer(&self, other: &(dyn stringer + Send + Sync)) -> bool;
    fn string(&mut self) -> Arc<Mutex<Option<String>>>;
}

impl Clone for Box<dyn stringer + Send + Sync> {
    fn clone(&self) -> Self {
        stringer::__go_clone_box_stringer(self.as_ref())
    }
}

impl stringer for time::r#mod::Duration {
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        <time::r#mod::Duration>::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn stringer + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<time::r#mod::Duration>() {
            false
        } else {
            false
        }
    }
}

impl stringer for time::r#mod::Month {
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        <time::r#mod::Month>::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn stringer + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<time::r#mod::Month>() {
            false
        } else {
            false
        }
    }
}

impl stringer for time::r#mod::Weekday {
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        <time::r#mod::Weekday>::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn stringer + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<time::r#mod::Weekday>() {
            false
        } else {
            false
        }
    }
}

impl stringer for time::zoneinfo::Location {
    fn string(&mut self) -> Arc<Mutex<Option<String>>> {
        <time::zoneinfo::Location>::string(self)
    }
    fn __go_clone_box_stringer(&self) -> Box<dyn stringer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn stringer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_stringer(&self, other: &(dyn stringer + Send + Sync)) -> bool {
        if let Some(_other) = other.__go_as_any().downcast_ref::<time::zoneinfo::Location>() {
            false
        } else {
            false
        }
    }
}

pub fn stringify(v: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    {
    let _ts_subject = v.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().map_or(true, |__v| (**__v).is::<GoNilAny>());
    let _ts_val: Option<&dyn Any> = _ts_guard.as_ref().map(|__v| {
        let mut __any = __v.as_ref() as &dyn Any;
        while let Some(__boxed) = __any.downcast_ref::<Box<dyn Any + Send + Sync>>() {
            __any = __boxed.as_ref() as &dyn Any;
        }
        __any
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Duration>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Month>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Weekday>()).is_some() || _ts_val.and_then(|__v| __v.downcast_ref::<time::zoneinfo::Location>()).is_some() {
        let s: Arc<Mutex<Option<Box<dyn stringer + Send + Sync>>>> = if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Duration>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn stringer + Send + Sync>)))
        } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Month>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn stringer + Send + Sync>)))
        } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<time::r#mod::Weekday>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn stringer + Send + Sync>)))
        } else if let Some(typed_val) = _ts_val.and_then(|__v| __v.downcast_ref::<time::zoneinfo::Location>()) {
            Arc::new(Mutex::new(Some(Box::new(typed_val.clone()) as Box<dyn stringer + Send + Sync>)))
        } else {
            panic!("type switch interface case condition matched no concrete implementor")
        };
        std::mem::drop(_ts_guard);
        return (*s.lock().unwrap().as_mut().unwrap()).string();;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<String>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<String>()).unwrap().clone())));
        std::mem::drop(_ts_guard);
        return { let __owned = s.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
    } else if _ts_is_nil {
        let s = _ts_subject.clone();
        std::mem::drop(_ts_guard);
        return Arc::new(Mutex::new(Some("<nil>".to_string())));;
    }
    }
    Arc::new(Mutex::new(Some("?".to_string())))
}

fn main() {
    go_install_panic_exit();
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    println!("{} {} {} {}", format!("{}", (*stringify(Arc::new(Mutex::new(Some(Box::new(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::SECOND as i64))))) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*stringify(Arc::new(Mutex::new(Some(Box::new("x".to_string()) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())), format!("{}", (*stringify(Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>))).lock().unwrap().as_ref().unwrap())), format!("{}", (*stringify(Arc::new(Mutex::new(Some(Box::new(3i64) as Box<dyn Any + Send + Sync>)))).lock().unwrap().as_ref().unwrap())));
}