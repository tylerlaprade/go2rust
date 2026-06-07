use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct printer {
    pub output: Arc<Mutex<Option<io_Writer>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        Self { output: self.output.clone() }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.output.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for printer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl printer {
    pub fn write(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let (mut n, mut err) = { let __s = format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }); let __n = __s.len() as i32; (*self.output.lock().unwrap().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (__n, Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))) };
        return (n, err.clone());
    }
}

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut p = Arc::new(Mutex::new(Some(printer { output: Arc::new(Mutex::new(Some({ let __writer = buf.clone(); io_Writer::__go_from_with_write(__writer.clone(), move |__data| { let mut __guard = __writer.lock().unwrap(); if let Some(__target) = __guard.as_mut() { let _ = __target.write(Arc::new(Mutex::new(Some(__data.to_vec())))); } }) }))), ..Default::default() })));
    let (mut n, mut err) = { let __recv = p.clone(); let __recv_ptr: *const printer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const printer }; let __result = unsafe { &*__recv_ptr }.write(Arc::new(Mutex::new(Some("hello".to_string())))); __result };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));
        return;
    }
    println!("{} {} {} {}", format!("{}", "wrote".to_string()), format!("{}", n), format!("{}", "bytes:".to_string()), format!("{}", (*(*buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for printer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
