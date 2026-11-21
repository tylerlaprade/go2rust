#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use unicode_utf8::*;

use std::any::Any;
use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
/// TestConstants is copied verbatim from unicode/utf8's own utf8_test.go.
/// Keeping the package test body unchanged makes this fixture an oracle for the
/// source-transpiled package rather than a hand-written approximation of it.
pub fn test_constants(t: Rc<RefCell<Option<oracle_testing::r#mod::T>>>) {
    if MAX_RUNE != unicode::MAX_RUNE {
        (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("utf8.MaxRune is wrong: %x should be %x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(1114111i32) as Box<dyn Any>, Box::new(1114111i32) as Box<dyn Any>]))));
    }
    if RUNE_ERROR != unicode::REPLACEMENT_CHAR {
        (*t.borrow_mut().as_mut().unwrap()).errorf(Rc::new(RefCell::new(Some("utf8.RuneError is wrong: %x should be %x".to_string()))), Rc::new(RefCell::new(Some(vec![Box::new(65533i32) as Box<dyn Any>, Box::new(65533i32) as Box<dyn Any>]))));
    }
}

fn main() {
    ::oracle_testing::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut t = Rc::new(RefCell::new(Some(oracle_testing::r#mod::T { ..Default::default() })));
    test_constants(t.clone());
    if (*t.borrow().as_ref().unwrap()).failed() {
        {
            let __go_print_arg_0 = format!("{}", "FAIL TestConstants".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
        {
            let __go_print_arg_0 = format!("{}", "summary 0 passed, 1 failed".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
        return;
    }
    {
            let __go_print_arg_0 = format!("{}", "PASS TestConstants".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
    {
            let __go_print_arg_0 = format!("{}", "summary 1 passed, 0 failed".to_string());
            eprintln!("{}", __go_print_arg_0)
        };
}