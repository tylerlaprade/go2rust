use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn singleton_from_range(elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
    { let __range_holder = elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for elt in __range_values.iter() {
        return Arc::new(Mutex::new(Some(vec![elt.clone()])));
    } }
    return Arc::new(Mutex::new(None));
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    let mut done = GoChannel::<bool>::new_buffered(1 as usize);
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send((*singleton_from_range(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])))).lock().unwrap()).is_some());;;
    });
    println!("{}", format!("{}", done.recv().unwrap_or_default()));
}