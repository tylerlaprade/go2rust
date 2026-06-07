use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct holder {
    pub out: Arc<Mutex<Option<io_Writer>>>,
    pub err: Arc<Mutex<Option<io_Writer>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        Self { out: self.out.clone(), err: self.err.clone() }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.out.lock().unwrap().as_ref().unwrap()), (*self.err.lock().unwrap().as_ref().unwrap()))
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
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut h: Arc<Mutex<Option<holder>>> = Arc::new(Mutex::new(Some(Default::default())));
    {
        let (_, mut ok) = ({
        let val = (*h.lock().unwrap().as_ref().unwrap()).out.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<os_File>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<os_File>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<os_File>)), false)
        }
    });;
        if ok {
            println!("{}", format!("{}", "file".to_string()));;
        } else {
            println!("{}", format!("{}", "not file".to_string()));;
        }
    }
    {
        let (mut buf, _) = ({
        let val = (*h.lock().unwrap().as_ref().unwrap()).err.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<Arc<Mutex<Option<bytes::buffer::Buffer>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<bytes::buffer::Buffer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<bytes::buffer::Buffer>)), false)
        }
    });;
        if { let __nil_result = (*buf.lock().unwrap()).is_some(); __nil_result } {
            println!("{}", format!("{}", "buffer".to_string()));;
        } else {
            println!("{}", format!("{}", "not buffer".to_string()));;
        }
    }
}

impl GoValueClone for holder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
