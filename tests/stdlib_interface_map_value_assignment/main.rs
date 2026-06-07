use go2rust_stdlib_stubs::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

pub fn remember(values: Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>>>>>) {
    let mut tn = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    let mut tp = go_types::new_type_param(tn.clone(), Arc::new(Mutex::new(None)));
    { let __map_key = "T".to_string(); let __map_value = Arc::new(Mutex::new(Some(Box::new(go_types::typeparam::TypeParamPtr(tp.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))); (*values.lock().unwrap().as_mut().unwrap()).insert(__map_key, __map_value); };
}

pub fn literal() -> Arc<Mutex<Option<BTreeMap<String, Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>>>>> {
    let mut tn = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("U".to_string()))), Arc::new(Mutex::new(None)));
    let mut tp = go_types::new_type_param(tn.clone(), Arc::new(Mutex::new(None)));
    return Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>>::from([("U".to_string(), Arc::new(Mutex::new(Some(Box::new(go_types::typeparam::TypeParamPtr(tp.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))]))));
}

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_buildcfg::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_godebug::__go_init_all();
    internal_goexperiment::__go_init_all();
    internal_gover::__go_init_all();
    internal_goversion::__go_init_all();
    internal_stringslite::__go_init_all();
    internal_types_errors::__go_init_all();
    math::__go_init_all();
    math_big::__go_init_all();
    math_bits::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    strconv::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut values = Arc::new(Mutex::new(Some(BTreeMap::<String, Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>>::new())));
    remember(values.clone());
    println!("{} {}", format!("{}", (*values.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)), format!("{}", (*literal().lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}