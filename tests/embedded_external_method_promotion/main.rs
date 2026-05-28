use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Debug, Clone)]
pub struct bytes_Buffer {
    pub __go_data: std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
}

impl Default for bytes_Buffer {
    fn default() -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(Vec::new())) }
    }
}

impl std::fmt::Display for bytes_Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.__go_string())
    }
}

impl bytes_Buffer {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }

    pub fn __go_from_string(value: String) -> Self {
        Self { __go_data: std::sync::Arc::new(std::sync::Mutex::new(value.into_bytes())) }
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
        self.__go_data.lock().unwrap().extend_from_slice(data);
    }

    pub fn __go_bytes(&self) -> Vec<u8> {
        self.__go_data.lock().unwrap().clone()
    }

    pub fn __go_string(&self) -> String {
        String::from_utf8_lossy(&self.__go_data.lock().unwrap()).into_owned()
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        Rc::new(RefCell::new(Some::<String>(self.__go_string())))
    }

    pub fn bytes(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(self.__go_bytes())))
    }

    pub fn len(&self) -> i32 {
        self.__go_data.lock().unwrap().len() as i32
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> i32 {
        self.len()
    }

    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn cap(&self) -> i32 {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (0 as i32, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_byte(&self) -> (u8, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (0 as u8, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (i64, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (0 as i64, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_rune(&self) -> (i32, i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (0 as i32, 0 as i32, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_string<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<String>(String::new()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn truncate<T0>(&self, _arg0: T0) {
        self.reset();
    }

    pub fn unread_byte(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn unread_rune(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<String>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<&str>() {
            (*v).to_string()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<String>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            String::new()
        };
        let bytes = value.into_bytes();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_byte<T0: 'static>(&self, arg0: T0) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<u8>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            *v as u8
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<u8>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i32>>>>() {
            v.borrow().as_ref().copied().unwrap_or_default() as u8
        } else {
            0
        };
        self.__go_write_bytes(&[value]);
        Rc::new(RefCell::new(None::<Box<dyn StdError>>))
    }

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let value = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<char>() {
            *v
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<i32>() {
            char::from_u32(*v as u32).unwrap_or('\0')
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<i32>>>>() {
            char::from_u32(v.borrow().as_ref().copied().unwrap_or_default() as u32).unwrap_or('\0')
        } else {
            '\0'
        };
        let mut encoded = [0u8; 4];
        let bytes = value.encode_utf8(&mut encoded).as_bytes().to_vec();
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (n, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (i64, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (self.__go_data.lock().unwrap().len() as i64, Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


#[derive(Debug, Clone)]
pub struct writer {
    pub buffer: Rc<RefCell<Option<bytes_Buffer>>>,
}

impl writer {
    pub fn __go_value_clone(&self) -> Self {
        Self { buffer: { let __guard = self.buffer.borrow(); Rc::new(RefCell::new((*__guard).clone())) } }
    }
}


impl Default for writer {
    fn default() -> Self {
        Self { buffer: Rc::new(RefCell::new(Some(Default::default()))) }
    }
}

impl std::fmt::Display for writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().borrow().as_ref().unwrap()))
    }
}


impl writer {
    pub fn available(&self) -> i32 {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.available()
    }

    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.available_buffer()
    }

    pub fn bytes(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.bytes()
    }

    pub fn cap(&self) -> i32 {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.cap()
    }

    pub fn grow<T0: 'static>(&self, _arg0: T0) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.grow(_arg0)
    }

    pub fn len(&self) -> i32 {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.len()
    }

    pub fn next<T0: 'static>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.next(_arg0)
    }

    pub fn read<T0: 'static>(&self, _arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read(_arg0)
    }

    pub fn read_byte(&self) -> (u8, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_byte()
    }

    pub fn read_bytes<T0: 'static>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_bytes(_arg0)
    }

    pub fn read_from<T0: 'static>(&self, _arg0: T0) -> (i64, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_from(_arg0)
    }

    pub fn read_rune(&self) -> (i32, i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_rune()
    }

    pub fn read_string<T0: 'static>(&self, _arg0: T0) -> (Rc<RefCell<Option<String>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.read_string(_arg0)
    }

    pub fn reset(&self) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.reset()
    }

    pub fn string(&self) -> Rc<RefCell<Option<String>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.string()
    }

    pub fn truncate<T0: 'static>(&self, _arg0: T0) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.truncate(_arg0)
    }

    pub fn unread_byte(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unread_byte()
    }

    pub fn unread_rune(&self) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.unread_rune()
    }

    pub fn write<T0: 'static>(&self, _arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write(_arg0)
    }

    pub fn write_byte<T0: 'static>(&self, _arg0: T0) -> Rc<RefCell<Option<Box<dyn StdError>>>> {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_byte(_arg0)
    }

    pub fn write_rune<T0: 'static>(&self, _arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_rune(_arg0)
    }

    pub fn write_string<T0: 'static>(&self, _arg0: T0) -> (i32, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_string(_arg0)
    }

    pub fn write_to<T0: 'static>(&self, _arg0: T0) -> (i64, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let embedded = self.buffer.clone();
        let guard = embedded.borrow();
        let embedded_ref = guard.as_ref().unwrap();
        embedded_ref.write_to(_arg0)
    }
}

fn main() {
    let mut w = Rc::new(RefCell::new(Some(writer { buffer: Rc::new(RefCell::new(Some(bytes_Buffer { ..Default::default() }))), ..Default::default() })));
    (*w.borrow_mut().as_mut().unwrap()).write_string(Rc::new(RefCell::new(Some("go".to_string()))));
    (*w.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some(("rust".to_string()).as_bytes().to_vec()))));
    (*w.borrow_mut().as_mut().unwrap()).write_byte(Rc::new(RefCell::new(Some(('!' as i32) as u8))));
    (*w.borrow_mut().as_mut().unwrap()).reset();
    let mut length = (*w.borrow_mut().as_mut().unwrap()).len();
    let mut text = (*w.borrow_mut().as_mut().unwrap()).string();
    print!("len={} string={:?}\n", length, { let __v = (*text.borrow().as_ref().unwrap()).clone(); __v });
}