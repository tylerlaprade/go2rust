use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::error::Error;
use std::any::Any;
use std::cmp::Ord;

pub fn get_hello() -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some("Hello".to_string())));
}

pub fn get_world() -> Arc<Mutex<Option<String>>> {

    return Arc::new(Mutex::new(Some("World".to_string())));
}

pub fn get_magic_number() -> Arc<Mutex<Option<i32>>> {

    return Arc::new(Mutex::new(Some(42)));
}