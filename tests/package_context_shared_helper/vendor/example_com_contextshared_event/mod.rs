use go2rust_stdlib_stubs::*;

use std::sync::{Arc, Mutex};

pub type Exporter = Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<GoContext>>>) -> Arc<Mutex<Option<GoContext>>> + Send + Sync>>>>;


pub fn set_exporter(exporter: Exporter) -> Arc<Mutex<Option<GoContext>>> {

    return example_com_contextshared_core::r#use(Arc::new(Mutex::new(Some(GoContext::background()))), exporter.clone());
}