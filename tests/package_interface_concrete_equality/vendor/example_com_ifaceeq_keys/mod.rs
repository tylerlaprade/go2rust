use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct String_ {
    pub name: Arc<Mutex<Option<String>>>,
}

impl String_ {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for String_ {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for String_ {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.name.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub static Msg: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<String_>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Other: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<String_>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *Msg.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Other.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Msg.lock().unwrap() = Some(new_string(Arc::new(Mutex::new(Some("message".to_string())))));
    *Other.lock().unwrap() = Some(new_string(Arc::new(Mutex::new(Some("other".to_string())))));
}


impl String_ {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    pub fn label(&self) -> Arc<Mutex<Option<example_com_ifaceeq_label::r#mod::Label>>> {
        example_com_ifaceeq_label::new(Arc::new(Mutex::new(Some(Box::new(String_Ptr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync>))))
    }
}

#[derive(Clone)]
pub struct String_Ptr(pub Arc<Mutex<Option<String_>>>);

impl std::fmt::Display for String_Ptr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl example_com_ifaceeq_label::r#mod::Key for String_Ptr {
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        String_::name(__recv)
    }
    fn __go_clone_box_key(&self) -> Box<dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync> {
        Box::new(self.clone()) as Box<dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_key(&self, other: &(dyn example_com_ifaceeq_label::r#mod::Key + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<String_Ptr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn new_string(name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String_>>> {
    Arc::new(Mutex::new(Some(String_ { name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })))
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for String_ {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
