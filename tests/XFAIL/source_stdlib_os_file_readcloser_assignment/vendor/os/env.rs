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

use std::sync::{Arc, Mutex};

/// Getenv retrieves the value of the environment variable named by the key.
/// It returns the value, which will be empty if the variable is not present.
/// To distinguish between an empty value and an unset value, use [LookupEnv].
pub fn getenv(key: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    internal_testlog::getenv(Arc::new(Mutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let (mut v, _) = syscall::getenv(Arc::new(Mutex::new(Some({ let __arg_holder = key.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    return { let __owned = v.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}