use std::sync::{Arc, Mutex};

pub fn clean(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some({ let __s = (*Arc::new(Mutex::new(Some({ let __s = (*Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); __s.to_lowercase() }))).lock().unwrap().as_ref().unwrap()).clone(); let __prefix = "r#".to_string(); __s.strip_prefix(&__prefix).unwrap_or(&__s).to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); __s.to_uppercase() })));
}

pub fn clean_suffix(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some({ let __s = (*Arc::new(Mutex::new(Some({ let __s = (*name.lock().unwrap().as_ref().unwrap()).clone(); let __suffix = ".RS".to_string(); __s.strip_suffix(&__suffix).unwrap_or(&__s).to_string() }))).lock().unwrap().as_ref().unwrap()).clone(); __s.to_lowercase() })));
}

fn main() {
    println!("{}", format!("{}", (*clean(Arc::new(Mutex::new(Some("R#Go2Rust".to_string())))).lock().unwrap().as_ref().unwrap())));
    println!("{}", format!("{}", (*clean_suffix(Arc::new(Mutex::new(Some("Go2Rust.RS".to_string())))).lock().unwrap().as_ref().unwrap())));
}