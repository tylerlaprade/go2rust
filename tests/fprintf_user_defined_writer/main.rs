#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct userWriter {
    pub count: Arc<Mutex<Option<i64>>>,
    pub buf: Arc<Mutex<Option<bytes::buffer::Buffer>>>,
}

impl userWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.count.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.buf.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            count: __go_clone_0_0,
            buf: __go_clone_1_0,
        }
    }
}


impl Default for userWriter {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(Default::default())));
        Self {
            count: __go_default_0_0,
            buf: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for userWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.count.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.buf.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl userWriter {
    pub fn write(&mut self, data: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        { let __target = self.count.clone(); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
        (*self.buf.lock().unwrap().as_mut().unwrap()).write(data.clone())
    }
}

#[derive(Clone)]
pub struct userWriterPtr(pub Arc<Mutex<Option<userWriter>>>);

impl std::fmt::Display for userWriterPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::Writer for userWriterPtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        userWriter::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn io::r#mod::Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn io::r#mod::Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<userWriterPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

fn main() {
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut u = Arc::new(Mutex::new(Some(userWriter { count: Arc::new(Mutex::new(Some(0))), buf: Arc::new(Mutex::new(Some(Default::default()))) })));
    {
        let (_, mut err) = { let __s = format!("a={} b={}", format!("{}", 1), format!("{}", 2)); (*u.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
            return;;
        }
    }
    {
        let (_, mut err) = { let __s = format!(" c={}", 3); (*u.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
            return;;
        }
    }
    println!("{} {}", format!("{}", "count:".to_string()), format!("{}", (*{ let __field = (*u.lock().unwrap().as_ref().unwrap()).count.clone(); __field }.lock().unwrap().as_ref().unwrap())));
    println!("{} {}", format!("{}", "buf:".to_string()), format!("{}", (*(*(*u.lock().unwrap().as_ref().unwrap()).buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for userWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
