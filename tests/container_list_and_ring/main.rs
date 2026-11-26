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
    } else {
        panic!("go2rust: cannot print this value held in an interface: its type has no formatting case yet")
    }
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

pub fn print_list(l: Rc<RefCell<Option<container_list::r#mod::List>>>) {
    let mut e = (*l.borrow().as_ref().unwrap()).front();
    while { let __nil_result = (*e.borrow()).is_some(); __nil_result } {
        print!("{}{}", format!("{}", match (*e.borrow().as_ref().unwrap()).value.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", " ".to_string()));
        { let new_val = (*e.borrow().as_ref().unwrap()).next().clone(); e = new_val; };
    }
    println!("{} {}", format!("{}", "| len".to_string()), format!("{}", (*l.borrow().as_ref().unwrap()).len()));
}

fn main() {
    go_install_panic_exit();
    ::container_list::__go_init_all();
    ::container_ring::__go_init_all();

    let mut l = container_list::new();
    let mut e4 = (*l.borrow_mut().as_mut().unwrap()).push_back(Rc::new(RefCell::new(Some(Box::new(4i64) as Box<dyn Any>))));
    let mut e1 = (*l.borrow_mut().as_mut().unwrap()).push_front(Rc::new(RefCell::new(Some(Box::new(1i64) as Box<dyn Any>))));
    (*l.borrow_mut().as_mut().unwrap()).insert_before(Rc::new(RefCell::new(Some(Box::new(3i64) as Box<dyn Any>))), e4.clone());
    (*l.borrow_mut().as_mut().unwrap()).insert_after(Rc::new(RefCell::new(Some(Box::new(2i64) as Box<dyn Any>))), e1.clone());
    print_list(l.clone());
    (*l.borrow().as_ref().unwrap()).move_to_back(e1.clone());
    (*l.borrow().as_ref().unwrap()).move_before(e4.clone(), (*l.borrow().as_ref().unwrap()).front());
    print_list(l.clone());
    (*l.borrow_mut().as_mut().unwrap()).remove(e4.clone());
    let mut other = container_list::new();
    (*other.borrow_mut().as_mut().unwrap()).push_back(Rc::new(RefCell::new(Some(Box::new(9i64) as Box<dyn Any>))));
    (*l.borrow_mut().as_mut().unwrap()).push_back_list(other.clone());
    print_list(l.clone());
    let mut e = (*l.borrow().as_ref().unwrap()).back();
    while { let __nil_result = (*e.borrow()).is_some(); __nil_result } {
        print!("{}{}", format!("{}", match (*e.borrow().as_ref().unwrap()).value.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", " ".to_string()));
        { let new_val = (*e.borrow().as_ref().unwrap()).prev().clone(); e = new_val; };
    }
    println!();
    let mut r = container_ring::new(Rc::new(RefCell::new(Some(4))));
    let mut i = Rc::new(RefCell::new(Some(0i64)));
    while (*i.borrow().as_ref().unwrap()) < container_ring::__go_nil_recv_ring_len(r.clone()) {
        { let new_val = Box::new({ let __bin_i = (*i.borrow().as_ref().unwrap()).clone(); __bin_i * __bin_i }) as Box<dyn Any>; *(*r.borrow().as_ref().unwrap()).value.borrow_mut() = Some(new_val); };
        { let new_val = (*r.borrow_mut().as_mut().unwrap()).next().clone(); r = new_val; };
        { let mut guard = i.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    container_ring::__go_nil_recv_ring_r_do(r.clone(), Rc::new(RefCell::new(Some(Rc::new(move |v: Rc<RefCell<Option<Box<dyn Any>>>>| {
        print!("{}{}", format!("{}", match v.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", " ".to_string()));
    }) as Rc<dyn Fn(Rc<RefCell<Option<Box<dyn Any>>>>) -> ()>))));
    println!("{}", format!("{}", container_ring::__go_nil_recv_ring_len(r.clone())));
}