use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_any, format_map, format_nested_pointer_slice, format_nested_pointer_slice_wrapped, format_nested_slice, format_nested_slice_wrapped, format_slice, format_slice_values, format_slice_wrapped, format_slice_wrapped_values, go_any_clone, go_const_str_eq, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{lock_sema::{notewakeup}, note_other::{note}, os_darwin::{__N_S_I_G, sig_note_wakeup}, panic::{throw}, r#extern::{G_O_O_S}};

use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const SIG_IDLE: i32 = 0;
pub(crate) const SIG_RECEIVING: i32 = 1;
pub(crate) const SIG_SENDING: i32 = 2;


pub(crate) static sig: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<AnonymousStruct31>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *sig.lock().unwrap() = Some(Default::default());
}


pub(crate) fn __go_zero_globals() {
    *sig.lock().unwrap() = Some(Default::default());
}


/// sigsend delivers a signal from sighandler to the internal signal delivery queue.
/// It reports whether the signal was sent. If not, the caller typically crashes the program.
/// It runs from the signal handler, so it's limited in what it can do.
pub fn sigsend(s: Arc<Mutex<Option<u32>>>) -> bool {
    let mut bit = Arc::new(Mutex::new(Some({ let __tmp_x = (1 as u32); let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as u32; __tmp_x & __tmp_y }) as u64))).lock().unwrap().as_ref().unwrap()); __tmp_x << __tmp_y })));
    if { let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = (*Arc::new(Mutex::new(Some(({ let __tmp_x = 32; let __tmp_y = 1; __tmp_x * __tmp_y }) as u32))).lock().unwrap().as_ref().unwrap()) as u32; __tmp_x >= __tmp_y } {
        return false;
    }

    (*(*sig.lock().unwrap().as_ref().unwrap()).delivering.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(1 as i32))));

        // We are running in the signal handler; defer is not available.
    {
        let mut w = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*sig.lock().unwrap().as_ref().unwrap()).wanted.clone(), ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x / __tmp_y }) as usize)));;
        if { let __tmp_x = { let __tmp_x = w; let __tmp_y = { let __v = (*bit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x == __tmp_y } {
            (*(*sig.lock().unwrap().as_ref().unwrap()).delivering.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));;
            return false;;
        }
    }

        // Add signal to outgoing queue.
    loop {
        let mut mask = Arc::new(Mutex::new(Some({ let __seq = { let __seq_holder = (*sig.lock().unwrap().as_ref().unwrap()).mask.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __cloned = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); __cloned }; __seq[({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x / __tmp_y }) as usize].clone() })));
        if { let __tmp_x = { let __tmp_x = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y } {
        (*(*sig.lock().unwrap().as_ref().unwrap()).delivering.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
        return true;
    }
                // signal already in queue
        if internal_runtime_atomic::cas(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*sig.lock().unwrap().as_ref().unwrap()).mask.clone(), ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x / __tmp_y }) as usize)), Arc::new(Mutex::new(Some({ let __arg_holder = mask.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*mask.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = { let __v = (*bit.lock().unwrap().as_ref().unwrap()).clone(); __v }; __tmp_x | __tmp_y })))) {
        break
    }
    }

        // signal already in queue
        // Notify receiver that queue has new bit.
    'send: loop {
        { let _switch_val = (*(*sig.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).load();
    if _switch_val == (SIG_IDLE as u32) {
            if (*(*sig.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(SIG_IDLE as u32))), Arc::new(Mutex::new(Some(SIG_SENDING as u32)))) {
        break 'send
    }
        } else if _switch_val == (SIG_SENDING as u32) {
                        // notification already pending
            break 'send
        } else if _switch_val == (SIG_RECEIVING as u32) {
            if (*(*sig.lock().unwrap().as_ref().unwrap()).state.lock().unwrap().as_mut().unwrap()).compare_and_swap(Arc::new(Mutex::new(Some(SIG_RECEIVING as u32))), Arc::new(Mutex::new(Some(SIG_IDLE as u32)))) {
        if { let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y } {
        sig_note_wakeup((*sig.lock().unwrap().as_ref().unwrap()).note.clone());
        break 'send
    }
        notewakeup((*sig.lock().unwrap().as_ref().unwrap()).note.clone());
        break 'send
    }
        } else {
            throw(Arc::new(Mutex::new(Some("sigsend: inconsistent state".to_string()))));
        }
    }
    }

        // notification already pending
    (*(*sig.lock().unwrap().as_ref().unwrap()).delivering.lock().unwrap().as_mut().unwrap()).add(Arc::new(Mutex::new(Some(-1 as i32))));
    true
}

/// Checked by signal handlers.
///
///go:linkname signal_ignored os/signal.signal_ignored
pub fn signal_ignored(s: Arc<Mutex<Option<u32>>>) -> bool {
    let mut i = internal_runtime_atomic::load(internal_runtime_atomic::GoPtr::array_elem(internal_runtime_atomic::GoArrayElemPtr::new((*sig.lock().unwrap().as_ref().unwrap()).ignored.clone(), ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 32 as u32; __tmp_x / __tmp_y }) as usize)));
    return { let __tmp_x = { let __tmp_x = i; let __tmp_y = ({ let __tmp_x = (1 as u32); let __tmp_y = ({ let __tmp_x = { let __v = (*s.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 31 as u32; __tmp_x & __tmp_y }); __tmp_x << __tmp_y }); __tmp_x & __tmp_y }; let __tmp_y = 0 as u32; __tmp_x != __tmp_y };
}

#[derive(Clone)]
pub struct AnonymousStruct31 {
    pub note: Arc<Mutex<Option<note>>>,
    pub mask: Arc<Mutex<Option<[u32; 1]>>>,
    pub wanted: Arc<Mutex<Option<[u32; 1]>>>,
    pub ignored: Arc<Mutex<Option<[u32; 1]>>>,
    pub recv: Arc<Mutex<Option<[u32; 1]>>>,
    pub state: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub delivering: Arc<Mutex<Option<internal_runtime_atomic::types::Uint32>>>,
    pub inuse: Arc<Mutex<Option<bool>>>,
}
impl AnonymousStruct31 {
    pub fn __go_value_clone(&self) -> Self {
        Self { note: { let __guard = self.note.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, mask: { let __guard = self.mask.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, wanted: { let __guard = self.wanted.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, ignored: { let __guard = self.ignored.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, recv: { let __guard = self.recv.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, state: { let __guard = self.state.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, delivering: { let __guard = self.delivering.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, inuse: { let __guard = self.inuse.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for AnonymousStruct31 {
    fn default() -> Self {
        Self { note: Arc::new(Mutex::new(Some(note::default()))), mask: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), wanted: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), ignored: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), recv: Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0)))), state: Arc::new(Mutex::new(Some(Default::default()))), delivering: Arc::new(Mutex::new(Some(Default::default()))), inuse: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for AnonymousStruct31 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {} {} {}}}", (*self.note.lock().unwrap().as_ref().unwrap()), format_slice(&self.mask), format_slice(&self.wanted), format_slice(&self.ignored), format_slice(&self.recv), (*self.state.lock().unwrap().as_ref().unwrap()), (*self.delivering.lock().unwrap().as_ref().unwrap()), (*self.inuse.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for AnonymousStruct31 {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


pub(crate) type sig = AnonymousStruct31;


pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
