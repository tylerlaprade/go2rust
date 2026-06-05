use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::collections::BTreeMap;
use std::error::Error as StdError;
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
    go_parser::__go_init_all();
    go_scanner::__go_init_all();
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
    internal_stringslite::__go_init_all();
    internal_sync::__go_init_all();
    internal_types_errors::__go_init_all();
    math::__go_init_all();
    math_big::__go_init_all();
    math_bits::__go_init_all();
    path_filepath::__go_init_all();
    slices::__go_init_all();
    sort::__go_init_all();
    strings::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("p.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package p\nvar x int\nvar y = x\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(go_parser::PARSE_COMMENTS as u64))))))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "parse".to_string()), format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
        return;
    }
    let mut info = Arc::new(Mutex::new(Some(go_types::api::Info { types: Arc::new(Mutex::new(Some(BTreeMap::<go_types::GoLocalPtrKey<Box<dyn go_ast::r#mod::Expr + Send + Sync>>, Arc<Mutex<Option<go_types::api::TypeAndValue>>>>::from([])))), ..Default::default() })));
    let (mut pkg, __tmp_1) = { let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(Arc::new(Mutex::new(Some("p".to_string()))), fset.clone(), Arc::new(Mutex::new(Some(vec![file.clone()]))), info.clone()); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    println!("{} {} {}", format!("{}", { let __nil_result = (*pkg.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", { let __tmp_x = (({ let __len_target = { let __field = (*info.lock().unwrap().as_ref().unwrap()).types.clone(); __field }; let __len_guard = __len_target.lock().unwrap(); __len_guard.as_ref().map(|__v| __v.len()).unwrap_or(0) }) as i32); let __tmp_y = 0; __tmp_x > __tmp_y }));
    let (mut nilInfoPkg, mut nilInfoErr) = { let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(Arc::new(Mutex::new(Some("p".to_string()))), fset.clone(), Arc::new(Mutex::new(Some(vec![file.clone()]))), Arc::new(Mutex::new(None))); __result };
    println!("{} {}", format!("{}", { let __nil_result = (*nilInfoPkg.lock().unwrap()).is_some(); __nil_result }), format!("{}", { let __nil_result = (*nilInfoErr.lock().unwrap()).is_none(); __nil_result }));
}