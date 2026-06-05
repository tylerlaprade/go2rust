use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub fn decl_kinds(decls: Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>>>>>) -> Arc<Mutex<Option<String>>> {
    { let __range_holder = decls.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for d in __range_values.iter() {
        return decl_kind(d.clone());
    } }
    Arc::new(Mutex::new(Some("none".to_string())))
}

pub fn decl_kind(mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
    let mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>> = Arc::new(Mutex::new(d.lock().unwrap().as_ref().map(|__v| go_ast::r#mod::Decl::__go_clone_box_decl(__v.as_ref()))));
    {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Decl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::GenDeclPtr>()).unwrap().0.clone();
        let _ = (*d.lock().unwrap().as_ref().unwrap());;
        return Arc::new(Mutex::new(Some("gen".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).is_some() {
        let d = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).unwrap().0.clone();
        let _ = (*d.lock().unwrap().as_ref().unwrap());;
        return Arc::new(Mutex::new(Some("func".to_string())));;
    } else {
        let d = _ts_subject.clone();
        let _ = { let __v = (*d.lock().unwrap().as_ref().unwrap()).clone(); __v };;
        return Arc::new(Mutex::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}