use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{error::{ErrInvalid, PathError}, file_unix::{file, fix_long_path}, types::{File, FileMode, MODE_SETGID, MODE_SETUID, MODE_STICKY}};

use std::any::Any;
use std::error::Error as StdError;
use std::sync::{Arc, Mutex};

impl crate::types::File {
    /// Close closes the [File], rendering it unusable for I/O.
    /// On files that support [File.SetDeadline], any pending I/O operations will
    /// be canceled and return immediately with an [ErrClosed] error.
    /// Close will return an error if it has already been called.
    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if false {
        return ErrInvalid.clone();
    }
        (*self.file.lock().unwrap().as_ref().unwrap()).close()
    }

    /// read reads up to len(b) bytes from the File.
    /// It returns the number of bytes read and an error, if any.
    pub fn read_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let (__tmp_0, __tmp_1) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).read(b.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// pread reads len(b) bytes from the File starting at byte offset off.
    /// It returns the number of bytes read and the error, if any.
    /// EOF is signaled by a zero count with err set to nil.
    pub fn pread(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let (__tmp_0, __tmp_1) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).pread(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// write writes len(b) bytes to the File.
    /// It returns the number of bytes written and an error, if any.
    pub fn write_1(&self, b: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let (__tmp_0, __tmp_1) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).write(b.clone()); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// pwrite writes len(b) bytes to the File starting at byte offset off.
    /// It returns the number of bytes written and an error, if any.
    pub fn pwrite(&self, b: Arc<Mutex<Option<Vec<u8>>>>, off: Arc<Mutex<Option<i64>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut n: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        { let (__tmp_0, __tmp_1) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).pwrite(b.clone(), Arc::new(Mutex::new(Some({ let __arg_holder = off.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *n.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
        return ({ let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }

    /// See docs in file.go:(*File).Chmod.
    pub fn chmod_1(&self, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("chmod".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        {
        let mut e = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).fchmod(Arc::new(Mutex::new(Some(syscall_mode(Arc::new(Mutex::new(Some({ let __arg_holder = mode.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            return self.wrap_err(Arc::new(Mutex::new(Some("chmod".to_string()))), e.clone());;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// Chown changes the numeric uid and gid of the named file.
    /// If there is an error, it will be of type [*PathError].
    ///
    /// On Windows, it always returns the [syscall.EWINDOWS] error, wrapped
    /// in *PathError.
    pub fn chown(&self, uid: Arc<Mutex<Option<i32>>>, gid: Arc<Mutex<Option<i32>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("chown".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        {
        let mut e = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).fchown(Arc::new(Mutex::new(Some({ let __arg_holder = uid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = gid.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            return self.wrap_err(Arc::new(Mutex::new(Some("chown".to_string()))), e.clone());;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// Truncate changes the size of the file.
    /// It does not change the I/O offset.
    /// If there is an error, it will be of type [*PathError].
    pub fn truncate(&self, size: Arc<Mutex<Option<i64>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("truncate".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        {
        let mut e = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).ftruncate(Arc::new(Mutex::new(Some({ let __arg_holder = size.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            return self.wrap_err(Arc::new(Mutex::new(Some("truncate".to_string()))), e.clone());;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// Sync commits the current contents of the file to stable storage.
    /// Typically, this means flushing the file system's in-memory copy
    /// of recently written data to disk.
    pub fn sync(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("sync".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        {
        let mut e = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).fsync();;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            return self.wrap_err(Arc::new(Mutex::new(Some("sync".to_string()))), e.clone());;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// Chdir changes the current working directory to the file,
    /// which must be a directory.
    /// If there is an error, it will be of type [*PathError].
    pub fn chdir(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("chdir".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        {
        let mut e = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).fchdir();;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            return self.wrap_err(Arc::new(Mutex::new(Some("chdir".to_string()))), e.clone());;
        }
    }
        return Arc::new(Mutex::new(None));
    }

    /// setDeadline sets the read and write deadline.
    pub fn set_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("SetDeadline".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).set_deadline(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// setReadDeadline sets the read deadline.
    pub fn set_read_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("SetReadDeadline".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).set_read_deadline(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// setWriteDeadline sets the write deadline.
    pub fn set_write_deadline_1(&self, t: Arc<Mutex<Option<time::r#mod::Time>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        {
        let mut err = self.check_valid(Arc::new(Mutex::new(Some("SetWriteDeadline".to_string()))));;
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
            return err.clone();;
        }
    }
        (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).set_write_deadline(Arc::new(Mutex::new(Some({ let __arg_holder = t.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))
    }

    /// checkValid checks whether f is valid for use.
    /// If not, it returns an appropriate error, perhaps incorporating the operation name op.
    pub fn check_valid(&self, op: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if false {
        return ErrInvalid.clone();
    }
        return Arc::new(Mutex::new(None));
    }
}

/// syscallMode returns the syscall-specific mode bits from Go's portable mode bits.
pub fn syscall_mode(i: FileMode) -> u32 {
    let mut o: Arc<Mutex<Option<u32>>> = Arc::new(Mutex::new(Some(0)));

    { let __rhs = (*Arc::new(Mutex::new(Some((*(*io_fs::r#mod::FileMode::perm(&(*i.lock().unwrap().as_ref().unwrap())).lock().unwrap().as_ref().unwrap()).0.lock().unwrap().as_ref().unwrap()) as u32))).lock().unwrap().as_ref().unwrap()); let mut guard = o.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    if { let __tmp_x = { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETUID as u32)))); __tmp_x & __tmp_y }; let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        { let __rhs = syscall::S__I_S_U_I_D as u32; let mut guard = o.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SETGID as u32)))); __tmp_x & __tmp_y }; let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        { let __rhs = syscall::S__I_S_G_I_D as u32; let mut guard = o.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }
    if { let __tmp_x = { let __tmp_x = (*i.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_STICKY as u32)))); __tmp_x & __tmp_y }; let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        { let __rhs = syscall::S__I_S_V_T_X as u32; let mut guard = o.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() | __rhs); };
    }

        // No mapping for Go's ModeTemporary (plan9 only).
    return (*o.lock().unwrap().as_ref().unwrap());
}

/// See docs in file.go:Chmod.
pub fn chmod_1(name: Arc<Mutex<Option<String>>>, mode: FileMode) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    let mut longName = fix_long_path(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    let longName_closure_clone = longName.clone(); let mode_closure_clone = mode.clone(); let mut e = ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        return syscall::chmod(Arc::new(Mutex::new(Some({ let __arg_holder = longName_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(syscall_mode(Arc::new(Mutex::new(Some({ let __arg_holder = mode_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
    if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("chmod".to_string()))), path: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: e.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>)));
    }
    return Arc::new(Mutex::new(None));
}

/// ignoringEINTR makes a function call and repeats it if it returns an
/// EINTR error. This appears to be required even though we install all
/// signal handlers with SA_RESTART: see #22838, #38033, #38836, #40846.
/// Also #20400 and #36644 are issues in which a signal handler is
/// installed without setting SA_RESTART. None of these are the common case,
/// but there are enough of them that it seems that we can't avoid
/// an EINTR loop.
pub fn ignoring_e_i_n_t_r(r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    loop {
        let mut err = { let __f_ptr: *mut Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); !__matched } {
        return err.clone();
    }
    }
}

/// ignoringEINTR2 is ignoringEINTR, but returning an additional value.
pub fn ignoring_e_i_n_t_r2<T: Any + GoValueClone + Send + Sync + 'static>(r#fn: Arc<Mutex<Option<Box<dyn FnMut() -> (Arc<Mutex<Option<T>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync>>>>) -> (Arc<Mutex<Option<T>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    loop {
        let (mut v, mut err) = { let __f_ptr: *mut Box<dyn FnMut() -> (Arc<Mutex<Option<T>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = r#fn.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut() -> (Arc<Mutex<Option<T>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)() };
        if { let __err_holder = err.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_I_N_T_R as usize)).unwrap_or(false); !__matched } {
        return (v.clone(), err.clone());
    }
    }
}