use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

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
use crate::file_unix::*;
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

use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub(crate) const READDIR_NAME: i32 = 0;
pub(crate) const READDIR_DIR_ENTRY: i32 = 1;
pub(crate) const READDIR_FILE_INFO: i32 = 2;


#[derive(Debug, Clone, Default)]
pub struct readdirMode(pub Arc<Mutex<Option<i32>>>);

impl Display for readdirMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0.lock().unwrap().as_ref().unwrap())
    }
}

impl PartialEq for readdirMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<i32> for readdirMode {
    fn eq(&self, other: &i32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for readdirMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<i32> for readdirMode {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<readdirMode> for i32 {
    fn eq(&self, other: &readdirMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<readdirMode> for i32 {
    fn partial_cmp(&self, other: &readdirMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for readdirMode {
    type Output = readdirMode;
    fn add(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<i32> for readdirMode {
    type Output = readdirMode;
    fn add(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<readdirMode> for i32 {
    type Output = readdirMode;
    fn add(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for readdirMode {
    type Output = readdirMode;
    fn sub(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<i32> for readdirMode {
    type Output = readdirMode;
    fn sub(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<readdirMode> for i32 {
    type Output = readdirMode;
    fn sub(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for readdirMode {
    type Output = readdirMode;
    fn mul(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<i32> for readdirMode {
    type Output = readdirMode;
    fn mul(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<readdirMode> for i32 {
    type Output = readdirMode;
    fn mul(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for readdirMode {
    type Output = readdirMode;
    fn div(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<i32> for readdirMode {
    type Output = readdirMode;
    fn div(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<readdirMode> for i32 {
    type Output = readdirMode;
    fn div(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Neg for readdirMode {
    type Output = readdirMode;
    fn neg(self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(-*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for readdirMode {
    type Output = readdirMode;
    fn rem(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<i32> for readdirMode {
    type Output = readdirMode;
    fn rem(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<readdirMode> for i32 {
    type Output = readdirMode;
    fn rem(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for readdirMode {
    type Output = readdirMode;
    fn bitand(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<i32> for readdirMode {
    type Output = readdirMode;
    fn bitand(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<readdirMode> for i32 {
    type Output = readdirMode;
    fn bitand(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for readdirMode {
    type Output = readdirMode;
    fn bitor(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<i32> for readdirMode {
    type Output = readdirMode;
    fn bitor(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<readdirMode> for i32 {
    type Output = readdirMode;
    fn bitor(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for readdirMode {
    type Output = readdirMode;
    fn bitxor(self, other: Self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<i32> for readdirMode {
    type Output = readdirMode;
    fn bitxor(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<readdirMode> for i32 {
    type Output = readdirMode;
    fn bitxor(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for readdirMode {
    type Output = readdirMode;
    fn not(self) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: i8) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: i16) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: i64) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: u32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: u8) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: u16) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: u64) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for readdirMode {
    type Output = readdirMode;
    fn shl(self, other: usize) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: readdirMode) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: i32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: i8) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: i16) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: i64) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: u32) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: u8) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: u16) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: u64) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for readdirMode {
    type Output = readdirMode;
    fn shr(self, other: usize) -> readdirMode {
        readdirMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for readdirMode {}

impl Ord for readdirMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// A DirEntry is an entry read from a directory
/// (using the [ReadDir] function or a [File.ReadDir] method).
pub type DirEntry = Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>;


pub(crate) static testingForceReadDirLstat: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<bool>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *testingForceReadDirLstat.lock().unwrap() = Some(false);
}


pub(crate) fn __go_zero_globals() {
    *testingForceReadDirLstat.lock().unwrap() = Some(false);
}


impl crate::types::File {
    /// Readdir reads the contents of the directory associated with file and
    /// returns a slice of up to n [FileInfo] values, as would be returned
    /// by [Lstat], in directory order. Subsequent calls on the same file will yield
    /// further FileInfos.
    ///
    /// If n > 0, Readdir returns at most n FileInfo structures. In this case, if
    /// Readdir returns an empty slice, it will return a non-nil error
    /// explaining why. At the end of a directory, the error is [io.EOF].
    ///
    /// If n <= 0, Readdir returns all the FileInfo from the directory in
    /// a single slice. In this case, if Readdir succeeds (reads all
    /// the way to the end of the directory), it returns the slice and a
    /// nil error. If it encounters an error before the end of the
    /// directory, Readdir returns the FileInfo read until that point
    /// and a non-nil error.
    ///
    /// Most clients are better served by the more efficient ReadDir method.
    pub fn readdir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), ErrInvalid.clone());
    }
        let (_, _, mut infos, mut err) = self.readdir_1(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(readdirMode(Arc::new(Mutex::new(Some(READDIR_FILE_INFO as i32))))))));
        if { let __nil_result = (*infos.lock().unwrap()).is_none(); __nil_result } {
                // Readdir has historically always returned a non-nil empty slice, never nil,
                // even on error (except misuse with nil receiver above).
                // Keep it that way to avoid breaking overly sensitive callers.
        { let new_val = Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>>::new()))); infos = new_val; };
    }
                // Readdir has historically always returned a non-nil empty slice, never nil,
                // even on error (except misuse with nil receiver above).
                // Keep it that way to avoid breaking overly sensitive callers.
        return (infos.clone(), err.clone());
    }

    /// Readdirnames reads the contents of the directory associated with file
    /// and returns a slice of up to n names of files in the directory,
    /// in directory order. Subsequent calls on the same file will yield
    /// further names.
    ///
    /// If n > 0, Readdirnames returns at most n names. In this case, if
    /// Readdirnames returns an empty slice, it will return a non-nil error
    /// explaining why. At the end of a directory, the error is [io.EOF].
    ///
    /// If n <= 0, Readdirnames returns all the names from the directory in
    /// a single slice. In this case, if Readdirnames succeeds (reads all
    /// the way to the end of the directory), it returns the slice and a
    /// nil error. If it encounters an error before the end of the
    /// directory, Readdirnames returns the names read until that point and
    /// a non-nil error.
    pub fn readdirnames(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<String>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
    let mut names: Arc<Mutex<Option<Vec<String>>>> = Arc::new(Mutex::new(None));
    let mut err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> = Arc::new(Mutex::new(None));

        if false {
        return (Arc::new(Mutex::new(None)), ErrInvalid.clone());
    }
        { let (__tmp_0, __tmp_1, __tmp_2, __tmp_3) = self.readdir_1(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(readdirMode(Arc::new(Mutex::new(Some(READDIR_NAME as i32)))))))); let __moved_tmp_0 = { let mut __guard = __tmp_0.lock().unwrap(); __guard.take() }; *names.lock().unwrap() = __moved_tmp_0; let __moved_tmp_3 = { let mut __guard = __tmp_3.lock().unwrap(); __guard.take() }; *err.lock().unwrap() = __moved_tmp_3; };
        if { let __nil_result = (*names.lock().unwrap()).is_none(); __nil_result } {
                // Readdirnames has historically always returned a non-nil empty slice, never nil,
                // even on error (except misuse with nil receiver above).
                // Keep it that way to avoid breaking overly sensitive callers.
        { let new_val = Arc::new(Mutex::new(Some(Vec::<String>::new()))); names = new_val; };
    }
                // Readdirnames has historically always returned a non-nil empty slice, never nil,
                // even on error (except misuse with nil receiver above).
                // Keep it that way to avoid breaking overly sensitive callers.
        return (names.clone(), err.clone());
    }

    /// ReadDir reads the contents of the directory associated with the file f
    /// and returns a slice of [DirEntry] values in directory order.
    /// Subsequent calls on the same file will yield later DirEntry records in the directory.
    ///
    /// If n > 0, ReadDir returns at most n DirEntry records.
    /// In this case, if ReadDir returns an empty slice, it will return an error explaining why.
    /// At the end of a directory, the error is [io.EOF].
    ///
    /// If n <= 0, ReadDir returns all the DirEntry records remaining in the directory.
    /// When it succeeds, it returns a nil error (not io.EOF).
    pub fn read_dir(&self, n: Arc<Mutex<Option<i32>>>) -> (Arc<Mutex<Option<Vec<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        if false {
        return (Arc::new(Mutex::new(None)), ErrInvalid.clone());
    }
        let (_, mut dirents, _, mut err) = self.readdir_1(Arc::new(Mutex::new(Some({ let __arg_holder = n.clone(); let __arg_guard = __arg_holder.lock().unwrap(); (*__arg_guard.as_ref().unwrap()).clone() }))), Arc::new(Mutex::new(Some(readdirMode(Arc::new(Mutex::new(Some(READDIR_DIR_ENTRY as i32))))))));
        if { let __nil_result = (*dirents.lock().unwrap()).is_none(); __nil_result } {
                // Match Readdir and Readdirnames: don't return nil slices.
        { let new_val = Arc::new(Mutex::new(Some(Vec::<Arc<Mutex<Option<Box<dyn io_fs::r#mod::DirEntry + Send + Sync>>>>>::new()))); dirents = new_val; };
    }
                // Match Readdir and Readdirnames: don't return nil slices.
        return (dirents.clone(), err.clone());
    }
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}
