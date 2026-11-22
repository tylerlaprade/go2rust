#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::rc::{Rc};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}

fn __go_collect_os_args() -> Vec<String> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        std::env::args_os()
            .map(|arg| __go_string_from_bytes(arg.into_vec()))
            .collect::<Vec<String>>()
    }
    #[cfg(not(unix))]
    {
        std::env::args().collect::<Vec<String>>()
    }
}

thread_local! {
    static __GO_OS_ARGS: std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> =
        std::rc::Rc::new(std::cell::RefCell::new(Some(__go_collect_os_args())));
}

fn go_os_args() -> std::rc::Rc<std::cell::RefCell<Option<Vec<String>>>> {
    __GO_OS_ARGS.with(|args| args.clone())
}

fn main() {
    ::example_com_package_stdlib_interface_argument_helper::__go_init_all();

    let mut __defer_stack: std::vec::Vec<Box<dyn FnOnce()>> = std::vec::Vec::new();

    let (mut f, mut err) = os::open(go_os_args().borrow().as_ref().unwrap().get_cloned((0) as usize));
    if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
        panic!("{}", (*err.borrow().as_ref().unwrap()));
    }
    let f_defer_captured = f.clone(); __defer_stack.push(Box::new(move || {
        (*f_defer_captured.borrow_mut().as_mut().unwrap()).close();
    }));

    { let __s = { let mut __go_sprint_bytes = std::vec::Vec::<u8>::new(); let __go_sprint_arg_0 = (*example_com_package_stdlib_interface_argument_helper::r#use({ let __arg = f.clone(); let __converted = { let __arg_guard = __arg.borrow(); let __converted: Option<io_Reader> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Rc::new(RefCell::new(__converted)) }).borrow().as_ref().unwrap()).clone(); __go_sprint_bytes.extend(__go_string_bytes(&__go_sprint_arg_0)); __go_sprint_bytes.push(b'\n'); __go_string_from_bytes(__go_sprint_bytes) }; let __bytes = __go_string_bytes(&__s); let __n = __bytes.len() as i64; match std::io::Write::write_all(&mut std::io::stdout(), &__bytes) { Ok(()) => (__n, Rc::new(RefCell::new(None::<Rc<dyn StdError>>))), Err(e) => (0, Rc::new(RefCell::new(Some::<Rc<dyn StdError>>(Rc::<dyn StdError>::from(Box::<dyn StdError>::from(e)))))) } };

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}