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


#[derive(Debug, Clone, Default)]
pub struct ast_ExprStmt {
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ExprStmt>")
    }
}


impl ast_ExprStmt {
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


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ast_Stmt;

impl std::fmt::Display for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}


impl ast_Stmt {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::default()
    }
}


impl From<ast_ExprStmt> for ast_Stmt {
    fn from(_value: ast_ExprStmt) -> Self {
        Self::default()
    }
}


pub mod ast {
    use super::*;
    pub fn new_ident<T0>(_arg0: T0) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(Default::default())))
    }
}


pub fn has_stmt(stmts: Arc<Mutex<Option<Vec<ast_Stmt>>>>) -> Arc<Mutex<Option<bool>>> {

    let mut prev: Arc<Mutex<Option<ast_Stmt>>> = Arc::new(Mutex::new(None));
    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for stmt in __range_values.iter() {
        { let new_val = (*stmt).clone(); *prev.lock().unwrap() = Some(new_val); };
        if (*accept_stmt(Arc::new(Mutex::new(Some((*stmt).clone())))).lock().unwrap().as_ref().unwrap()) {
        return accept_stmt(Arc::new(Mutex::new(Some((*prev.lock().unwrap().as_ref().unwrap()).clone()))));
    }
    } }
    return Arc::new(Mutex::new(Some(false)));
}

pub fn accept_stmt(stmt: Arc<Mutex<Option<ast_Stmt>>>) -> Arc<Mutex<Option<bool>>> {

    return Arc::new(Mutex::new(Some((*stmt.lock().unwrap()).is_some())));
}

fn main() {
    let mut stmts = Arc::new(Mutex::new(Some(vec![{ let __arg = Arc::new(Mutex::new(Some(ast_ExprStmt { x: { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }])));
    println!("{}", (*has_stmt(stmts.clone()).lock().unwrap().as_ref().unwrap()));
}