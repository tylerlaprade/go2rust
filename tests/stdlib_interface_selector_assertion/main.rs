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

#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
#[derive(Clone, Default)]
pub struct holder {
    pub out: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
    pub err: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>,
}

impl holder {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.out.clone();
        let __go_clone_1_0 = self.err.clone();
        Self {
            out: __go_clone_0_0,
            err: __go_clone_1_0,
        }
    }
}

impl std::fmt::Display for holder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.out.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.err.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
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
    ::sync::__go_init_all();
    ::sync_atomic::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut h: Arc<Mutex<Option<holder>>> = Arc::new(Mutex::new(Some(Default::default())));
    {
        let (_, mut ok) = ({
        let val = (*h.lock().unwrap().as_ref().unwrap()).out.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn io::r#mod::Writer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<Arc<Mutex<Option<os_File>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<os_File>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<os_File>)), false)
        }
    });;
        if ok {
            println!("{}", format!("{}", "file".to_string()));;
        } else {
            println!("{}", format!("{}", "not file".to_string()));;
        }
    }
    {
        let (mut buf, _) = ({
        let val = (*h.lock().unwrap().as_ref().unwrap()).err.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn io::r#mod::Writer + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<bytes::buffer::BufferPtr>() {
                (typed_val.0.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<bytes::buffer::Buffer>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<bytes::buffer::Buffer>)), false)
        }
    });;
        if { let __nil_result = (*buf.lock().unwrap()).is_some(); __nil_result } {
            println!("{}", format!("{}", "buffer".to_string()));;
        } else {
            println!("{}", format!("{}", "not buffer".to_string()));;
        }
    }
}

impl GoValueClone for holder {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
