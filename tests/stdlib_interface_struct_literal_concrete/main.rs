use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct entry {
    pub typ: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>,
    pub name: Arc<Mutex<Option<String>>>,
}

impl entry {
    pub fn __go_value_clone(&self) -> Self {
        Self { typ: self.typ.clone(), name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for entry {
    fn default() -> Self {
        Self { typ: Arc::new(Mutex::new(None)), name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.typ.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for entry {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub fn make_entry() -> Arc<Mutex<Option<entry>>> {
    let mut tn = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i32))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    let mut tp = go_types::new_type_param(tn.clone(), Arc::new(Mutex::new(None)));
    return Arc::new(Mutex::new(Some(entry { typ: Arc::new(Mutex::new(Some(Box::new(go_types::typeparam::TypeParamPtr(tp.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), name: Arc::new(Mutex::new(Some("ok".to_string()))), ..Default::default() })));
}

fn main() {
    cmp::__go_init_all();
    go_ast::__go_init_all();
    go_constant::__go_init_all();
    go_token::__go_init_all();
    go_types::__go_init_all();
    go_version::__go_init_all();
    internal_buildcfg::__go_init_all();
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
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    if false {
        let _ = make_entry();
    }
    println!("{}", format!("{}", (*(*make_entry().lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap()).clone()));
}

impl GoValueClone for entry {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
