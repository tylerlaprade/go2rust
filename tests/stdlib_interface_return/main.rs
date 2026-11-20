#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
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
    return (*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i64;
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
    ::go_ast::__go_init_all();
    ::go_token::__go_init_all();
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