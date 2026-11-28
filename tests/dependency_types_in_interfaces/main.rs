#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::fmt::{Display};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
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
    } else if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<example_com_deptypesany_dep::r#mod::tally>>>>>() {
        format_slice(v)
    } else if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::label_>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::named>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::point>() {
        format!("{}", v)
    } else if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::tally>() {
        format!("{}", v)
    } else if let Some(__go_text) = go_format_registered_any(value) {
        __go_text
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
}

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
    if let Some(v) = value.downcast_ref::<Rc<RefCell<Option<Vec<example_com_deptypesany_dep::r#mod::tally>>>>>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::label_>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::named>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::point>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<example_com_deptypesany_dep::r#mod::tally>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
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

fn __go_type_name(val: &dyn Any) -> &'static str {
    if val.is::<i64>() { return "int" }
    if val.is::<i32>() { return "int32" }
    if val.is::<char>() { return "int32" }
    if val.is::<i8>() { return "int8" }
    if val.is::<i16>() { return "int16" }
    if val.is::<u64>() { return "uint" }
    if val.is::<u32>() { return "uint32" }
    if val.is::<u8>() { return "uint8" }
    if val.is::<u16>() { return "uint16" }
    if val.is::<f64>() { return "float64" }
    if val.is::<f32>() { return "float32" }
    if val.is::<bool>() { return "bool" }
    if val.is::<String>() { return "string" }
    if val.is::<Vec<i64>>() { return "[]int" }
    if val.is::<Vec<i32>>() { return "[]int32" }
    if val.is::<Vec<f64>>() { return "[]float64" }
    if val.is::<Vec<String>>() { return "[]string" }
    if val.is::<Vec<bool>>() { return "[]bool" }
    if val.is::<Rc<RefCell<Option<Vec<example_com_deptypesany_dep::r#mod::tally>>>>>() { return "[]dep.tally" }
    if val.is::<example_com_deptypesany_dep::r#mod::label_>() { return "dep.label" }
    if val.is::<example_com_deptypesany_dep::r#mod::named>() { return "dep.named" }
    if val.is::<example_com_deptypesany_dep::r#mod::point>() { return "dep.point" }
    if val.is::<example_com_deptypesany_dep::r#mod::tally>() { return "dep.tally" }
    if let Some(__go_name) = go_registered_any_type_name(val) { return __go_name }
    std::any::type_name_of_val(val)
}

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

pub fn show(v: Rc<RefCell<Option<Box<dyn Any>>>>) {
    print!("{} {}\n", match v.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }, match v.borrow().as_ref() { Some(__any_value) => __go_type_name(&**__any_value), None => "<nil>" });
}

fn main() {
    go_install_panic_exit();
    ::example_com_deptypesany_dep::__go_init_all();

    let mut a: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new({ let __v = example_com_deptypesany_dep::new(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>)));
    let mut b: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new({ let __v = example_com_deptypesany_dep::label(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>)));
    let mut c: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some({ go_register_any_runtime_type::<example_com_deptypesany_dep::r#mod::point>("dep.point", |__value| { let __typed = __value.downcast_ref::<example_com_deptypesany_dep::r#mod::point>().expect("registered interface formatter type mismatch"); format!("{}", __typed) }, |__value| { let __typed = __value.downcast_ref::<example_com_deptypesany_dep::r#mod::point>().expect("registered interface clone type mismatch"); Box::new(__typed.__go_value_clone()) as Box<dyn Any> }, None); Box::new({ let __v = example_com_deptypesany_dep::p(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any> })));
    let mut d: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new({ let __v = example_com_deptypesany_dep::n(); let __owned = (*__v.borrow().as_ref().unwrap()).clone(); __owned }) as Box<dyn Any>)));
    let mut e: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new(example_com_deptypesany_dep::scores().clone()) as Box<dyn Any>)));
    show(go_any_handle_copy(&a));
    show(go_any_handle_copy(&b));
    show(go_any_handle_copy(&c));
    show(go_any_handle_copy(&d));
    show(go_any_handle_copy(&e));
    println!("{} {} {} {} {}", format!("{}", match a.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match b.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match c.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match d.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", match e.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }));
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(format!("{}", format!("{}", match a.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }))))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(format!("{}|{}|{}", match a.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }, match d.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }, match b.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() })))).borrow().as_ref().unwrap())));
    {
        let (mut t, mut ok) = ({
        let val = a.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<example_com_deptypesany_dep::r#mod::named>() {
                (Rc::new(RefCell::new(Some(Box::new(typed_val.clone()) as Box<dyn GoAnonymousInterface1>))), true)
            } else {
                (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<Box<dyn GoAnonymousInterface1>>)), false)
        }
    });;
        if ok {
            println!("{} {}", format!("{}", "stringer?".to_string()), format!("{}", (*(*t.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap())));;
        } else {
            println!("{}", format!("{}", "tally has no String".to_string()));;
        }
    }
}

pub trait GoAnonymousInterface1: std::fmt::Display + Any {
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_print(&self, plus: bool) -> String { if plus { format!("{:#}", self) } else { format!("{}", self) } }
    fn __go_eq_go_anonymous_interface1(&self, other: &dyn GoAnonymousInterface1) -> bool;
    fn string(&self) -> Rc<RefCell<Option<String>>>;
}

impl Clone for Box<dyn GoAnonymousInterface1> {
    fn clone(&self) -> Self {
        GoAnonymousInterface1::__go_clone_box_go_anonymous_interface1(self.as_ref())
    }
}

impl GoAnonymousInterface1 for example_com_deptypesany_dep::r#mod::named {
    fn string(&self) -> Rc<RefCell<Option<String>>> {
        example_com_deptypesany_dep::r#mod::named::string(self)
    }
    fn __go_clone_box_go_anonymous_interface1(&self) -> Box<dyn GoAnonymousInterface1> {
        Box::new(self.clone()) as Box<dyn GoAnonymousInterface1>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_go_anonymous_interface1(&self, other: &dyn GoAnonymousInterface1) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<example_com_deptypesany_dep::r#mod::named>() {
            false
        } else {
            false
        }
    }
}