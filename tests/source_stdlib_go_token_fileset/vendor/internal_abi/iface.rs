use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
};

use crate::{r#type::{Type}};

use std::any::Any;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// EmptyInterface describes the layout of a "interface{}" or a "any."
/// These are represented differently than non-empty interface, as the first
/// word always points to an abi.Type.
#[derive(Clone)]
pub struct EmptyInterface {
    pub r#type: Arc<Mutex<Option<Type>>>,
    pub data: Arc<Mutex<Option<usize>>>,
}

impl EmptyInterface {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.r#type.clone();
        let __go_clone_1_0 = { let __guard = self.data.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            r#type: __go_clone_0_0,
            data: __go_clone_1_0,
        }
    }
}


impl Default for EmptyInterface {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(None));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#type: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for EmptyInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { let __guard = self.r#type.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl GoValueClone for EmptyInterface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
