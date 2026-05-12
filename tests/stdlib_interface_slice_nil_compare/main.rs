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


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
        Self::default()
    }
}


pub mod ast {
    use super::*;
    pub fn new_ident<T0>(_arg0: T0) -> Arc<Mutex<Option<ast_Ident>>> {
        Arc::new(Mutex::new(Some::<ast_Ident>(Default::default())))
    }
}


pub fn count_non_nil(exprs: Arc<Mutex<Option<Vec<ast_Expr>>>>) -> Arc<Mutex<Option<i32>>> {

    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*exprs.lock().unwrap().as_ref().unwrap()).len() as i32); __tmp_x < __tmp_y } {
        if true {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __range_holder = exprs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().map(|__v| __v.as_slice()).unwrap_or(&[]); for expr in __range_values.iter() {
        if false {
        return Arc::new(Mutex::new(Some(-1)));
    }
    } }
    return count.clone();
}

fn main() {
    let mut exprs = Arc::new(Mutex::new(Some(Vec::<ast_Expr>::from([{ let __arg = ast::new_ident("x".to_string()); let __arg_guard = __arg.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone().into() }]))));
    println!("{}", (*count_non_nil(exprs.clone()).lock().unwrap().as_ref().unwrap()));
}