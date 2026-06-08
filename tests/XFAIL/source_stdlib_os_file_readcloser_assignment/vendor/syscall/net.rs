use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoLocalPtrKey, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display};
use std::sync::{Arc, Mutex};

/// A RawConn is a raw network connection.
pub trait RawConn: std::fmt::Display + Any {
    fn __go_clone_box_raw_conn(&self) -> Box<dyn RawConn + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_raw_conn(&self, other: &(dyn RawConn + Send + Sync)) -> bool;
    fn control(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> () + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn read(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (bool) + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
    fn write(&self, f: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<usize>>>) -> (bool) + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>;
}

impl Clone for Box<dyn RawConn + Send + Sync> {
    fn clone(&self) -> Self {
        RawConn::__go_clone_box_raw_conn(self.as_ref())
    }
}