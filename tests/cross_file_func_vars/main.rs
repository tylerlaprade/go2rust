#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
include!("__go2rust_helpers.rs");
mod funcs;
use funcs::*;

use std::cell::{RefCell};
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! print {
    ($($__arg:tt)*) => {{
        std::print!($($__arg)*);
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }};
}
fn main() {
    go_install_panic_exit();
    __go_init_all();

        // Call function variable - transpiler needs to know ProcessData is a function
    let mut result = { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = ProcessData.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(5)))) };
    print!("ProcessData(5) = {}\n", result);

        // Call another function variable
    let mut combined = { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>, Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = CombineStrings.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some("Hello".to_string()))), Rc::new(RefCell::new(Some("World".to_string())))) };
    print!("Combined: {}\n", { let __v = (*combined.borrow().as_ref().unwrap()).clone(); __v });

        // Pass function variable to another function variable
    let mut twice = { let __f: Rc<dyn Fn(Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>>, Rc<RefCell<Option<i64>>>) -> i64> = ApplyTwice.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some({ let __global_func = GoFuncValue::go_func_value(&(*ProcessData.borrow())); Rc::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { (__global_func.as_ref().expect("runtime error: invalid memory address or nil pointer dereference"))(__arg0) }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> }))), Rc::new(RefCell::new(Some(3)))) };
    print!("ApplyTwice(ProcessData, 3) = {}\n", twice);

        // Call no-parameter function variable
    let mut greeting = { let __f: Rc<dyn Fn() -> Rc<RefCell<Option<String>>>> = GetGreeting.borrow().as_ref().unwrap().clone(); (__f)() };
    println!("{}", format!("{}", { let __v = (*greeting.borrow().as_ref().unwrap()).clone(); __v }));

        // Call function with multiple returns
    let (mut q, mut r) = { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<i64>>>) -> (i64, i64)> = DivMod.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(17))), Rc::new(RefCell::new(Some(5)))) };
    print!("17 / 5 = {} remainder {}\n", q, r);

        // Compare with regular function call
    let mut regular = regular_double(Rc::new(RefCell::new(Some(5))));
    print!("RegularDouble(5) = {}\n", regular);

        // Use function returned by function
    let mut triple = make_multiplier(Rc::new(RefCell::new(Some(3))));
    let mut triple = Rc::new(RefCell::new(GoFuncValue::go_func_value(&triple)));
    print!("Triple(4) = {}\n", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = triple.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(4)))) });

        // Call dynamically assigned function
    let mut dynamic = { let __f: Rc<dyn Fn(Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>>> = DynamicFunc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some("test".to_string())))) };
    println!("{}", format!("{}", { let __v = (*dynamic.borrow().as_ref().unwrap()).clone(); __v }));

        // Assign function variable to local variable
    let mut localFunc = Rc::new(RefCell::new(Some({ let __global_func = GoFuncValue::go_func_value(&(*ProcessData.borrow())); Rc::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { (__global_func.as_ref().expect("runtime error: invalid memory address or nil pointer dereference"))(__arg0) }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> })));
    let mut localFunc = Rc::new(RefCell::new(GoFuncValue::go_func_value(&(*ProcessData.borrow()))));
    print!("Local func call: {}\n", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = localFunc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(7)))) });

        // Function variable in conditional
    let mut conditionalFunc: Rc<RefCell<Option<Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>>>> = Rc::new(RefCell::new(None));
    if result > 0 {
        { let __func_value = GoFuncValue::go_func_value(&(*ProcessData.borrow())); *conditionalFunc.borrow_mut() = __func_value; };
    } else {
        { let new_val = Rc::new(move |__arg0: Rc<RefCell<Option<i64>>>| -> i64 { regular_double(__arg0) }) as Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64>; *conditionalFunc.borrow_mut() = Some(new_val); };
    }
    print!("Conditional func(6) = {}\n", { let __f: Rc<dyn Fn(Rc<RefCell<Option<i64>>>) -> i64> = conditionalFunc.borrow().as_ref().unwrap().clone(); (__f)(Rc::new(RefCell::new(Some(6)))) });
}

pub(crate) fn __go_init_all() {
    static __GO_INIT_ONCE: std::sync::Once = std::sync::Once::new();
    __GO_INIT_ONCE.call_once(|| {
        funcs::__go_zero_globals();
        funcs::__go_init_order_0();
        funcs::__go_init_order_1();
        funcs::__go_init_order_2();
        funcs::__go_init_order_3();
        funcs::__go_init_order_4();
        funcs::__go_init_functions();
    });
}
