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

use crate::{
    dir::{DirEntry, READDIR_DIR_ENTRY, READDIR_NAME, readdirMode},
    error::{PathError, is_not_exist},
    file::{lstat_1},
    file_unix::{new_unix_dirent},
    types::{File, FileInfo, FileMode, MODE_CHAR_DEVICE, MODE_DEVICE, MODE_DIR, MODE_NAMED_PIPE, MODE_SOCKET, MODE_SYMLINK},
};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

/// Auxiliary information if the File describes a directory
#[derive(Debug, Clone)]
pub struct dirInfo {
    pub dir: Arc<Mutex<Option<usize>>>,
}

impl dirInfo {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.dir.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        Self {
            dir: __go_clone_0_0,
        }
    }
}


impl Default for dirInfo {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(0)));
        Self {
            dir: __go_default_0_0,
        }
    }
}

impl std::fmt::Display for dirInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __go_fmt_0 = format!("{}", (*self.dir.lock().unwrap().as_ref().unwrap()));
        write!(f, "{{{}}}", __go_fmt_0)
    }
}

impl GoJsonDecode for dirInfo {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl dirInfo {
    pub fn close(&mut self) {
        if { let __tmp_x = (*self.dir.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as usize; __tmp_x == __tmp_y } {
        return;
    }
        closedir(Arc::new(Mutex::new(Some({ let __selector_holder = self.dir.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))));
        { let new_val = 0 as usize; *self.dir.lock().unwrap() = Some(new_val); };
    }
}

impl crate::types::File {
    pub fn readdir_1(&self, mut n: Arc<Mutex<Option<i32>>>, mode: Arc<Mutex<Option<readdirMode>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut dirents: Arc<Mutex<Option<Vec<DirEntry>>>> = Arc::new(Mutex::new(None));
    let mut infos: Arc<Mutex<Option<Vec<FileInfo>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

                // If this file has no dirinfo, create one.
        let mut d: GoPtr<dirInfo> = GoPtr::nil();
        loop {
        d = { let __go_ptr = (*(*self.file.lock().unwrap().as_ref().unwrap()).dirinfo.lock().unwrap().as_mut().unwrap()).load().clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };
        if !d.is_nil() {
        break
    }
        let (mut dir, mut call, mut errno) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).open_dir();
        if { let __nil_result = (*errno.lock().unwrap()).is_some(); __nil_result } {
        return (
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(None)),
            Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some({ let __arg_holder = call.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), path: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: errno.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
        );
    }
        d = GoPtr::local(Arc::new(Mutex::new(Some(dirInfo { dir: Arc::new(Mutex::new(Some(dir))), ..Default::default() }))));
        if (*(*self.file.lock().unwrap().as_ref().unwrap()).dirinfo.lock().unwrap().as_mut().unwrap()).compare_and_swap(sync_atomic::GoPtr::nil(), { let __go_ptr = d.clone(); match __go_ptr { GoPtr::Nil => sync_atomic::GoPtr::nil(), GoPtr::Local(__value) => sync_atomic::GoPtr::local(__value.clone()), GoPtr::Raw(__addr) => sync_atomic::GoPtr::raw(__addr), GoPtr::SliceElem(__value) => sync_atomic::GoPtr::slice_elem(sync_atomic::GoSliceElemPtr::new(__value.slice_handle(), __value.index())), GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } }) {
        break
    }

                // We lost the race: try again.
        { let __result = d.with_mut(|__recv_value| __recv_value.close()); __result };
    }
                // We lost the race: try again.
        let mut size = { let __owned = n.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
        if { let __tmp_x = { let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x <= __tmp_y } {
        { let new_val = 100; *size.lock().unwrap() = Some(new_val); };
        { let new_val = -1; *n.lock().unwrap() = Some(new_val); };
    }
        let mut dirent: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Dirent>>> = Arc::new(Mutex::new(Some(Default::default())));
        let mut entptr: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Dirent>>> = Arc::new(Mutex::new(None));
        while { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ((*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*dirents.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = ((*infos.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = ({ let __v = (*size.lock().unwrap().as_ref().unwrap()).clone(); __v } as i32); __tmp_x < __tmp_y } || { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = -1; __tmp_x == __tmp_y } {
        {
        let mut errno = readdir_r(Arc::new(Mutex::new(Some({ let __selector_holder = { let __ptr_value = d.with_mut(|__ptr_value| __ptr_value.dir.clone()); __ptr_value }.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), dirent.clone(), Arc::new(Mutex::new(Some(entptr.clone()))));;
        if { let __tmp_x = (*errno.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(0 as usize)))); __tmp_x != __tmp_y } {
            if {
                let __tmp_x = (*errno.lock().unwrap().as_ref().unwrap()).clone();
                let __tmp_y = syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_N_T_R as usize))));
                __tmp_x == __tmp_y
            } {
        continue
    };
            return (
                names.clone(),
                dirents.clone(),
                infos.clone(),
                Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("readdir".to_string()))), path: Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: Arc::new(Mutex::new(Some(Box::new((*errno.lock().unwrap().as_ref().unwrap()).clone()) as Box<dyn StdError + Send + Sync>))), ..Default::default() }) as Box<dyn StdError + Send + Sync>)))
            );;
        }
    }
        if { let __nil_result = (*entptr.lock().unwrap()).is_none(); __nil_result } {
        break
    }

                // Darwin may return a zero inode when a directory entry has been
                // deleted but not yet removed from the directory. The man page for
                // getdirentries(2) states that programs are responsible for skipping
                // those entries:
                //
                //   Users of getdirentries() should skip entries with d_fileno = 0,
                //   as such entries represent files which have been deleted but not
                //   yet removed from the directory entry.
                //
        if { let __tmp_x = (*{ let __field = (*dirent.lock().unwrap().as_ref().unwrap()).ino.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = 0 as u64; __tmp_x == __tmp_y } {
        continue
    }
        let mut name = Arc::new(Mutex::new(Some({
            let __seq_holder = Arc::new(Mutex::new({ let __ptr = Arc::new(Mutex::new(Some(Arc::as_ptr(&(*dirent.lock().unwrap().as_ref().unwrap()).name.clone()) as usize))).clone(); let __ptr_guard = __ptr.lock().unwrap(); if __ptr_guard.as_ref().map(|__v| *__v == 0).unwrap_or(true) { None } else { Some::<[u8; 1024]>(unimplemented!("unsafe.Pointer conversion to [u8; 1024]")) } })).clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0);
            let mut __seq = (*__seq_guard.as_ref().unwrap()).clone();
            drop(__seq_guard);
            let __low = 0;
            let __high = __seq.len();
            let __max = __source_cap;
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        })));
        { let __range_holder = name.clone(); let __range_guard = __range_holder.lock().unwrap(); let __range_values = __range_guard.as_ref().cloned().unwrap_or_default(); drop(__range_guard); for (i, c) in __range_values.iter().copied().enumerate() {
        if { let __tmp_x = c; let __tmp_y = 0 as u8; __tmp_x == __tmp_y } {
        { let new_val = Arc::new(Mutex::new(Some({
            let __seq_holder = name.clone();
            let __seq_guard = __seq_holder.lock().unwrap();
            let __source_cap = __seq_guard.as_ref().map(|__v| __v.capacity()).unwrap_or(0);
            let mut __seq = __seq_guard.as_ref().cloned().unwrap_or_default();
            drop(__seq_guard);
            let __low = 0;
            let __high = (i) as usize;
            let __max = __source_cap;
            if __seq.len() < __high { __seq.resize_with(__high, Default::default); }
            let _slice = &__seq[__low..__high];
            let mut _v = Vec::with_capacity((__max - __low) as usize);
            _v.extend_from_slice(_slice);
            _v
        }))); name = new_val; };
        break
    }
    } }

                // Check for useless names before allocating a string.
        if { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*name.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = ".".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = (*Arc::new(Mutex::new(Some(String::from_utf8((*name.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = "..".to_string(); __tmp_x == __tmp_y } {
        continue
    }
        if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::dir::readdirMode(Arc::new(Mutex::new(Some(READDIR_NAME as i32)))); __tmp_x == __tmp_y } {
        { let new_val = { let __append_target = names.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push((*Arc::new(Mutex::new(Some(String::from_utf8((*name.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap()).clone()); __append_target.clone() }; names = new_val; };
    } else if { let __tmp_x = (*mode.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = crate::dir::readdirMode(Arc::new(Mutex::new(Some(READDIR_DIR_ENTRY as i32)))); __tmp_x == __tmp_y } {
        let (mut de, mut err) = new_unix_dirent(
            Arc::new(Mutex::new(Some({ let __selector_holder = (*self.file.lock().unwrap().as_ref().unwrap()).name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))),
            Arc::new(Mutex::new(Some(String::from_utf8((*name.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))),
            dt_to_type(Arc::new(Mutex::new(Some({ let __selector_holder = (*dirent.lock().unwrap().as_ref().unwrap()).r#type.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))))
        );
        if is_not_exist(err.clone()) {
        continue
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), dirents.clone(), Arc::new(Mutex::new(None)), err.clone());
    }
        { let new_val = { let __append_target = dirents.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(de.clone()); __append_target.clone() }; dirents = new_val; };
    } else {
        let (mut info, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = lstat_1.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*(*self.file.lock().unwrap().as_ref().unwrap()).name.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", "/".to_string()));
            __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(String::from_utf8((*name.lock().unwrap().as_ref().unwrap()).clone()).unwrap()))).lock().unwrap().as_ref().unwrap())));
            __s
        })))) };
        if is_not_exist(err.clone()) {
        continue
    }
        if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), infos.clone(), err.clone());
    }
        { let new_val = { let __append_target = infos.clone(); (*__append_target.lock().unwrap()).get_or_insert_with(Vec::new).push(info.clone()); __append_target.clone() }; infos = new_val; };
    }
                // File disappeared between readdir and stat.
                // Treat as if it didn't exist.
                // File disappeared between readdir + stat.
                // Treat as if it didn't exist.
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
    }
                // EOF
                // Darwin may return a zero inode when a directory entry has been
                // deleted but not yet removed from the directory. The man page for
                // getdirentries(2) states that programs are responsible for skipping
                // those entries:
                //
                //   Users of getdirentries() should skip entries with d_fileno = 0,
                //   as such entries represent files which have been deleted but not
                //   yet removed from the directory entry.
                //
                // Check for useless names before allocating a string.
                // File disappeared between readdir and stat.
                // Treat as if it didn't exist.
                // File disappeared between readdir + stat.
                // Treat as if it didn't exist.
        if { let __tmp_x = { let __v = (*n.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x > __tmp_y } && { let __tmp_x = ({ let __tmp_x = ({ let __tmp_x = ((*names.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); let __tmp_y = ((*dirents.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = ((*infos.lock().unwrap()).as_ref().map(|__v| __v.len()).unwrap_or(0) as i32); __tmp_x + __tmp_y } as i32); let __tmp_y = 0; __tmp_x == __tmp_y } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), Arc::new(Mutex::new(None)), { let __return_value_3 = io::EOF.clone(); __return_value_3 });
    }
        return (names.clone(), dirents.clone(), infos.clone(), Arc::new(Mutex::new(None)));
    }
}

pub fn dt_to_type(typ: Arc<Mutex<Option<u8>>>) -> Arc<Mutex<Option<io_fs::r#mod::FileMode>>> {
    { let _switch_val = { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v };
    if _switch_val == (syscall::D_T__B_L_K as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DEVICE as u32)))))));
        } else if _switch_val == (syscall::D_T__C_H_R as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some((MODE_DEVICE as u32 | MODE_CHAR_DEVICE as u32) as u32)))))));
        } else if _switch_val == (syscall::D_T__D_I_R as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_DIR as u32)))))));
        } else if _switch_val == (syscall::D_T__F_I_F_O as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_NAMED_PIPE as u32)))))));
        } else if _switch_val == (syscall::D_T__L_N_K as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SYMLINK as u32)))))));
        } else if _switch_val == (syscall::D_T__R_E_G as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32)))))));
        } else if _switch_val == (syscall::D_T__S_O_C_K as u8) {
            return Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_SOCKET as u32)))))));
        }
    }
    Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(!0 as u32)))))))
}

///go:linkname closedir syscall.closedir
pub fn closedir(dir: Arc<Mutex<Option<usize>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    unimplemented!("Go function declaration has no body");
}


///go:linkname readdir_r syscall.readdir_r
pub fn readdir_r(dir: Arc<Mutex<Option<usize>>>, entry: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Dirent>>>, result: Arc<Mutex<Option<Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Dirent>>>>>>) -> Arc<Mutex<Option<syscall::syscall_unix::Errno>>> {
    unimplemented!("Go function declaration has no body");
}


impl GoValueClone for dirInfo {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
