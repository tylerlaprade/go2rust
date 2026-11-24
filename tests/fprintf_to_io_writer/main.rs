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

#[derive(Clone, Default)]
pub struct printer {
    pub output: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl printer {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.output.clone();
        Self {
            output: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for printer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.output.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl printer {
    pub fn write(&self, s: Arc<Mutex<Option<String>>>) -> (i64, Arc<Mutex<Option<Arc<dyn StdError + Send + Sync>>>>) {
        let (mut n, mut err) = { let __s = format!("{}", { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }); (*self.output.lock().unwrap().as_mut().unwrap()).write(Arc::new(Mutex::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };
        return (n, { let __err_source = err.clone(); let __err_copy = (*__err_source.lock().unwrap()).clone(); Arc::new(Mutex::new(__err_copy)) });
    }
}

fn main() {
    ::bytes::__go_init_all();
    ::internal_abi::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::internal_race::__go_init_all();
    ::internal_sync::__go_init_all();
    ::io::__go_init_all();
    ::math_bits::__go_init_all();
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut buf: Arc<Mutex<Option<bytes::buffer::Buffer>>> = Arc::new(Mutex::new(Some(Default::default())));
    let mut p = Arc::new(Mutex::new(Some(printer { output: Arc::new(Mutex::new(Some(Box::new(bytes::buffer::BufferPtr(buf.clone().clone())) as Box<dyn io::r#mod::Writer + Send + Sync>))), ..Default::default() })));
    let (mut n, mut err) = { let __recv = p.clone(); let __recv_ptr: *const printer = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const printer }; let __result = unsafe { &*__recv_ptr }.write(Arc::new(Mutex::new(Some("hello".to_string())))); __result };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        println!("{} {}", format!("{}", "err:".to_string()), format!("{}", { let __text = match err.lock().unwrap().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));
        return;
    }
    println!("{} {} {} {}", format!("{}", "wrote".to_string()), format!("{}", n), format!("{}", "bytes:".to_string()), format!("{}", (*(*buf.lock().unwrap().as_ref().unwrap()).string().lock().unwrap().as_ref().unwrap())));
}

impl GoValueClone for printer {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
