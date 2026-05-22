use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct file {
    pub name: Arc<Mutex<Option<String>>>,
}

impl file {
    pub fn __go_value_clone(&self) -> Self {
        Self { name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for file {
    fn default() -> Self {
        Self { name: Arc::new(Mutex::new(Some(String::new()))) }
    }
}

impl std::fmt::Display for file {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


fn main() {
    let mut files = Arc::new(Mutex::new(Some(vec![Arc::new(Mutex::new(Some(file { name: Arc::new(Mutex::new(Some("a".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(file { name: Arc::new(Mutex::new(Some("b".to_string()))), ..Default::default() }))), Arc::new(Mutex::new(Some(file { name: Arc::new(Mutex::new(Some("c".to_string()))), ..Default::default() })))])));
    { let new_val = { let __slice_holder = files.clone(); let mut __slice_guard = __slice_holder.lock().unwrap(); let __slice = __slice_guard.get_or_insert_with(Vec::new); let __i = (1) as usize; let __j = (2) as usize; if __i <= __j && __j <= __slice.len() { __slice.drain(__i..__j); } __slice_holder.clone() }; files = new_val; };
    println!("{}", format!("{}", (*files.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0)));
    println!("{}", format!("{}", (*(*{ let __seq = { let __seq_holder = files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(0) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*(*{ let __seq = { let __seq_holder = files.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[(1) as usize].clone() }.lock().unwrap().as_ref().unwrap()).name.lock().unwrap().as_ref().unwrap())));
}