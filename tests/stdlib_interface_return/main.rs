use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_Expr;

impl std::fmt::Display for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}


impl ast_Expr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_Ident;

impl std::fmt::Display for ast_Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Ident>")
    }
}


impl ast_Ident {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct ast_UnaryExpr {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_UnaryExpr>")
    }
}


impl ast_UnaryExpr {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::default()
    }
}


impl From<ast_UnaryExpr> for ast_Expr {
    fn from(_value: ast_UnaryExpr) -> Self {
        Self::default()
    }
}


pub mod ast {
    use super::*;
    pub fn new_ident<T0>(_arg0: T0) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(Default::default())))
    }
}


pub fn make_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = Arc::new(Mutex::new(Some(ast_Ident { ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) };
}

pub fn make_ident_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) };
}

pub fn make_unary_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = Arc::new(Mutex::new(Some(ast_UnaryExpr { x: { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) };
}

fn main() {
    println!("{}", (*make_expr().lock().unwrap()).is_some());
    println!("{}", (*make_ident_expr().lock().unwrap()).is_some());
    println!("{}", (*make_unary_expr().lock().unwrap()).is_some());
}