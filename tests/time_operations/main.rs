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

#[derive(Clone)]
pub struct event {
    pub when: Arc<Mutex<Option<time::r#mod::Time>>>,
}

impl event {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.when.lock().unwrap(); Arc::new(Mutex::new((*__guard).as_ref().map(|__go_field| __go_field.__go_value_clone()))) };
        Self {
            when: __go_clone_0_0,
        }
    }
}


impl Default for event {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            when: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.when.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


fn main() {
    ::internal_bisect::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_godebug::__go_init_all();
    ::internal_godebugs::__go_init_all();
    ::internal_stringslite::__go_init_all();
    ::math_bits::__go_init_all();
    ::time::__go_init_all();

    let mut localBase = time::unix(Arc::new(Mutex::new(Some(1700000000 as i64))), Arc::new(Mutex::new(Some(0 as i64))));
    let mut base = (*localBase.lock().unwrap().as_ref().unwrap()).u_t_c();
    println!("{} {}", format!("{}", "Base time:".to_string()), format!("{}", { let __v = (*base.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));
    let mut future = (*base.lock().unwrap().as_ref().unwrap()).add(Arc::new(Mutex::new(Some(time::r#mod::Duration(Arc::new(Mutex::new(Some(time::HOUR as i64))))))));
    println!("{} {}", format!("{}", "One hour later:".to_string()), format!("{}", { let __v = (*future.lock().unwrap().as_ref().unwrap()).__go_value_clone(); __v }));
    println!("{} {}", format!("{}", "Unix timestamp:".to_string()), format!("{}", (*base.lock().unwrap().as_ref().unwrap()).unix()));
    let mut ev: Arc<Mutex<Option<event>>> = Arc::new(Mutex::new(Some(Default::default())));
    println!("{} {}", format!("{}", "Zero field:".to_string()), format!("{}", (*(*ev.lock().unwrap().as_ref().unwrap()).when.lock().unwrap().as_ref().unwrap()).is_zero()));
}

impl GoValueClone for event {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
