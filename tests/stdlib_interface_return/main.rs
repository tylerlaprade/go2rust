#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};


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
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send + Sync>>() {
        let __text = go_panic_payload_text(&**__inner);
        return __text;
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any + Send>>() {
        return go_panic_payload_text(&**__inner);
    }
    if let Some(__inner) = __payload.downcast_ref::<Box<dyn Any>>() {
        return go_panic_payload_text(&**__inner);
    }
    if let Some(__err) = __payload.downcast_ref::<std::sync::Arc<std::sync::Mutex<Option<std::sync::Arc<dyn std::error::Error + Send + Sync>>>>>() {
        return __err.lock().unwrap().as_ref().map(|__err| __err.to_string()).unwrap_or_else(|| String::from("nil"));
    }
    macro_rules! go_panic_display_text {
        ($($t:ty),*) => {
            $(if let Some(__value) = __payload.downcast_ref::<$t>() {
                return __value.to_string();
            })*
        };
    }
    go_panic_display_text!(i64, i32, i16, i8, u64, u32, u16, u8, usize, isize, bool, char);
    if let Some(__value) = __payload.downcast_ref::<f64>() {
        // Go prints a float in a panic as +1.500000e+000.
        if __value.is_nan() {
            return String::from("NaN");
        }
        if __value.is_infinite() {
            return String::from(if *__value > 0.0 { "+Inf" } else { "-Inf" });
        }
        let __text = format!("{:+.6e}", __value);
        if let Some((__mantissa, __exponent)) = __text.split_once('e') {
            let __exponent: i32 = __exponent.parse().unwrap_or(0);
            return format!("{}e{}{:03}", __mantissa, if __exponent < 0 { '-' } else { '+' }, __exponent.abs());
        }
        return __text;
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

pub fn make_expr() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))
}

pub fn make_ident_expr() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))
}

pub fn make_unary_expr() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::UnaryExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::UnaryExpr { x: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))
}

pub fn make_var_expr() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    let mut expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    return expr.clone();
}

pub fn make_assigned_selector_expr() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    let mut expr: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)));
    { let __iface_handle = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr {
        x: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("pkg".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))),
        sel: go_ast::new_ident(Arc::new(Mutex::new(Some("Name".to_string())))).clone(),
        ..Default::default()
    }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *expr.lock().unwrap() = __iface_value; };
    return expr.clone();
}

pub fn make_expr_slice_len() -> i64 {
    let mut exprs = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])));
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = exprs.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("y".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *exprs.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = exprs.clone(); let __append_value = make_ident_expr().clone(); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *exprs.lock().unwrap() = __slice_header; };
    { let __slice_header = GoSliceHeader::go_slice_header(&({ let __append_target = exprs.clone(); let __append_value = Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::SelectorExprPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::SelectorExpr {
        x: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("pkg".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))),
        sel: go_ast::new_ident(Arc::new(Mutex::new(Some("Name".to_string())))).clone(),
        ..Default::default()
    }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))); let mut __appended: Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>> = (*__append_target.lock().unwrap()).clone().unwrap_or_default(); __appended.push(__append_value); Arc::new(Mutex::new(Some(__appended))) })); *exprs.lock().unwrap() = __slice_header; };
    return ({ let __len = (*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0); __len }) as i64;
}

pub fn asserted_field_element() -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>> {
    let mut field = Arc::new(Mutex::new(Some(go_ast::r#mod::Field { r#type: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ArrayTypePtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ArrayType { elt: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("int".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() })));
    return { let __iface_value = (*({
        let val = (*field.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ArrayTypePtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).elt.lock().unwrap().clone(); Arc::new(Mutex::new(__iface_value)) };
}

fn main() {
    go_install_panic_exit();
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_token::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math::__go_init_all();
    ::math_bits::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    if false {
        println!("{}", format!("{}", { let __nil_result = (*asserted_field_element().lock().unwrap()).is_some(); __nil_result }));
    }
    println!("{}", format!("{}", { let __nil_result = (*make_expr().lock().unwrap()).is_some(); __nil_result }));
    println!("{}", format!("{}", { let __nil_result = (*make_ident_expr().lock().unwrap()).is_some(); __nil_result }));
    println!("{}", format!("{}", { let __nil_result = (*make_unary_expr().lock().unwrap()).is_some(); __nil_result }));
    println!("{}", format!("{}", { let __nil_result = (*make_var_expr().lock().unwrap()).is_some(); __nil_result }));
    println!("{}", format!("{}", { let __nil_result = (*make_assigned_selector_expr().lock().unwrap()).is_some(); __nil_result }));
    println!("{}", format!("{}", make_expr_slice_len()));
}