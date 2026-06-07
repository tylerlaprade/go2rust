use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct holder {
    pub w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { w: self.w.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.w.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for holder {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


fn main() {
    ::bytes::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut h = Arc::new(Mutex::new(Some(holder { w: Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(bytes::new_buffer(Arc::new(Mutex::new(None))).clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))), ..Default::default() })));
    let mut err = Arc::new(Mutex::new(Some(Box::<dyn StdError + Send + Sync>::from(format!("{}", format!("{}", (*(*h.lock().unwrap().as_ref().unwrap()).w.lock().unwrap().as_ref().unwrap())))))));
    println!("{}", format!("{}", { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result }));
}

impl GoValueClone for holder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
