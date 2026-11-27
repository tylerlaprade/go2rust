#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
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
    ::math_bits::__go_init_all();
    ::sort::__go_init_all();

    let mut point: Arc<Mutex<Option<AnonymousStruct1>>> = Arc::new(Mutex::new(Some(Default::default())));
    { let new_val = 10; *(*point.lock().unwrap().as_ref().unwrap()).x.lock().unwrap() = Some(new_val); };
    { let new_val = 20; *(*point.lock().unwrap().as_ref().unwrap()).y.lock().unwrap() = Some(new_val); };
    print!("Point: ({}, {})\n", format!("{}", (*{ let __field = (*point.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*point.lock().unwrap().as_ref().unwrap()).y.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut person = Arc::new(Mutex::new(Some(AnonymousStruct2 { name: Arc::new(Mutex::new(Some("Alice".to_string()))), age: Arc::new(Mutex::new(Some(30))) })));
    print!("Person: {}, {} years old\n", format!("{}", (*{ let __field = (*person.lock().unwrap().as_ref().unwrap()).name.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*person.lock().unwrap().as_ref().unwrap()).age.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    let mut config = Arc::new(Mutex::new(Some(AnonymousStruct3 { host: Arc::new(Mutex::new(Some("localhost".to_string()))), port: Arc::new(Mutex::new(Some(8080))), settings: Arc::new(Mutex::new(Some(AnonymousStruct4::default()))) })));
    { let new_val = true; *(*(*config.lock().unwrap().as_ref().unwrap()).settings.lock().unwrap().as_ref().unwrap()).debug.lock().unwrap() = Some(new_val); };
    { let new_val = false; *(*(*config.lock().unwrap().as_ref().unwrap()).settings.lock().unwrap().as_ref().unwrap()).verbose.lock().unwrap() = Some(new_val); };
    print!("Config: {}:{} (Debug: {}, Verbose: {})\n", format!("{}", (*{ let __field = (*config.lock().unwrap().as_ref().unwrap()).host.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*config.lock().unwrap().as_ref().unwrap()).port.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*config.lock().unwrap().as_ref().unwrap()).settings.lock().unwrap().as_ref().unwrap()).debug.lock().unwrap().as_ref().unwrap())), format!("{}", (*(*(*config.lock().unwrap().as_ref().unwrap()).settings.lock().unwrap().as_ref().unwrap()).verbose.lock().unwrap().as_ref().unwrap())));
    let mut items: Arc<Mutex<Option<[AnonymousStruct5; 2]>>> = Arc::new(Mutex::new(Some(std::array::from_fn::<_, 2, _>(|_| Default::default()))));
    { let new_val = 1; *{ let __seq = { let __seq_holder = items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.i_d.lock().unwrap() = Some(new_val); };
    { let new_val = "first".to_string(); *{ let __seq = { let __seq_holder = items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.value.lock().unwrap() = Some(new_val); };
    { let new_val = 2; *{ let __seq = { let __seq_holder = items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.i_d.lock().unwrap() = Some(new_val); };
    { let new_val = "second".to_string(); *{ let __seq = { let __seq_holder = items.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); std::mem::drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.value.lock().unwrap() = Some(new_val); };
    { let __range_holder = items.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.iter().map(|__e| __e.__go_value_clone()).collect::<Vec<_>>()).unwrap_or_default(); std::mem::drop(__range_guard); for (i, item) in __range_values.iter().enumerate() {
        let item = &item.__go_value_clone();
        print!("Item {}: {{ID: {}, Value: {}}}\n", format!("{}", i), format!("{}", (*item.i_d.lock().unwrap().as_ref().unwrap())), format!("{}", (*item.value.lock().unwrap().as_ref().unwrap()).clone()));
    } }
    let mut events = Arc::new(Mutex::new(Some(vec![AnonymousStruct6 { r#type: Arc::new(Mutex::new(Some("info".to_string()))), message: Arc::new(Mutex::new(Some("System started".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Arc::new(Mutex::new(Some("warning".to_string()))), message: Arc::new(Mutex::new(Some("Low memory".to_string()))), ..Default::default() }, AnonymousStruct6 { r#type: Arc::new(Mutex::new(Some("error".to_string()))), message: Arc::new(Mutex::new(Some("Connection failed".to_string()))), ..Default::default() }])));
    { let __range_holder = events.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for event in __range_values.iter() {
        let event = &event.__go_value_clone();
        print!("Event [{}]: {}\n", format!("{}", (*event.r#type.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*event.message.lock().unwrap().as_ref().unwrap()).clone()));
    } }
    let mut users = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<AnonymousStruct7>>>>::from([("alice".to_string(), Arc::new(Mutex::new(Some(AnonymousStruct7 { email: Arc::new(Mutex::new(Some("alice@example.com".to_string()))), admin: Arc::new(Mutex::new(Some(true))), ..Default::default() })))), ("bob".to_string(), Arc::new(Mutex::new(Some(AnonymousStruct7 { email: Arc::new(Mutex::new(Some("bob@example.com".to_string()))), admin: Arc::new(Mutex::new(Some(false))), ..Default::default() }))))]))));
    let mut userNames: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    for (name, _) in { let __range_holder = users.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_map = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); __range_map } {
        { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = userNames.clone(); let __append_value = name.clone(); let mut __appended: Vec<String> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *userNames.lock().unwrap() = __slice_header; };
    }
    sort::strings(userNames.clone());
    { let __range_holder = userNames.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__range_guard); for name in __range_values.iter() {
        let mut user = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = users.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); std::mem::drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(name)).map(|__v| __v.lock().unwrap().as_ref().unwrap().__go_value_clone()).unwrap_or_else(|| Default::default()) })));
        print!("User {}: {} (admin: {})\n", format!("{}", name), format!("{}", (*{ let __field = (*user.lock().unwrap().as_ref().unwrap()).email.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone()), format!("{}", (*{ let __field = (*user.lock().unwrap().as_ref().unwrap()).admin.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    } }
}

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub x: Arc<Mutex<Option<i64>>>,
    pub y: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.y.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct2 {
    pub name: Arc<Mutex<Option<String>>>,
    pub age: Arc<Mutex<Option<i64>>>,
}
impl AnonymousStruct2 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.age.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
            age: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct2 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            name: __go_default_0_0,
            age: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.age.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct3 {
    pub host: Arc<Mutex<Option<String>>>,
    pub port: Arc<Mutex<Option<i64>>>,
    pub settings: Arc<Mutex<Option<AnonymousStruct4>>>,
}
impl AnonymousStruct3 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.host.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.port.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = { let __guard = self.settings.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            host: __go_clone_0_0,
            port: __go_clone_1_0,
            settings: __go_clone_2_0,
        }
    }
}


impl Default for AnonymousStruct3 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_2_0 = Arc::new(Mutex::new(Some(AnonymousStruct4::default())));
        Self {
            host: __go_default_0_0,
            port: __go_default_1_0,
            settings: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.host.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.port.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_2 = format!("{}", (*self.settings.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct4 {
    pub debug: Arc<Mutex<Option<bool>>>,
    pub verbose: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct4 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.debug.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.verbose.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            debug: __go_clone_0_0,
            verbose: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct4 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(false)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            debug: __go_default_0_0,
            verbose: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct4 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.debug.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.verbose.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct5 {
    pub i_d: Arc<Mutex<Option<i64>>>,
    pub value: Arc<Mutex<Option<String>>>,
}
impl AnonymousStruct5 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.i_d.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            i_d: __go_clone_0_0,
            value: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct5 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            i_d: __go_default_0_0,
            value: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.i_d.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct6 {
    pub r#type: Arc<Mutex<Option<String>>>,
    pub message: Arc<Mutex<Option<String>>>,
}
impl AnonymousStruct6 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.r#type.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.message.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            message: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct6 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            r#type: __go_default_0_0,
            message: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct6 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.r#type.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.message.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct AnonymousStruct7 {
    pub email: Arc<Mutex<Option<String>>>,
    pub admin: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct7 {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.email.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.admin.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            email: __go_clone_0_0,
            admin: __go_clone_1_0,
        }
    }
}


impl Default for AnonymousStruct7 {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(false)));
        Self {
            email: __go_default_0_0,
            admin: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for AnonymousStruct7 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.email.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.admin.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}
