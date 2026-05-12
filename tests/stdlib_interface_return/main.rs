use std::sync::{Arc, Mutex};

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
        Self::default()
    }
}


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::default()
    }
}


impl From<ast_SelectorExpr> for ast_Expr {
    fn from(_value: ast_SelectorExpr) -> Self {
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

pub fn make_var_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut expr: Arc<Mutex<Option<ast_Expr>>> = { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) };
    return expr.clone();
}

pub fn make_assigned_selector_expr() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut expr: Arc<Mutex<Option<ast_Expr>>> = { let __arg = ast::new_ident("x".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) };
    { let new_val = { let __arg = Arc::new(Mutex::new(Some(ast_SelectorExpr { x: { let __arg = ast::new_ident("pkg".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, sel: ast::new_ident("Name".to_string()).clone(), ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; *expr.lock().unwrap() = Some(new_val); };
    return expr.clone();
}

pub fn make_expr_slice_len() -> Arc<Mutex<Option<i32>>> {

    let mut exprs = Arc::new(Mutex::new(Some(Vec::<ast_Expr>::from([{ let __arg = ast::new_ident("x".to_string()); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }]))));
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __arg = ast::new_ident("y".to_string()); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }); __append_target.clone() }; exprs = new_val; };
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*make_ident_expr().lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; exprs = new_val; };
    { let new_val = { let __append_target = exprs.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __arg = Arc::new(Mutex::new(Some(ast_SelectorExpr { x: { let __arg = ast::new_ident("pkg".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, sel: ast::new_ident("Name".to_string()).clone(), ..Default::default() }))); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }); __append_target.clone() }; exprs = new_val; };
    return Arc::new(Mutex::new(Some((*exprs.lock().unwrap().as_ref().unwrap()).len() as i32)));
}

pub fn asserted_field_element() -> Arc<Mutex<Option<ast_Expr>>> {

    let mut field = Arc::new(Mutex::new(Some(ast_Field { r#type: { let __arg = Arc::new(Mutex::new(Some(ast_ArrayType { elt: { let __arg = ast::new_ident("int".to_string()); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() }))); let __converted = { let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }; Arc::new(Mutex::new(Some(__converted))) }, ..Default::default() })));
    return Arc::new(Mutex::new(Some({ let __selector_holder = (*({
        let val = (*field.lock().unwrap().as_ref().unwrap()).r#type.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            Arc::new(Mutex::new(Some(any_val.downcast_ref::<ast_ArrayType>().expect("type assertion failed").clone())))
        } else {
            panic!("type assertion on nil interface")
        }
    }).lock().unwrap().as_ref().unwrap()).elt.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned })));
}

fn main() {
    if false {
        println!("{}", (*asserted_field_element().lock().unwrap()).is_some());
    }
    println!("{}", (*make_expr().lock().unwrap()).is_some());
    println!("{}", (*make_ident_expr().lock().unwrap()).is_some());
    println!("{}", (*make_unary_expr().lock().unwrap()).is_some());
    println!("{}", (*make_var_expr().lock().unwrap()).is_some());
    println!("{}", (*make_assigned_selector_expr().lock().unwrap()).is_some());
    println!("{}", (*make_expr_slice_len().lock().unwrap().as_ref().unwrap()));
}