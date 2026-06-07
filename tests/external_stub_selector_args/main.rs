use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Default)]
pub struct checkerInputs {
    pub fset: Arc<Mutex<Option<go_token::position::FileSet>>>,
    pub pkg: Arc<Mutex<Option<go_types::package::Package>>>,
    pub info: Arc<Mutex<Option<go_types::api::Info>>>,
}

impl checkerInputs {
    pub fn __go_value_clone(&self) -> Self {
        Self { fset: self.fset.clone(), pkg: self.pkg.clone(), info: self.info.clone() }
    }
}

impl std::fmt::Display for checkerInputs {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", { let __guard = self.fset.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.pkg.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } }, { let __guard = self.info.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for checkerInputs {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


fn main() {
    cmp::__go_init_all();
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
    internal_goarch::__go_init_all();
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
    strconv::__go_init_all();
    sync::__go_init_all();
    sync_atomic::__go_init_all();
    unicode::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut done = GoChannel::<bool>::new();
    let done_thread = done.clone(); std::thread::spawn(move || {
        done_thread.send(true);;;
    });
    done.recv().unwrap_or_default();
    let mut inputs = Arc::new(Mutex::new(Some(checkerInputs { fset: go_token::new_file_set().clone(), pkg: go_types::new_package(Arc::new(Mutex::new(Some("example.com/p".to_string()))), Arc::new(Mutex::new(Some("p".to_string())))).clone(), info: Arc::new(Mutex::new(Some(go_types::api::Info { ..Default::default() }))).clone(), ..Default::default() })));
    let mut checker = go_types::new_checker(Arc::new(Mutex::new(None)), { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).fset.clone(); __field }, { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).pkg.clone(); __field }, { let __field = (*inputs.lock().unwrap().as_ref().unwrap()).info.clone(); __field });
    println!("{}", format!("{}", { let __nil_result = (*checker.lock().unwrap()).is_some(); __nil_result }));
}

impl GoValueClone for checkerInputs {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
