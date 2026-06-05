use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn first_func(args: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<go_ast::r#mod::FuncLit>>> {
    ({
        let val = { let __seq = { let __seq_holder = args.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::FuncLitPtr>() {
                typed_val.0.clone()
            } else {
                panic!("type assertion failed")
            }
        } else {
            panic!("type assertion on nil interface")
        }
    })
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    if false {
        let mut done = GoChannel::<bool>::new_buffered(1 as usize);
        let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send((*first_func(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::FuncLitPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::FuncLit { ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])))).lock().unwrap()).is_some());;;
    });
        println!("{}", format!("{}", done.recv().unwrap_or_default()));
    }
    println!("{}", format!("{}", "ok".to_string()));
}