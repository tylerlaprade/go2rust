use go2rust_stdlib_stubs::*;

use crate::{
    GoArrayElemMutRef,
    GoArrayElemPtr,
    GoArrayElemRef,
    GoPtr,
    GoSliceElemMutRef,
    GoSliceElemPtr,
    GoSliceElemRef,
    format_any,
    format_map,
    format_nested_pointer_slice,
    format_nested_pointer_slice_wrapped,
    format_nested_slice,
    format_nested_slice_wrapped,
    format_slice,
    format_slice_values,
    format_slice_wrapped,
    format_slice_wrapped_values,
    go_any_clone,
    go_const_str_eq,
    go_recover,
    go_resume_unrecovered_panic,
    go_store_panic_payload,
};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// sleep and wakeup on one-time events.
/// before any calls to notesleep or notewakeup,
/// must call noteclear to initialize the Note.
/// then, exactly one thread can call notesleep
/// and exactly one thread can call notewakeup (once).
/// once notewakeup has been called, the notesleep
/// will return.  future notesleep will return immediately.
/// subsequent noteclear must be called only after
/// previous notesleep has returned, e.g. it's disallowed
/// to call noteclear straight after notewakeup.
///
/// notetsleep is like notesleep but wakes up after
/// a given number of nanoseconds even if the event
/// has not yet happened.  if a goroutine uses notetsleep to
/// wake up early, it must wait to call noteclear until it
/// can be sure that no other goroutine is calling
/// notewakeup.
///
/// notesleep/notetsleep are generally called on g0,
/// notetsleepg is similar to notetsleep but is called on user g.
#[derive(Debug, Clone)]
pub struct note {
    pub key: Arc<Mutex<Option<usize>>>,
}

impl note {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.key.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            key: __go_clone_0_0,
        }
    }
}


impl Default for note {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            key: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for note {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.key.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for note {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl GoValueClone for note {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
