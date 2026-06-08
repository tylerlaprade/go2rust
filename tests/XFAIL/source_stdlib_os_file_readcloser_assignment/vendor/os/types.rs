use go2rust_stdlib_stubs::*;

use crate::{GoArrayElemMutRef, GoArrayElemPtr, GoArrayElemRef, GoPtr, GoSliceElemMutRef, GoSliceElemPtr, GoSliceElemRef};

use crate::{file_unix::{file}, types_unix::{fileStat, same_file_1}};

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const MODE_DIR: u32 = io_fs::MODE_DIR;
pub const MODE_APPEND: u32 = io_fs::MODE_APPEND;
pub const MODE_EXCLUSIVE: u32 = io_fs::MODE_EXCLUSIVE;
pub const MODE_TEMPORARY: u32 = io_fs::MODE_TEMPORARY;
pub const MODE_SYMLINK: u32 = io_fs::MODE_SYMLINK;
pub const MODE_DEVICE: u32 = io_fs::MODE_DEVICE;
pub const MODE_NAMED_PIPE: u32 = io_fs::MODE_NAMED_PIPE;
pub const MODE_SOCKET: u32 = io_fs::MODE_SOCKET;
pub const MODE_SETUID: u32 = io_fs::MODE_SETUID;
pub const MODE_SETGID: u32 = io_fs::MODE_SETGID;
pub const MODE_CHAR_DEVICE: u32 = io_fs::MODE_CHAR_DEVICE;
pub const MODE_STICKY: u32 = io_fs::MODE_STICKY;
pub const MODE_IRREGULAR: u32 = io_fs::MODE_IRREGULAR;
pub const MODE_TYPE: u32 = io_fs::MODE_TYPE;
pub const MODE_PERM: u32 = io_fs::MODE_PERM;


/// File represents an open file descriptor.
///
/// The methods of File are safe for concurrent use.
#[derive(Clone, Default)]
pub struct File {
    pub file: Arc<Mutex<Option<file>>>,
}

impl File {
    pub fn __go_value_clone(&self) -> Self {
        Self { file: self.file.clone() }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{{{}}}", { let __guard = self.file.lock().unwrap(); match __guard.as_ref() { Some(__v) => format!("{:p}", __v as *const _), None => "<nil>".to_string() } })
    }
}

impl GoJsonDecode for File {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        Ok(out)
    }
}


/// A FileInfo describes a file and is returned by [Stat] and [Lstat].
pub type FileInfo = Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>;


/// A FileMode represents a file's mode and permission bits.
/// The bits have the same definition on all systems, so that
/// information about files can be moved from one system
/// to another portably. Not all bits apply to all systems.
/// The only required bit is [ModeDir] for directories.
pub type FileMode = Arc<Mutex<Option<io_fs::r#mod::FileMode>>>;


impl crate::types_unix::fileStat {
    pub fn name(&self) -> Arc<Mutex<Option<String>>> {
        return self.name.clone();
    }

    pub fn is_dir(&self) -> bool {
        io_fs::r#mod::FileMode::is_dir(&(*self.mode().lock().unwrap().as_ref().unwrap()))
    }
}

impl File {
}

#[derive(Clone)]
pub struct FilePtr(pub Arc<Mutex<Option<File>>>);

impl std::fmt::Display for FilePtr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let __guard = self.0.lock().unwrap();
        match __guard.as_ref() { Some(__v) => write!(f, "{:p}", __v as *const _), None => write!(f, "<nil>") }
    }
}

impl io::r#mod::Closer for FilePtr {
    fn close(&mut self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        File::close(__recv)
    }
    fn __go_clone_box_closer(&self) -> Box<dyn io::r#mod::Closer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Closer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_closer(&self, other: &(dyn io::r#mod::Closer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::ReadCloser for FilePtr {
    fn __go_clone_box_read_closer(&self) -> Box<dyn io::r#mod::ReadCloser + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::ReadCloser + Send + Sync>
    }
    fn __go_eq_read_closer(&self, other: &(dyn io::r#mod::ReadCloser + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::Reader for FilePtr {
    fn read(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        File::read(__recv, p)
    }
    fn __go_clone_box_reader(&self) -> Box<dyn io::r#mod::Reader + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Reader + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader(&self, other: &(dyn io::r#mod::Reader + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::ReaderFrom for FilePtr {
    fn read_from(&self, r: Arc<Mutex<Option<Box<dyn io::r#mod::Reader + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        File::read_from(__recv, r)
    }
    fn __go_clone_box_reader_from(&self) -> Box<dyn io::r#mod::ReaderFrom + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::ReaderFrom + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_reader_from(&self, other: &(dyn io::r#mod::ReaderFrom + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::StringWriter for FilePtr {
    fn write_string(&self, s: Arc<Mutex<Option<String>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_ref().unwrap();
        File::write_string(__recv, s)
    }
    fn __go_clone_box_string_writer(&self) -> Box<dyn io::r#mod::StringWriter + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::StringWriter + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_string_writer(&self, other: &(dyn io::r#mod::StringWriter + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::Writer for FilePtr {
    fn write(&mut self, p: Arc<Mutex<Option<Vec<u8>>>>) -> (i32, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        File::write(__recv, p)
    }
    fn __go_clone_box_writer(&self) -> Box<dyn io::r#mod::Writer + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::Writer + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer(&self, other: &(dyn io::r#mod::Writer + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

impl io::r#mod::WriterTo for FilePtr {
    fn write_to(&mut self, w: Arc<Mutex<Option<Box<dyn io::r#mod::Writer + Send + Sync>>>>) -> (i64, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>) {
        let mut __recv_guard = self.0.lock().unwrap();
        let __recv = __recv_guard.as_mut().unwrap();
        File::write_to(__recv, w)
    }
    fn __go_clone_box_writer_to(&self) -> Box<dyn io::r#mod::WriterTo + Send + Sync> {
        Box::new(self.clone()) as Box<dyn io::r#mod::WriterTo + Send + Sync>
    }
    fn __go_as_any(&self) -> &dyn Any {
        self
    }
    fn __go_eq_writer_to(&self, other: &(dyn io::r#mod::WriterTo + Send + Sync)) -> bool {
        if let Some(__other) = other.__go_as_any().downcast_ref::<FilePtr>() {
            Arc::ptr_eq(&self.0, &__other.0)
        } else {
            false
        }
    }
}

/// SameFile reports whether fi1 and fi2 describe the same file.
/// For example, on Unix this means that the device and inode fields
/// of the two underlying structures are identical; on other systems
/// the decision may be based on the path names.
/// SameFile only applies to results returned by this package's [Stat].
/// It returns false in other cases.
pub fn same_file(fi1: Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>, fi2: Arc<Mutex<Option<Box<dyn io_fs::r#mod::FileInfo + Send + Sync>>>>) -> bool {
    let (mut fs1, mut ok1) = ({
        let val = fi1.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn io_fs::r#mod::FileInfo + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<Arc<Mutex<Option<fileStat>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<fileStat>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<fileStat>)), false)
        }
    });
    let (mut fs2, mut ok2) = ({
        let val = fi2.clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = <dyn io_fs::r#mod::FileInfo + Send + Sync>::__go_as_any(any_val.as_ref()).downcast_ref::<Arc<Mutex<Option<fileStat>>>>() {
                (typed_val.clone(), true)
            } else {
                (Arc::new(Mutex::new(None::<fileStat>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<fileStat>)), false)
        }
    });
    if !ok1 || !ok2 {
        return false;
    }
    return same_file_1(fs1.clone(), fs2.clone());
}

impl GoValueClone for File {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
