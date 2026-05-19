use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct bufio_Reader;

impl std::fmt::Display for bufio_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<bufio_Reader>")
    }
}


impl bufio_Reader {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone, Default)]
pub struct http_Response {
    pub body: Arc<Mutex<Option<io_ReadCloser>>>,
}

impl std::fmt::Display for http_Response {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<http_Response>")
    }
}


impl http_Response {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct io_ReadCloser {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_ReadCloser {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}

impl Default for io_ReadCloser {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_ReadCloser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ReadCloser>")
    }
}

impl std::fmt::Display for io_ReadCloser {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_ReadCloser>")
    }
}

impl PartialEq for io_ReadCloser {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_ReadCloser {}

impl PartialOrd for io_ReadCloser {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_ReadCloser {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct strings_Reader;

impl std::fmt::Display for strings_Reader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<strings_Reader>")
    }
}


impl strings_Reader {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


pub mod bufio {
    use super::*;
    pub fn new_reader<T0>(_arg0: T0) -> Arc<Mutex<Option<bufio_Reader>>> {
        Arc::new(Mutex::new(Some::<bufio_Reader>(Default::default())))
    }
}


pub mod http {
    use super::*;
    pub fn read_response<T0, T1>(_arg0: T0, _arg1: T1) -> (Arc<Mutex<Option<http_Response>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<http_Response>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


pub mod io {
    use super::*;
    pub fn read_all<T0>(_arg0: T0) -> (Arc<Mutex<Option<Vec<u8>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        (Arc::new(Mutex::new(Some::<Vec<u8>>(Default::default()))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}


pub mod strings {
    use super::*;
    pub fn new_reader<T0>(_arg0: T0) -> Arc<Mutex<Option<strings_Reader>>> {
        Arc::new(Mutex::new(Some::<strings_Reader>(Default::default())))
    }
}


fn main() {
    let mut __defer_stack: Vec<Box<dyn FnOnce()>> = Vec::new();

    let mut payload = Arc::new(Mutex::new(Some("{\"slideshow\":{\"author\":\"Yours Truly\",\"slides\":[1,2,3]}}".to_string())));
    let mut raw = Arc::new(Mutex::new(Some(format!("{}{}", "HTTP/1.1 200 OK\r\nContent-Length: 57\r\n\r\n".to_string(), { let __v = (*payload.lock().unwrap().as_ref().unwrap()).clone(); __v }))));
    let (mut resp, mut err) = http::read_response(bufio::new_reader(strings::new_reader(raw.clone())), ());
    if (*err.lock().unwrap()).is_some() {
        println!("{} {}", "Error:".to_string(), format!("{}", (*err.lock().unwrap().as_ref().unwrap())));
        {
        // Execute deferred functions
        while let Some(f) = __defer_stack.pop() {
            f();
        }
        return
    }
    }
    let resp_defer_captured = resp.clone(); __defer_stack.push(Box::new(move || {
        (*(*resp_defer_captured.lock().unwrap().as_ref().unwrap()).body.lock().unwrap().as_ref().unwrap()).close();
    }));

    let (mut body, _) = io::read_all({ let __selector_holder = (*resp.lock().unwrap().as_ref().unwrap()).body.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned });
    let mut text = Arc::new(Mutex::new(Some(String::from_utf8((*body.lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    if { let __tmp_x = ((*text.lock().unwrap().as_ref().unwrap()).len() as i32); let __tmp_y = (100 as i32); __tmp_x > __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({ let __s = (*text.lock().unwrap().as_ref().unwrap()).clone(); __s[..(100) as usize].to_string() }))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *text.lock().unwrap() = __moved_val; };
    }
    println!("{} {}", "Response:".to_string(), { let __v = (*text.lock().unwrap().as_ref().unwrap()).clone(); __v });

    // Execute deferred functions
    while let Some(f) = __defer_stack.pop() {
        f();
    }
}