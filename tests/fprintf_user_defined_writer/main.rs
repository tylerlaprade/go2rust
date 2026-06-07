use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct userWriter {
    pub count: Arc<Mutex<Option<i32>>>,
    pub buf: Arc<Mutex<Option<bytes::buffer::Buffer>>>,
}

impl userWriter {
    pub fn __go_value_clone(&self) -> Self {
        Self { count: { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, buf: { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for userWriter {
    fn default() -> Self {
        Self { count: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for userWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.count.lock().unwrap().as_ref().unwrap()), (*self.buf.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for userWriter {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl userWriter {
    pub fn write(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        { let __target = self.count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        (*self.buf.lock().unwrap().as_mut().unwrap()).write(data.clone())
    }
}

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut u = Arc::new(Mutex::new(Some(userWriter { count: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(Default::default()))) })));
    {
        let (_, mut err) = { let __s = format!("a={} b={}", 1, 2); (*u.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(__s.into_bytes())))) };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));;
            return;;
        }
    }
    {
        let (_, mut err) = { let __s = format!(" c={}", 3); (*u.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(__s.into_bytes())))) };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.lock().unwrap().as_ref().unwrap()))));;
            return;;
        }
    }
    println!("{} {}", format!("{}", "count:".to_string()), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).count.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "buf:".to_string()), format!("{}", (*(*(*u.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for userWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
