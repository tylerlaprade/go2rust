#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
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

#[derive(Debug, Clone, Default)]
pub struct localType {
}

impl localType {
    pub fn __go_value_clone(&self) -> Self {
        Self {
        }
    }
}

impl std::fmt::Display for localType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl localType {
    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("local".to_string())))
    }
}

impl go_types::r#type::Type for localType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("local".to_string())))
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
    }
    fn __go_clone_box_type_(&self) -> Box<dyn go_types::r#type::Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn go_types::r#type::Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn go_types::r#type::Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localType>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct localTypePtr(pub Arc<Mutex<Option<localType>>>);

impl std::fmt::Display for localTypePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl go_types::r#type::Type for localTypePtr {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        localType::string(__recv)
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        localType::underlying(__recv)
    }
    fn __go_clone_box_type_(&self) -> Box<dyn go_types::r#type::Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn go_types::r#type::Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_print(&self, plus: bool) -> String {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => format!("{}", __v), None => "<nil>".to_string() }
    }
    fn __go_eq_type_(&self, other: &(dyn go_types::r#type::Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localTypePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn make_type() -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
}

fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bisect::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut typesList = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>))), make_type().clone()])));
    println!("{}", format!("{}", ({ let __len = (*typesList.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
}

impl GoValueClone for localType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
