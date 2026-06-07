use go2rust_stdlib_stubs::*;
use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};


pub struct GoLocalPtrKey<T>(pub Arc<Mutex<Option<T>>>);

impl<T> Clone for GoLocalPtrKey<T> {
    fn clone(&self) -> Self { GoLocalPtrKey(self.0.clone()) }
}

impl<T> GoLocalPtrKey<T> {
    pub fn new(value: Arc<Mutex<Option<T>>>) -> Self { GoLocalPtrKey(value) }
    pub fn value(&self) -> Arc<Mutex<Option<T>>> { self.0.clone() }
    fn addr(&self) -> usize { Arc::as_ptr(&self.0) as usize }
}

impl<T> PartialEq for GoLocalPtrKey<T> {
    fn eq(&self, other: &Self) -> bool { self.addr() == other.addr() }
}
impl<T> Eq for GoLocalPtrKey<T> {}
impl<T> PartialOrd for GoLocalPtrKey<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> { Some(self.cmp(other)) }
}
impl<T> Ord for GoLocalPtrKey<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering { self.addr().cmp(&other.addr()) }
}
impl<T> std::fmt::Debug for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}
impl<T> std::fmt::Display for GoLocalPtrKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "0x{:x}", self.addr()) }
}

pub fn version(info: Arc<Mutex<Option<go_types::api::Info>>>, file: Arc<Mutex<Option<go_ast::r#mod::File>>>) -> Arc<Mutex<Option<String>>> {
    {
        let mut v = Arc::new(Mutex::new(Some({ let __map = { let __map_holder = (*info.lock().unwrap().as_ref().unwrap()).file_versions.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&go_types::GoLocalPtrKey::new(file.clone()))).map(|__v| __v.lock().unwrap().as_ref().unwrap().clone()).unwrap_or_else(|| String::new()) })));;
        if { let __tmp_x = (*v.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "".to_string(); __tmp_x != __tmp_y } {
            return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };;
        }
    }
    Arc::new(Mutex::new(Some("".to_string())))
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

    let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File { ..Default::default() })));
    let mut info = Arc::new(Mutex::new(Some(go_types::api::Info { file_versions: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<go_ast::r#mod::File>, Arc<Mutex<Option<String>>>>::from([(go_types::GoLocalPtrKey::new(file.clone()), Arc::new(Mutex::new(Some("go1.22".to_string()))))])))), ..Default::default() })));
    let _ = Arc::new(Mutex::new(Some(go_types::api::Info { instances: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<go_ast::r#mod::Ident>, Arc<Mutex<Option<go_types::api::Instance>>>>::from([])))), implicits: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<Box<dyn go_ast::r#mod::Node + Send + Sync>>, Arc<Mutex<Option<Box<dyn go_types::object::Object + Send + Sync>>>>>::from([])))), scopes: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<Box<dyn go_ast::r#mod::Node + Send + Sync>>, Arc<Mutex<Option<go_types::scope::Scope>>>>::from([])))), ..Default::default() })));
    let _ = Arc::new(Mutex::new(Some(go_ast::r#mod::ChanType { dir: Arc::new(Mutex::new(Some(go_ast::r#mod::ChanDir(Arc::new(Mutex::new(Some(go_ast::S_E_N_D as i32))))))), ..Default::default() })));
    println!("{}", format!("{}", (*version(info.clone(), file.clone()).lock().unwrap().as_ref().unwrap())));
}