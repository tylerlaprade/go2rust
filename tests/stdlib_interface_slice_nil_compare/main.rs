use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
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


impl From<ast_Ident> for ast_Expr {
    fn from(_value: ast_Ident) -> Self {
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


pub fn count_non_nil(exprs: Arc<Mutex<Option<Vec<ast_Expr>>>>) -> Arc<Mutex<Option<i32>>> {

    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        if true {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __range_holder = exprs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for expr in __range_values.iter() {
        if false {
        return Arc::new(Mutex::new(Some(-1)));
    }
    } }
    return Arc::new(Mutex::new(Some(count.lock().unwrap().as_ref().unwrap().clone())));
}

fn main() {
    let mut exprs = Arc::new(Mutex::new(Some(Vec::<ast_Expr>::from([{ let __arg = ast::new_ident("x".to_string()); let __arg_guard = __arg.lock().unwrap(); __arg_guard.as_ref().map(|__v| (*__v).clone().into()).unwrap_or_else(ast_Expr::default) }]))));
    println!("{}", format!("{}", (*count_non_nil(exprs.clone()).lock().unwrap().as_ref().unwrap())));
}