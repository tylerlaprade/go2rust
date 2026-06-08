use go2rust_stdlib_stubs::*;

use std::any::Any;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, Mutex};

pub const MODE_DIR: u32 = 1 << (32 - 1 - 0);
pub const MODE_APPEND: u32 = 1 << (32 - 1 - 1);
pub const MODE_EXCLUSIVE: u32 = 1 << (32 - 1 - 2);
pub const MODE_TEMPORARY: u32 = 1 << (32 - 1 - 3);
pub const MODE_SYMLINK: u32 = 1 << (32 - 1 - 4);
pub const MODE_DEVICE: u32 = 1 << (32 - 1 - 5);
pub const MODE_NAMED_PIPE: u32 = 1 << (32 - 1 - 6);
pub const MODE_SOCKET: u32 = 1 << (32 - 1 - 7);
pub const MODE_SETUID: u32 = 1 << (32 - 1 - 8);
pub const MODE_SETGID: u32 = 1 << (32 - 1 - 9);
pub const MODE_CHAR_DEVICE: u32 = 1 << (32 - 1 - 10);
pub const MODE_STICKY: u32 = 1 << (32 - 1 - 11);
pub const MODE_IRREGULAR: u32 = 1 << (32 - 1 - 12);
pub const MODE_TYPE: u32 = MODE_DIR as u32 | MODE_SYMLINK as u32 as u32 | MODE_NAMED_PIPE as u32 as u32 | MODE_SOCKET as u32 as u32 | MODE_DEVICE as u32 as u32 | MODE_CHAR_DEVICE as u32 as u32 | MODE_IRREGULAR as u32;
pub const MODE_PERM: u32 = 0777;


/// A DirEntry is an entry read from a directory
/// (using the [ReadDir] function or a [ReadDirFile]'s ReadDir method).
pub trait DirEntry: std::fmt::Display + Any {
    fn __go_clone_box_dir_entry(&self) -> Box<dyn DirEntry + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_dir_entry(&self, other: &(dyn DirEntry + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn is_dir(&self) -> bool;
    fn r#type(&self) -> Arc<Mutex<Option<FileMode>>>;
    fn info(&self) -> (Arc<Mutex<Option<Box<dyn FileInfo + Send + Sync>>>>, Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>);
}

impl Clone for Box<dyn DirEntry + Send + Sync> {
    fn clone(&self) -> Self {
        DirEntry::__go_clone_box_dir_entry(self.as_ref())
    }
}

impl GoValueClone for Box<dyn DirEntry + Send + Sync> {
    fn go_value_clone(&self) -> Self {
        DirEntry::__go_clone_box_dir_entry(self.as_ref())
    }
}

/// A FileInfo describes a file and is returned by [Stat].
pub trait FileInfo: std::fmt::Display + Any {
    fn __go_clone_box_file_info(&self) -> Box<dyn FileInfo + Send + Sync>;
    fn __go_as_any(&self) -> &dyn Any;
    fn __go_eq_file_info(&self, other: &(dyn FileInfo + Send + Sync)) -> bool;
    fn name(&self) -> Arc<Mutex<Option<String>>>;
    fn size(&self) -> i64;
    fn mode(&self) -> Arc<Mutex<Option<FileMode>>>;
    fn mod_time(&self) -> Arc<Mutex<Option<time::r#mod::Time>>>;
    fn is_dir(&self) -> bool;
    fn sys(&self) -> Arc<Mutex<Option<Box<dyn Any + Send + Sync>>>>;
}

impl Clone for Box<dyn FileInfo + Send + Sync> {
    fn clone(&self) -> Self {
        FileInfo::__go_clone_box_file_info(self.as_ref())
    }
}

/// A FileMode represents a file's mode and permission bits.
/// The bits have the same definition on all systems, so that
/// information about files can be moved from one system
/// to another portably. Not all bits apply to all systems.
/// The only required bit is [ModeDir] for directories.
#[derive(Debug, Clone, Default)]
pub struct FileMode(pub Arc<Mutex<Option<u32>>>);

impl Display for FileMode {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.string().lock().unwrap().as_ref().unwrap()))
    }
}

impl PartialEq for FileMode {
    fn eq(&self, other: &Self) -> bool {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left == __right
    }
}

impl PartialEq<u32> for FileMode {
    fn eq(&self, other: &u32) -> bool {
        *self.0.lock().unwrap().as_ref().unwrap() == *other
    }
}

impl PartialOrd for FileMode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.partial_cmp(&__right)
    }
}

impl PartialOrd<u32> for FileMode {
    fn partial_cmp(&self, other: &u32) -> Option<std::cmp::Ordering> {
        self.0.lock().unwrap().as_ref().unwrap().partial_cmp(other)
    }
}

impl PartialEq<FileMode> for u32 {
    fn eq(&self, other: &FileMode) -> bool {
        *self == *other.0.lock().unwrap().as_ref().unwrap()
    }
}

impl PartialOrd<FileMode> for u32 {
    fn partial_cmp(&self, other: &FileMode) -> Option<std::cmp::Ordering> {
        self.partial_cmp(other.0.lock().unwrap().as_ref().unwrap())
    }
}

impl std::ops::Add for FileMode {
    type Output = FileMode;
    fn add(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Add<u32> for FileMode {
    type Output = FileMode;
    fn add(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() + other))))
    }
}

impl std::ops::Add<FileMode> for u32 {
    type Output = FileMode;
    fn add(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self + *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub for FileMode {
    type Output = FileMode;
    fn sub(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Sub<u32> for FileMode {
    type Output = FileMode;
    fn sub(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() - other))))
    }
}

impl std::ops::Sub<FileMode> for u32 {
    type Output = FileMode;
    fn sub(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self - *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul for FileMode {
    type Output = FileMode;
    fn mul(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Mul<u32> for FileMode {
    type Output = FileMode;
    fn mul(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() * other))))
    }
}

impl std::ops::Mul<FileMode> for u32 {
    type Output = FileMode;
    fn mul(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self * *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div for FileMode {
    type Output = FileMode;
    fn div(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Div<u32> for FileMode {
    type Output = FileMode;
    fn div(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() / other))))
    }
}

impl std::ops::Div<FileMode> for u32 {
    type Output = FileMode;
    fn div(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self / *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem for FileMode {
    type Output = FileMode;
    fn rem(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Rem<u32> for FileMode {
    type Output = FileMode;
    fn rem(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() % other))))
    }
}

impl std::ops::Rem<FileMode> for u32 {
    type Output = FileMode;
    fn rem(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self % *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd for FileMode {
    type Output = FileMode;
    fn bitand(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitAnd<u32> for FileMode {
    type Output = FileMode;
    fn bitand(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() & other))))
    }
}

impl std::ops::BitAnd<FileMode> for u32 {
    type Output = FileMode;
    fn bitand(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self & *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr for FileMode {
    type Output = FileMode;
    fn bitor(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitOr<u32> for FileMode {
    type Output = FileMode;
    fn bitor(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() | other))))
    }
}

impl std::ops::BitOr<FileMode> for u32 {
    type Output = FileMode;
    fn bitor(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self | *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor for FileMode {
    type Output = FileMode;
    fn bitxor(self, other: Self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::BitXor<u32> for FileMode {
    type Output = FileMode;
    fn bitxor(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() ^ other))))
    }
}

impl std::ops::BitXor<FileMode> for u32 {
    type Output = FileMode;
    fn bitxor(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(self ^ *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Not for FileMode {
    type Output = FileMode;
    fn not(self) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(!*self.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl for FileMode {
    type Output = FileMode;
    fn shl(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shl<i32> for FileMode {
    type Output = FileMode;
    fn shl(self, other: i32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i8> for FileMode {
    type Output = FileMode;
    fn shl(self, other: i8) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i16> for FileMode {
    type Output = FileMode;
    fn shl(self, other: i16) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<i64> for FileMode {
    type Output = FileMode;
    fn shl(self, other: i64) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u32> for FileMode {
    type Output = FileMode;
    fn shl(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u8> for FileMode {
    type Output = FileMode;
    fn shl(self, other: u8) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u16> for FileMode {
    type Output = FileMode;
    fn shl(self, other: u16) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<u64> for FileMode {
    type Output = FileMode;
    fn shl(self, other: u64) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shl<usize> for FileMode {
    type Output = FileMode;
    fn shl(self, other: usize) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() << other))))
    }
}

impl std::ops::Shr for FileMode {
    type Output = FileMode;
    fn shr(self, other: FileMode) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> *other.0.lock().unwrap().as_ref().unwrap()))))
    }
}

impl std::ops::Shr<i32> for FileMode {
    type Output = FileMode;
    fn shr(self, other: i32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i8> for FileMode {
    type Output = FileMode;
    fn shr(self, other: i8) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i16> for FileMode {
    type Output = FileMode;
    fn shr(self, other: i16) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<i64> for FileMode {
    type Output = FileMode;
    fn shr(self, other: i64) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u32> for FileMode {
    type Output = FileMode;
    fn shr(self, other: u32) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u8> for FileMode {
    type Output = FileMode;
    fn shr(self, other: u8) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u16> for FileMode {
    type Output = FileMode;
    fn shr(self, other: u16) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<u64> for FileMode {
    type Output = FileMode;
    fn shr(self, other: u64) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl std::ops::Shr<usize> for FileMode {
    type Output = FileMode;
    fn shr(self, other: usize) -> FileMode {
        FileMode(Arc::new(Mutex::new(Some(*self.0.lock().unwrap().as_ref().unwrap() >> other))))
    }
}

impl Eq for FileMode {}

impl Ord for FileMode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let __left = { self.0.lock().unwrap().as_ref().cloned() };
        let __right = { other.0.lock().unwrap().as_ref().cloned() };
        __left.cmp(&__right)
    }
}


/// PathError records an error and the operation and file path that caused it.
#[derive(Clone)]
pub struct PathError {
    pub op: Arc<Mutex<Option<String>>>,
    pub path: Arc<Mutex<Option<String>>>,
    pub err: Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>>,
}

impl PathError {
    pub fn __go_value_clone(&self) -> Self {
        let __go_clone_0_0 = { let __guard = self.op.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_1_0 = { let __guard = self.path.lock().unwrap(); Arc::new(Mutex::new((*__guard).clone())) };
        let __go_clone_2_0 = self.err.clone();
        Self {
            op: __go_clone_0_0,
            path: __go_clone_1_0,
            err: __go_clone_2_0,
        }
    }
}


impl Default for PathError {
    fn default() -> Self {
        let __go_default_0_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_1_0 = Arc::new(Mutex::new(Some(String::new())));
        let __go_default_2_0 = Arc::new(Mutex::new(None));
        Self {
            op: __go_default_0_0,
            path: __go_default_1_0,
            err: __go_default_2_0,
        }
    }
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", (*self.error().lock().unwrap().as_ref().unwrap()))
    }
}
impl std::fmt::Debug for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl GoJsonDecode for PathError {
    fn go_json_decode(value: &serde_json::Value) -> Result<Self, String> {
        let object = value.as_object().ok_or_else(|| go_json_expected(value, "object"))?;
        let mut out = Self::default();
        if let Some(field_value) = object.get("Op") {
            out.op = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        if let Some(field_value) = object.get("Path") {
            out.path = <Arc<Mutex<Option<String>>> as GoJsonDecode>::go_json_decode(field_value)?;
        }
        Ok(out)
    }
}


pub static ErrInvalid: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrPermission: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrNotExist: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));

pub static ErrClosed: std::sync::LazyLock<std::sync::Arc<std::sync::Mutex<Option<Box<dyn StdError + Send + Sync>>>>> = std::sync::LazyLock::new(|| std::sync::Arc::new(std::sync::Mutex::new(None)));


fn __go_init_globals() {
    *ErrInvalid.lock().unwrap() = None;
    *ErrPermission.lock().unwrap() = None;
    *ErrExist.lock().unwrap() = None;
    *ErrNotExist.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
    { let __rhs_holder = err_invalid().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrInvalid.lock().unwrap() = new_val; }
    { let __rhs_holder = err_permission().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrPermission.lock().unwrap() = new_val; }
    { let __rhs_holder = err_exist().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrExist.lock().unwrap() = new_val; }
    { let __rhs_holder = err_not_exist().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotExist.lock().unwrap() = new_val; }
    { let __rhs_holder = err_closed().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosed.lock().unwrap() = new_val; }
}


pub(crate) fn __go_zero_globals() {
    *ErrInvalid.lock().unwrap() = None;
    *ErrPermission.lock().unwrap() = None;
    *ErrExist.lock().unwrap() = None;
    *ErrNotExist.lock().unwrap() = None;
    *ErrClosed.lock().unwrap() = None;
}


pub(crate) fn __go_init_order_0() {
    { let __rhs_holder = err_invalid().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrInvalid.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_1() {
    { let __rhs_holder = err_permission().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrPermission.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_2() {
    { let __rhs_holder = err_exist().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrExist.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_3() {
    { let __rhs_holder = err_not_exist().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrNotExist.lock().unwrap() = new_val; }
}


pub(crate) fn __go_init_order_4() {
    { let __rhs_holder = err_closed().clone(); let new_val = { let mut guard = __rhs_holder.lock().unwrap(); guard.take() }; *ErrClosed.lock().unwrap() = new_val; }
}


impl FileMode {
    pub fn string(&self) -> Arc<Mutex<Option<String>>> {
        const str: &'static str = "dalTLDpSugct?";

        let mut buf: Arc<Mutex<Option<[u8; 32]>>> = Arc::new(Mutex::new(Some(std::array::from_fn(|_| 0))));
        let mut w = Arc::new(Mutex::new(Some(0)));
        for (i, c) in str.char_indices() {
        if { let __tmp_x = FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & ((1 << Arc::new(Mutex::new(Some(({ let __tmp_x = 31; let __tmp_y = i as i32; __tmp_x - __tmp_y }) as u64)))))))))); let __tmp_y = FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((c as i32) as u8))).lock().unwrap().as_ref().unwrap()).clone();
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
    }
        if { let __tmp_x = { let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }; let __tmp_y = 0; __tmp_x == __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('-' as i32) as u8;
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        const rwx: &'static str = "rwxrwxrwx";

        for (i, c) in rwx.char_indices() {
        if { let __tmp_x = FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & ((1 << Arc::new(Mutex::new(Some(({ let __tmp_x = 8; let __tmp_y = i as i32; __tmp_x - __tmp_y }) as u64)))))))))); let __tmp_y = FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y } {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = (*Arc::new(Mutex::new(Some((c as i32) as u8))).lock().unwrap().as_ref().unwrap()).clone();
    } else {
        (*buf.lock().unwrap().as_mut().unwrap())[({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize] = ('-' as i32) as u8;
    }
        { let mut guard = w.lock().unwrap(); *guard = Some(guard.as_ref().unwrap() + 1); }
    }
        return Arc::new(Mutex::new(Some(String::from_utf8((*Arc::new(Mutex::new(Some({ let __seq_holder = buf.clone(); let __seq_guard = __seq_holder.lock().unwrap(); let __source_cap = __seq_guard.as_ref().map(|__v| __v.len()).unwrap_or(0); let mut __seq = (*__seq_guard.as_ref().unwrap()).clone(); drop(__seq_guard); let __low = 0; let __high = ({ let __v = (*w.lock().unwrap().as_ref().unwrap()).clone(); __v }) as usize; let __max = __source_cap; let _slice = &__seq[__low..__high]; let mut _v = Vec::with_capacity((__max - __low) as usize); _v.extend_from_slice(_slice); _v }))).lock().unwrap().as_ref().unwrap()).clone()).unwrap())));
    }

    /// IsDir reports whether m describes a directory.
    /// That is, it tests for the [ModeDir] bit being set in m.
    pub fn is_dir(&self) -> bool {
        return { let __tmp_x = FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & MODE_DIR as u32))))); let __tmp_y = FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x != __tmp_y };
    }

    /// IsRegular reports whether m describes a regular file.
    /// That is, it tests that no mode type bits are set.
    pub fn is_regular(&self) -> bool {
        return { let __tmp_x = FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & MODE_TYPE as u32))))); let __tmp_y = FileMode(Arc::new(Mutex::new(Some(0 as u32)))); __tmp_x == __tmp_y };
    }

    /// Perm returns the Unix permission bits in m (m & [ModePerm]).
    pub fn perm(&self) -> Arc<Mutex<Option<FileMode>>> {
        return Arc::new(Mutex::new(Some(FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & MODE_PERM as u32))))))));
    }

    /// Type returns type bits in m (m & [ModeType]).
    pub fn r#type(&self) -> Arc<Mutex<Option<FileMode>>> {
        return Arc::new(Mutex::new(Some(FileMode(Arc::new(Mutex::new(Some(((*self.0.lock().unwrap().as_ref().unwrap()) & MODE_TYPE as u32))))))));
    }
}

impl PathError {
    pub fn error(&self) -> Arc<Mutex<Option<String>>> {
        return Arc::new(Mutex::new(Some({
            let mut __s = String::new();
            __s.push_str(&format!("{}", (*self.op.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", " ".to_string()));
            __s.push_str(&format!("{}", (*self.path.clone().lock().unwrap().as_ref().unwrap())));
            __s.push_str(&format!("{}", ": ".to_string()));
            __s.push_str(&format!("{}", (*Arc::new(Mutex::new(Some(format!("{}", self.err.lock().unwrap().as_ref().unwrap())))).lock().unwrap().as_ref().unwrap())));
            __s
        })));
    }

    pub fn unwrap(&self) -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
        self.err.clone()
    }

    /// Timeout reports whether this error represents a timeout.
    pub fn timeout(&self) -> bool {
        let (mut t, mut ok) = ({
        let val = self.err.clone().clone();
        let guard = val.lock().unwrap();
        if let Some(ref any_val) = *guard {
            if let Some(typed_val) = any_val.downcast_ref::<PathError>() {
                (Arc::new(Mutex::new(Some(typed_val.clone()))), true)
            } else {
                (Arc::new(Mutex::new(None::<PathError>)), false)
            }
        } else {
            (Arc::new(Mutex::new(None::<PathError>)), false)
        }
    });
        return ok && (*t.lock().unwrap().as_ref().unwrap()).timeout();
    }
}

impl StdError for PathError {}


pub fn err_invalid() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_oserror::ErrInvalid.clone()
}

pub fn err_permission() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_oserror::ErrPermission.clone()
}

pub fn err_exist() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_oserror::ErrExist.clone()
}

pub fn err_not_exist() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_oserror::ErrNotExist.clone()
}

pub fn err_closed() -> Arc<Mutex<Option<Box<dyn StdError + Send + Sync>>>> {
    internal_oserror::ErrClosed.clone()
}

pub(crate) fn __go_init_functions() {
}


pub(crate) fn __go_init_all() {
    self::__go_init_globals();
}


impl GoValueClone for PathError {
    fn go_value_clone(&self) -> Self {
        self.__go_value_clone()
    }
}
