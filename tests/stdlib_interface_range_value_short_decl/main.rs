use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn normalize(elts: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> {
    let mut values: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Expr + Send + Sync>>>>>>>> = Arc::new(Mutex::new(None));
    { let __range_holder = elts.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for elt in __range_values.iter() {
        let mut value = (*elt).clone();
        {
        let (mut kv, mut ok) = ({
        let val = elt.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Expr + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::KeyValueExprPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::KeyValueExpr>)), false)
        }
    });;
        if ok {
            { let __iface_handle = (*kv.lock().unwrap().as_ref().unwrap()).value.clone(); let __iface_guard = __iface_handle.lock().unwrap(); *value.lock().unwrap() = (*__iface_guard).clone(); };;
        }
    }
        { let new_val = { let __append_target = values.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(value.clone()); __append_target.clone() }; values = new_val; };
    } }
    return values.clone();
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    if false {
        let mut done = GoChannel::<bool>::new_buffered(1 as usize);
        let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send((*normalize(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::Ident { ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))])))).lock().unwrap()).is_some());;;
    });
        println!("{}", format!("{}", done.recv().unwrap_or_default()));
    }
    println!("{}", format!("{}", "ok".to_string()));
}