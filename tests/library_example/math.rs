use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

/// Add adds two numbers
pub fn add(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) + (*b.lock().unwrap().as_mut().unwrap()))));
}

/// Multiply multiplies two numbers
pub fn multiply(a: Arc<Mutex<Option<i32>>>, b: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some((*a.lock().unwrap().as_mut().unwrap()) * (*b.lock().unwrap().as_mut().unwrap()))));
}