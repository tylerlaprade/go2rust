use go2rust_stdlib_stubs::*;

use crate::{format_slice, format_slice_values, format_slice_wrapped};

use crate::r#mod::*;
use crate::read::*;
use crate::zcgo::*;

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AnonymousStruct1 {
    pub vendor: Arc<Mutex<Option<Vec<String>>>>,
    pub goroot: Arc<Mutex<Option<String>>>,
    pub gopath: Arc<Mutex<Option<Vec<String>>>>,
}
impl AnonymousStruct1 {
    pub fn __go_value_clone(&self) -> Self {
        Self { vendor: self.vendor.clone(), goroot: { let __guard = self.goroot.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, gopath: self.gopath.clone() }
    }
}


impl Default for AnonymousStruct1 {
    fn default() -> Self {
        Self { vendor: Arc::new(Mutex::new(None)), goroot: Arc::new(Mutex::new(Some(String::new()))), gopath: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for AnonymousStruct1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {}}}", format_slice(&self.vendor), (*self.goroot.lock().unwrap().as_ref().unwrap()), format_slice(&self.gopath))
    }
}

impl GoJsonDecode for AnonymousStruct1 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// getToolDir returns the default value of ToolDir.
pub fn get_tool_dir() -> Arc<Mutex<Option<String>>> {
    path_filepath::join(Arc::new(Mutex::new(Some(vec![(*runtime::g_o_r_o_o_t().lock().unwrap().as_ref().unwrap()).clone(), "pkg/tool/darwin_arm64".to_string()]))))
}