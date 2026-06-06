use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

pub fn count_non_nil(exprs: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> i32 {
    let mut count = Arc::new(Mutex::new(Some(0)));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while { let __tmp_x = ({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); let __tmp_y = ((*exprs.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x < __tmp_y } {
        if { let __nil_result = (*{ let __seq = { let __seq_holder = exprs.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[({ let __v = (*i.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize].clone() }.lock().unwrap()).is_some(); __nil_result } {
        { let mut guard = count.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    { let __range_holder = exprs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for expr in __range_values.iter() {
        if { let __nil_result = (*expr.lock().unwrap()).is_none(); __nil_result } {
        return -(1);
    }
    } }
    return { let __v = (*count.lock().unwrap().as_ref().unwrap()).clone(); __v };
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();

    let mut exprs = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])));
    println!("{}", format!("{}", count_non_nil(exprs.clone())));
}