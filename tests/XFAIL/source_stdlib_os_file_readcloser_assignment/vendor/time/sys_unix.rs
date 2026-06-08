use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoByteSequence, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef, format_slice, format_slice_values, format_slice_wrapped, go_recover, go_resume_unrecovered_panic, go_store_panic_payload};

use crate::{zoneinfo_read::{SEEK_END, SEEK_START}};

use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

pub fn open(name: Arc<Mutex<Option<String>>>) -> (usize, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let (mut fd, mut err) = syscall::open(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(syscall::O__R_D_O_N_L_Y))), Arc::new(Mutex::new(Some(0 as u32))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (0, err.clone());
    }
    ((*Arc::new(Mutex::new(Some(fd as usize))).lock().unwrap().as_ref().unwrap()), Arc::new(Mutex::new(None)))
}

pub fn read(fd: Arc<Mutex<Option<usize>>>, buf: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    syscall::read(Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))), buf.clone())
}

pub fn closefd(fd: Arc<Mutex<Option<usize>>>) {
    syscall::close(Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))));
}

pub fn preadn(fd: Arc<Mutex<Option<usize>>>, mut buf: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut whence = Arc::new(Mutex::new(Some(SEEK_START)));
    if { let __tmp_x = { let __v = (*off.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        { let new_val = 2; *whence.lock().unwrap() = Some(new_val); };
    }
    {
        let (_, mut err) = syscall::seek(Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))), Arc::new(Mutex::new(Some((*off.lock().unwrap().as_ref().unwrap()) as i64))), Arc::new(Mutex::new(Some({ let __arg_holder = whence.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
    while { let __tmp_x = ((*buf.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = 0; __tmp_x > __tmp_y } {
        let (mut m, mut err) = syscall::read(Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32))), buf.clone());
        if { let __tmp_x = m; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
        return errors::new(Arc::new(Mutex::new(Some("short read".to_string()))));
    }
        return err.clone();
    }
        { let new_val = Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0); let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default(); drop(__seq_guard); let __low = (m) as usize; let __high = __seq.len(); let __max = __source_cap; if __seq.len() < __high { __seq.resize_with(__high, Default::default); }; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))); buf = new_val; };
    }
    return Arc::new(Mutex::new(None));
}