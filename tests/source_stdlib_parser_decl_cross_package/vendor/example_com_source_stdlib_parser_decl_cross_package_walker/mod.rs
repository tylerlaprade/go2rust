use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

pub fn decl_kind(d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>) -> Arc<Mutex<Option<String>>> {
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
        return Arc::new(Mutex::new(Some("gen".to_string())));;
    } else if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::FuncDeclPtr>()).is_some() {
        return Arc::new(Mutex::new(Some("func".to_string())));;
    } else {
        return Arc::new(Mutex::new(Some("other".to_string())));;
    }
    }
    unreachable!()
}