use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::sync::{Arc, Mutex};

pub fn has_stmt(stmts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> bool {
    let mut prev: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        { let __iface_handle = stmt.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *prev.lock().unwrap() = __iface_value; };
        if accept_stmt(stmt.clone()) {
        return accept_stmt(prev.clone());
    }
    } }
    false
}

pub fn accept_stmt(stmt: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>) -> bool {
    return { let __nil_result = (*stmt.lock().unwrap()).is_some(); __nil_result };
}

pub fn stmt_kind(stmts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<String>>> {
    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        {
    let _ts_subject = stmt.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ExprStmtPtr>()).unwrap().0.clone();
        let _ = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap()).clone();;
        return Arc::new(Mutex::new(Some("expr".to_string())));;
    } else {
        let s = _ts_subject.clone();
        return Arc::new(Mutex::new(Some("other".to_string())));;
    }
    }
    unreachable!()
    } }
    Arc::new(Mutex::new(Some("none".to_string())))
}

pub fn assert_expr_stmt(stmts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Stmt + Send + Sync>>>>>>>>) -> bool {
    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        let mut expr = ({
        let val = stmt.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Stmt + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::ExprStmtPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    }).clone();
        return { let __nil_result = (*expr.lock().unwrap()).is_some(); __nil_result };
    } }
    false
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();

    let mut stmts = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ExprStmtPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ExprStmt { x: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Stmt + Send + Sync>)))])));
    if false {
        println!("{}", format!("{}", (*stmt_kind(stmts.clone()).lock().unwrap().as_ref().unwrap())));
        println!("{}", format!("{}", assert_expr_stmt(stmts.clone())));
    }
    println!("{}", format!("{}", has_stmt(stmts.clone())));
}