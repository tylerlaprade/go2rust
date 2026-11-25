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
    container_ring::__go_nil_recv_ring_r_do(r.clone(), Rc::new(RefCell::new(Some(Box::new(move |v: Rc<RefCell<Option<Box<dyn Any>>>>| {
        print!("{}{}", format!("{}", match v.borrow().as_ref() { Some(__any_value) => format_any(__any_value.as_ref()), None => "<nil>".to_string() }), format!("{}", " ".to_string()));
    }) as Box<dyn FnMut(Rc<RefCell<Option<Box<dyn Any>>>>) -> ()>))));
    println!("{}", format!("{}", container_ring::__go_nil_recv_ring_len(r.clone())));
}