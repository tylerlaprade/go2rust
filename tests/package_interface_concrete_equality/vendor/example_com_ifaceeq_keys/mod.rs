use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct String_ {
    pub name: Arc<Mutex<Option<String>>>,
}

impl std::fmt::Display for String_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", (*self.name.lock().unwrap().as_ref().unwrap()))
    }
}


pub static Msg: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<String_>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Other: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<String_>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Msg.lock().unwrap() = Some(Default::default());
    *Other.lock().unwrap() = Some(Default::default());
    *Msg.lock().unwrap() = Some(new_string(Arc::new(Mutex::new(Some("message".to_string())))));
    *Other.lock().unwrap() = Some(new_string(Arc::new(Mutex::new(Some("other".to_string())))));
}


impl String_ {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    pub fn label(&self) -> Arc<Mutex<Option<example_com_ifaceeq_label::Label>>> {
        return example_com_ifaceeq_label::new(self);
    }
}

impl example_com_ifaceeq_label::Key for String_ {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }
    fn __go_clone_box(&self) -> Box<dyn example_com_ifaceeq_label::Key + Send + Sync> {
        Box::new(self.clone()) as Box<dyn example_com_ifaceeq_label::Key + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq(&self, other: &(dyn example_com_ifaceeq_label::Key + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String_>() {
            false
        } else {
            false
        }
    }
}

pub fn new_string(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String_>>> {

    return Arc::new(Mutex::new(Some(String_ { name: name.clone(), ..Default::default() })));
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
