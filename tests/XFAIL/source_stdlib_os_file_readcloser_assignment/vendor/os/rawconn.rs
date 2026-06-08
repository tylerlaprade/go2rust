use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
};

use crate::{types::{File}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// rawConn implements syscall.RawConn.
#[derive(Clone, Default)]
pub struct rawConn {
    pub file: Arc<Mutex<Option<File>>>,
}

impl rawConn {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.file.clone();
        Self {
            file: __go_clone_0_0,
        }
    }
}

impl std::fmt::Display for rawConn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        write!(f, "{{{}}}", __go_fmt_0)
    }
}


impl rawConn {
    pub fn control(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = (*self.file.lock().unwrap().as_ref().unwrap()).check_valid(Arc::new(Mutex::new(Some("SyscallConn.Control".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        let mut err = (*(*self.file.lock().unwrap().as_ref().unwrap()).file.lock().unwrap().as_ref().unwrap().pfd.lock().unwrap().as_mut().unwrap()).raw_control(f.clone());
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.file.clone()) as Box<dyn Any + Send + Sync>))));
        return err.clone();
    }

    pub fn read(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = (*self.file.lock().unwrap().as_ref().unwrap()).check_valid(Arc::new(Mutex::new(Some("SyscallConn.Read".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        let mut err = (*(*self.file.lock().unwrap().as_ref().unwrap()).file.lock().unwrap().as_ref().unwrap().pfd.lock().unwrap().as_mut().unwrap()).raw_read(f.clone());
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.file.clone()) as Box<dyn Any + Send + Sync>))));
        return err.clone();
    }

    pub fn write(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = (*self.file.lock().unwrap().as_ref().unwrap()).check_valid(Arc::new(Mutex::new(Some("SyscallConn.Write".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        let mut err = (*(*self.file.lock().unwrap().as_ref().unwrap()).file.lock().unwrap().as_ref().unwrap().pfd.lock().unwrap().as_mut().unwrap()).raw_write(f.clone());
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.file.clone()) as Box<dyn Any + Send + Sync>))));
        return err.clone();
    }
}

#[derive(Clone)]
pub struct rawConnPtr(pub Arc<Mutex<Option<rawConn>>>);

impl std::fmt::Display for rawConnPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl syscall::net::RawConn for rawConnPtr {
    fn control(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rawConn::control(__recv, f)
    }
    fn read(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rawConn::read(__recv, f)
    }
    fn write(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> bool + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        rawConn::write(__recv, f)
    }
    fn __go_clone_box_raw_conn(&self) -> Box<dyn syscall::net::RawConn + Send + Sync> {
        Box::new(self.clone()) as Box<dyn syscall::net::RawConn + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_raw_conn(&self, other: &(dyn syscall::net::RawConn + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<rawConnPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

pub fn new_raw_conn(file: Arc<Mutex<Option<File>>>) -> (Arc<Mutex<Option<rawConn>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    (
        Arc::new(Mutex::new(Some(rawConn { file: file.clone(), ..Default::default() }))),
        Arc::new(Mutex::new(None))
    )
}

impl GoValueClone for rawConn {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
