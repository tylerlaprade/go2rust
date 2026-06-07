use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct writer {
    pub buffer: Arc<Mutex<Option<bytes::buffer::Buffer>>>,
}

impl writer {
    pub fn __go_value_clone(&self) -> Self {
        Self { buffer: { let __guard = self.buffer.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for writer {
    fn default() -> Self {
        Self { buffer: Arc::new(Mutex::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for writer {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl writer {
    pub fn available(&self) -> i32 {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.available()
    }

    pub fn available_buffer(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.available_buffer()
    }

    pub fn bytes(&self) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.bytes()
    }

    pub fn cap(&self) -> i32 {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.cap()
    }

    pub fn grow(&self, _arg0: Arc<Mutex<Option<i32>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.grow(_arg0)
    }

    pub fn len(&self) -> i32 {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.len()
    }

    pub fn next(&self, _arg0: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.next(_arg0)
    }

    pub fn read(&self, _arg0: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read(_arg0)
    }

    pub fn read_byte(&self) -> (u8, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read_byte()
    }

    pub fn read_bytes(&self, _arg0: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read_bytes(_arg0)
    }

    pub fn read_from(&self, _arg0: Arc<Mutex<Option<io_Reader>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read_from(_arg0)
    }

    pub fn read_rune(&self) -> (i32, i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read_rune()
    }

    pub fn read_string(&self, _arg0: Arc<Mutex<Option<u8>>>) -> (Arc<Mutex<Option<String>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.read_string(_arg0)
    }

    pub fn reset(&self) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.reset()
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.string()
    }

    pub fn truncate(&self, _arg0: Arc<Mutex<Option<i32>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.truncate(_arg0)
    }

    pub fn unread_byte(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.unread_byte()
    }

    pub fn unread_rune(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.unread_rune()
    }

    pub fn write(&self, _arg0: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write(_arg0)
    }

    pub fn write_byte(&self, _arg0: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write_byte(_arg0)
    }

    pub fn write_rune(&self, _arg0: Arc<Mutex<Option<i32>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write_rune(_arg0)
    }

    pub fn write_string(&self, _arg0: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write_string(_arg0)
    }

    pub fn write_to(&self, _arg0: Arc<Mutex<Option<io_Writer>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let embedded = self.buffer.clone();
        let mut guard = embedded.lock().unwrap();
        let embedded_ref = guard.as_mut().unwrap();
        embedded_ref.write_to(_arg0)
    }
}

fn main() {
    bytes::__go_init_all();
    internal_bytealg::__go_init_all();
    internal_cpu::__go_init_all();
    unicode_utf8::__go_init_all();

    let mut w = Arc::new(Mutex::new(Some(writer { buffer: Arc::new(Mutex::new(Some(bytes::buffer::Buffer { ..Default::default() }))), ..Default::default() })));
    (*w.lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some("go".to_string()))));
    (*w.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some(("rust".to_string()).as_bytes().to_vec()))));
    (*w.lock().unwrap().as_mut().unwrap()).write_byte(Arc::new(Mutex::new(Some(('!' as i32) as u8))));
    (*w.lock().unwrap().as_mut().unwrap()).reset();
    let mut length = (*w.lock().unwrap().as_ref().unwrap()).len();
    let mut text = (*w.lock().unwrap().as_ref().unwrap()).string();
    print!("len={} string={:?}\n", length, { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v });
}

impl GoValueClone for writer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
