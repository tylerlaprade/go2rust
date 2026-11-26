#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::cmp::Ord;
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

fn format_map<K: Display + Ord + Clone, V>(map: &Rc<RefCell<Option<BTreeMap<K, Rc<RefCell<Option<V>>>>>>>) -> String
where
    V: Display,
{
    let guard = map.borrow();
    if let Some(ref m) = *guard {
        let mut items: Vec<_> = m.iter().collect();
        items.sort_by_key(|(k, _)| (*k).clone());

        let formatted: Vec<String> = items
            .into_iter()
            .map(|(k, v)| {
                let v_guard = v.borrow();
                if let Some(ref val) = *v_guard {
                    format!("{}:{}", go_display_value(k), go_display_value(val))
                } else {
                    format!("{}:<nil>", k)
                }
            })
            .collect();

        format!("map[{}]", formatted.join(" "))
    } else {
        "map[]".to_string()
    }
}
fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[T]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| go_display_value(v)).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| go_display_value(v)).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: AsRef<[Rc<RefCell<Option<T>>>]>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: Vec<String> = s.as_ref().iter().map(|v| {
            let inner = v.borrow();
            match inner.as_ref() {
                Some(value) => format!("&{}", value),
                None => "<nil>".to_string(),
            }
        }).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_any(value: &dyn Any) -> String {
    if let Some(v) = value.downcast_ref::<i32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<f64>() {
        go_float_string(*v, false)
    } else if let Some(v) = value.downcast_ref::<f32>() {
        go_float_string(*v as f64, true)
    } else if let Some(v) = value.downcast_ref::<String>() {
        v.clone()
    } else if let Some(v) = value.downcast_ref::<&str>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<bool>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<i16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<isize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u8>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u16>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u32>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<u64>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<usize>() {
        v.to_string()
    } else if let Some(v) = value.downcast_ref::<GoNilAny>() {
        "<nil>".to_string()
    } else if let Some(v) = value.downcast_ref::<Point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() {
        format_map(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() {
        match (*v.borrow()).as_ref() { Some(__err) => format!("{}", __err), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() {
        match (*v.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() }
    } else if let Some(v) = value.downcast_ref::<callerError>() {
        format!("{}", v)
    } else if let Some(__go_text) = go_format_registered_any(value) {
        __go_text
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

/// A nil interface value in a slot that holds a Box<dyn Any>, such as a
/// []any element: it copies, prints and compares as nil.
#[derive(Clone, Copy, Debug, PartialEq)]
struct GoNilAny;

fn go_any_try_clone(value: &dyn Any) -> Option<Box<dyn Any>> {
    if let Some(v) = value.downcast_ref::<i32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<i16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u8>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<u16>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<usize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<isize>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f64>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<f32>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<&'static str>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<bool>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<char>() { return Some(Box::new(*v) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<String>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<GoNilAny>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Point>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<BTreeMap<String, Rc<RefCell<Option<i64>>>>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Rc<dyn StdError>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<String>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<f64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<i64>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<callerError>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    go_clone_registered_any(value)
}

fn go_any_clone(value: &dyn Any) -> Box<dyn Any> {
    go_any_try_clone(value).unwrap_or_else(|| panic!("go_any_clone: unsupported dynamic type; add typed lowering instead of cloning Box<dyn Any>"))
}

#[allow(dead_code)]
fn go_any_handle_copy(handle: &Rc<RefCell<Option<Box<dyn Any>>>>) -> Rc<RefCell<Option<Box<dyn Any>>>> {
    let __copy = match handle.borrow().as_ref() {
        None => None,
        Some(value) => match go_any_try_clone(&**value) {
            Some(copy) => Some(copy),
            None => return handle.clone(),
        },
    };
    Rc::new(RefCell::new(__copy))
}

/// Stores Go's copy of source's value in target's cell. A source that no
/// one else holds gives up its value; a value whose type has no copy comes
/// back for the caller to share instead.
#[allow(dead_code)]
fn go_any_store(target: &Rc<RefCell<Option<Box<dyn Any>>>>, source: Rc<RefCell<Option<Box<dyn Any>>>>) -> Option<Rc<RefCell<Option<Box<dyn Any>>>>> {
    if Rc::ptr_eq(target, &source) {
        return None;
    }
    let __value = if Rc::strong_count(&source) == 1 {
        source.borrow_mut().take()
    } else {
        let __copy = match source.borrow().as_ref() {
            None => None,
            Some(value) => match go_any_try_clone(&**value) {
                Some(copy) => Some(copy),
                None => return Some(source.clone()),
            },
        };
        __copy
    };
    *target.borrow_mut() = __value;
    None
}

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

struct GoPointerError<T> {
    handle: Rc<RefCell<Option<T>>>,
}

impl<T> GoPointerError<T> {
    fn new(handle: Rc<RefCell<Option<T>>>) -> Self { Self { handle } }
}

impl<T: std::fmt::Display> std::fmt::Display for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let guard = self.handle.borrow();
        match guard.as_ref() {
            Some(value) => write!(f, "{}", value),
            None => Ok(()),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Debug for GoPointerError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl<T: std::fmt::Display + 'static> std::error::Error for GoPointerError<T> {}

fn go_float_string(__value: f64, __single: bool) -> String {
    // The names are prefixed: this helper is at the crate root, where a Go
    // package variable of the same name would be a static it cannot shadow.
    if __value.is_nan() {
        return "NaN".to_string();
    }
    if __value.is_infinite() {
        return if __value < 0.0 { "-Inf".to_string() } else { "+Inf".to_string() };
    }
    let __sign = if __value.is_sign_negative() { "-" } else { "" };
    let __shortest = if __single { format!("{:e}", __value.abs() as f32) } else { format!("{:e}", __value.abs()) };
    let (__mantissa, __exponent_text) = __shortest.split_once('e').unwrap();
    let __digits: String = __mantissa.chars().filter(|__c| __c.is_ascii_digit()).collect();
    let __exponent: i32 = __exponent_text.parse().unwrap();
    if __exponent < -4 || __exponent >= 6 {
        let mut __text = String::from(__sign);
        __text.push_str(&__digits[..1]);
        if __digits.len() > 1 {
            __text.push('.');
            __text.push_str(&__digits[1..]);
        }
        __text.push('e');
        __text.push(if __exponent < 0 { '-' } else { '+' });
        __text.push_str(&format!("{:02}", __exponent.abs()));
        return __text;
    }
    if __exponent < 0 {
        return format!("{}0.{}{}", __sign, "0".repeat((-__exponent - 1) as usize), __digits);
    }
    let __whole = (__exponent + 1) as usize;
    if __digits.len() <= __whole {
        return format!("{}{}{}", __sign, __digits, "0".repeat(__whole - __digits.len()));
    }
    format!("{}{}.{}", __sign, &__digits[..__whole], &__digits[__whole..])
}

// A value inside a slice or a map prints as fmt prints it: a float through
// go_float_string, anything else through its Display. The element type may
// borrow, so it is recognized by name rather than by TypeId.
fn go_display_value<T: Display>(__value: &T) -> String {
    let __name = std::any::type_name::<T>();
    if __name == "f64" {
        // T is f64.
        return go_float_string(unsafe { *(__value as *const T as *const f64) }, false);
    }
    if __name == "f32" {
        // T is f32.
        return go_float_string(unsafe { *(__value as *const T as *const f32) } as f64, true);
    }
    __value.to_string()
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: Rc<RefCell<Option<i64>>>,
    pub y: Rc<RefCell<Option<i64>>>,
}

impl Point {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.x.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.y.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            x: __go_clone_0_0,
            y: __go_clone_1_0,
        }
    }
}


impl Default for Point {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(0)));
        Self {
            x: __go_default_0_0,
            y: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.x.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.y.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


#[derive(Debug, Clone)]
pub struct callerError {
    pub message: Rc<RefCell<Option<String>>>,
}

impl callerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.message.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            message: __go_clone_0_0,
        }
    }
}


impl Default for callerError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            message: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for callerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


#[derive(Debug, Clone)]
pub struct pointerError {
    pub message: Rc<RefCell<Option<String>>>,
}

impl pointerError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.message.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            message: __go_clone_0_0,
        }
    }
}


impl Default for pointerError {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(String::new())));
        Self {
            message: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for pointerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().borrow().as_ref().unwrap()))
    }
}


impl callerError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.message.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }
}

impl StdError for callerError {}


impl pointerError {
    pub fn error(&self) -> Rc<RefCell<Option<String>>> {
        return Rc::new(RefCell::new(Some({ let __selector_holder = self.message.clone(); let __selector_guard = __selector_holder.borrow(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); std::mem::drop(__selector_guard); __cloned })));
    }
}

impl StdError for pointerError {}


/// A value put into an interface does not stay in the package that put it
/// there. main creates these values; another package prints and copies them.
fn main() {
    go_install_panic_exit();
    ::example_com_anyfromcaller_logger::__go_init_all();

    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("bad".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value }))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{ let __arg_value = { let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("later".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value }]))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{ let __arg_value = { let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(format!("wrapped {}", 7)))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value }, Box::new("text".to_string()) as Box<dyn Any>, Box::new(3i64) as Box<dyn Any>]))));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = Rc::new(RefCell::new(Some(Rc::<dyn StdError>::from(Box::<dyn StdError>::from("described".to_string()))))).clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some(Box::new(3i64) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec![1i64, 2i64]))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["a".to_string(), "b".to_string()]))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("k".to_string(), Rc::new(RefCell::new(Some(1i64))))])))).clone()) as Box<dyn Any>))));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, None); Box::new(Point { x: Rc::new(RefCell::new(Some(1))), y: Rc::new(RefCell::new(Some(2))), ..Default::default() }) as Box<dyn Any> }))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![Box::new(Rc::new(RefCell::new(Some(vec![3i64, 4i64]))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([("m".to_string(), Rc::new(RefCell::new(Some(2i64))))])))).clone()) as Box<dyn Any>, Box::new(Rc::new(RefCell::new(Some(vec![1.5]))).clone()) as Box<dyn Any>]))));
    println!("{} {}", format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(vec!["x".to_string()]))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some(Box::new(Rc::new(RefCell::new(Some(BTreeMap::<String, Rc<RefCell<Option<i64>>>>::from([])))).clone()) as Box<dyn Any>)))).borrow().as_ref().unwrap())));
    let mut asError: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(callerError { message: Rc::new(RefCell::new(Some("typed error".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = asError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value }))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = asError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = asError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = asError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())));
    let mut value = Rc::new(RefCell::new(Some(callerError { message: Rc::new(RefCell::new(Some("caller error".to_string()))), ..Default::default() })));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ let __arg_value = { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any> }; __arg_value }))));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({ let __arg_value = { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any> }; __arg_value })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some({ let __arg_value = { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any> }; __arg_value })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::classify(Rc::new(RefCell::new(Some({ let __arg_value = { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new({ let __arg_holder = value.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).__go_value_clone() }) as Box<dyn Any> }; __arg_value })))).borrow().as_ref().unwrap())));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{ go_register_any_runtime_type::<Point>("main.Point", |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<Point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, None); Box::new(Point { x: Rc::new(RefCell::new(Some(5))), y: Rc::new(RefCell::new(Some(6))), ..Default::default() }) as Box<dyn Any> }, { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new(callerError { message: Rc::new(RefCell::new(Some("copied error".to_string()))), ..Default::default() }) as Box<dyn Any> }]))));
    let mut boxed: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); Box::new(callerError { message: Rc::new(RefCell::new(Some("preboxed".to_string()))), ..Default::default() }) as Box<dyn Any> })));
    println!("{}", format!("{}", (*example_com_anyfromcaller_logger::describe(go_any_handle_copy(&boxed)).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", example_com_anyfromcaller_logger::require_error(go_any_handle_copy(&boxed)).borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    let mut pointerAsError: Rc<RefCell<Option<Rc<dyn StdError>>>> = Rc::new(RefCell::new(Some(Rc::new(pointerError { message: Rc::new(RefCell::new(Some("typed pointer error".to_string()))), ..Default::default() }) as Rc<dyn StdError>)));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value }))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Rc::new(RefCell::new(Some({ let __arg_value = { let __err_holder = pointerAsError.clone(); let __err_guard = __err_holder.borrow(); match __err_guard.as_ref() { None => Box::new(GoNilAny) as Box<dyn Any>, Some(__err) => if let Some(typed_val) = __err.downcast_ref::<callerError>() { { go_register_any_runtime_type::<callerError>("main.callerError", |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<callerError>().expect("registered interface error type mismatch"); Rc::new(__typed.__go_value_clone()) as Rc<dyn StdError> })); go_box_any_with_metadata(typed_val.clone(), "struct", true) } } else if let Some(typed_val) = __err.downcast_ref::<pointerError>() { { go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); go_box_any_with_metadata(Rc::new(RefCell::new(Some(typed_val.clone()))), "pointer", true) } } else { Box::new(Rc::new(RefCell::new(Some(__err.clone())))) as Box<dyn Any> } } }; __arg_value })))).borrow().as_ref().unwrap())));
    let mut pointer = Rc::new(RefCell::new(Some(pointerError { message: Rc::new(RefCell::new(Some("pointer error".to_string()))), ..Default::default() })));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> }))));
    example_com_anyfromcaller_logger::each(Rc::new(RefCell::new(Some(vec![{ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> }]))));
    println!("{} {} {}", format!("{}", (*example_com_anyfromcaller_logger::describe(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::kind(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> })))).borrow().as_ref().unwrap())), format!("{}", (*example_com_anyfromcaller_logger::classify(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> })))).borrow().as_ref().unwrap())));
    let mut bound = example_com_anyfromcaller_logger::require_error(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(pointer.clone()) as Box<dyn Any> }))));
    { let new_val = pointerError { message: Rc::new(RefCell::new(Some("replaced".to_string()))), ..Default::default() }; *pointer.borrow_mut() = Some(new_val); };
    println!("{}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", bound.borrow().as_ref().unwrap())))).borrow().as_ref().unwrap())));
    let mut typedNil: Rc<RefCell<Option<pointerError>>> = Rc::new(RefCell::new(None));
    println!("{}", format!("{}", { let __nil_result = (*example_com_anyfromcaller_logger::require_error(Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<Rc<RefCell<Option<pointerError>>>>("*main.pointerError", |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface formatter type mismatch"); match (*__typed.borrow()).as_ref() { Some(__pointee) => format!("{}", __pointee), None => "<nil>".to_string() } }, |__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface clone type mismatch"); Box::new(__typed.clone()) as Box<dyn Any> }, Some(|__value| { let __typed = __value.downcast_ref::<Rc<RefCell<Option<pointerError>>>>().expect("registered interface error type mismatch"); Rc::new(GoPointerError::new(__typed.clone())) as Rc<dyn StdError> })); Box::new(typedNil.clone()) as Box<dyn Any> })))).borrow()).is_none(); __nil_result }));
    let mut numbers: Rc<RefCell<Option<Box<dyn Any>>>> = example_com_anyfromcaller_logger::numbers().clone();
    println!("{}", format!("{}", match numbers.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));
    example_com_anyfromcaller_logger::show(Rc::new(RefCell::new(Some(Box::new(7i64) as Box<dyn Any>))));
}

impl GoValueClone for Point {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for callerError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for pointerError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
