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

use crate::{r#type::{InterfaceType, Type}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// The first word of every non-empty interface type contains an *ITab.
/// It records the underlying concrete type (Type), the interface type it
/// is implementing (Inter), and some ancillary information.
///
/// allocated in non-garbage-collected memory
#[derive(Clone)]
pub struct ITab {
    pub inter: GoPtr<crate::r#type::InterfaceType>,
    pub r#type: GoPtr<crate::r#type::Type>,
    pub hash: Arc<Mutex<Option<u32>>>,
    pub fun: Arc<Mutex<Option<[usize; 1]>>>,
}

impl ITab {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = self.inter.clone();
        let __go_clone_1_0 = self.r#type.clone();
        let __go_clone_2_0 = { let __guard = self.hash.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_3_0 = { let __guard = self.fun.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            inter: __go_clone_0_0,
            r#type: __go_clone_1_0,
            hash: __go_clone_2_0,
            fun: __go_clone_3_0,
        }
    }
}


impl Default for ITab {
    fn default() -> Self {
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = GoPtr::nil();
        let __go_default_2_0 = Arc::new(Mutex::new(Some(0)));
        let __go_default_3_0 = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        Self {
            inter: __go_default_0_0,
            r#type: __go_default_1_0,
            hash: __go_default_2_0,
            fun: __go_default_3_0,
        }
    }
}

impl std::fmt::Display for ITab {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.inter.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", { if self.r#type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_2 = format!("{}", (*self.hash.lock().unwrap().as_ref().unwrap()));
        let __go_fmt_3 = format!("{}", format_slice(&self.fun));
        write!(f, "{{{} {} {} {}}}", __go_fmt_0, __go_fmt_1, __go_fmt_2, __go_fmt_3)
    }
}


/// EmptyInterface describes the layout of a "interface{}" or a "any."
/// These are represented differently than non-empty interface, as the first
/// word always points to an abi.Type.
#[derive(Clone)]
pub struct EmptyInterface {
    pub r#type: GoPtr<crate::r#type::Type>,
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
        let __go_default_0_0 = GoPtr::nil();
        let __go_default_1_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            r#type: __go_default_0_0,
            data: __go_default_1_0,
        }
    }
}

impl std::fmt::Display for EmptyInterface {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", { if self.r#type.is_nil() { "<nil>".to_string() } else { "<ptr>".to_string() } });
        let __go_fmt_1 = format!("{}", (*self.data.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{} {}}}", __go_fmt_0, __go_fmt_1)
    }
}


impl GoValueClone for ITab {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for EmptyInterface {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
