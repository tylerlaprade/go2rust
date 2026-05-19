use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

fn main() {
    example_com_package_stdlib_interface_argument_helper::__go_init_all();

    let __go_os_args = Rc::new(RefCell::new(Some(std::env::args().collect::<Vec<String>>())));

    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let (mut f, mut err) = os::open((*__go_os_args.clone().borrow().as_ref().unwrap())[(0) as usize].clone());
    if (*err.borrow()).is_some() {
        panic!("{:?}", (*err.borrow().as_ref().unwrap()));
    }
    let f_defer_captured = f.clone(); __defer_stack.push(Box::new(move || {
        (*f_defer_captured.borrow_mut().as_mut().unwrap()).close();
    }));
    println!("{}", format!("{}", (*example_com_package_stdlib_interface_argument_helper::r#use({ let __arg = f.clone(); let __converted = { let __arg_guard = __arg.borrow(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Rc::new(RefCell::new(Some(__converted))) }).borrow().as_ref().unwrap())));

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}