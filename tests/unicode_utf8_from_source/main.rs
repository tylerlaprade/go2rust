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
        let formatted: Vec<String> = s.as_ref().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T>(slice: &[T]) -> String
where
    T: Display,
{
    let formatted: Vec<String> = slice.iter().map(|v| v.to_string()).collect();
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

fn main() {
    ::unicode_utf8::__go_init_all();

    for s in &vec!["\u{263a}".to_string(), "\u{e9}".to_string(), "a".to_string(), "\u{6f22}\u{5b57}".to_string(), "\u{1d11e}x".to_string()] {
        let (mut r, mut size) = unicode_utf8::decode_rune_in_string(Rc::new(RefCell::new(Some((*s).clone()))));
        println!("{} {} {} {} {}", format!("{}", r), format!("{}", size), format!("{}", unicode_utf8::rune_len(Rc::new(RefCell::new(Some(r))))), format!("{}", unicode_utf8::valid_string(Rc::new(RefCell::new(Some((*s).clone()))))), format!("{}", unicode_utf8::rune_count_in_string(Rc::new(RefCell::new(Some((*s).clone()))))));
    }
    let mut b = unicode_utf8::append_rune(Rc::new(RefCell::new(None)), Rc::new(RefCell::new(Some(0x263A as i32))));
    let mut b = { let __slice_header = GoSliceHeader::go_slice_header(&b); Rc::new(RefCell::new(__slice_header)) };
    println!("{} {} {} {}", format!("{}", format_slice(&b)), format!("{}", unicode_utf8::full_rune(Rc::new(RefCell::new(Some({ let __seq_holder = b.clone(); let __seq_guard = __seq_holder.borrow(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = 0; let __high = (2) as usize; let __max = __source_cap; __seq.reslice3(__low, __high, __max) }))))), format!("{}", unicode_utf8::valid(b.clone())), format!("{}", unicode_utf8::rune_count(b.clone())));
    let (mut r, mut n) = unicode_utf8::decode_last_rune_in_string(Rc::new(RefCell::new(Some("x\u{263a}".to_string()))));
    println!("{} {} {} {}", format!("{}", r), format!("{}", n), format!("{}", unicode_utf8::valid_rune(Rc::new(RefCell::new(Some(0xD800 as i32))))), format!("{}", unicode_utf8::valid_rune(Rc::new(RefCell::new(Some(('\u{263a}' as i32)))))));
    let mut buf = Rc::new(RefCell::new(Some(vec![0; (4) as usize])));
    println!("{} {} {}", format!("{}", unicode_utf8::encode_rune(buf.clone(), Rc::new(RefCell::new(Some(('\u{1d11e}' as i32)))))), format!("{}", format_slice(&buf)), format!("{}", unicode_utf8::rune_start(Rc::new(RefCell::new(Some((*buf.borrow().as_ref().unwrap())[(1) as usize].clone()))))));
}