use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Default)]
pub struct ast_ArrayType {
    pub elt: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_ArrayType>")
    }
}


impl ast_ArrayType {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct ast_Expr {
    pub __go_id: usize,
    pub __go_pos: i32,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl ast_Expr {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: 0, __go_value: Arc::new(value) }
    }
    pub fn __go_from_with_pos<T: 'static + Send + Sync>(value: T, pos: i32) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_pos: pos, __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for ast_Expr {
    fn default() -> Self {
        Self { __go_id: 0, __go_pos: 0, __go_value: Arc::new(()) }
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
pub struct ast_Field {
    pub r#type: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_Field>")
    }
}


impl ast_Field {
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


#[derive(Debug, Clone, Default)]
pub struct ast_SelectorExpr {
    pub sel: Arc<Mutex<Option<ast_Ident>>>,
    pub x: Arc<Mutex<Option<ast_Expr>>>,
}

impl std::fmt::Display for ast_SelectorExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<ast_SelectorExpr>")
    }
}


impl ast_SelectorExpr {
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


impl From<ast_ArrayType> for ast_Expr {
    fn from(_value: ast_ArrayType) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_SelectorExpr> for ast_Expr {
    fn from(_value: ast_SelectorExpr) -> Self {
        Self::__go_from(_value)
    }
}


impl From<ast_UnaryExpr> for ast_Expr {
    fn from(_value: ast_UnaryExpr) -> Self {
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


pub fn make_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = Arc::new(Mutex::new(Some(ast_Ident { ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
}

pub fn make_ident_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
}

pub fn make_unary_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    return { let __arg = Arc::new(Mutex::new(Some(ast_UnaryExpr { x: { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
}

pub fn make_var_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut expr: Arc<Mutex<Option<ast_Expr>>> = { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
    return expr.clone();
}

pub fn make_assigned_selector_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut expr: Arc<Mutex<Option<ast_Expr>>> = { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) };
    { let new_val = { let __arg = Arc::new(Mutex::new(Some(ast_SelectorExpr { x: { let __arg = ast::new_ident("pkg".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, sel: ast::new_ident("Name".to_string()).clone(), ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(ast_Expr::default) }; *expr.lock().unwrap() = Some(new_val); };
    return expr.clone();
}

pub fn make_expr_slice_len() -> i32 {

    let mut exprs = Arc::new(Mutex::new(Some(Vec::<ast_Expr>::from([{ let __arg = ast::new_ident("x".to_string()); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(ast_Expr::default) }]))));
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __arg = ast::new_ident("y".to_string()); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(ast_Expr::default) }); __append_target.clone() }; exprs = new_val; };
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*make_ident_expr().lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; exprs = new_val; };
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __arg = Arc::new(Mutex::new(Some(ast_SelectorExpr { x: { let __arg = ast::new_ident("pkg".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, sel: ast::new_ident("Name".to_string()).clone(), ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(ast_Expr::default) }); __append_target.clone() }; exprs = new_val; };
    return (*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32;
}

pub fn asserted_field_element() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut field = Arc::new(Mutex::new(Some(ast_Field { r#type: { let __arg = Arc::new(Mutex::new(Some(ast_ArrayType { elt: { let __arg = ast::new_ident("int".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); let __converted: Option<ast_Expr> = __arg_guard.as_ref().map(|__v| (*__v).clone().into()); __converted }; Arc::new(Mutex::new(__converted)) }, ..Default::default() })));
    return (*({
        let val = (*field.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            Arc::new(Mutex::new(Some(any_val.downcast_ref::<ast_ArrayType>().expect("type assertion failed").clone())))
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).elt.clone();
}

fn main() {
    if false {
        println!("{}", format!("{}", (*asserted_field_element().lock().unwrap()).is_some()));
    }
    println!("{}", format!("{}", (*make_expr().lock().unwrap()).is_some()));
    println!("{}", format!("{}", (*make_ident_expr().lock().unwrap()).is_some()));
    println!("{}", format!("{}", (*make_unary_expr().lock().unwrap()).is_some()));
    println!("{}", format!("{}", (*make_var_expr().lock().unwrap()).is_some()));
    println!("{}", format!("{}", (*make_assigned_selector_expr().lock().unwrap()).is_some()));
    println!("{}", format!("{}", make_expr_slice_len()));
}