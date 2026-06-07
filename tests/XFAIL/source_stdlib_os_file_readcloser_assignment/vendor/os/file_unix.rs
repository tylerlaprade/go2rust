use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::dir::*;
use crate::dir_darwin::*;
use crate::eloop_other::*;
use crate::env::*;
use crate::error::*;
use crate::error_errno::*;
use crate::exec::*;
use crate::exec_nohandle::*;
use crate::exec_posix::*;
use crate::exec_unix::*;
use crate::executable::*;
use crate::executable_darwin::*;
use crate::file::*;
use crate::file_open_unix::*;
use crate::file_posix::*;
use crate::getwd::*;
use crate::path::*;
use crate::path_unix::*;
use crate::pidfd_other::*;
use crate::pipe_unix::*;
use crate::proc::*;
use crate::rawconn::*;
use crate::removeall_at::*;
use crate::root::*;
use crate::root_nonwindows::*;
use crate::root_openat::*;
use crate::root_unix::*;
use crate::stat::*;
use crate::stat_darwin::*;
use crate::stat_unix::*;
use crate::sticky_bsd::*;
use crate::sys::*;
use crate::sys_bsd::*;
use crate::sys_unix::*;
use crate::tempfile::*;
use crate::types::*;
use crate::types_unix::*;
use crate::wait_unimp::*;
use crate::zero_copy_posix::*;
use crate::zero_copy_stub::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const __U_T_I_M_E__O_M_I_T: i32 = internal_syscall_unix::U_T_I_M_E__O_M_I_T as i32;


pub(crate) const KIND_NEW_FILE: i32 = 0;
pub(crate) const KIND_OPEN_FILE: i32 = 1;
pub(crate) const KIND_PIPE: i32 = 2;
pub(crate) const KIND_SOCK: i32 = 3;
pub(crate) const KIND_NO_POLL: i32 = 4;


pub const DEV_NULL: &'static str = "/dev/null";


/// file is the real representation of *File.
/// The extra level of indirection ensures that no clients of os
/// can overwrite this data, which could cause the finalizer
/// to close the wrong file descriptor.
#[derive(Clone)]
pub struct file {
    pub pfd: Arc<Mutex<Option<internal_poll::fd_unix::FD>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub dirinfo: Arc<Mutex<Option<sync_atomic::r#type::Pointer<crate::dir_darwin::dirInfo>>>>,
    pub nonblock: Arc<Mutex<Option<bool>>>,
    pub stdout_or_err: Arc<Mutex<Option<bool>>>,
    pub append_mode: Arc<Mutex<Option<bool>>>,
}

impl file {
    pub fn __go_value_clone(&self) -> Self {
        Self { pfd: { let __guard = self.pfd.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, dirinfo: { let __guard = self.dirinfo.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, nonblock: { let __guard = self.nonblock.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, stdout_or_err: { let __guard = self.stdout_or_err.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, append_mode: { let __guard = self.append_mode.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) } }
    }
}


impl Default for file {
    fn default() -> Self {
        Self { pfd: Arc::new(Mutex::new(Some(Default::default()))), name: Arc::new(Mutex::new(Some(String::new()))), dirinfo: Arc::new(Mutex::new(Some(Default::default()))), nonblock: Arc::new(Mutex::new(Some(false))), stdout_or_err: Arc::new(Mutex::new(Some(false))), append_mode: Arc::new(Mutex::new(Some(false))) }
    }
}

impl std::fmt::Display for file {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{} {} {} {} {} {}}}", (*self.pfd.lock().unwrap().as_ref().unwrap()), (*self.name.lock().unwrap().as_ref().unwrap()), (*self.dirinfo.lock().unwrap().as_ref().unwrap()), (*self.nonblock.lock().unwrap().as_ref().unwrap()), (*self.stdout_or_err.lock().unwrap().as_ref().unwrap()), (*self.append_mode.lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for file {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// newFileKind describes the kind of file to newFile.
#[derive(Debug, Clone, Default)]
pub struct newFileKind(pub Arc<Mutex<Option<i32>>>);

impl Display for newFileKind {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for newFileKind {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for newFileKind {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for newFileKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for newFileKind {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<newFileKind> for i32 {
    fn eq(&self, other: &newFileKind) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<newFileKind> for i32 {
    fn partial_cmp(&self, other: &newFileKind) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for newFileKind {
    type Output = newFileKind;
    fn add(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for newFileKind {
    type Output = newFileKind;
    fn add(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<newFileKind> for i32 {
    type Output = newFileKind;
    fn add(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for newFileKind {
    type Output = newFileKind;
    fn sub(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for newFileKind {
    type Output = newFileKind;
    fn sub(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<newFileKind> for i32 {
    type Output = newFileKind;
    fn sub(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for newFileKind {
    type Output = newFileKind;
    fn mul(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for newFileKind {
    type Output = newFileKind;
    fn mul(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<newFileKind> for i32 {
    type Output = newFileKind;
    fn mul(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for newFileKind {
    type Output = newFileKind;
    fn div(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for newFileKind {
    type Output = newFileKind;
    fn div(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<newFileKind> for i32 {
    type Output = newFileKind;
    fn div(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for newFileKind {
    type Output = newFileKind;
    fn neg(self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for newFileKind {
    type Output = newFileKind;
    fn rem(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for newFileKind {
    type Output = newFileKind;
    fn rem(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<newFileKind> for i32 {
    type Output = newFileKind;
    fn rem(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for newFileKind {
    type Output = newFileKind;
    fn bitand(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for newFileKind {
    type Output = newFileKind;
    fn bitand(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<newFileKind> for i32 {
    type Output = newFileKind;
    fn bitand(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for newFileKind {
    type Output = newFileKind;
    fn bitor(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for newFileKind {
    type Output = newFileKind;
    fn bitor(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<newFileKind> for i32 {
    type Output = newFileKind;
    fn bitor(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for newFileKind {
    type Output = newFileKind;
    fn bitxor(self, other: Self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for newFileKind {
    type Output = newFileKind;
    fn bitxor(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<newFileKind> for i32 {
    type Output = newFileKind;
    fn bitxor(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for newFileKind {
    type Output = newFileKind;
    fn not(self) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: i8) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: i16) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: i64) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: u32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: u8) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: u16) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: u64) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for newFileKind {
    type Output = newFileKind;
    fn shl(self, other: usize) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: newFileKind) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: i32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: i8) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: i16) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: i64) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: u32) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: u8) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: u16) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: u64) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for newFileKind {
    type Output = newFileKind;
    fn shr(self, other: usize) -> newFileKind {
        newFileKind(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for newFileKind {}

impl Ord for newFileKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


#[derive(Clone)]
pub struct unixDirent {
    pub parent: Arc<Mutex<Option<String>>>,
    pub name: Arc<Mutex<Option<String>>>,
    pub typ: FileMode,
    pub info: FileInfo,
}

impl unixDirent {
    pub fn __go_value_clone(&self) -> Self {
        Self { parent: { let __guard = self.parent.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, name: { let __guard = self.name.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, typ: { let __guard = self.typ.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) }, info: self.info.clone() }
    }
}


impl Default for unixDirent {
    fn default() -> Self {
        Self { parent: Arc::new(Mutex::new(Some(String::new()))), name: Arc::new(Mutex::new(Some(String::new()))), typ: Arc::new(Mutex::new(Some(io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0))))))), info: Arc::new(Mutex::new(None)) }
    }
}

impl std::fmt::Display for unixDirent {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl GoJsonDecode for unixDirent {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


impl crate::types::File {
    /// Fd returns the integer Unix file descriptor referencing the open file.
    /// If f is closed, the file descriptor becomes invalid.
    /// If f is garbage collected, a finalizer may close the file descriptor,
    /// making it invalid; see [runtime.SetFinalizer] for more information on when
    /// a finalizer might be run. On Unix systems this will cause the [File.SetDeadline]
    /// methods to stop working.
    /// Because file descriptors can be reused, the returned file descriptor may
    /// only be closed through the [File.Close] method of f, or by its finalizer during
    /// garbage collection. Otherwise, during garbage collection the finalizer
    /// may close an unrelated file descriptor with the same (reused) number.
    ///
    /// As an alternative, see the f.SyscallConn method.
    pub fn fd(&self) -> usize {
        if false {
        return !(0 as usize);
    }
                // If we put the file descriptor into nonblocking mode,
                // then set it to blocking mode before we return it,
                // because historically we have always returned a descriptor
                // opened in blocking mode. The File will continue to work,
                // but any blocking operation will tie up a thread.
        if (*(*self.file.lock().unwrap().as_ref().unwrap()).nonblock.clone().lock().unwrap().as_ref().unwrap()) {
        (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).set_blocking();
    }
        (*Arc::new(Mutex::new(Some({ let __selector_holder = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).sysfd.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned } as usize))).lock().unwrap().as_ref().unwrap())
    }

    /// seek sets the offset for the next Read or Write on file to offset, interpreted
    /// according to whence: 0 means relative to the origin of the file, 1 means
    /// relative to the current offset, and 2 means relative to the end.
    /// It returns the new offset and an error, if any.
    pub fn seek_1(&self, offset: Arc<Mutex<Option<i64>>>, whence: Arc<Mutex<Option<i32>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ret: Arc<Mutex<Option<i64>>> = Arc::new(Mutex::new(Some(0)));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        {
        let mut info: GoPtr<crate::dir_darwin::dirInfo> = { let __go_ptr = (*(*self.file.lock().unwrap().as_ref().unwrap()).dirinfo.lock().unwrap().as_mut().unwrap()).swap(sync_atomic::GoPtr::nil()).clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };;
        if !info.is_nil() {
            { let __result = info.with_mut(|__recv_value| __recv_value.close()); __result };;
        }
    }
                // Free cached dirinfo, so we allocate a new one if we
                // access this file as a directory again. See #35767 and #37161.
        { let (__tmp_0, __tmp_1) = (*(*self.file.lock().unwrap().as_ref().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).seek(Arc::new(Mutex::new(Some({ let __arg_holder = offset.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = whence.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))); *ret.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_1; };
        runtime::keep_alive(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))));
        return ({ let __v = (*ret.lock().unwrap().as_ref().unwrap()).clone(); __v }, err.clone());
    }
}

impl file {
    pub fn close(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        if false {
        return Arc::new(Mutex::new(Some(Box::new(syscall::syscall_unix::Errno(Arc::new(Mutex::new(Some(syscall::E_I_N_V_A_L as usize))))) as Box<dyn StdError + Send + Sync>)));
    }
        {
        let mut info: GoPtr<crate::dir_darwin::dirInfo> = { let __go_ptr = (*self.dirinfo.lock().unwrap().as_mut().unwrap()).swap(sync_atomic::GoPtr::nil()).clone(); match __go_ptr { sync_atomic::GoPtr::Nil => GoPtr::nil(), sync_atomic::GoPtr::Local(__value) => GoPtr::local(__value.clone()), sync_atomic::GoPtr::Raw(__addr) => GoPtr::raw(__addr), sync_atomic::GoPtr::SliceElem(__value) => GoPtr::slice_elem(GoSliceElemPtr::new(__value.slice_handle(), __value.index())), sync_atomic::GoPtr::ArrayElem(_) => unimplemented!("cross-package GoPtr array element forwarding requires shared GoPtr helpers") } };;
        if !info.is_nil() {
            { let __result = info.with_mut(|__recv_value| __recv_value.close()); __result };;
        }
    }
        let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
        {
        let mut e = (*self.pfd.lock().unwrap().as_mut().unwrap()).close();;
        if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
            if { let __left = e.clone(); let __right = internal_poll::ErrFileClosing.clone(); let __same_handle = Arc::ptr_eq(&__left, &__right); let __eq = if __same_handle { true } else { let __left_guard = __left.lock().unwrap(); let __right_guard = __right.lock().unwrap(); if __left_guard.is_none() || __right_guard.is_none() { __left_guard.is_none() == __right_guard.is_none() } else { false } }; __eq } {
        { let __rhs_holder = ErrClosed.clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *e.lock().unwrap() = new_val; };
    };
            { let new_val = Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("close".to_string()))), path: Arc::new(Mutex::new(Some({ let __selector_holder = self.name.clone(); let __selector_guard = __selector_holder.lock().unwrap(); let __cloned = (*__selector_guard.as_ref().unwrap()).clone(); drop(__selector_guard); __cloned }))), err: e.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>; *err.lock().unwrap() = Some(new_val); };;
        }
    }
                // no need for a finalizer anymore
        runtime::set_finalizer(Arc::new(Mutex::new(Some(Box::new(self.clone()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(None::<Box<dyn Any + Send + Sync>>)));
        return err.clone();
    }
}

impl unixDirent {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    pub fn is_dir(&self) -> bool {
        io_fs::r#mod::FileMode::is_dir(&(*self.typ.lock().unwrap().as_ref().unwrap()))
    }

    pub fn r#type(&self) -> Arc<Mutex<Option<io_fs::r#mod::FileMode>>> {
        return self.typ.clone();
    }

    pub fn info(&self) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if { let __iface_handle = { let __field = self.info.clone(); __field }; let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).is_some() } {
        return ({ let __field = self.info.clone(); __field }, Arc::new(Mutex::new(None)));
    }
        { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = lstat_1.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", (*self.parent.clone().lock().unwrap().as_ref().unwrap()))); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", (*self.name.clone().lock().unwrap().as_ref().unwrap()))); __s })))) }
    }

    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        io_fs::format_dir_entry(Arc::new(Mutex::new(Some(Box::new(unixDirentPtr(Arc::new(Mutex::new(Some(self.clone()))))) as Box<dyn io_fs::r#mod::DirEntry + Send + Sync>))))
    }
}

#[derive(Clone)]
pub struct unixDirentPtr(pub Arc<Mutex<Option<unixDirent>>>);

impl std::fmt::Display for unixDirentPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io_fs::r#mod::DirEntry for unixDirentPtr {
    fn info(&self) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unixDirent::info(__recv)
    }
    fn is_dir(&self) -> bool {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unixDirent::is_dir(__recv)
    }
    fn name(&self) -> Arc<Mutex<Option<String>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unixDirent::name(__recv)
    }
    fn r#type(&self) -> Arc<Mutex<Option<io_fs::r#mod::FileMode>>> {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        unixDirent::r#type(__recv)
    }
    fn __go_clone_box_dir_entry(&self) -> Box<dyn io_fs::r#mod::DirEntry + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io_fs::r#mod::DirEntry + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_dir_entry(&self, other: &(dyn io_fs::r#mod::DirEntry + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<unixDirentPtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// fixLongPath is a noop on non-Windows platforms.
pub fn fix_long_path(path: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<String>>> {
    return { let __owned = path.lock().unwrap().as_ref().unwrap().clone(); Arc::new(Mutex::new(Some(__owned))) };
}

/// NewFile returns a new File with the given file descriptor and
/// name. The returned value will be nil if fd is not a valid file
/// descriptor. On Unix systems, if the file descriptor is in
/// non-blocking mode, NewFile will attempt to return a pollable File
/// (one for which the SetDeadline methods work).
///
/// After passing it to NewFile, fd may become invalid under the same
/// conditions described in the comments of the Fd method, and the same
/// constraints apply.
pub fn new_file(fd: Arc<Mutex<Option<usize>>>, name: Arc<Mutex<Option<String>>>) -> Arc<Mutex<Option<crate::types::File>>> {
    let mut fdi = Arc::new(Mutex::new(Some((*fd.lock().unwrap().as_ref().unwrap()) as i32)));
    if { let __tmp_x = { let __v = (*fdi.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x < __tmp_y } {
        return Arc::new(Mutex::new(None));
    }

    let (mut flags, mut err) = internal_syscall_unix::fcntl(Arc::new(Mutex::new(Some({ let __arg_holder = fdi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(syscall::F__G_E_T_F_L))), Arc::new(Mutex::new(Some(0))));
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        { let new_val = 0; flags = new_val; };
    }
    let mut f = new_file_1(Arc::new(Mutex::new(Some({ let __arg_holder = fdi.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(newFileKind(Arc::new(Mutex::new(Some(KIND_NEW_FILE as i32))))))), Arc::new(Mutex::new(Some(internal_syscall_unix::has_nonblock_flag(Arc::new(Mutex::new(Some(flags))))))));
    { let new_val = { let __tmp_x = { let __tmp_x = flags; let __tmp_y = syscall::O__A_P_P_E_N_D; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y }; *(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).append_mode.lock().unwrap() = Some(new_val); };
    return f.clone();
}

/// newFile is like NewFile, but if called from OpenFile or Pipe
/// (as passed in the kind parameter) it tries to add the file to
/// the runtime poller.
pub fn new_file_1(fd: Arc<Mutex<Option<i32>>>, name: Arc<Mutex<Option<String>>>, kind: Arc<Mutex<Option<newFileKind>>>, nonBlocking: Arc<Mutex<Option<bool>>>) -> Arc<Mutex<Option<crate::types::File>>> {
    let mut f = { let __owner = Arc::new(Mutex::new(Some(crate::types::File { file: Arc::new(Mutex::new(Some(file { pfd: Arc::new(Mutex::new(Some(internal_poll::fd_unix::FD { sysfd: Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), is_stream: Arc::new(Mutex::new(Some(true))), zero_read_is_e_o_f: Arc::new(Mutex::new(Some(true))), ..Default::default() }))), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), stdout_or_err: Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*fd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 1; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*fd.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 2; __tmp_x == __tmp_y }))), ..Default::default() }))).clone(), ..Default::default() }))); let __embedded_key = { let __owner_guard = __owner.lock().unwrap(); let __embedded = __owner_guard.as_ref().unwrap().file.clone(); let __embedded_guard = __embedded.lock().unwrap(); __embedded_guard.as_ref().map(|__v| __v as *const _ as usize).unwrap_or(0) }; go_register_embedded_owner(__embedded_key, __owner.clone()); __owner };

    let mut pollable = Arc::new(Mutex::new(Some({ let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = newFileKind(Arc::new(Mutex::new(Some(KIND_OPEN_FILE as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = newFileKind(Arc::new(Mutex::new(Some(KIND_PIPE as i32)))); __tmp_x == __tmp_y } || { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = newFileKind(Arc::new(Mutex::new(Some(KIND_SOCK as i32)))); __tmp_x == __tmp_y } || { let __v = (*nonBlocking.lock().unwrap().as_ref().unwrap()).clone(); __v })));

        // Things like regular files and FIFOs in kqueue on *BSD/Darwin
        // may not work properly (or accurately according to its manual).
        // As a result, we should avoid adding those to the kqueue-based
        // netpoller. Check out #19093, #24164, and #66239 for more contexts.
        //
        // If the fd was passed to us via any path other than OpenFile,
        // we assume those callers know what they were doing, so we won't
        // perform this check and allow it to be added to the kqueue.
    if { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = newFileKind(Arc::new(Mutex::new(Some(KIND_OPEN_FILE as i32)))); __tmp_x == __tmp_y } {
        { let _switch_val = runtime::G_O_O_S;
    if _switch_val == ("darwin".to_string()) || _switch_val == ("ios".to_string()) || _switch_val == ("dragonfly".to_string()) || _switch_val == ("freebsd".to_string()) || _switch_val == ("netbsd".to_string()) || _switch_val == ("openbsd".to_string()) {
            let mut st: Arc<Mutex<Option<syscall::ztypes_darwin_arm64::Stat_t>>> = Arc::new(Mutex::new(Some(Default::default())));
            let fd_closure_clone = fd.clone(); let st_closure_clone = st.clone(); let mut err = ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        return syscall::fstat(Arc::new(Mutex::new(Some({ let __arg_holder = fd_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), st_closure_clone.clone());
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
            let mut typ = Arc::new(Mutex::new(Some({ let __tmp_x = (*{ let __field = (*st.lock().unwrap().as_ref().unwrap()).mode.clone(); __field }.lock().unwrap().as_ref().unwrap()); let __tmp_y = syscall::S__I_F_M_T as u16; __tmp_x & __tmp_y })));
                        // Don't try to use kqueue with regular files on *BSDs.
                        // On FreeBSD a regular file is always
                        // reported as ready for writing.
                        // On Dragonfly, NetBSD and OpenBSD the fd is signaled
                        // only once as ready (both read and write).
                        // Issue 19093.
                        // Also don't add directories to the netpoller.
            if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } && ({ let __tmp_x = { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::S__I_F_R_E_G as u16; __tmp_x == __tmp_y } || { let __tmp_x = { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::S__I_F_D_I_R as u16; __tmp_x == __tmp_y }) {
        { let new_val = false; *pollable.lock().unwrap() = Some(new_val); };
    }
                        // In addition to the behavior described above for regular files,
                        // on Darwin, kqueue does not work properly with fifos:
                        // closing the last writer does not cause a kqueue event
                        // for any readers. See issue #24164.
            if ({ let __tmp_x = "darwin".to_string(); let __tmp_y = "darwin".to_string(); __tmp_x == __tmp_y } || { let __tmp_x = "darwin".to_string(); let __tmp_y = "ios".to_string(); __tmp_x == __tmp_y }) && { let __tmp_x = { let __v = (*typ.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::S__I_F_I_F_O as u16; __tmp_x == __tmp_y } {
        { let new_val = false; *pollable.lock().unwrap() = Some(new_val); };
    }
        }
    }
    }

        // Don't try to use kqueue with regular files on *BSDs.
        // On FreeBSD a regular file is always
        // reported as ready for writing.
        // On Dragonfly, NetBSD and OpenBSD the fd is signaled
        // only once as ready (both read and write).
        // Issue 19093.
        // Also don't add directories to the netpoller.
        // In addition to the behavior described above for regular files,
        // on Darwin, kqueue does not work properly with fifos:
        // closing the last writer does not cause a kqueue event
        // for any readers. See issue #24164.
    let mut clearNonBlock = Arc::new(Mutex::new(Some(false)));
    if { let __v = (*pollable.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // The descriptor is already in non-blocking mode.
                // We only set f.nonblock if we put the file into
                // non-blocking mode.
        if { let __v = (*nonBlocking.lock().unwrap().as_ref().unwrap()).clone(); __v } {
                // See the comments on net_newUnixFile.
        if { let __tmp_x = (*kind.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = newFileKind(Arc::new(Mutex::new(Some(KIND_SOCK as i32)))); __tmp_x == __tmp_y } {
        { let new_val = true; *(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).nonblock.lock().unwrap() = Some(new_val); };
    }
    } else {
        let mut err = syscall::set_nonblock(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(true))));;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = true; *(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).nonblock.lock().unwrap() = Some(new_val); };;
            { let new_val = true; *clearNonBlock.lock().unwrap() = Some(new_val); };;
        } else {
            { let new_val = false; *pollable.lock().unwrap() = Some(new_val); };;
        }
    }
    }

        // The descriptor is already in non-blocking mode.
        // We only set f.nonblock if we put the file into
        // non-blocking mode.
        // See the comments on net_newUnixFile.
        // tell Fd to return blocking descriptor
        // An error here indicates a failure to register
        // with the netpoll system. That can happen for
        // a file descriptor that is not supported by
        // epoll/kqueue; for example, disk files on
        // Linux systems. We assume that any real error
        // will show up in later I/O.
        // We do restore the blocking behavior if it was set by us.
    {
        let mut pollErr = (*(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).pfd.lock().unwrap().as_mut().unwrap()).init(Arc::new(Mutex::new(Some("file".to_string()))), Arc::new(Mutex::new(Some({ let __arg_holder = pollable.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if { let __nil_result = (*pollErr.lock().unwrap()).is_some(); __nil_result } && { let __v = (*clearNonBlock.lock().unwrap().as_ref().unwrap()).clone(); __v } {
            {
        let mut err = syscall::set_nonblock(Arc::new(Mutex::new(Some({ let __arg_holder = fd.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(false))));;
        if { let __nil_result = (*err.lock().unwrap()).is_none(); __nil_result } {
            { let new_val = false; *(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).nonblock.lock().unwrap() = Some(new_val); };;
        }
    };
        }
    }

    runtime::set_finalizer(Arc::new(Mutex::new(Some(Box::new((*f.lock().unwrap().as_ref().unwrap()).file.clone()) as Box<dyn Any + Send + Sync>))), Arc::new(Mutex::new(Some(Box::new(Arc::new(Mutex::new(Some(Box::new(move |__arg0: Arc<Mutex<Option<file>>>| -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> { { let __recv = __arg0.clone(); let __recv_ptr: *const file = { let __recv_guard = __recv.lock().unwrap(); __recv_guard.as_ref().unwrap() as *const file }; let __result = unsafe { &*__recv_ptr }.close(); __result } }) as Box<dyn FnMut(Arc<Mutex<Option<file>>>) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))).clone()) as Box<dyn Any + Send + Sync>))));
    return f.clone();
}

pub fn sigpipe() {
    unimplemented!("Go function declaration has no body");
}


/// epipecheck raises SIGPIPE if we get an EPIPE error on standard
/// output or standard error. See the SIGPIPE docs in os/signal, and
/// issue 11845.
pub fn epipecheck(file: Arc<Mutex<Option<File>>>, e: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    if { let __err_holder = e.clone(); let __err_guard = __err_holder.lock().unwrap(); let __matched = __err_guard.as_ref().and_then(|__e| __e.downcast_ref::<syscall::syscall_unix::Errno>()).map(|__e| *__e.0.lock().unwrap().as_ref().unwrap() == (syscall::E_P_I_P_E as usize)).unwrap_or(false); __matched } && (*(*(*file.lock().unwrap().as_ref().unwrap()).file.lock().unwrap().as_ref().unwrap()).stdout_or_err.lock().unwrap().as_ref().unwrap()) {
        sigpipe();
    }
}

/// openFileNolog is the Unix implementation of OpenFile.
/// Changes here should be reflected in openDirAt and openDirNolog, if relevant.
pub fn open_file_nolog(name: Arc<Mutex<Option<String>>>, flag: Arc<Mutex<Option<i32>>>, perm: FileMode) -> (Arc<Mutex<Option<crate::types::File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut setSticky = Arc::new(Mutex::new(Some(false)));
    if !SUPPORTS_CREATE_WITH_STICKY_BIT && { let __tmp_x = { let __tmp_x = { let __v = (*flag.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 512; __tmp_x & __tmp_y }; let __tmp_y = 0; __tmp_x != __tmp_y } && { let __tmp_x = { let __tmp_x = (*perm.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(MODE_STICKY as u32)))); __tmp_x & __tmp_y }; let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        {
        let (_, mut err) = stat(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));;
        if is_not_exist(err.clone()) {
            { let new_val = true; *setSticky.lock().unwrap() = Some(new_val); };;
        }
    }
    }

    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut s: Arc<Mutex<Option<internal_poll::fd_unixjs::SysFile>>> = Arc::new(Mutex::new(Some(Default::default())));let mut e: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        // We have to check EINTR here, per issues 11180 and 39237.
    let mut e_closure_clone = e.clone(); let flag_closure_clone = flag.clone(); let name_closure_clone = name.clone(); let perm_closure_clone = perm.clone(); let mut r_closure_clone = r.clone(); let mut s_closure_clone = s.clone(); ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let (__tmp_0, __tmp_1, __tmp_2) = open_1(Arc::new(Mutex::new(Some({ let __arg_holder = name_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __tmp_x = { let __v = (*flag_closure_clone.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = syscall::O__C_L_O_E_X_E_C; __tmp_x | __tmp_y }))), Arc::new(Mutex::new(Some(syscall_mode(Arc::new(Mutex::new(Some({ let __arg_holder = perm_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() })))))))); *r_closure_clone.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s_closure_clone.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *e_closure_clone.lock().unwrap() = __moved_tmp_2; };
        return e_closure_clone.clone();
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
    if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("open".to_string()))), path: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: e.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }

        // open(2) itself won't handle the sticky bit on *BSD and Solaris
    if { let __v = (*setSticky.lock().unwrap().as_ref().unwrap()).clone(); __v } {
        set_sticky_bit(Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

        // There's a race here with fork/exec, which we are
        // content to live with. See ../syscall/exec_unix.go.
    if !SUPPORTS_CLOSE_ON_EXEC {
        syscall::close_on_exec(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    let mut f = new_file_1(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(newFileKind(Arc::new(Mutex::new(Some(KIND_OPEN_FILE as i32))))))), Arc::new(Mutex::new(Some(internal_syscall_unix::has_nonblock_flag(Arc::new(Mutex::new(Some({ let __arg_holder = flag.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))))))));
    { let new_val = s.lock().unwrap().as_ref().unwrap().clone(); *(*(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).sys_file.lock().unwrap() = Some(new_val); };
    return (f.clone(), Arc::new(Mutex::new(None)));
}

pub fn open_dir_nolog(name: Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<crate::types::File>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut r: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(0)));let mut s: Arc<Mutex<Option<internal_poll::fd_unixjs::SysFile>>> = Arc::new(Mutex::new(Some(Default::default())));let mut e: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));
    let mut e_closure_clone = e.clone(); let name_closure_clone = name.clone(); let mut r_closure_clone = r.clone(); let mut s_closure_clone = s.clone(); ignoring_e_i_n_t_r(Arc::new(Mutex::new(Some(Box::new(move || -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        { let (__tmp_0, __tmp_1, __tmp_2) = open_1(Arc::new(Mutex::new(Some({ let __arg_holder = name_closure_clone.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(17825792))), Arc::new(Mutex::new(Some(0 as u32)))); *r_closure_clone.lock().unwrap() = Some(__tmp_0); let __moved_tmp_1 = { let mut __guard = __tmp_1.lock().unwrap(); __guard.take() }; *s_closure_clone.lock().unwrap() = __moved_tmp_1; let __moved_tmp_2 = { let mut __guard = __tmp_2.lock().unwrap(); __guard.take() }; *e_closure_clone.lock().unwrap() = __moved_tmp_2; };
        return e_closure_clone.clone();
    }) as Box<dyn FnMut() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> + Send + Sync>))));
    if { let __nil_result = (*e.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), Arc::new(Mutex::new(Some(Box::new(io_fs::r#mod::PathError { op: Arc::new(Mutex::new(Some("open".to_string()))), path: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), err: e.clone(), ..Default::default() }) as Box<dyn StdError + Send + Sync>))));
    }

    if !SUPPORTS_CLOSE_ON_EXEC {
        syscall::close_on_exec(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))));
    }

    let mut f = new_file_1(Arc::new(Mutex::new(Some({ let __arg_holder = r.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(newFileKind(Arc::new(Mutex::new(Some(KIND_NO_POLL as i32))))))), Arc::new(Mutex::new(Some(false))));
    { let new_val = s.lock().unwrap().as_ref().unwrap().clone(); *(*(*(*f.lock().unwrap().as_mut().unwrap()).file.lock().unwrap().as_mut().unwrap()).pfd.lock().unwrap().as_ref().unwrap()).sys_file.lock().unwrap() = Some(new_val); };
    return (f.clone(), Arc::new(Mutex::new(None)));
}

pub fn new_unix_dirent(parent: Arc<Mutex<Option<String>>>, name: Arc<Mutex<Option<String>>>, typ: FileMode) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut ude = Arc::new(Mutex::new(Some(unixDirent { parent: Arc::new(Mutex::new(Some({ let __arg_holder = parent.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), name: Arc::new(Mutex::new(Some({ let __arg_holder = name.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), typ: Arc::new(Mutex::new(Some({ let __arg_holder = typ.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), ..Default::default() })));
    if { let __tmp_x = (*typ.lock().unwrap().as_ref().unwrap()).clone(); let __tmp_y = io_fs::r#mod::FileMode(Arc::new(Mutex::new(Some(!0 as u32)))); __tmp_x != __tmp_y } && !(*testingForceReadDirLstat.lock().unwrap().as_ref().unwrap()) {
        return (Arc::new(Mutex::new(Some(Box::new(unixDirentPtr(ude.clone())) as Box<dyn io_fs::r#mod::DirEntry + Send + Sync>))), Arc::new(Mutex::new(None)));
    }

    let (mut info, mut err) = { let __f_ptr: *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> = { let mut __f_guard = lstat_1.lock().unwrap(); __f_guard.as_mut().unwrap() as *mut Box<dyn FnMut(Arc<Mutex<Option<String>>>) -> (Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) + Send + Sync> }; let __f = unsafe { &mut *__f_ptr }; (*__f)(Arc::new(Mutex::new(Some({ let mut __s = String::new(); __s.push_str(&format!("{}", { let __v = (*parent.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s.push_str(&format!("{}", "/".to_string())); __s.push_str(&format!("{}", { let __v = (*name.lock().unwrap().as_ref().unwrap()).clone(); __v })); __s })))) };
    if { let __nil_result = (*err.lock().unwrap()).is_some(); __nil_result } {
        return (Arc::new(Mutex::new(None)), err.clone());
    }

    { let new_val = io_fs::r#mod::FileMode::r#type(&(*(*info.lock().unwrap().as_ref().unwrap()).mode().lock().unwrap().as_ref().unwrap())); let __moved_val = { let mut __guard = new_val.lock().unwrap(); __guard.take() }; *(*ude.lock().unwrap().as_ref().unwrap()).typ.lock().unwrap() = __moved_val; };
    { let __iface_handle = info.clone(); let __iface_value = { let __iface_guard = __iface_handle.lock().unwrap(); (*__iface_guard).clone() }; *(*ude.lock().unwrap().as_mut().unwrap()).info.lock().unwrap() = __iface_value; };
    return (Arc::new(Mutex::new(Some(Box::new(unixDirentPtr(ude.clone())) as Box<dyn io_fs::r#mod::DirEntry + Send + Sync>))), Arc::new(Mutex::new(None)));
}

impl GoValueClone for file {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}


impl GoValueClone for unixDirent {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
