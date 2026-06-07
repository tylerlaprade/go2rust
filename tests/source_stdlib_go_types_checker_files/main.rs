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

fn main() {
    cmp::__go_init_all();
    container_heap::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_abi::__go_init_all();
    internal_buildcfg::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    internal_filepathlite::__go_init_all();
    internal_godebug::__go_init_all();
    internal_godebugs::__go_init_all();
    internal_goexperiment::__go_init_all();
    internal_gover::__go_init_all();
    internal_goversion::__go_init_all();
    internal_race::__go_init_all();
    internal_stringslite::__go_init_all();
    internal_sync::__go_init_all();
    internal_types_errors::__go_init_all();
    math::__go_init_all();
    math_big::__go_init_all();
    math_bits::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    sort::__go_init_all();
    strconv::__go_init_all();
    strings::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut fset = go_token::new_file_set();
    let mut name = go_ast::new_ident(Arc::new(Mutex::new(Some("x".to_string()))));
    let mut file = Arc::new(Mutex::new(Some(go_ast::r#mod::File { name: go_ast::new_ident(Arc::new(Mutex::new(Some("main".to_string())))).clone(), decls: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::GenDeclPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::GenDecl { tok: Arc::new(Mutex::new(Some(go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32))))))), specs: Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(Arc::new(Mutex::new(Some(go_ast::r#mod::ValueSpec { names: Arc::new(Mutex::new(Some(vec![name.clone()]))), r#type: Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::IdentPtr(go_ast::new_ident(Arc::new(Mutex::new(Some("int".to_string())))).clone())) as Box<dyn go_ast::r#mod::Expr + Send + Sync>))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Spec + Send + Sync>)))]))), ..Default::default() }))).clone())) as Box<dyn go_ast::r#mod::Decl + Send + Sync>)))]))), ..Default::default() })));
    let mut info = Arc::new(Mutex::new(Some(go_types::api::Info { defs: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<go_ast::r#mod::Ident>, Arc<Mutex<Option<Box<dyn go_types::object::Object + Send + Sync>>>>>::from([])))), ..Default::default() })));
    let mut pkg = go_types::new_package(Arc::new(Mutex::new(Some("example.com/main".to_string()))), Arc::new(Mutex::new(Some("main".to_string()))));
    let mut checker = go_types::new_checker(Arc::new(Mutex::new(None)), fset.clone(), pkg.clone(), info.clone());
    let mut err = { let __recv = checker.clone(); let __recv_ptr: *mut go_types::check::Checker = { let mut __recv_guard = __recv.lock().unwrap(); __recv_guard.as_mut().unwrap() as *mut go_types::check::Checker }; let __result = unsafe { &mut *__recv_ptr }.files(Arc::new(Mutex::new(Some(vec![file.clone()])))); __result };
    println!("{} {} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *const go_types::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::package::Package }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())), format!("{}", { let __nil_result = (*{ let __map = { let __map_holder = (*info.lock().unwrap().as_ref().unwrap()).defs.clone(); let __map_guard = __map_holder.lock().unwrap(); let __cloned = __map_guard.as_ref().cloned(); drop(__map_guard); __cloned }; __map.as_ref().and_then(|__map| __map.get(&go_types::GoLocalPtrKey::new(name.clone()))).map(|__v| __v.clone()).unwrap_or_else(|| Default::default()) }.lock().unwrap()).is_some(); __nil_result }));
}