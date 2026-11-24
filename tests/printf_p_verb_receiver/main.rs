#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Scope {
    pub name: Arc<Mutex<Option<String>>>,
}

impl Scope {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            name: __go_clone_0_0,
        }
    }
}


impl Default for Scope {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        Self {
            name: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}


impl Scope {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        let mut buf: Arc<Mutex<Option<strings::builder::Builder>>> = Arc::new(Mutex::new(Some(Default::default())));
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("scope {:p} {{", self)))));
        (*buf.clone().lock().unwrap().as_mut().unwrap()).write_string(Arc::new(Mutex::new(Some(format!("name={}}}", (*self.name.lock().unwrap().as_ref().unwrap()))))));
        return (*buf.lock().unwrap().as_ref().unwrap()).string();
    }
}

fn main() {
    ::internal_bytealg::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::strings::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut s = Arc::new(Mutex::new(Some(Scope { name: Arc::new(Mutex::new(Some("outer".to_string()))), ..Default::default() })));
    let mut out = { let __recv = s.clone(); let __recv_ptr: *const Scope = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const Scope }; let __result = unsafe { &*__recv_ptr }.string(); __result };
    if !strings::has_prefix(Arc::new(Mutex::new(Some({ let __arg_holder = out.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some("scope ".to_string())))) || !strings::contains(Arc::new(Mutex::new(Some({ let __arg_holder = out.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(" {name=outer}".to_string())))) {
        println!("{} {}", format!("{}", "FAIL:".to_string()), format!("{}", { let __v = (*out.lock().unwrap().as_ref().unwrap()).clone(); __v }));
        return;
    }
    println!("{}", format!("{}", "OK".to_string()));
}

impl GoValueClone for Scope {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
