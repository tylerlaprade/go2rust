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

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
fn format_slice<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: GoSliceView<T>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| v.to_string()).collect();
        format!("[{}]", formatted.join(" "))
    } else {
        "[]".to_string()
    }
}

fn format_slice_values<T, C>(slice: &C) -> String
where
    C: GoSliceView<T> + ?Sized,
    T: Display,
{
    let __view = slice.go_slice_view();
    let formatted: std::vec::Vec<String> = __view.iter().map(|v| v.to_string()).collect();
    format!("[{}]", formatted.join(" "))
}

fn format_slice_wrapped<T, C>(slice: &Rc<RefCell<Option<C>>>) -> String
where
    C: GoSliceView<Rc<RefCell<Option<T>>>>,
    T: Display,
{
    let guard = slice.borrow();
    if let Some(ref s) = *guard {
        let formatted: std::vec::Vec<String> = s.go_slice_view().iter().map(|v| {
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

    check(Rc::new(RefCell::new(Some(vec![66 as u8, 250 as u8, 67 as u8]))));
    check(Rc::new(RefCell::new(Some(vec![0x80 as u8]))));
    check(Rc::new(RefCell::new(Some(vec![0xc0 as u8, 0x80 as u8]))));
    check(Rc::new(RefCell::new(Some(vec![0xe2 as u8, 0x82 as u8]))));
    check(Rc::new(RefCell::new(Some(vec![0xf0 as u8, 0x9f as u8, 0x92 as u8, 0xa9 as u8]))));
    check(Rc::new(RefCell::new(Some(vec![0xef as u8, 0xb7 as u8, 0x90 as u8, 0xef as u8, 0xb7 as u8, 0x91 as u8, 65 as u8]))));
    check(Rc::new(RefCell::new(Some(Vec::from(__go_string_bytes(&("ordinary \u{4e16}\u{754c}".to_string())))))));
}

pub fn check(bytes: Rc<RefCell<Option<Vec<u8>>>>) {
    let mut s = Rc::new(RefCell::new(Some(__go_string_from_bytes(std::vec::Vec::from((*bytes.borrow().as_ref().unwrap()).clone())))));
    println!("{} {} {}", format!("{}", unicode_utf8::valid_string(Rc::new(RefCell::new(Some({ let __arg_holder = s.clone(); let __arg_guard = __arg_holder.borrow(); (*__arg_guard.as_ref().unwrap()).clone() }))))), format!("{}", { let __s = (*s.borrow().as_ref().unwrap()).clone(); __go_string_len(&__s) }), format!("{}", format_slice(&Rc::new(RefCell::new(Some(Vec::from(__go_string_bytes(&((*s.borrow().as_ref().unwrap()))))))))));
}