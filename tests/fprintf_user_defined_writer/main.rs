#[allow(unused_macros)]
macro_rules! vec {
    () => (GoSlice::new());
    ($__elem:expr; $__n:expr) => (GoSlice::from(std::vec::from_elem($__elem, $__n)));
    ($($__x:expr),+ $(,)?) => (GoSlice::from(<[_]>::into_vec(std::boxed::Box::new([$($__x),+]))));
}
use go2rust_stdlib_stubs::*;
use std::any::Any;
use std::cell::{RefCell};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::rc::{Rc};

#[derive(Clone)]
pub struct userWriter {
    pub count: Rc<RefCell<Option<i64>>>,
    pub buf: Rc<RefCell<Option<bytes::buffer::Buffer>>>,
}

impl userWriter {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.count.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.buf.borrow(); Rc::new(RefCell::new((*__guard).clone())) };
        Self {
            count: __go_clone_0_0,
            buf: __go_clone_1_0,
        }
    }
}


impl Default for userWriter {
    fn default() -> Self {
        let __go_default_0_0 = Rc::new(RefCell::new(Some(0)));
        let __go_default_1_0 = Rc::new(RefCell::new(Some(Default::default())));
        Self {
            count: __go_default_0_0,
            buf: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for userWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.count.borrow().as_ref().unwrap()));
        let __go_fmt_1 = format!("{}", (*self.buf.borrow().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl userWriter {
    pub fn write(&mut self, data: Rc<RefCell<Option<Vec<u8>>>>) -> (i64, Rc<RefCell<Option<Rc<dyn StdError>>>>) {
        { let __target = self.count.clone(); let mut guard = __target.borrow_mut(); *guard = Some(guard.as_ref().unwrap() + 1); }
        (*self.buf.borrow_mut().as_mut().unwrap()).write(data.clone())
    }
}

fn main() {
    ::bytes::__go_init_all();
    ::internal_bytealg::__go_init_all();
    ::internal_cpu::__go_init_all();
    ::unicode_utf8::__go_init_all();

    let mut u = Rc::new(RefCell::new(Some(userWriter { count: Rc::new(RefCell::new(Some(0))), buf: Rc::new(RefCell::new(Some(Default::default()))) })));
    {
        let (_, mut err) = { let __s = format!("a={} b={}", 1, 2); (*u.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
            return;;
        }
    }
    {
        let (_, mut err) = { let __s = format!(" c={}", 3); (*u.borrow_mut().as_mut().unwrap()).write(Rc::new(RefCell::new(Some::<Vec<u8>>(Vec::from(__s.into_bytes()))))) };;
        if { let __nil_result = (*err.borrow()).is_some(); __nil_result } {
            println!("{} {}", format!("{}", "err:".to_string()), format!("{}", { let __text = match err.borrow().as_ref() { Some(__value) => format!("{}", __value), None => "<nil>".to_string() }; __text }));;
            return;;
        }
    }
    println!("{} {}", format!("{}", "count:".to_string()), format!("{}", (*(*u.borrow().as_ref().unwrap()).count.borrow().as_ref().unwrap())));
    println!("{} {}", format!("{}", "buf:".to_string()), format!("{}", (*(*(*u.borrow().as_ref().unwrap()).buf.borrow().as_ref().unwrap()).string().borrow().as_ref().unwrap())));
}

impl GoValueClone for userWriter {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
