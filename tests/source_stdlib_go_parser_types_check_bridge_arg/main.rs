use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub trait localDecl: std::fmt::Display + Any {
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool;
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>>;
}

impl Clone for Box<dyn localDecl + Send + Sync> {
    fn clone(&self) -> Self {
        localDecl::__go_clone_box_local_decl(self.as_ref())
    }
}

#[derive(Clone, Default)]
pub struct localVarDecl {
    pub spec: Arc<Mutex<Option<go_ast::r#mod::ValueSpec>>>,
}

impl localVarDecl {
    pub fn __go_value_clone(&self) -> Self {
        Self { spec: self.spec.clone() }
    }
}

impl std::fmt::Display for localVarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.spec.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for localVarDecl {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl localVarDecl {
    pub fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(go_ast::r#mod::ValueSpecPtr(self.spec.clone())) as Box<dyn go_ast::r#mod::Node + Send + Sync>)))
    }
}

impl localDecl for localVarDecl {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        localVarDecl::node(self)
    }
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn localDecl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localVarDecl>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct localVarDeclPtr(pub Arc<Mutex<Option<localVarDecl>>>);

impl std::fmt::Display for localVarDeclPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl localDecl for localVarDeclPtr {
    fn node(&self) -> Arc<Mutex<Option<Box<dyn go_ast::r#mod::Node + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        localVarDecl::node(__recv)
    }
    fn __go_clone_box_local_decl(&self) -> Box<dyn localDecl + Send + Sync> {
        Box::new(self.clone()) as Box<dyn localDecl + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_local_decl(&self, other: &(dyn localDecl + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localVarDeclPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn local_walk_decl(mut d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync>>>>) {
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
        { let __range_holder = (*d.lock().unwrap().as_ref().unwrap()).specs.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for mut s in __range_values.iter().cloned() {
        {
    let _ts_subject = s.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn go_ast::r#mod::Spec + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).is_some() {
        let s = _ts_val.and_then(|__v| __v.downcast_ref::<go_ast::r#mod::ValueSpecPtr>()).unwrap().0.clone();
        if { let __tmp_x = { let __selector_holder = (*d.lock().unwrap().as_ref().unwrap()).tok.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; let __tmp_y = go_token::r#mod::Token(Arc::new(Mutex::new(Some(go_token::V_A_R as i32)))); __tmp_x == __tmp_y } {
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync> = { let mut __f_guard = f.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(Box::new(localVarDecl { spec: s.clone(), ..Default::default() }) as Box<dyn localDecl + Send + Sync>)))) };
    };
    }
    }
    } };
    }
    }
}

pub fn local_decl_stmt(d: Arc<Mutex<Option<Box<dyn go_ast::r#mod::Decl + Send + Sync>>>>) {
    local_walk_decl(d.clone(), Arc::new(Mutex::new(Some(Box::new(move |d: Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>| {
        {
    let _ts_subject = d.clone();
    let _ts_guard = _ts_subject.lock().unwrap();
    let _ts_is_nil = _ts_guard.as_ref().is_none();
    let _ts_owned = _ts_guard.as_ref().cloned();
    drop(_ts_guard);
    let _ts_val: Option<&dyn Any> = _ts_owned.as_ref().map(|__v| {
        let __any = __v.as_ref().__go_as_any();
        if let Some(__boxed) = __any.downcast_ref::<Box<dyn localDecl + Send + Sync>>() {
            __boxed.as_ref().__go_as_any()
        } else {
            __any
        }
    });
    if _ts_val.and_then(|__v| __v.downcast_ref::<localVarDecl>()).is_some() {
        println!("{}", format!("{}", "local var".to_string()));;
    } else {
        println!("{}", format!("{}", "local unknown".to_string()));;
    }
    }
    }) as Box<dyn FnMut(Arc<Mutex<Option<Box<dyn localDecl + Send + Sync>>>>) -> () + Send + Sync>))));
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
    strconv::__go_init_all();
    strings::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut fset = go_token::new_file_set();
    let (mut file, mut err) = go_parser::parse_file(fset.clone(), Arc::new(Mutex::new(Some("input.go".to_string()))), Arc::new(Mutex::new(Some(Box::new("package main\nvar x int\n".to_string()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(go_parser::interface::Mode(Arc::new(Mutex::new(Some(0 as u64))))))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", "parse failed".to_string()));
        return;
    }
    {
    let _ts_subject = { let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone();
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
        println!("{}", format!("{}", "gen".to_string()));;
    } else {
        println!("{}", format!("{}", "other".to_string()));;
    }
    }
    local_decl_stmt({ let __seq = { let __seq_holder = (*file.lock().unwrap().as_ref().unwrap()).decls.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.clone());
    let (mut pkg, __tmp_1) = { let __recv = Arc::new(Mutex::new(Some(go_types::api::Config::default()))); let __result = (*__recv.lock().unwrap().as_ref().unwrap()).check(Arc::new(Mutex::new(Some("main".to_string()))), fset.clone(), Arc::new(Mutex::new(Some(vec![file.clone()]))), Arc::new(Mutex::new(None))); __result }; let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1;;
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{}", format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
    }
    println!("{} {}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result }), format!("{}", (*{ let __recv = pkg.clone(); let __recv_ptr: *const go_types::package::Package = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const go_types::package::Package }; let __result = unsafe { &*__recv_ptr }.name(); __result }.lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for localVarDecl {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
