use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct localType {
}

impl localType {
    pub fn __go_value_clone(&self) -> Self {
        Self {  }
    }
}

impl std::fmt::Display for localType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for localType {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl localType {
    pub fn underlying(&self) -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("local".to_string())))
    }
}

impl go_types::r#type::Type for localType {
    fn string(&self) -> Arc<Mutex<Option<String>>> {
        Arc::new(Mutex::new(Some("local".to_string())))
    }
    fn underlying(&mut self) -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
        Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
    }
    fn __go_clone_box_type_(&self) -> Box<dyn go_types::r#type::Type + Send + Sync> {
        Box::new(self.clone()) as Box<dyn go_types::r#type::Type + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_type_(&self, other: &(dyn go_types::r#type::Type + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<localType>() {
            false
        } else {
            false
        }
    }
}

pub fn make_type() -> Arc<Mutex<Option<Box<dyn go_types::r#type::Type + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>)))
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

    let mut typesList = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(Box::new(localType {  }) as Box<dyn go_types::r#type::Type + Send + Sync>))), make_type().clone()])));
    println!("{}", format!("{}", (*typesList.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
}

impl GoValueClone for localType {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
