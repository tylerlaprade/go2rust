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


fn go_install_panic_exit() {
    std::panic::set_hook(Box::new(|__info| {
        let __text = go_panic_payload_text(__info.payload());
        {
            use std::io::Write;
            std::io::stdout().flush().ok();
        }
        eprintln!("panic: {}", __text);
        std::process::exit(2);
    }));
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text(__text.clone());
    }
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
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

pub fn local__string(v: Rc<RefCell<Option<String>>>) -> Rc<RefCell<Option<String>>> {
    { let __owned = v.borrow().as_ref().unwrap().clone(); Rc::new(RefCell::new(Some(__owned))) }
}

pub fn local__int(v: Rc<RefCell<Option<i64>>>) -> i64 {
    (*v.borrow().as_ref().unwrap())
}

fn main() {
    go_install_panic_exit();
    ::example_com_genericcallresultsprint_dep::__go_init_all();

    println!("{} {}", format!("{}", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("qualified".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("inferred".to_string())))).borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", (*local__string(Rc::new(RefCell::new(Some("local".to_string())))).borrow().as_ref().unwrap())), format!("{}", local__int(Rc::new(RefCell::new(Some(3))))));
    println!("{} {}", format!("{}", (*example_com_genericcallresultsprint_dep::join::<String, i64>(Rc::new(RefCell::new(Some("a".to_string()))), Rc::new(RefCell::new(Some(12))), Rc::new(RefCell::new(Some("-".to_string())))).borrow().as_ref().unwrap())), format!("{}", (*example_com_genericcallresultsprint_dep::join::<String, String>(Rc::new(RefCell::new(Some("b".to_string()))), Rc::new(RefCell::new(Some("c".to_string()))), Rc::new(RefCell::new(Some("+".to_string())))).borrow().as_ref().unwrap())));
    print!("{}|{}\n", (*example_com_genericcallresultsprint_dep::pick::<String>(Rc::new(RefCell::new(Some("formatted".to_string())))).borrow().as_ref().unwrap()), (*example_com_genericcallresultsprint_dep::join::<i64, i64>(Rc::new(RefCell::new(Some(1))), Rc::new(RefCell::new(Some(2))), Rc::new(RefCell::new(Some(":".to_string())))).borrow().as_ref().unwrap()));
}