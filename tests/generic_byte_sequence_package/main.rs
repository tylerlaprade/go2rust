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

fn main() {
    go_install_panic_exit();
    ::example_com_seqpkg_seq::__go_init_all();

    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::trim::<String>(Rc::new(RefCell::new(Some("hello".to_string()))), Rc::new(RefCell::new(Some(2)))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*example_com_seqpkg_seq::trim::<Vec<u8>>(Rc::new(RefCell::new(Some(("hello".to_string()).as_bytes().to_go_slice()))), Rc::new(RefCell::new(Some(3)))).borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::mid::<String>(Rc::new(RefCell::new(Some("abcdef".to_string()))), Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(4)))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*example_com_seqpkg_seq::mid::<Vec<u8>>(Rc::new(RefCell::new(Some(("abcdef".to_string()).as_bytes().to_go_slice()))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(5)))).borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", example_com_seqpkg_seq::digits::<String>(Rc::new(RefCell::new(Some("a1b22c".to_string()))))), format!("{}", example_com_seqpkg_seq::digits::<Vec<u8>>(Rc::new(RefCell::new(Some(("9x9".to_string()).as_bytes().to_go_slice()))))));
    println!("{} {}", format!("{}", (*example_com_seqpkg_seq::prefix::<String>(Rc::new(RefCell::new(Some("xyz".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_seqpkg_seq::prefix::<Vec<u8>>(Rc::new(RefCell::new(Some(("pq!".to_string()).as_bytes().to_go_slice())))).borrow().as_ref().unwrap())));
    let (mut n, mut rest) = example_com_seqpkg_seq::atoi::<String>(Rc::new(RefCell::new(Some("123abc".to_string()))));
    let (mut m, mut restb) = example_com_seqpkg_seq::atoi::<Vec<u8>>(Rc::new(RefCell::new(Some(("42!".to_string()).as_bytes().to_go_slice()))));
    let mut restb = { let __slice_header = GoSliceHeader::go_slice_header(&restb); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {} {} {}", format!("{}", n), format!("{}", { let __v = (*rest.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", m), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*restb.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    let mut b = Rc::new(RefCell::new(Some(("shared".to_string()).as_bytes().to_go_slice())));
    let mut t = example_com_seqpkg_seq::trim::<Vec<u8>>(b.clone(), Rc::new(RefCell::new(Some(1))));
    let mut t = { let __slice_header = GoSliceHeader::go_slice_header(&t); Rc::new(RefCell::new(__slice_header)) };
    { let __idx = (0) as usize; let __rhs = ('H' as u8) as u8; (*t.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*b.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())), format!("{}", (*Rc::new(RefCell::new(Some(String::from_utf8(std::vec::Vec::from((*t.borrow().as_ref().unwrap()).clone())).unwrap()))).borrow().as_ref().unwrap())));
    let mut short = Rc::new(RefCell::new(Some({ let mut v: Vec<u8> = (0..(8) as usize).map(|_| 0u8).collect(); v.truncate((1) as usize); v })));
    { let __idx = (0) as usize; let __rhs = ('a' as u8) as u8; (*short.borrow_mut().as_mut().unwrap())[__idx] = __rhs; };
    println!("{} {}", format!("{}", format_slice(&example_com_seqpkg_seq::mid::<Vec<u8>>(short.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(5)))))), format!("{}", ({ let __len = (*example_com_seqpkg_seq::mid::<Vec<u8>>(short.clone(), Rc::new(RefCell::new(Some(0))), Rc::new(RefCell::new(Some(5)))).borrow()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len })));
}