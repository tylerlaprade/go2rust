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
    // Go lets a program call main again; the hook goes in once, so a
    // later call keeps the hook that go_silence_panic_hook wrapped around it.
    static __GO_PANIC_EXIT: std::sync::Once = std::sync::Once::new();
    __GO_PANIC_EXIT.call_once(|| {
        std::panic::set_hook(Box::new(|__info| {
            let __text = go_panic_payload_text(__info.payload());
            {
                use std::io::Write;
                std::io::stdout().flush().ok();
            }
            eprintln!("panic: {}", __text);
            std::process::exit(2);
        }));
    });
}

fn go_panic_payload_text(__payload: &dyn Any) -> String {
    if let Some(__text) = __payload.downcast_ref::<&'static str>() {
        return go_runtime_error_text(__text.to_string());
    }
    if let Some(__text) = __payload.downcast_ref::<String>() {
        return go_runtime_error_text_of_string(__text.clone());
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

fn go_runtime_error_text_of_string(__text: String) -> String {
    // panic(s) in Go raises a String, and so does a runtime error whose text
    // Rust formats: an index out of range, or a nil pointer read through
    // expect(). The constant runtime errors arrive as a &'static str, so a
    // Go string that reads like one keeps its text.
    if __text.starts_with("index out of bounds: ") || __text == "nil pointer dereference" || __text == "invalid memory address or nil pointer dereference" {
        return go_runtime_error_text(__text);
    }
    __text
}

/// Comma-ok type assertion on a field whose type is an interface imported from
/// another package (ast.Expr). The assertion operand must stay the wrapped
/// interface handle so the downcast can unwrap it; go/parser does this on
/// ast.Expr/ast.Stmt fields throughout (`typ, ok := x.(*ast.ChanType)`).
pub fn describe(f: Rc<RefCell<Option<example_com_commaok_ast::r#mod::Field>>>) -> Rc<RefCell<Option<String>>> {
    {
        let (mut ct, mut ok) = ({
        let val = (*f.borrow().as_ref().unwrap()).value.clone();
        let guard = val.borrow();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn example_com_commaok_ast::r#mod::Expr>::__go_as_any(any_val.as_ref()).downcast_ref::<example_com_commaok_ast::r#mod::ChanTypePtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Rc::new(RefCell::new(None::<example_com_commaok_ast::r#mod::ChanType>)), false)
            }
        } else {
            (Rc::new(RefCell::new(None::<example_com_commaok_ast::r#mod::ChanType>)), false)
        }
    });;
        if ok {
            return Rc::new(RefCell::new(Some(format!("chan {}", (*(*ct.borrow().as_ref().unwrap()).dir.borrow().as_ref().unwrap())))));;
        }
    }
    Rc::new(RefCell::new(Some("other".to_string())))
}

fn main() {
    go_install_panic_exit();
    ::example_com_commaok_ast::__go_init_all();

    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Field { value: Rc::new(RefCell::new(Some(Box::new(example_com_commaok_ast::r#mod::ChanTypePtr(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::ChanType { dir: Rc::new(RefCell::new(Some(3))), ..Default::default() }))).clone())) as Box<dyn example_com_commaok_ast::r#mod::Expr>))), ..Default::default() })))).borrow().as_ref().unwrap())));
    println!("{}", format!("{}", (*describe(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Field { value: Rc::new(RefCell::new(Some(Box::new(example_com_commaok_ast::r#mod::IdentPtr(Rc::new(RefCell::new(Some(example_com_commaok_ast::r#mod::Ident { name: Rc::new(RefCell::new(Some("x".to_string()))), ..Default::default() }))).clone())) as Box<dyn example_com_commaok_ast::r#mod::Expr>))), ..Default::default() })))).borrow().as_ref().unwrap())));
}