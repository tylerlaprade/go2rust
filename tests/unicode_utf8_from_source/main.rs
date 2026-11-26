#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
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
    ::unicode_utf8::__go_init_all();

    for s in &vec!["\u{263a}".to_string(), "\u{e9}".to_string(), "a".to_string(), "\u{6f22}\u{5b57}".to_string(), "\u{1d11e}x".to_string()] {
        let (mut r, mut size) = unicode_utf8::decode_rune_in_string(Rc::new(RefCell::new(Some((*s).clone()))));
        println!("{} {} {} {} {}", format!("{}", r), format!("{}", size), format!("{}", unicode_utf8::rune_len(Rc::new(RefCell::new(Some(r))))), format!("{}", unicode_utf8::valid_string(Rc::new(RefCell::new(Some((*s).clone()))))), format!("{}", unicode_utf8::rune_count_in_string(Rc::new(RefCell::new(Some((*s).clone()))))));
    }
    let mut b = unicode_utf8::append_rune(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(0x263A as i32))));
    let mut b = { let __slice_header = GoSliceHeader::go_slice_header(&b); Rc::new(RefCell::new(__slice_header)) };
    { let __go_print_arg_1 = Rc::new(RefCell::new(Some(unicode_utf8::full_rune(Rc::new(RefCell::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); std::mem::drop(__seq_guard); let __low = 0; let __high = (2) as usize; let __max = __source_cap; __seq.reslice3_zeroed(__low, __high, __max) }))))))); let __go_print_arg_2 = Rc::new(RefCell::new(Some(unicode_utf8::valid(b.clone())))); let __go_print_arg_3 = Rc::new(RefCell::new(Some(unicode_utf8::rune_count(b.clone())))); let __go_print_value = println!("{} {} {} {}", format!("{}", format_slice(&b)), format!("{}", { let __v = (*__go_print_arg_1.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_2.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", { let __v = (*__go_print_arg_3.borrow().as_ref().unwrap()).clone(); __v })); __go_print_value };
    let (mut r, mut n) = unicode_utf8::decode_last_rune_in_string(Rc::new(RefCell::new(Some("x\u{263a}".to_string()))));
    println!("{} {} {} {}", format!("{}", r), format!("{}", n), format!("{}", unicode_utf8::valid_rune(Rc::new(RefCell::new(Some(0xD800 as i32))))), format!("{}", unicode_utf8::valid_rune(Rc::new(RefCell::new(Some(('\u{263a}' as i32)))))));
    let mut buf = Rc::new(RefCell::new(Some(vec![0u8; (4) as usize])));
    { let __go_print_arg_4 = Rc::new(RefCell::new(Some(unicode_utf8::encode_rune(buf.clone(), Rc::new(RefCell::new(Some(('\u{1d11e}' as i32)))))))); let __go_print_arg_5 = Rc::new(RefCell::new(Some(unicode_utf8::rune_start(Rc::new(RefCell::new(Some({ let __arg_value = (*buf.borrow().as_ref().unwrap())[(1) as usize].clone(); __arg_value }))))))); let __go_print_value = println!("{} {} {}", format!("{}", { let __v = (*__go_print_arg_4.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", format_slice(&buf)), format!("{}", { let __v = (*__go_print_arg_5.borrow().as_ref().unwrap()).clone(); __v })); __go_print_value };
}