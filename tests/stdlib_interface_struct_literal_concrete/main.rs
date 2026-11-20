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
        let __go_clone_0_0 = self.typ.clone();
        let __go_clone_1_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            typ: __go_clone_0_0,
            name: __go_clone_1_0,
        }
    }
}


impl Default for entry {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            typ: __go_default_0_0,
            name: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for entry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.typ.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


pub fn make_entry() -> Arc<Mutex<Option<entry>>> {
    let mut tn = go_types::new_type_name(Arc::new(Mutex::new(Some(go_token::position::Pos(Arc::new(Mutex::new(Some(go_token::NO_POS as i64))))))), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some("T".to_string()))), Arc::new(Mutex::new(None)));
    let mut tp = go_types::new_type_param(tn.clone(), Arc::new(Mutex::new(None)));
    return Arc::new(Mutex::new(Some(entry { typ: Arc::new(Mutex::new(Some(Box::new(go_types::typeparam::TypeParamPtr(tp.clone())) as Box<dyn go_types::r#type::Type + Send + Sync>))), name: Arc::new(Mutex::new(Some("ok".to_string()))), ..Default::default() })));
}

fn main() {
    ::bytes::__go_init_all();
    ::cmp::__go_init_all();
    ::container_heap::__go_init_all();
    ::go_ast::__go_init_all();
    ::go_constant::__go_init_all();
    ::go_token::__go_init_all();
    ::go_types::__go_init_all();
    ::go_version::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_buildcfg::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_filepathlite::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_goexperiment::__go_init_all();
    ::internal_gover::__go_init_all();
    ::internal_goversion::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::internal_types_errors::__go_init_all();
    ::math::__go_init_all();
    ::math_big::__go_init_all();
    ::math_bits::__go_init_all();
    ::path_filepath::__go_init_all();
    ::slices::__go_init_all();
    ::sort::__go_init_all();
    ::strconv::__go_init_all();
    ::strings::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

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
