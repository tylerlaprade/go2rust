use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

/// Repeat repeats a string n times
pub fn repeat(s: Arc<Mutex<Option<String>>>, n: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<String>>> {

    let mut result = Arc::new(Mutex::new(Some("".to_string())));
    let mut i = Arc::new(Mutex::new(Some(0)));
    while (*i.lock().unwrap().as_mut().unwrap()) < (*n.lock().unwrap().as_mut().unwrap()) {
        (*result.lock().unwrap().as_mut().unwrap()).push_str(&(*s.lock().unwrap().as_mut().unwrap()));
        { let mut guard = i.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    return result.clone();
}