use std::cell::{RefCell};
use std::error::Error as StdError;
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

    pub fn len(&self) -> Rc<RefCell<Option<i32>>> {
        Rc::new(RefCell::new(Some::<i32>(self.__go_data.lock().unwrap().len() as i32)))
    }

    pub fn reset(&self) {
        self.__go_data.lock().unwrap().clear();
    }

    pub fn available(&self) -> Rc<RefCell<Option<i32>>> {
        self.len()
    }

    pub fn available_buffer(&self) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn cap(&self) -> Rc<RefCell<Option<i32>>> {
        self.len()
    }

    pub fn grow<T0>(&self, _arg0: T0) {
    }

    pub fn next<T0>(&self, _arg0: T0) -> Rc<RefCell<Option<Vec<u8>>>> {
        Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new())))
    }

    pub fn read<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_byte(&self) -> (Rc<RefCell<Option<u8>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<u8>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_bytes<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<Vec<u8>>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::new()))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_from<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn read_rune(&self) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(Some::<i32>(0))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
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

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Rc<RefCell<Option<Vec<u8>>>>>() {
            v.borrow().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_string<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
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
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
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

    pub fn write_rune<T0: 'static>(&self, arg0: T0) -> (Rc<RefCell<Option<i32>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
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
        (Rc::new(RefCell::new(Some::<i32>(n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }

    pub fn write_to<T0>(&self, _arg0: T0) -> (Rc<RefCell<Option<i64>>>, Rc<RefCell<Option<Box<dyn StdError>>>>) {
        (Rc::new(RefCell::new(Some::<i64>(self.__go_data.lock().unwrap().len() as i64))), Rc::new(RefCell::new(None::<Box<dyn StdError>>)))
    }
}


/// Mirrors go/ast/print.go's `n, err := fmt.Fprintf(p.output, ...)` pattern
/// where p.output is an io.Writer. Without proper handling, the call lowers
/// to `print!(...)` which returns () and the destructure fails.
fn main() {
    let mut buf: Rc<RefCell<Option<bytes_Buffer>>> = Rc::new(RefCell::new(Some(Default::default())));
    let (mut n, mut err) = { let __s = format!("answer={}", 42); let __n = __s.len() as i32; (*buf.clone().borrow().as_ref().unwrap()).__go_write_bytes(__s.as_bytes()); (Rc::new(RefCell::new(Some::<i32>(__n))), Rc::new(RefCell::new(None::<Box<dyn StdError>>))) };
    if (*err.borrow()).is_some() {
        println!("{} {}", format!("{}", "err:".to_string()), format!("{}", format!("{}", (*err.borrow().as_ref().unwrap()))));
        return;
    }
    println!("{} {} {} {}", format!("{}", "wrote".to_string()), format!("{}", { let __v = (*n.borrow().as_ref().unwrap()).clone(); __v }), format!("{}", "bytes:".to_string()), format!("{}", (*(*buf.borrow_mut().as_mut().unwrap()).string().borrow().as_ref().unwrap())));
}