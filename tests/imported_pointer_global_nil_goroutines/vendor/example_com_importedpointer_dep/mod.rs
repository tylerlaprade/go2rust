use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Node {
    pub value: Arc<Mutex<Option<i32>>>,
}

impl Node {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.value.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            value: __go_clone_0_0,
        }
    }
}


impl Default for Node {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            value: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.value.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


pub trait Reader: std::fmt::Display + Any {
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool;
    fn read(&self) -> i32;
}

impl Clone for Box<dyn Reader + Send + Sync> {
    fn clone(&self) -> Self {
        Reader::__go_clone_box_reader(self.as_ref())
    }
}

pub static Current: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Node>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static Other: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Arc<Mutex<Option<Node>>>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


pub fn __go_get_global_Current() -> Arc<Mutex<Option<Node>>> { (*Current.lock().unwrap().as_ref().unwrap()).clone() }
pub fn __go_clear_global_Current() { *Current.lock().unwrap() = Some(Arc::new(Mutex::new(None))); }

pub fn __go_get_global_Other() -> Arc<Mutex<Option<Node>>> { (*Other.lock().unwrap().as_ref().unwrap()).clone() }
pub fn __go_clear_global_Other() { *Other.lock().unwrap() = Some(Arc::new(Mutex::new(None))); }

fn __go_init_globals() {
    *Current.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Other.lock().unwrap() = Some(Arc::new(Mutex::new(None)));
    *Current.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(7))), ..Default::default() }))));
    *Other.lock().unwrap() = Some(Arc::new(Mutex::new(Some(Node { value: Arc::new(Mutex::new(Some(9))), ..Default::default() }))));
}


impl Node {
    pub fn read(&self) -> i32 {
        return (*self.value.lock().unwrap().as_ref().unwrap());
    }
}

impl Reader for Node {
    fn read(&self) -> i32 {
        Node::read(self)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<Node>() {
            false
        } else {
            false
        }
    }
}

#[derive(Clone)]
pub struct NodePtr(pub Arc<Mutex<Option<Node>>>);

impl std::fmt::Display for NodePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl Reader for NodePtr {
    fn read(&self) -> i32 {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        Node::read(__recv)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<NodePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn get() -> Arc<Mutex<Option<Node>>> {
    (*Current.lock().unwrap().as_ref().unwrap()).clone()
}

pub fn view() -> Arc<Mutex<Option<Box<dyn Reader + Send + Sync>>>> {
    Arc::new(Mutex::new(Some(Box::new(NodePtr({ let __arg_holder = Current.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })) as Box<dyn Reader + Send + Sync>)))
}

pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for Node {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
