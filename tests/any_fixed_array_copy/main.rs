#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};


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
    if let Some(v) = value.downcast_ref::<[[i64; 2]; 1]>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    if let Some(v) = value.downcast_ref::<[i64; 2]>() { return Some(Box::new(v.clone()) as Box<dyn Any>); }
    None
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

fn main() {
    ::example_com_anyfixedarraycopy_check::__go_init_all();

    let mut original: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new([1i64, 2i64]) as Box<dyn Any>)));
    println!("{} {}", format!("{}", example_com_anyfixedarraycopy_check::preserved(go_any_handle_copy(&original))), format!("{}", { let __nil_result = (*original.borrow()).is_none(); __nil_result }));
    let mut copied = go_any_handle_copy(&original);
    *original.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*original.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*copied.borrow()).is_none(); __nil_result }));
    let mut nested: Rc<RefCell<Option<Box<dyn Any>>>> = Rc::new(RefCell::new(Some(Box::new([[3i64, 4i64]]) as Box<dyn Any>)));
    let mut nestedCopy = go_any_handle_copy(&nested);
    *nested.borrow_mut() = None;
    println!("{} {}", format!("{}", { let __nil_result = (*nested.borrow()).is_none(); __nil_result }), format!("{}", { let __nil_result = (*nestedCopy.borrow()).is_none(); __nil_result }));
}