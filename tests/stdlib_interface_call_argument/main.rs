use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct Walker {
}

impl Walker {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{}}")
    }
}

impl GoJsonDecode for Walker {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl Walker {
    pub fn has(&self, t: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>) -> bool {
        true
    }
}

pub fn has(t: Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>>) -> bool {
    true
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

    let mut w: Arc<Mutex<Option<Walker>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{}", format!("{}", has(Arc::new(Mutex::new(Some(Box::new(go_types::tuple::TuplePtr(go_types::new_tuple(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))));
    println!("{}", format!("{}", (*w.lock().unwrap().as_ref().unwrap()).has(Arc::new(Mutex::new(Some(Box::new(go_types::tuple::TuplePtr(go_types::new_tuple(Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(None))])))).clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))))));
}

impl GoValueClone for Walker {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
