use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{types::{FileMode, MODE_CHAR_DEVICE, MODE_DEVICE, MODE_DIR, MODE_NAMED_PIPE, MODE_SETGID, MODE_SETUID, MODE_SOCKET, MODE_STICKY, MODE_SYMLINK}, types_unix::{fileStat}};

use std::sync::{Arc, Mutex};

pub fn fill_file_stat_from_sys(fs: Arc<Mutex<Option<fileStat>>>, name: Arc<Mutex<Option<String>>>) {
    { let new_val = internal_filepathlite::base(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*fs.lock().unwrap().as_ref().unwrap()).name.lock().unwrap() = __moved_val; };
    { let new_val = { let __selector_holder = (*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).size.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }; *(*fs.lock().unwrap().as_ref().unwrap()).size.lock().unwrap() = Some(new_val); };
    { let new_val = time::unix((*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mtimespec.lock().unwrap().as_ref().unwrap()).unix()); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*fs.lock().unwrap().as_ref().unwrap()).mod_time.lock().unwrap() = __moved_val; };
    { let new_val = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some({ let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0777 as u16; __tmp_x & __tmp_y } as u32)))); *(*fs.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap() = Some(new_val); };
    { let _switch_val = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_F_M_T as u16; __tmp_x & __tmp_y };
    if _switch_val == (syscall::S__I_F_B_L_K as u16) || _switch_val == (syscall::S__I_F_W_H_T as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DEVICE as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_C_H_R as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some((MODE_DEVICE as u32 | MODE_CHAR_DEVICE as u32) as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_D_I_R as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DIR as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_I_F_O as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_NAMED_PIPE as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_L_N_K as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SYMLINK as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        } else if _switch_val == (syscall::S__I_F_R_E_G as u16) {
        } else if _switch_val == (syscall::S__I_F_S_O_C_K as u16) {
            { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SOCKET as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
        }
    }
        // nothing to do
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_G_I_D as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETGID as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_U_I_D as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETUID as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*(*(*fs.lock().unwrap().as_ref().unwrap()).sys.lock().unwrap().as_ref().unwrap()).mode.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_S_V_T_X as u16; __tmp_x & __tmp_y }; let __tmp_y = 0 as u16; __tmp_x != __tmp_y } {
        { let __target = (*fs.lock().unwrap().as_ref().unwrap()).mode.clone(); let __rhs = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_STICKY as u32)))); let mut guard = __target.lock().unwrap(); *guard = Some(guard.as_ref().unwrap().clone() | __rhs); };
    }
}