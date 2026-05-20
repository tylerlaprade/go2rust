use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct binary_littleEndian;

impl std::fmt::Display for binary_littleEndian {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<binary_littleEndian>")
    }
}


impl binary_littleEndian {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Clone)]
pub struct io_Writer {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl io_Writer {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }

    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }

    pub fn __go_write_bytes(&self, data: &[u8]) {
    }

    pub fn write<T0: 'static>(&self, arg0: T0) -> (Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let bytes = if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Vec<u8>>() {
            v.clone()
        } else if let Some(v) = (&arg0 as &dyn std::any::Any).downcast_ref::<Arc<Mutex<Option<Vec<u8>>>>>() {
            v.lock().unwrap().as_ref().cloned().unwrap_or_default()
        } else {
            Vec::new()
        };
        let n = bytes.len() as i32;
        self.__go_write_bytes(&bytes);
        (Arc::new(Mutex::new(Some::<i32>(n))), Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>)))
    }
}

impl Default for io_Writer {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl std::fmt::Display for io_Writer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<io_Writer>")
    }
}

impl PartialEq for io_Writer {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for io_Writer {}

impl PartialOrd for io_Writer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for io_Writer {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
    }
}


pub mod binary {
    use super::*;
    pub fn LittleEndian() -> Arc<Mutex<Option<binary_littleEndian>>> {
        Arc::new(Mutex::new(Some::<binary_littleEndian>(Default::default())))
    }

    pub fn write<T0, T1, T2>(_arg0: T0, _arg1: T1, _arg2: T2) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        Arc::new(Mutex::new(None::<Box<dyn StdError + Send + Sync>>))
    }
}


pub mod io {
    use super::*;
    pub fn Discard() -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }

    pub fn multi_writer<T0>(_arg0: T0) -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }
}


fn main() {
    let mut out = io::multi_writer(({ let __go_arg = { let __selector_holder = io::Discard().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg },));
    let out_closure_clone = out.clone(); let mut write = Arc::new(Mutex::new(Some(Box::new(move |x: Arc<Mutex<Option<u32>>>| {
        let _ = binary::write(out_closure_clone.clone(), { let __go_arg = { let __selector_holder = binary::LittleEndian().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }, x.clone());
    }) as Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync>)));
    { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> = { let mut __f_guard = write.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<u32>>>) -> () + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some(7)))) };
    println!("{}", format!("{}", "ok".to_string()));
}