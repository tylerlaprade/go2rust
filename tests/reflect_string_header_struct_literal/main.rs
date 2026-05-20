use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct reflect_StringHeader {
    pub data: Arc<Mutex<Option<usize>>>,
    pub len: Arc<Mutex<Option<i32>>>,
}

impl std::fmt::Display for reflect_StringHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<reflect_StringHeader>")
    }
}


impl reflect_StringHeader {
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        None
    }
}


#[derive(Debug, Clone)]
pub struct Label {
    pub packed: Arc<Mutex<Option<u64>>>,
    pub again: Arc<Mutex<Option<u64>>>,
}

impl Label {
    pub fn __go_value_clone(&self) -> Self {
        Self { packed: { let __guard = self.packed.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, again: { let __guard = self.again.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for Label {
    fn default() -> Self {
        Self { packed: Arc::new(Mutex::new(Some(0))), again: Arc::new(Mutex::new(Some(0))) }
    }
}

impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {}}}", (*self.packed.lock().unwrap().as_ref().unwrap()), (*self.again.lock().unwrap().as_ref().unwrap()))
    }
}


pub fn make_label(v: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Label>>> {

    let mut hdr = Arc::new(Mutex::new(Some(reflect_StringHeader { data: Arc::new(Mutex::new(Some(0 as usize))), len: Arc::new(Mutex::new(Some({ let __s = (*v.lock().unwrap().as_ref().unwrap()).clone(); __s.len() as i32 }))), ..Default::default() })));
    return Arc::new(Mutex::new(Some(Label { packed: Arc::new(Mutex::new(Some({ let __selector_holder = (*hdr.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), again: Arc::new(Mutex::new(Some({ let __selector_holder = (*hdr.lock().unwrap().as_ref().unwrap()).len.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as u64))), ..Default::default() })));
}

fn main() {
    let mut label = make_label(Arc::new(Mutex::new(Some("test".to_string()))));
    println!("{} {}", format!("{}", (*{ let __field = (*label.lock().unwrap().as_ref().unwrap()).packed.clone(); __field }.lock().unwrap().as_ref().unwrap())), format!("{}", (*{ let __field = (*label.lock().unwrap().as_ref().unwrap()).again.clone(); __field }.lock().unwrap().as_ref().unwrap())));
}