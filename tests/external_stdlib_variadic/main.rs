use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

fn __go_next_external_interface_id() -> usize {
    static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}



#[derive(Clone)]
pub struct hash_Hash {
    pub __go_id: usize,
    pub __go_value: Arc<dyn std::any::Any + Send + Sync>,
}

impl hash_Hash {
    pub fn __go_from<T: 'static + Send + Sync>(value: T) -> Self {
        Self { __go_id: __go_next_external_interface_id(), __go_value: Arc::new(value) }
    }
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        self.__go_value.as_ref().downcast_ref::<T>()
    }
}

impl Default for hash_Hash {
    fn default() -> Self {
        Self { __go_id: 0, __go_value: Arc::new(()) }
    }
}

impl std::fmt::Debug for hash_Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<hash_Hash>")
    }
}

impl std::fmt::Display for hash_Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<hash_Hash>")
    }
}

impl PartialEq for hash_Hash {
    fn eq(&self, other: &Self) -> bool {
        self.__go_id == other.__go_id
    }
}

impl Eq for hash_Hash {}

impl PartialOrd for hash_Hash {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for hash_Hash {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.__go_id.cmp(&other.__go_id)
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


pub mod io {
    use super::*;
    pub fn Discard() -> Arc<Mutex<Option<io_Writer>>> {
        Arc::new(Mutex::new(Some::<io_Writer>(Default::default())))
    }

    pub fn multi_writer<T0>(_arg0: T0) -> Arc<Mutex<Option<io_Writer>>> {
        panic!("multi_writer bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


pub mod md5 {
    use super::*;
    pub fn new() -> Arc<Mutex<Option<hash_Hash>>> {
        panic!("new bridge: generic stub function body has no implementation; add a custom emitter or remove the call — see AGENTS.md 'Strategy: Transpile stdlib, don't bridge it' and docs/bridge_debt.md")
    }
}


fn main() {
    io::multi_writer(({ let __go_arg = { let __selector_holder = io::Discard().clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; __go_arg }, md5::new()));
    println!("{}", format!("{}", "ok".to_string()));
}