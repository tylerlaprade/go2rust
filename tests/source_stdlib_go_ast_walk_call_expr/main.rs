use go2rust_stdlib_stubs::*;
use std::sync::{Arc, Mutex};

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut call = Arc::new(Mutex::new(Some(go_ast::r#mod::CallExpr { fun: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("f".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), args: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>)))]))), ..Default::default() })));
    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut names_closure_clone = names.clone(); go_ast::inspect(Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::CallExprPtr(call.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(move |node: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>| -> bool {
        {
        let (mut ident, mut ok) = ({
        let val = node.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn go_ast::r#mod::Node + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<go_ast::r#mod::IdentPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<go_ast::r#mod::Ident>)), false)
        }
    });;
        if ok {
            { let __append_target = names_closure_clone.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push({ let __selector_holder = (*ident.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }); __append_target.clone() };;
        }
    }
        true
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>) -> bool + Send + Sync>))));
    println!("{} {} {}", format!("{}", (*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", { let __seq = { let __seq_holder = names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }), format!("{}", { let __seq = { let __seq_holder = names.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }));
    println!("{}", format!("{}", (*(*go_ast::new_ident(Arc::new(Mutex::new(Some("z".to_string())))).lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()));
}