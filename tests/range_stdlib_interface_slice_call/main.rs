use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct ast_Expr {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl std::fmt::Display for ast_Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Expr>")
    }
}

impl PartialEq for ast_Expr {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Expr {}

impl PartialOrd for ast_Expr {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Expr {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
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


#[derive(Debug, Clone, Default)]
pub struct ast_Ident {
    pub name: Arc<Mutex<Option<String>>>,
}

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


#[derive(Clone)]
pub struct ast_Stmt {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Stmt {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Stmt {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl std::fmt::Display for ast_Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Stmt>")
    }
}

impl PartialEq for ast_Stmt {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for ast_Stmt {}

impl PartialOrd for ast_Stmt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ast_Stmt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_ExprStmt> for ast_Stmt {
    fn from(_value: ast_ExprStmt) -> Self {
        Self::__go_from(_value)
    }
}


pub mod ast {
    use super::*;

    pub trait GoStringArg {
        fn into_go_string(self) -> String;
    }

    impl GoStringArg for String {
        fn into_go_string(self) -> String {
            self
        }
    }

    impl<'a> GoStringArg for &'a str {
        fn into_go_string(self) -> String {
            self.to_string()
        }
    }

    impl<'a> GoStringArg for &'a String {
        fn into_go_string(self) -> String {
            self.clone()
        }
    }

    impl GoStringArg for Arc<Mutex<Option<String>>> {
        fn into_go_string(self) -> String {
            self.lock().unwrap().as_ref().cloned().unwrap_or_default()
        }
    }

    pub fn new_ident<T0: GoStringArg>(_arg0: T0) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(ast_Ident { name: Arc::new(Mutex::new(Some::<String>(_arg0.into_go_string()))), ..Default::default() })))
    }
}


pub fn has_stmt(stmts: Arc<Mutex<Option<Vec<ast_Stmt>>>>) -> Arc<Mutex<Option<bool>>> {

    let mut prev: Arc<Mutex<Option<ast_Stmt>>> = Arc::new(Mutex::new(None));
    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
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

pub fn stmt_kind(stmts: Arc<Mutex<Option<Vec<ast_Stmt>>>>) -> Arc<Mutex<Option<String>>> {

    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        {
    let _ts_is_nil = false;
    let _ts_val: Option<&ast_Stmt> = Some(stmt);
    if _ts_val.and_then(|__v| __v.downcast_ref::<ast_ExprStmt>()).is_some() {
        let s = Arc::new(Mutex::new(Some(_ts_val.and_then(|__v| __v.downcast_ref::<ast_ExprStmt>()).unwrap().clone())));
        let _ = (*{ let __field = (*s.lock().unwrap().as_ref().unwrap()).x.clone(); __field }.lock().unwrap().as_ref().unwrap());;
        return Arc::new(Mutex::new(Some("expr".to_string())));;
    } else {
        let s = Arc::new(Mutex::new(Some((*_ts_val.unwrap()).clone())));
        return Arc::new(Mutex::new(Some("other".to_string())));;
    }
    }
    unreachable!()
    } }
    return Arc::new(Mutex::new(Some("none".to_string())));
}

pub fn assert_expr_stmt(stmts: Arc<Mutex<Option<Vec<ast_Stmt>>>>) -> Arc<Mutex<Option<bool>>> {

    { let __range_holder = stmts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for stmt in __range_values.iter() {
        let mut expr = ({
        let val = stmt;
        Arc::new(Mutex::new(Some(val.downcast_ref::<ast_ExprStmt>().expect("type assertion failed").clone())))
    }).clone();
        return Arc::new(Mutex::new(Some((*expr.lock().unwrap()).is_some())));
    } }
    return Arc::new(Mutex::new(Some(false)));
}

fn main() {
    let mut stmts = Arc::new(Mutex::new(Some(Vec::<ast_Stmt>::from([{ let __arg = Arc::new(Mutex::new(Some(ast_ExprStmt { x: { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }]))));
    if false {
        println!("{}", format!("{}", (*stmt_kind(stmts.clone()).lock().unwrap().as_ref().unwrap())));
        println!("{}", format!("{}", (*assert_expr_stmt(stmts.clone()).lock().unwrap().as_ref().unwrap())));
    }
    println!("{}", format!("{}", (*has_stmt(stmts.clone()).lock().unwrap().as_ref().unwrap())));
}