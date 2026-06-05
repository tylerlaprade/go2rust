use go2rust_stdlib_stubs::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub fn version(info: Arc<Mutex<Option<types_Info>>>, file: Arc<Mutex<Option<go_ast::r#mod::File>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut v = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*info.lock().unwrap().as_ref().unwrap()).file_versions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&GoAnyPtrKey::new(file.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })));;
        if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    Arc::new(Mutex::new(Some("".to_string())))
}

fn main() {
    go_ast::__go_init_all();
    go_token::__go_init_all();
    strings::__go_init_all();

    let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File { ..Default::default() })));
    let mut info = Arc::new(Mutex::new(Some(types_Info { file_versions: Arc::new(Mutex::new(Some(BTreeMap::<GoAnyPtrKey, Arc<Mutex<Option<String>>>>::from([(GoAnyPtrKey::new(file.clone()), Arc::new(Mutex::new(Some("go1.22".to_string()))))])))), ..Default::default() })));
    let _ = Arc::new(Mutex::new(Some(types_Info { instances: Arc::new(Mutex::new(Some(BTreeMap::<GoAnyPtrKey, Arc<Mutex<Option<types_Instance>>>>::from([])))), implicits: Arc::new(Mutex::new(Some(BTreeMap::<GoAnyPtrKey, Arc<Mutex<Option<types_Object>>>>::from([])))), scopes: Arc::new(Mutex::new(Some(BTreeMap::<GoAnyPtrKey, Arc<Mutex<Option<types_Scope>>>>::from([])))), ..Default::default() })));
    let _ = Arc::new(Mutex::new(Some(go_ast::r#mod::ChanType { dir: Arc::new(Mutex::new(Some(go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32))))))), ..Default::default() })));
    println!("{}", format!("{}", (*version(info.clone(), file.clone()).lock().unwrap().as_ref().unwrap())));
}