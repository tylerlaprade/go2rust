use go2rust_stdlib_stubs::*;

use crate::search::*;
use crate::r#mod::*;
use crate::zsortfunc::*;
use crate::zsortinterface::*;

use std::any::Any;
use std::sync::{Arc, Mutex};

/// Slice sorts the slice x given the provided less function.
/// It panics if x is not a slice.
///
/// The sort is not guaranteed to be stable: equal elements
/// may be reversed from their original order.
/// For a stable sort, use [SliceStable].
///
/// The less function must satisfy the same requirements as
/// the Interface type's Less method.
///
/// Note: in many situations, the newer [slices.SortFunc] function is more
/// ergonomic and runs faster.
pub fn slice(x: Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>, less: Arc<Mutex<Option<Box<dyn FnMut(Arc<Mutex<Option<i32>>>, Arc<Mutex<Option<i32>>>) -> bool + Send + Sync>>>>) {
    let mut rv = reflectlite::value_of(x.clone());
    let mut swap = reflectlite::swapper(x.clone());
    let mut length = (*rv.lock().unwrap().as_ref().unwrap()).len();
    let mut limit = math_bits::len(Arc::new(Mutex::new(Some(length as u64))));
    pdqsort_func(Arc::new(Mutex::new(Some(lessSwap { less: less.clone(), swap: swap.clone(), ..Default::default() }))), Arc::new(Mutex::new(Some(0))), Arc::new(Mutex::new(Some(length))), Arc::new(Mutex::new(Some(limit))));
}